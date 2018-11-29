#ifndef MODEL_HPP
#define MODEL_HPP

#define NOMINMAX

#include <tensorflow/core/public/session.h>

struct ModelConfig {
    std::string init = "init";
    std::string train = "Momentum";
    std::string ckpt_filename = "save/Const:0";
    std::string save_ckpt = "save/control_dependency:0";
    std::string restore_ckpt = "save/restore_all";

    std::string plc_player = "plc_player";
    std::string plc_board = "plc_board";
    std::string plc_value = "plc_value";
    std::string plc_policy = "plc_policy";

    std::string value = "value";
    std::string policy = "policy";

    static ModelConfig DefaultConfig;
};
ModelConfig ModelConfig::DefaultConfig;

class Model {
public:
    Model(std::string const& graph_def_filename, ModelConfig const& config = ModelConfig::DefaultConfig) :
        m_sess(nullptr), m_config(config)
    {
        tensorflow::GraphDef graph_def;
        TF_CHECK_OK(tensorflow::ReadBinaryProto(
            tensorflow::Env::Default(),
            graph_def_filename, &graph_def)
        );
        m_sess.reset(tensorflow::NewSession(tensorflow::SessionOptions()));
        TF_CHECK_OK(m_sess->Create(graph_def));
    }

    void Init() {
        TF_CHECK_OK(m_sess->Run({}, {}, { m_config.init }, nullptr));
    }

    void Restore(std::string const& ckpt_prefix) {
        CkptOp(ckpt_prefix, m_config.restore_ckpt);
    }

    void Checkpoint(std::string const& ckpt_prefix) {
        CkptOp(ckpt_prefix, m_config.save_ckpt);
    }

    std::vector<tensorflow::Tensor> Inference(
        tensorflow::Tensor const& player,
        tensorflow::Tensor const& board)
    {
        std::vector<tensorflow::Tensor> out_tensors;
        TF_CHECK_OK(m_sess->Run(
            { { m_config.plc_player, player }, { m_config.plc_board, board } },
            { m_config.value, m_config.policy }, {}, &out_tensors));
        return out_tensors;
    }

    void TrainStep(
        tensorflow::Tensor const& turn,
        tensorflow::Tensor const& board,
        tensorflow::Tensor const& value,
        tensorflow::Tensor const& policy)
    {
        TF_CHECK_OK(m_sess->Run(
            {
                { m_config.plc_player, turn },
                { m_config.plc_board, board },
                { m_config.plc_value, value },
                { m_config.plc_policy, policy },
            },
            {}, { m_config.train }, nullptr));
    }

private:
    std::unique_ptr<tensorflow::Session> m_sess;
    ModelConfig m_config;

    void CkptOp(std::string const& ckpt_prefix, std::string const& op_name) {
        tensorflow::Tensor t(tensorflow::DT_STRING, tensorflow::TensorShape());
        t.scalar<std::string>()() = ckpt_prefix;
        TF_CHECK_OK(m_sess->Run({{ m_config.save_ckpt, t }}, {}, { op_name }, nullptr));
    }
};

#endif