import numpy as np


class Buffer(object):
    """ Replay Buffer for Training AlphaZero

    Attributes:
        max_size: int, max size of buffer
        board_size: int, size of the board
        num_sample: int, number of replay data return from method `sample`
    """
    def __init__(self, max_size, board_size, num_sample):
        self.max_size = max_size
        self.board_size = board_size
        self.num_sample = num_sample

        self.buffer = []

    def __len__(self):
        return len(self.buffer)

    def push_game(self, game_result):
        """push game result to the buffer, each element consist of (winner, player, board, position)"""
        win, path = game_result
        for (player, board, pos) in path:
            row, col = pos
            pos = row * self.board_size + col
            self.buffer.append((win, player, board, pos))

        if len(self.buffer) > self.max_size:
            self.buffer = self.buffer[-self.max_size:]

    def sample(self):
        """sample from buffer, return `self.num_sample` values"""
        index_set = np.random.choice(len(self.buffer), self.num_sample, replace=False)
        wins, players, boards, policies = [], [], [], []
        for i in index_set:
            win, player, board, policy = self.buffer[i]
            wins.append(win)
            players.append(player)
            boards.append(board)
            policies.append(policy)

        return wins, players, boards, policies

    def clear_half(self):
        length = len(self.buffer) // 2
        self.buffer = self.buffer[length:]
