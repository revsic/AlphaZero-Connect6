#include "../connect6_dec.hpp"
#include "catch2/catch.hpp"
#include <random>

void cppeval_callback(int player, float* values, float* policies, int len_) {
    using Connect6::BOARD_SIZE;
    using Connect6::BOARD_CAPACITY;

    size_t len = len_;    
    for (size_t i = 0; i < len; ++i) {
        values[i] = i + player;
    }

    for (size_t i = 0; i < len; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            for (size_t k = 0; k < BOARD_SIZE; ++k) {
                policies[i * BOARD_CAPACITY + j * BOARD_SIZE + k] *= 2;
            }
        }
    }
}

TEST_CASE("Echo CppEval", "[CppEval]") {
    using namespace Connect6_RustFFI;

    std::random_device rd;
    std::default_random_engine gen(rd());

    auto rand_player = [&]{ return gen() % 3 - 1; };

    int turn = static_cast<int>(rand_player());
    int len = gen() % 100 + 100;

    int* boards = new int[len * BOARD_CAPACITY];

    size_t idx = 0;
    for (int i = 0; i < len; ++i) {
        for (size_t r = 0; r < BOARD_SIZE; ++r) {
            for (size_t c = 0; c < BOARD_SIZE; ++c) {
                boards[idx++] = rand_player();
            }
        }
    }

    Test_FFI::VecFloat res = Test_FFI::test_echo_cppeval(turn, boards, len, cppeval_callback, &allocator<float>);
    REQUIRE(res.len == len + len * BOARD_CAPACITY);

    for (size_t i = 0; i < len; ++i) {
        REQUIRE(res.vec[i] == i + turn);
    }

    idx = 0;
    float* res_vec = res.vec + len;
    for (int i = 0; i < len; ++i) {
        for (size_t r = 0; r < BOARD_SIZE; ++r) {
            for (size_t c = 0; c < BOARD_SIZE; ++c) {
                REQUIRE(boards[idx] * 2 == res_vec[idx]);
                idx += 1;
            }
        }
    }
    delete[] res.vec;
}
