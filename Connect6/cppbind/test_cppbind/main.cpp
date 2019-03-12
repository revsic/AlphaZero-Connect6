#define CATCH_CONFIG_MAIN

#include "connect6.hpp"
#include "catch2/catch.hpp"
#include <random>

void main_callback(int player, float* values, float* policies, int len_) {
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

void main_policy(float* boards, int* position) {
    using Connect6::BOARD_SIZE;
    using Connect6::BOARD_CAPACITY;

    for (size_t i = 0; i < BOARD_CAPACITY; ++i) {
        if (boards[i] == 0) {
            int row = i / BOARD_SIZE;
            int col = i % BOARD_SIZE;

            position[0] = row;
            position[1] = col;
        }
    }
}

TEST_CASE("Check Connect6::self_play", "[Connect6]") {
    auto param = Connect6::Param()
        .NumSimulation(2)
        .NumGameThread(2);
    auto result = Connect6::self_play(main_callback, param);
    REQUIRE(result.size() == 2);
}

TEST_CASE("Check Connect6::play", "[Connect6]") {
    auto result = Connect6::play(main_policy, false, 2);
    REQUIRE(result.size() == 2);
}
