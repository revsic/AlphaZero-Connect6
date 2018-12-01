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

TEST_CASE("RawPath::new", "[RawPath]") {
    using namespace Connect6_RustFFI;

    Path path = Test_FFI::test_new_raw_path();
    REQUIRE(path.turn == 0);
    REQUIRE(path.row == 0);
    REQUIRE(path.col == 0);

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            REQUIRE(path.board[i][j] == 0);
        }
    }
}

TEST_CASE("RawPath::with_path", "[RawPath]") {
    using namespace Connect6_RustFFI;

    Path path = Test_FFI::test_with_raw_path();
    REQUIRE(path.turn == static_cast<int>(Connect6::Player::White));
    REQUIRE(path.row == 0);
    REQUIRE(path.col == BOARD_SIZE % 5 + 1);

    for (size_t i = 0; i < BOARD_SIZE; ++i) {
        for (size_t j = 0; j < BOARD_SIZE; ++j) {
            REQUIRE(path.board[i][j] == static_cast<int>(i * BOARD_SIZE + j) % 3 - 1);
        }
    }
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

TEST_CASE("RawPlayResult::with_result", "[RawPlayResult]") {
    using namespace Connect6_RustFFI;

    PlayResult res = Test_FFI::test_with_raw_play_result(&allocator<Path>);

    REQUIRE(res.len == 10);
    REQUIRE(res.winner == static_cast<int>(Connect6::Player::Black));

    for (size_t i = 0; i < 10; ++i) {
        REQUIRE(res.paths[i].turn == (i % 2 == 0 ? -1 : 1));
        REQUIRE(res.paths[i].row == i);
        REQUIRE(res.paths[i].col == i + 1);

        for (size_t j = 0; j < i + 1; ++j) {
            REQUIRE(res.paths[i].board[j][j] == static_cast<int>(i + j) % 3 - 1);
        }
    }
    delete[] res.paths;
}

TEST_CASE("Echo RawPlayResult", "[RawPlayResult]") {
    using namespace Connect6_RustFFI;

    std::random_device rd;
    std::default_random_engine gen(rd());

    auto rand_player = [&]{ return gen() % 3 - 1; };
    auto rand_position = [&]{ return gen() % BOARD_SIZE; };

    int winner = rand_player();
    int len = gen() % 150 + 100;

    Path* paths = new Path[len];
    for (size_t i = 0; i < len; ++i) {
        paths[i].turn = rand_player();
        paths[i].row = rand_position();
        paths[i].col = rand_position();

        for (size_t r = 0; r < BOARD_SIZE; ++r) {
            for (size_t c = 0; c < BOARD_SIZE; ++c) {
                paths[i].board[r][c] = 0;
            }
        }

        int iter_len = gen() % BOARD_CAPACITY;
        for (size_t j = 0; j < iter_len; ++j) {
            size_t row = rand_position();
            size_t col = rand_position();
            paths[i].board[row][col] = rand_player();
        }
    }

    PlayResult res = Test_FFI::test_echo_raw_play_result(winner, paths, len, &allocator<Path>);
    REQUIRE(res.winner == winner);
    REQUIRE(res.len == len);

    for (size_t i = 0; i < len; ++i) {
        REQUIRE(res.paths[i].turn == paths[i].turn);
        REQUIRE(res.paths[i].row == paths[i].row);
        REQUIRE(res.paths[i].col == paths[i].col);

        for (size_t r = 0; r < BOARD_SIZE; ++r) {
            for (size_t c = 0; c < BOARD_SIZE; ++c) {
                REQUIRE(res.paths[i].board[r][c] == paths[i].board[r][c]);
            }
        }
    }
    delete[] res.paths;
}

TEST_CASE("RawVec::with_vec", "[RawVec]") {
    using namespace Connect6_RustFFI;

    Test_FFI::VecInt res = Test_FFI::test_with_raw_vec(&allocator<int>);

    REQUIRE(res.len ==  6);
    for (size_t i = 0; i < 6; ++i) {
        REQUIRE(res.vec[i] == i);
    }
    delete[] res.vec;
}

TEST_CASE("Echo RawVec", "[RawVec]") {
    using namespace Connect6_RustFFI;

    std::random_device rd;
    std::default_random_engine gen(rd());

    int len = gen() % 100 + 50;
    int* ptr = new int[len];

    for (size_t i = 0; i < len; ++i) {
        ptr[i] = gen() % 100 + 50;
    }

    Test_FFI::VecInt res = Test_FFI::test_echo_raw_vec(ptr, len, &allocator<int>);

    REQUIRE(res.len == len);
    for (size_t i = 0; i < len; ++i) {
        REQUIRE(res.vec[i] == ptr[i]);
    }
    
    delete[] res.vec;
    delete[] ptr;
}

void callback_x2(int player, float* values, float* policies, int len_) {
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

    Test_FFI::VecFloat res = Test_FFI::test_echo_cppeval(turn, boards, len, callback_x2, &allocator<float>);
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
