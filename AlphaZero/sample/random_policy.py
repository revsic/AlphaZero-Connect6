import connect6
import numpy as np

class RandomPolicy:
    def __call__(self, board):
        size = len(board)
        value = np.random.rand(size)
        policy = np.random.rand(size, 12 * 12)
        return value, policy

policy = RandomPolicy()
def inference(board):
    return policy(board)

param = (
    800,    # num_simulation
    1,      # num_expansion
    0.25,   # epsilon
    0.03,   # dirichlet alpha
    1,      # c_puct
    True,   # debug
)
winner, path = connect6.with_param(inference, *param)
print('winner {}, len {}'.format(winner, len(path)))