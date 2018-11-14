#include <iostream>
#include <random>

constexpr size_t BOARD_SIZE = 15;
constexpr size_t BOARD_CAPACITY = BOARD_SIZE * BOARD_SIZE;

using Callback = void(*)(int player, float* values, float* policies, int len);

template <typename T>
T* allocator(int size) {
    return new T[size];
}

template <typename T>
using AllocatorType = T*(*)(int size);

extern "C" {
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
        RunResult* vec;
        int len;
    };

    Vec cpp_self_play(Callback callback,
                      AllocatorType<Path> alloc_path,
                      AllocatorType<RunResult> alloc_result,
                      int num_simulation,
                      float epsilon,
                      double dirichlet_alpha,
                      float c_puct,
                      bool debug,
                      int num_game_thread);
}

void callback(int player, float* values, float* policies, int len) {
    std::random_device rd;
    std::default_random_engine gen(rd());

    std::uniform_real_distribution<float> dist(-1.0, 1.0);

    for (size_t i = 0; i < len; ++i) {
        values[i] = dist(gen);
    }

    for (size_t i = 0; i < len; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            for (size_t k = 0; k < BOARD_SIZE; ++k) {
                policies[i * BOARD_CAPACITY + j * BOARD_SIZE + k] = dist(gen);
            }
        }
    }
}

int main() {
    Vec result_vec = cpp_self_play(
            callback,
            &allocator<Path>,
            &allocator<RunResult>,
            2, 0.25, 0.03, 1, true, 1);

    RunResult result = result_vec.vec[0];
    std::cout << "Winner : " << (result.winner < 0 ? "Black" : "White") << std::endl;

    for (size_t i = 0; i < result_vec.len; ++i) {
        delete[] result_vec.vec[i].paths;
    }

    delete[] result_vec.vec;
    return 0;
}
