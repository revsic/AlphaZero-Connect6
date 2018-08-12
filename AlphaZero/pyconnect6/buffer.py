import numpy as np


class Buffer(object):
    def __init__(self, max_size, board_size, num_sample):
        self.max_size = max_size
        self.board_size = board_size
        self.num_sample = num_sample

        self.buffer = []

    def __len__(self):
        return len(self.buffer)

    def push_game(self, game_result):
        win, path = game_result

        for (_, board, pos) in path:
            row, col = pos
            pos = row * self.board_size + col
            self.buffer.append((win, board, pos))

        if len(self.buffer) > self.max_size:
            self.buffer = self.buffer[-self.max_size:]

    def sample(self):
        index_set = np.random.choice(len(self.buffer), self.num_sample, replace=False)
        values, boards, poses = [], [], []
        for i in index_set:
            value, board, pos = self.buffer[i]
            values.append(value)
            boards.append(board)
            poses.append(pos)

        return values, boards, poses
