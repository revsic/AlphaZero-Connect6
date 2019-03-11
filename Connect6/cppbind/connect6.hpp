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

        // Vec cpp_play(PolicyCallback callback,
        //              AllocatorType<Path> alloc_path,
        //              AllocatorType<PlayResult> alloc_result,
        //              bool debug,
        //              int num_game_thread);

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

        int* GetBoard() {
            return board.get();
        }

        const int* GetBoard() const {
            return board.get();
        }

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

        size_t GetSize() const {
            return size;
        }

        Path& operator[](size_t idx) {
            return paths[idx];
        }

        const Path& operator[](size_t idx) const {
            return paths[idx];
        }

        Path* begin() {
            return paths.get();
        }

        const Path* begin() const {
            return paths.get();
        }

        const Path* cbegin() const {
            return paths.get();
        }

        Path* end() {
            return paths.get() + size;
        }

        const Path* end() const {
            return paths.get() + size;
        }

        const Path* cend() const {
            return paths.get() + size;
        }

        static std::vector<GameResult> from_vec(Connect6_RustFFI::Vec& result) {
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

        Param&& NumSimulation(int num_simulation) && {
            this->num_simulation = num_simulation;
            return std::move(*this);
        }

        Param&& Epsilon(float epsilon) && {
            this->epsilon = epsilon;
            return std::move(*this);
        }

        Param&& DirichletAlpha(double dirichlet_alpha) && {
            this->dirichlet_alpha = dirichlet_alpha;
            return std::move(*this);
        }

        Param&& CPuct(float c_puct) && {
            this->c_puct = c_puct;
            return std::move(*this);
        }

        Param&& Debug(bool debug) && {
            this->debug = debug;
            return std::move(*this);
        }

        Param&& NumGameThread(int num_game_thread) && {
            this->num_game_thread = num_game_thread;
            return std::move(*this);
        }
    };

    // std::vector<GameResult> play(PolicyCallback callback, bool debug, int num_game_thread)
    // {
    //     namespace FFI = Connect6_RustFFI;
    //     FFI::Vec result = FFI::cpp_play(
    //         callback,
    //         &FFI::allocator<FFI::Path>,
    //         &FFI::allocator<FFI::PlayResult>,
    //         debug,
    //         num_game_thread
    //     );

    //     return GameResult::from_vec(result);
    // }

    std::vector<GameResult> self_play(Callback callback, const Param& param)
    {
        namespace FFI = Connect6_RustFFI;
        FFI::Vec result = FFI::cpp_self_play(
                callback,
                &FFI::allocator<FFI::Path>,
                &FFI::allocator<FFI::PlayResult>,
                param.num_simulation,
                param.epsilon,
                param.dirichlet_alpha,
                param.c_puct,
                param.debug,
                param.num_game_thread);

        return GameResult::from_vec(result);
    }

    GameResult play_with(Callback callback, const Param& param)
    {
        namespace FFI = Connect6_RustFFI;
        FFI::PlayResult result = FFI::cpp_play_with(
            callback,
            &FFI::allocator<FFI::Path>,
            param.num_simulation,
            param.epsilon,
            param.dirichlet_alpha,
            param.c_puct);
        
        return GameResult(result);
    }
}

#endif