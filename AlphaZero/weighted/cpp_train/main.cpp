#include "connect6.hpp"
#include "model.hpp"

Model model("../weighted_policy.pb");

void callback(int player, float* values, float* policies, int len) {
    using Connect6::BOARD_SIZE;
    using Connect6::BOARD_CAPACITY;

    tensorflow::Tensor player_tensor(tensorflow::DT_FLOAT, tensorflow::TensorShape({ len }));
    auto& vec = player_tensor.vec<float>();
    for (int i = 0; i < len; ++i) {
        vec(i) = player;
    }

    tensorflow::Tensor board_tensor(tensorflow::DT_FLOAT, tensorflow::TensorShape({ len, BOARD_CAPACITY }));
    auto& flat = board_tensor.flat<float>();
    for (int i = 0; i < len * BOARD_CAPACITY; ++i) {
        flat(i) = policies[i];
    }

    std::vector<tensorflow::Tensor> res = model.Inference(player_tensor, board_tensor);
    auto const& value_res = res[0].vec<float>();
    auto const& policy_res = res[1].flat<float>();

    for (int i = 0; i < len; ++i) {
        values[i] = value_res(i);
    }

    for (int i = 0; i < len * BOARD_CAPACITY; ++i) {
        policies[i] = policy_res(i);
    }
}

int main() {
    model.Init();
    Connect6::self_play(callback, 2, 0.25, 0.03, 1, true, 1);
}