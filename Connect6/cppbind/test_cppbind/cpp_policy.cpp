#include "decl/connect6.hpp"
#include "catch2/catch.hpp"
#include <random>

int row, col;
float global_board[Connect6::BOARD_SIZE][Connect6::BOARD_SIZE];

void cpppolicy_callback(float* board, int* position) {
    using Connect6::BOARD_SIZE;

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            REQUIRE(global_board[i][j] == board[i * BOARD_SIZE + j]);
        }
    }

    std::random_device rd;
    std::default_random_engine gen(rd());

    row = gen() % 1024;
    col = gen() % 1024;

    position[0] = row;
    position[1] = col;
}

TEST_CASE("Evaluate CppPolicy", "[CppPolicy]") {
    using namespace Connect6_RustFFI;

    std::random_device rd;
    std::default_random_engine gen(rd());

    auto rand_player = [&]{ return static_cast<int>(gen()) % 3 - 1; };

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            global_board[i][j] = static_cast<float>(rand_player());
        }
    }

    float* board_ptr = reinterpret_cast<float*>(global_board);
    Test_FFI::VecInt vec = Test_FFI::test_cpp_policy(board_ptr, cpppolicy_callback, &allocator<int>);
    REQUIRE(vec.len == 2);
    REQUIRE(vec.vec[0] == row);
    REQUIRE(vec.vec[1] == col);
}
