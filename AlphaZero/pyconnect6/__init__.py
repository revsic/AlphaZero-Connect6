import json
import pyconnect6.connect6


def self_play(policy, param=None):
    """python wrapper for connect6::self_play

    Args:
        policy: callable object, connect6::self_play will play game with given policy to make choice
            IT MUST CONTAIN METHOD `__call__(self, turn, board): (value, prob)`,
            because connect6::self_play call object with cpython and get proper values from policy.

            turn: current turn, { -1: Black, 0: None, 1: White }
            board: param['num_simulation'] by board_capacity size 2D list, where board_capacity = board_size ** 2
                each board[i] represent board status reshaped as first dimension.
                each cell represent { -1: Black Stone Exist, 0: Empty Cell, 1: White Stone Exist }
            value: param['num_simulation'] size list, represent probability of winning at each expansion
            prob: param['num_simulation'] by board_capacity size 2D list
                represent probability of choosing each cell

        param: hyperparameter for playing combined mcts, reference `pyconnect6.default_param()`.

    Return tuple(winner, play_result):
        winner: int, winner of game { -1: Black, 0: Draw, 1: White }
        player_result: list, in-game data produced by self-play, each cell consists of (turn, board, choice)
    """
    if param is None:
        param = default_param()
    return connect6.self_play(policy, *param_to_tuple(param))


def play_with(policy, param=None):
    """python wrapper for connect6::play_with

    Play connect6 with given policy.
    User can input at white turn, format like "aA", "bS".
    """
    if param is None:
        param = default_param()
    return connect6.play_with(policy, *param_to_tuple(param)[:-2])


def default_param():
    """create default parameter base on connect6::pybind::HyperParameter"""
    return {
        'num_simulation': 800,      # number of mcts simulaton for each turn
        'num_expansion': 1,         # number of node expansion for each simulation
        'epsilon': 0.25,            # ratio for adding random probability from dirichlet distribution
        'dirichlet_alpha': 0.03,    # parameter of dirichlet distribution
        'c_puct': 1,                # parameter for puct (metamorphism of upper confidence tree algorithm)
        'debug': False,             # if debug, debug info from connect6 will be printed
        'num_game_thread': 1,       # number of thread run game asynchronously
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
    """dump parameter for later use"""
    if param is None:
        param = default_param()
    with open(path + '_mcts.json', 'w') as f:
        dump = json.dumps(param)
        f.write(dump)


def load_param(path):
    with open(path + '_mcts.json') as f:
        return json.loads(f.read())
