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
        """push game result to the buffer, each element consist of (player, board, position)"""
        win, path = game_result
        for (player, board, pos) in path:
            row, col = pos
            pos = row * self.board_size + col
            self.buffer.append((player, board, pos))

        if len(self.buffer) > self.max_size:
            self.buffer = self.buffer[-self.max_size:]

    def sample(self):
        """sample from buffer, return `self.num_sample` values"""
        index_set = np.random.choice(len(self.buffer), self.num_sample, replace=False)
        values, boards, poses = [], [], []
        for i in index_set:
            value, board, pos = self.buffer[i]
            values.append(value)
            boards.append(board)
            poses.append(pos)

        return values, boards, poses

    def clear_half(self):
        length = len(self.buffer) // 2
        self.buffer = self.buffer[length:]
