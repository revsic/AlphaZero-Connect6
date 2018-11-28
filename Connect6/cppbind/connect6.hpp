#ifndef CONNECT6_H
#define CONNECT6_H

#include <cstring>
#include <memory>
#include <string>
#include <tuple>
#include <vector>

namespace Connect6_RustFFI {
    constexpr size_t BOARD_SIZE = 15;
    constexpr size_t BOARD_CAPACITY = BOARD_SIZE * BOARD_SIZE;

    using Callback = void(*)(int player, float* values, float* policies, int len);

    template <typename T>
    using AllocatorType = T*(*)(int size);

    template <typename T>
    T* allocator(int size) {
        return new T[size];
    }

    extern "C" {
        struct Path {
            int turn;
            int board[BOARD_SIZE][BOARD_SIZE];
            int row;
            int col;
        };

        struct PlayResult {
            int winner;
            Path* paths;
            int len;
        };

        struct Vec {
            PlayResult* vec;
            int len;
        };

        Vec cpp_self_play(Callback callback,
                          AllocatorType<Path> alloc_path,
                          AllocatorType<PlayResult> alloc_result,
                          int num_simulation,
                          float epsilon,
                          double dirichlet_alpha,
                          float c_puct,
                          bool debug,
                          int num_game_thread);
    }

    namespace Test_FFI {
        extern "C" {
            Path test_new_raw_path();
            Path test_with_raw_path();
            Path test_echo_raw_path(int turn, int* board, int row, int col);

            PlayResult test_with_raw_play_result(AllocatorType<Path> allocator);
            PlayResult test_echo_raw_play_result(int winner, Path* path, int len, AllocatorType<Path> allocator);

            struct VecInt {
                int* vec;
                int len;
            };

            VecInt test_with_raw_vec(AllocatorType<int> allocator);
            VecInt test_echo_raw_vec(int* ptr, int len, AllocatorType<int> allocator);

            struct VecFloat {
                float* vec;
                int len;
            };

            VecFloat test_echo_cppeval(int turn, int* boards, int len, Callback callback, AllocatorType<float> allocator);
        }
    }
}

namespace Connect6 {
    using Connect6_RustFFI::BOARD_SIZE;
    using Connect6_RustFFI::BOARD_CAPACITY;

    using Connect6_RustFFI::Callback;

    enum class Player : int {
        Black = -1,
        None = 0,
        White = 1,
    };

    std::string to_string(Player player) {
        switch (player) {
        case Player::Black:
            return { "Black" };
        case Player::White:
            return { "White" };
        case Player::None:
            return { "None" };
        }
        return { "" };
    }

    class Path {
    public:
        Path() : turn(Player::None), position(std::make_tuple(0, 0)), board(nullptr) {
            // Do Nothing
        }

        Path(Player turn, 
             const std::tuple<size_t, size_t>& position, 
             int board_[BOARD_SIZE][BOARD_SIZE]) :
            turn(turn), position(position), 
            board(std::make_unique<int[]>(BOARD_CAPACITY))
        {
            std::memcpy(board.get(), board_, BOARD_CAPACITY);
        }

        Path(const Connect6_RustFFI::Path& path) :
            turn(static_cast<Player>(path.turn)), 
            position(std::make_tuple(path.row, path.col)),
            board(std::make_unique<int[]>(BOARD_CAPACITY))
        {
            std::memcpy(board.get(), path.board, BOARD_CAPACITY);
        }

        Path(const Path&) = delete;
        Path(Path&& other) : 
            turn(other.turn), position(other.position), board(std::move(other.board))
        {
            // Do Nothing
        }

        Path& operator=(const Path&) = delete;
        Path& operator=(Path&& other) {
            turn = other.turn;
            position = other.position;
            board = std::move(other.board);
            return *this;
        }

        Player GetTurn() const {
            return turn;
        }

        const std::tuple<size_t, size_t>& GetPos() const {
            return position;
        };

        int* operator[](size_t idx) {
            return &board[idx * BOARD_SIZE];
        }

        const int* operator[](size_t idx) const {
            return &board[idx * BOARD_SIZE];
        }

    private:
        Player turn;
        std::tuple<size_t, size_t> position;
        std::unique_ptr<int[]> board;
    };

    class GameResult {
    public:
        GameResult() : winner(Player::None), size(0), paths(nullptr) {
            // Do Nothing
        }

        GameResult(Player winner, size_t size, std::unique_ptr<Path[]>&& paths) :
            winner(winner), size(size), paths(std::move(paths))
        {
            // Do Nothing
        }

        GameResult(const Connect6_RustFFI::PlayResult& run_result) :
            winner(static_cast<Player>(run_result.winner)),
            size(run_result.len),
            paths(std::make_unique<Path[]>(size))
        {
            for (size_t i = 0; i < size; ++i) {
                paths[i] = Path(run_result.paths[i]);
            }
        }

        Player GetWinner() const {
            return winner;
        }

        const Path& operator[](size_t idx) const {
            return paths[idx];
        }

        size_t GetSize() const {
            return size;
        }

        Path* begin() {
            return paths.get();
        }

        const Path* begin() const {
            return paths.get();
        }

        Path* end() {
            return paths.get() + size;
        }

        const Path* end() const {
            return paths.get() + size;
        }

    private:
        Player winner;

        size_t size;
        std::unique_ptr<Path[]> paths;
    };

    std::vector<GameResult> self_play(Callback callback,
                                      int num_simulation,
                                      float epsilon,
                                      double dirichlet_alpha,
                                      float c_puct,
                                      bool debug,
                                      int num_game_thread)
    {
        namespace FFI = Connect6_RustFFI;
        FFI::Vec result = FFI::cpp_self_play(
                callback,
                &FFI::allocator<FFI::Path>,
                &FFI::allocator<FFI::PlayResult>,
                num_simulation,
                epsilon,
                dirichlet_alpha,
                c_puct,
                debug,
                num_game_thread);

        size_t len = result.len;

        std::vector<GameResult> game_result;
        game_result.reserve(len);

        for (size_t i = 0; i < len; ++i) {
            game_result.emplace_back(result.vec[i]);
            delete[] result.vec[i].paths;
        }
        delete[] result.vec;
        return game_result;
    }
}


#endif