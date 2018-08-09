import pyconnect6
import numpy as np

board_size = 15

class RandomPolicy:
    def __call__(self, board):
        size = len(board)
        value = np.random.rand(size)
        policy = np.random.rand(size, board_size * board_size)
        return value, policy

policy = RandomPolicy()
param = pyconnect6.default_param()
param['debug'] = True

winner, path = pyconnect6.with_param(policy, param)
print('winner {}, len {}'.format(winner, len(path)))