import pyconnect6
import numpy as np

class RandomPolicy:
    def __init__(self):
        self.board_size = 15

    def __call__(self, turn, board):
        size = len(board)
        value = np.random.rand(size)
        rand_policy = np.random.rand(size, self.board_size * self.board_size)
        return value, rand_policy

def test_result_length():
    policy = RandomPolicy()

    param = pyconnect6.default_param()
    param['num_simulation'] = 2
    param['num_game_thread'] = 2

    result = pyconnect6.self_play(policy, param)
    assert len(result) == 2
