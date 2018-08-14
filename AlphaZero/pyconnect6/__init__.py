import pyconnect6.connect6


def self_play(policy):
    return connect6.self_play(policy)


def play_with(policy):
    return connect6.play_with(policy)


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


def with_param(policy, param):
    return connect6.with_param(policy,
                              param['num_simulation'],
                              param['num_expansion'],
                              param['epsilon'],
                              param['dirichlet_alpha'],
                              param['c_puct'],
                              param['debug'],
                              param['num_game_thread'])
