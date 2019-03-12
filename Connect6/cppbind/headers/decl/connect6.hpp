#ifndef CONNECT6_DEC_H
#define CONNECT6_DEC_H

#include <cstring>
#include <memory>
#include <string>
#include <tuple>
#include <vector>

namespace Connect6_RustFFI {
    constexpr size_t BOARD_SIZE = 15;
    constexpr size_t BOARD_CAPACITY = BOARD_SIZE * BOARD_SIZE;

    using Callback = void(*)(int player, float* values, float* policies, int len);
    using PolicyCallback = void(*)(float* boards, int* position);

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

        Vec cpp_play(PolicyCallback callback,
                     AllocatorType<Path> alloc_path,
                     AllocatorType<PlayResult> alloc_result,
                     bool debug,
                     int num_game_thread);

        Vec cpp_self_play(Callback callback,
                          AllocatorType<Path> alloc_path,
                          AllocatorType<PlayResult> alloc_result,
                          int num_simulation,
                          float epsilon,
                          double dirichlet_alpha,
                          float c_puct,
                          bool debug,
                          int num_game_thread);

        PlayResult cpp_play_with(Callback callback,
                                 AllocatorType<Path> alloc_path,
                                 int num_simulation,
                                 float epsilon,
                                 double dirichlet_alpha,
                                 float c_puct);
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
            VecInt test_cpp_policy(float* board, PolicyCallback callback, AllocatorType<int> allocator);
        }
    }
}

namespace Connect6 {
    using Connect6_RustFFI::BOARD_SIZE;
    using Connect6_RustFFI::BOARD_CAPACITY;

    using Connect6_RustFFI::Callback;
    using Connect6_RustFFI::PolicyCallback;

    enum class Player : int {
        Black = -1,
        None = 0,
        White = 1,
    };

    std::string to_string(Player player);

    class Path {
    public:
        Path();

        Path(Player turn, 
             const std::tuple<size_t, size_t>& position, 
             int board_[BOARD_SIZE][BOARD_SIZE]);

        Path(const Connect6_RustFFI::Path& path);

        Path(const Path&) = delete;
        Path(Path&& other);

        Path& operator=(const Path&) = delete;
        Path& operator=(Path&& other);

        Player GetTurn() const;

        const std::tuple<size_t, size_t>& GetPos() const;

        int* GetBoard();
        const int* GetBoard() const;

        int* operator[](size_t idx);
        const int* operator[](size_t idx) const;

    private:
        Player turn;
        std::tuple<size_t, size_t> position;
        std::unique_ptr<int[]> board;
    };

    class GameResult {
    public:
        GameResult();
        GameResult(Player winner, size_t size, std::unique_ptr<Path[]>&& paths);
        GameResult(const Connect6_RustFFI::PlayResult& run_result);

        Player GetWinner() const;

        size_t GetSize() const;

        Path& operator[](size_t idx);
        const Path& operator[](size_t idx) const;

        Path* begin();
        const Path* begin() const;
        const Path* cbegin() const;

        Path* end();
        const Path* end() const;
        const Path* cend() const;

        static std::vector<GameResult> from_vec(Connect6_RustFFI::Vec& result);

    private:
        Player winner;

        size_t size;
        std::unique_ptr<Path[]> paths;
    };

    struct Param {
        int num_simulation = 800;
        float epsilon = 0.25;
        double dirichlet_alpha = 0.03;
        float c_puct = 1;
        bool debug = false;
        int num_game_thread = 11;

        Param&& NumSimulation(int num_simulation) &&;
        Param&& Epsilon(float epsilon) &&;
        Param&& DirichletAlpha(double dirichlet_alpha) &&;
        Param&& CPuct(float c_puct) &&;
        Param&& Debug(bool debug) &&;
        Param&& NumGameThread(int num_game_thread) &&;
    };

    std::vector<GameResult> play(PolicyCallback callback, bool debug, int num_game_thread);

    std::vector<GameResult> self_play(Callback callback, const Param& param);

    GameResult play_with(Callback callback, const Param& param);
}

#endif