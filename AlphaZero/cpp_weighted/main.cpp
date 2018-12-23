#include "connect6.hpp"
#include "model.hpp"
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

int main(int argc, char* argv[]) {
    torch::manual_seed(0);

    if (torch::cuda::is_available()) {
        std::cout << "CUDA available! Training on GPU" << std::endl;
        device = torch::Device(torch::kCUDA);
    }
    model.to(device);

    torch::NoGradGuard no_grad;
    model.eval();

    Connect6::self_play(callback, 2, 0.25, 0.03, 1, true, 1);
}