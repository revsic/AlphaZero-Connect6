#ifndef REPLAY_BUFFER_HPP
#define REPLAY_BUFFER_HPP

#include <connect6.hpp>
#include <torch/torch.h>

#include <algorithm>
#include <random>

class ReplayBuffer {
public:
    ReplayBuffer(size_t max_size, size_t num_sample) : 
        max_size(max_size),
        num_sample(num_sample), 
        tail(0), 
        num_data(0),
        paths(std::make_unique<std::tuple<Connect6::Player, Connect6::Path>[]>(max_size)) 
    {
        // Do Nothing
    }

    void push_game(Connect6::GameResult&& result) {
        Connect6::Player winner = result.GetWinner();
        num_data = std::min(num_data + result.GetSize(), max_size);
        for (Connect6::Path& path : result) {
            paths[tail] = std::make_tuple(winner, std::move(path));
            tail = (tail + 1) % max_size;
        }
    }

    void clear() {
        num_data = tail = 0;
    }

    size_t size() {
        return num_data;
    }

    auto sample() {
        int n_sample = static_cast<int>(num_sample);
        auto winners = torch::empty({ n_sample }, torch::kFloat32);
        auto players = torch::empty({ n_sample }, torch::kFloat32);
        auto boards = torch::empty({ n_sample, static_cast<int>(Connect6::BOARD_CAPACITY) }, torch::kFloat32);
        auto poses = torch::empty({ n_sample }, torch::kFloat32);

        std::random_device rd;
        std::default_random_engine gen(rd());

        for (size_t i = 0; i < num_sample; ++i) {
            auto&[win, path] = paths[gen() % num_data];
            winners[i] = static_cast<int>(win);
            players[i] = static_cast<int>(path.GetTurn());
            
            boards[i] = torch::from_blob(path.GetBoard(), { static_cast<int>(Connect6::BOARD_CAPACITY) }, torch::kFloat32);

            auto[row, col] = path.GetPos();
            poses[i] = static_cast<int>(row * Connect6::BOARD_SIZE + col);
        }

        return std::make_tuple(std::move(winners), std::move(players), std::move(boards), std::move(poses));
    }

private:
    size_t max_size;
    size_t num_sample;

    size_t tail;
    size_t num_data;
    std::unique_ptr<std::tuple<Connect6::Player, Connect6::Path>[]> paths;
};

#endif