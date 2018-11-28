#define CATCH_CONFIG_MAIN

#include "../connect6.hpp"
#include "catch.hpp"
#include <random>

void callback(int player, float* values, float* policies, int len_) {
    size_t len = len_;
    
    std::random_device rd;
    std::default_random_engine gen(rd());

    std::uniform_real_distribution<float> dist(-1.0, 1.0);

    for (size_t i = 0; i < len; ++i) {
        values[i] = dist(gen);
    }

    using Connect6::BOARD_SIZE;
    using Connect6::BOARD_CAPACITY;

    for (size_t i = 0; i < len; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            for (size_t k = 0; k < BOARD_SIZE; ++k) {
                policies[i * BOARD_CAPACITY + j * BOARD_SIZE + k] = dist(gen);
            }
        }
    }
}

TEST_CASE("Check result length", "[RandomPolicy]") {
    auto result = Connect6::self_play(callback, 2, 0.25, 0.03, 1, false, 2);
    REQUIRE(result.size() == 2);
}

TEST_CASE("Echo RawPath", "[RawPath]") {
    using namespace Connect6_RustFFI;

    std::random_device rd;
    std::default_random_engine gen(rd());

    auto rand_player = [&]{ return gen() % 3 - 1; };
    auto rand_position = [&]{ return gen() % BOARD_SIZE; };

    int turn = rand_player();
    int board[BOARD_SIZE][BOARD_SIZE] = { 0, };
    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            board[i][j] = rand_player();
        }
    }
    int row = rand_position();
    int col = rand_position();

    Path path = Test_FFI::test_echo_raw_path(turn, board[0], row, col);

    REQUIRE(turn == path.turn);
    REQUIRE(row == path.row);
    REQUIRE(col == path.col);

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            REQUIRE(board[i][j] == path.board[i][j]);
        }
    }
}

TEST_CASE("Sample RawPath", "[RawPath]") {
    using namespace Connect6_RustFFI;

    Path path = Test_FFI::test_sample_raw_path();

    REQUIRE(path.turn == static_cast<int>(Connect6::Player::Black));
    REQUIRE(path.row == 0);
    REQUIRE(path.col == BOARD_SIZE - 1);

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            REQUIRE(path.board[i][j] == i * BOARD_SIZE + j);
        }
    }
}