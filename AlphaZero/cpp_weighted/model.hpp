#ifndef MODEL_HPP
#define MODEL_HPP

#include <torch/torch.h>

class Model : torch::nn::Module {
public:
    Model() {

    }

    torch::Tensor inference(torch::Tensor player, torch::Tensor board) {

    }

    void train(torch::Tensor winner, torch::Tensor player, torch::Tensor board, torch::Tensor policy) {

    }

private:
};

#endif