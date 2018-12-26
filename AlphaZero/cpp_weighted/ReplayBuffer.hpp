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

    auto sample(const torch::Device& dev = torch::kCPU) {
        int n_sample = static_cast<int>(num_sample);
        auto winners = torch::empty({ n_sample }, torch::kFloat32);
        auto players = torch::empty({ n_sample }, torch::kFloat32);
        auto boards = torch::empty({ n_sample, static_cast<int>(Connect6::BOARD_CAPACITY) }, torch::kFloat32);
        auto poses = torch::empty({ n_sample }, torch::kFloat32);

        std::random_device rd;
        std::default_random_engine gen(rd());

        float* board_ptr = boards.data<float>();
        for (size_t i = 0; i < num_sample; ++i) {
            auto&[win, path] = paths[gen() % num_data];
            winners[i] = static_cast<int>(win);
            players[i] = static_cast<int>(path.GetTurn());
            
            int* ptr = path.GetBoard();
            for (size_t i = 0; i < Connect6::BOARD_CAPACITY; ++i) {
                *board_ptr++ = static_cast<float>(*ptr++);
            }

            auto[row, col] = path.GetPos();
            poses[i] = static_cast<int>(row * Connect6::BOARD_SIZE + col);
        }

        return std::make_tuple(winners.to(dev), players.to(dev), boards.to(dev), poses.to(dev));
    }

private:
    size_t max_size;
    size_t num_sample;

    size_t tail;
    size_t num_data;
    std::unique_ptr<std::tuple<Connect6::Player, Connect6::Path>[]> paths;
};

#endif