#ifndef REPLAY_BUFFER_HPP
#define REPLAY_BUFFER_HPP

#include "connect6.hpp"

#include <cstddef>

template <size_t MAX_SIZE>
class ReplayBuffer {
public:
    ReplayBuffer(size_t num_sample) : num_sample(num_sample), tail(0), num_data(0) {
        // Do Nothing
    }

    void push_game(Connect6::GameResult&& result) {
        Connect6::Player winner = result.GetWinner();
        for (auto iter = result.begin(); iter != result.end(); ++iter)
        {
            paths[tail] = std::make_tuple(winner, std::move(*iter));
            tail = (tail + 1) % MAX_SIZE;
            num_data = std::min(num_data + 1, MAX_SIZE);
        }
    }

    void clear() {
        num_data = tail = 0;
    }

    // TODO: Impl sample

private:
    size_t num_sample;

    size_t tail;
    size_t num_data;
    std::tuple<Player, Connect6::Path>[MAX_SIZE] paths;
};

#endif