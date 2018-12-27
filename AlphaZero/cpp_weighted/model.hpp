#ifndef MODEL_HPP
#define MODEL_HPP

#include <connect6.hpp>
#include <torch/torch.h>

#include <vector>

struct WeightedPolicy : torch::nn::Module {
    WeightedPolicy(size_t num_block) :
        num_block(num_block),
        conv(torch::nn::Conv2dOptions(2, 256, 3).padding(1)),
        norm(256),
        policy_conv(torch::nn::Conv2dOptions(256, 2, 1)),
        policy_norm(2),
        policy_linear(Connect6::BOARD_CAPACITY * 2, Connect6::BOARD_CAPACITY),
        value_conv(torch::nn::Conv2dOptions(256, 1, 1)),
        value_norm(Connect6::BOARD_CAPACITY),
        value_linear1(Connect6::BOARD_CAPACITY, 256),
        value_linear2(256, 1)
    {
        register_module("conv", conv);
        register_module("norm", norm);

        register_module("policy_conv", policy_conv);
        register_module("policy_norm", policy_norm);
        register_module("policy_linear", policy_linear);

        register_module("value_conv", value_conv);
        register_module("value_norm", value_norm);
        register_module("value_linear1", value_linear1);
        register_module("value_linear2", value_linear2);

        auto conv_macro = [this](size_t i, size_t j) {
            return register_module("conv" + std::to_string(i) + '_' + std::to_string(j),
                torch::nn::Conv2d(torch::nn::Conv2dOptions(256, 256, 3).padding(1)));
        };

        auto norm_macro = [this](size_t i, size_t j) {
            return register_module("norm" + std::to_string(i) + '_' + std::to_string(j),
                torch::nn::BatchNorm(256));
        };

        tower.reserve(num_block - 1);
        for (size_t i = 0; i < num_block - 1; ++i) {
            tower.emplace_back(std::make_tuple(
                conv_macro(i, 0), conv_macro(i, 1), norm_macro(i, 0), norm_macro(i, 1)
            ));
        }
    }

    auto forward(torch::Tensor player, torch::Tensor board) {
        int BOARD_CAPACITY = static_cast<int>(Connect6::BOARD_CAPACITY);
        int BOARD_SIZE = static_cast<int>(Connect6::BOARD_SIZE);

        player = player.view({-1, 1}).repeat({1, BOARD_CAPACITY}).view({-1, 1, BOARD_SIZE, BOARD_SIZE});
        torch::Tensor repr = torch::cat({ player, board.view({-1, 1, BOARD_SIZE, BOARD_SIZE}) }, 1);

        repr = torch::relu(norm->forward(conv->forward(repr)));

        auto res_block = [](torch::Tensor input, auto& block) {
            auto&[conv0, conv1, norm0, norm1] = block;

            torch::Tensor tensor = conv0->forward(input);
            tensor = norm0->forward(tensor);
            tensor = torch::relu(tensor);

            tensor = conv1->forward(tensor);
            tensor = norm1->forward(tensor);
            tensor = tensor + input;
            tensor = torch::relu(tensor);

            return tensor;
        };

        for (auto& block : tower) {
            repr = res_block(repr, block);
        }

        torch::Tensor policy = policy_conv->forward(repr);
        policy = policy_norm->forward(policy);
        policy = torch::relu(policy);
        policy = policy_linear->forward(policy.view({-1, BOARD_CAPACITY * 2}));
        policy = torch::softmax(policy, 1);

        torch::Tensor value = value_conv->forward(repr);
        value = value_norm->forward(value.view({-1, BOARD_CAPACITY}));
        value = torch::relu(value);
        value = value_linear1->forward(value);
        value = torch::relu(value);
        value = value_linear2->forward(value);
        value = torch::tanh(value);

        return std::make_tuple(policy, value);
    }

    torch::Tensor loss(torch::Tensor winner,
                       torch::Tensor player,
                       torch::Tensor board,
                       torch::Tensor policy,
                       const torch::Device& dev = torch::kCPU) 
    {
        auto[inf_policy, value] = forward(player, board);
        torch::Tensor value_loss = torch::mean((winner - value).pow(2));
        torch::Tensor one_hot = torch::zeros({policy.size(0), static_cast<int>(Connect6::BOARD_CAPACITY)}).to(dev).scatter_(1, policy.toType(torch::kLong).view({-1, 1}), 1);
        torch::Tensor policy_loss = -torch::mean(torch::sum(one_hot * torch::log(inf_policy + 1e-7), -1));
        return value_loss + policy_loss;   
    }

    size_t num_block;

    torch::nn::Conv2d conv;
    torch::nn::BatchNorm norm;

    torch::nn::Conv2d policy_conv;
    torch::nn::BatchNorm policy_norm;
    torch::nn::Linear policy_linear;

    torch::nn::Conv2d value_conv;
    torch::nn::BatchNorm value_norm;
    torch::nn::Linear value_linear1;
    torch::nn::Linear value_linear2;

    std::vector<
        std::tuple<
            torch::nn::Conv2d,
            torch::nn::Conv2d,
            torch::nn::BatchNorm,
            torch::nn::BatchNorm>> tower;
};

#endif