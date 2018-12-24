#include "model.hpp"
#include "ReplayBuffer.hpp"

#include <connect6.hpp>
#include <cxxopts.hpp>
#include <nlohmann/json.hpp>

#include <chrono>
#include <filesystem>
#include <iostream>

WeightedPolicy model;
torch::Device device(torch::kCPU);

void callback(int player, float* values, float* policies, int len) {
    using Connect6::BOARD_SIZE;
    using Connect6::BOARD_CAPACITY;

    torch::Tensor player_tensor = torch::empty(len, torch::kFloat32).to(device);
    player_tensor.fill_(static_cast<float>(player));

    torch::Tensor board_tensor = 
        torch::from_blob(policies, { len, static_cast<int>(BOARD_CAPACITY) }, torch::kFloat32).to(device);

    auto[policy_res, value_res] = model.forward(player_tensor, board_tensor);

    float* value_ptr = value_res.to(torch::kCPU).data<float>();
    for (size_t i = 0; i < len; ++i) {
        values[i] = value_ptr[i];
    }

    float* policy_ptr = policy_res.to(torch::kCPU).data<float>();
    for (size_t i = 0; i < len * BOARD_CAPACITY; ++i) {
        policies[i] = policy_ptr[i];
    }
}

template <typename... T>
void log(T&&... msg) {
    auto now = std::chrono::system_clock::now();
    auto time = std::chrono::system_clock::to_time_t(now);

    std::cout << std::put_time(std::localtime(&time), "[%Y-%m-%d %H:%M:%S] ");
    (std::cout << ... << std::forward<T>(msg)) << std::endl;
}

void dump_param(const std::string& path, const Connect6::Param& param) {
    std::ofstream(path + ".json") << nlohmann::json {
        { "num_simulation", param.num_simulation },
        { "epsilon", param.epsilon },
        { "dirichlet_alpha", param.dirichlet_alpha },
        { "c_puct", param.c_puct },
        { "debug", param.debug },
        { "num_game_thread", param.num_game_thread }
    };
}

Connect6::Param load_param(const std::string& path) {
    nlohmann::json loaded;
    std::ifstream(path + ".json") >> loaded;
    return Connect6::Param()
        .NumSimulation(loaded["num_simulation"])
        .Epsilon(loaded["epsilon"])
        .DirichletAlpha(loaded["dirichlet_alpha"])
        .CPuct(loaded["c_puct"])
        .Debug(loaded["debug"])
        .NumGameThread(loaded["num_game_thread"]);
}

void test() {
    torch::manual_seed(0);

    torch::NoGradGuard no_grad;
    model.eval();

    auto param = Connect6::Param()
        .NumSimulation(2)
        .Debug(true)
        .NumGameThread(11);
    Connect6::self_play(callback, param);
}

void train(const cxxopts::ParseResult& result) {
    namespace fs = std::filesystem;

    model.train();

    int load_ckpt = result["load_ckpt"].as<int>();
    std::string name = result["name"].as<std::string>();
    fs::path ckpt_path = fs::path(result["ckpt_dir"].as<std::string>()) / name;

    Connect6::Param param;
    int num_game_thread = result["num_game_thread"].as<int>();

    if (load_ckpt > 0 && fs::exists(ckpt_path)) {
        param = load_param(ckpt_path.string());
        torch::load(model, ckpt_path.string() + std::to_string(load_ckpt) + ".pt");
    }
    else {
        param.num_simulation = result["num_simulation"].as<int>();
        param.num_game_thread = num_game_thread;
    }

    torch::optim::SGD optimizer(
        model.parameters(), 
        torch::optim::SGDOptions(result["lr"].as<float>()).momentum(result["momentum"].as<float>()));
    
    ReplayBuffer buffer(result["max_buffer"].as<int>(), result["mini_batch"].as<int>());

    int num_game = 0;
    int epoch = load_ckpt;

    int start_train = result["start_train"].as<int>();
    int batch_size = result["batch_size"].as<int>();
    int ckpt_interval = result["ckpt_inverval"].as<int>();

    while (true) {
        num_game += num_game_thread;
        auto results = Connect6::self_play(callback, param);
        for (Connect6::GameResult& res : results) {
            buffer.push_game(std::move(res));
        }

        log("self-play async game#", num_game);

        if (buffer.size() > start_train) {
            epoch += 1;
            for (int i = 0; i < batch_size; ++i) {
                auto[winners, players, boards, poses] = buffer.sample(device);
                
                optimizer.zero_grad();
                auto loss = model.loss(winners, players, boards, poses);
                loss.backward();
                optimizer.step();
            }

            // TODO : create summary (result["summary_dir"], result["name"])

            if (epoch % ckpt_interval == 0) {
                torch::save(model, ckpt_path.string() + std::to_string(epoch) + ".pt");
                dump_param(ckpt_path.string(), param);
                log("ckpt saved");
            }

            log("epoch#", epoch);
        }
    }
}

void play(const cxxopts::ParseResult& result) {

}

int main(int argc, char* argv[]) {
    cxxopts::Options options("cpp_weighted", "torch c++ implementation of combined mcts policy");
    options.add_options()
        ("test", "run cpp_weighted in test mode")
        ("train", "run cpp_weighted in train mode")
        ("play", "play with cpp_weighted")
        ("c, cuda", "use cuda for torch")
        ("lr", "float, learning rate, default 1e-3", cxxopts::value<float>()->default_value("1e-3"))
        ("momentum", "float, momentum, default 0.9", cxxopts::value<float>()->default_value("0.9"))
        ("num_simulation", "int, number of simulation in mcts, default 800", cxxopts::value<int>()->default_value("800"))
        ("num_game_thread", "int, number of game threads, default 11", cxxopts::value<int>()->default_value("11"))
        ("max_buffer", "int, max size of buffer, default 100000", cxxopts::value<int>()->default_value("100000"))
        ("start_train", "int, start train when sizeof buffer over given, default 40000", cxxopts::value<int>()->default_value("40000"))
        ("batch_size", "int, size of batch, default 1024", cxxopts::value<int>()->default_value("1024"))
        ("mini_batch", "int, size of mini-batch, default 2048", cxxopts::value<int>()->default_value("2048"))
        ("ckpt_interval", "int, interval for writing checkpoint, default 10", cxxopts::value<int>()->default_value("10"))
        ("load_ckpt", "int, load ckpt with given epoch, if zero, train new, default 0", cxxopts::value<int>()->default_value("0"))
        ("name", "string, name of model, default weighted", cxxopts::value<std::string>()->default_value("weighted"))
        ("summary_dir", "string, dirname for saving summary, default ./summary", cxxopts::value<std::string>()->default_value("./summary"))
        ("ckpt_dir", "string, dirname for saving checkpoint, default ./ckpt_dir", cxxopts::value<std::string>()->default_value("./ckpt_dir"))
    ;

    auto result = options.parse(argc, argv);
    if (result.count("cuda") > 0 && torch::cuda::is_available()) {
        std::cout << "Cuda available, run on cuda" << std::endl;
        device = torch::Device(torch::kCUDA);
    }
    model.to(device);

    if (result.count("test")) {
        test();
    }

    if (result.count("train")) {
        train(result);
    }

    if (result.count("play")) {
        play(result);
    }

    return 0;
}