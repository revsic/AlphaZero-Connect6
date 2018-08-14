import json
import pyconnect6.connect6


def self_play(policy, param=None):
    if param is None:
        param = default_param()
    return connect6.self_play(policy, *param_to_tuple(param))


def play_with(policy, param=None):
    if param is None:
        param = default_param()
    return connect6.play_with(policy, *param_to_tuple(param)[:-2])


def default_param():
    return {
        'num_simulation': 800,
        'num_expansion': 1,
        'epsilon': 0.25,
        'dirichlet_alpha': 0.03,
        'c_puct': 1,
        'debug': False,
        'num_game_thread': 1,
    }


def param_to_tuple(param):
    return param['num_simulation'],\
           param['num_expansion'],\
           param['epsilon'],\
           param['dirichlet_alpha'],\
           param['c_puct'],\
           param['debug'],\
           param['num_game_thread']


def dump_param(path, param=None):
    if param is None:
        param = default_param()
    with open(path + '_mcts.json', 'w') as f:
        dump = json.dumps(param)
        f.write(dump)


def load_param(path):
    with open(path + '_mcts.json') as f:
        return json.loads(f.read())
