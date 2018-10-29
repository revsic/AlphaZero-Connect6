#include <iostream>
#include <random>

constexpr size_t BOARD_SIZE = 15;
constexpr size_t BOARD_CAPACITY = BOARD_SIZE * BOARD_SIZE;

extern "C" {
    struct Result {
        float* value;
        float* policy;
    };

    using Callback = Result(*)(int player, int* boards[BOARD_SIZE][BOARD_SIZE], int len);

    struct Path {
        int turn;
        int board[BOARD_SIZE][BOARD_SIZE];
        int row;
        int col;
    };

    struct RunResult {
        int winner;
        Path* paths;
        int len;
    };

    struct Vec {
        Result* vec;
        int len;
    };

    Vec cpp_self_play(Callback callback,
                      int num_simulation,
                      float epsilon,
                      double dirichlet_alpha,
                      float c_puct,
                      bool debug,
                      int num_game_thread);
}

Result callback(int player, int* boards[BOARD_SIZE][BOARD_SIZE], int len) {
    std::random_device rd;
    std::default_random_engine gen(rd());

    std::uniform_real_distribution<float> dist(-1.0, 1.0);

    float* value = new float[len];
    for (size_t i = 0; i < len; ++i) {
        value[i] = dist(gen);
    }

    float* policy = new float[len * BOARD_CAPACITY];
    for (size_t i = 0; i < len * BOARD_CAPACITY; ++i) {
        policy[i] = dist(gen);
    }

    return Result { value, policy };
}

int main() {
    Vec result = cpp_self_play(callback, 2, 0.25, 0.03, 1, true, 1);
    assert(result.len == 1);
    return 0;
}
