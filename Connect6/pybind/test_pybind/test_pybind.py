import pyconnect6
import numpy as np


class RandomPolicy:
    def __init__(self):
        self.board_size = pyconnect6.board_size()

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


def test_echo_pyeval():
    def gen_player(): return np.random.randint(3) - 1
    turn = gen_player()

    boards_len = 10
    board_size = pyconnect6.board_size()
    boards = [gen_player() for _ in range(boards_len * board_size ** 2)]

    def double_policy(turn, board):
        size = len(board)
        value = [i + turn for i in range(size)]
        board = [[i * 2 for i in x] for x in board]
        return value, board

    test_echo_pyeval = pyconnect6.pyconnect6.test_echo_pyeval
    value, policy = test_echo_pyeval(double_policy, turn, boards)

    idx = 0
    for i in range(boards_len):
        assert value[i] == i + turn
        for p in policy[i]:
            assert p == 2 * boards[idx]
            idx += 1
