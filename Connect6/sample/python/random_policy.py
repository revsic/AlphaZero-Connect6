"""Baseline for using pyconnect6 modules, example by random policy"""
import pyconnect6
import numpy as np

board_size = 15


class RandomPolicy:
    """ Random Policy for introducing pyconnect6 usage.

    Policy must have method `__call__(self, turn, board): (value, prob)`,
        where board is num_simulation by board_capacity size 2D list contains integers,
        where board_capacity = board_size ** 2
    turn represent current turn, { -1: Black, 0: None, 1: White }
    `board[i]` represent board states and each cell represent { -1: Black Stone, 0: Empty Cell, 1: White Stone }
    `value` is `num_simulation` size list contains floats, represent probability of winning.
    `prob` is num_simulation by board_capacity size 2D list contains floats,
        represent probability of choosing each cell.
    """
    def __call__(self, turn, board):
        size = len(board)
        value = np.random.rand(size)
        rand_policy = np.random.rand(size, board_size * board_size)
        return value, rand_policy


policy = RandomPolicy()
param = pyconnect6.default_param()
param['num_simulation'] = 2
param['num_game_thread'] = 1
param['debug'] = True

# pass policy to pyconnect6.self_play,
# and connect6::self_play method will be use given policy to make choice
winner, path = pyconnect6.self_play(policy, param)
print('winner {}, len {}'.format(winner, len(path)))
