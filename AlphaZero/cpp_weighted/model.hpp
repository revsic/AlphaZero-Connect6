#ifndef MODEL_HPP
#define MODEL_HPP

#include "connect6.hpp"
#include <torch/torch.h>

struct WeightedPolicy : torch::nn::Module {
    WeightedPolicy() : 
        policy_fc(Connect6::BOARD_CAPACITY + 1, Connect6::BOARD_CAPACITY),
        value_fc(Connect6::BOARD_CAPACITY + 1, 1)
    {
        register_module("policy_fc", policy_fc);
        register_module("value_fc", value_fc);
    }

    auto forward(torch::Tensor player, torch::Tensor board) {
        player = player.view({-1, 1});
        torch::Tensor repr = torch::cat({ player, board }, /*dim=*/1);

        torch::Tensor policy = policy_fc->forward(repr);
        policy = torch::softmax(policy, /*dim=*/1);

        torch::Tensor value = value_fc->forward(repr);
        value = torch::tanh(value);

        return std::make_tuple(policy, value.view({ -1, }));
    }

    torch::Tensor loss(torch::Tensor winner, 
                       torch::Tensor player, 
                       torch::Tensor board, 
                       torch::Tensor policy) 
    {
        auto[inf_policy, value] = forward(player, board);
        torch::Tensor value_loss = torch::mean((winner - value).pow(2));
        torch::Tensor policy_loss = -torch::mean(torch::sum(policy * torch::log(inf_policy), -1));
        return value_loss + policy_loss;
    }

    torch::nn::Linear policy_fc;
    torch::nn::Linear value_fc;
};

#endif