# AlphaZero-Connect6
[![License](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://github.com/revsic/AlphaZero-Connect6/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/revsic/AlphaZero-Connect6.svg?branch=master)](https://travis-ci.org/revsic/AlphaZero-Connect6/branches)

Tensorflow implementation of AlphaZero for Connect6 written in Rust, Python

- [Documentation](https://revsic.github.io/AlphaZero-Connect6)

Copyright (c) 2018 YoungJoong Kim.
AlphaZero-Connect6 is licensed under the [MIT license](http://opensource.org/licenses/MIT).

- Supported Python version for connect6 lib is dependent on [rust-cpython](https://github.com/dgrunwald/rust-cpython).
- Tensorflow impl of AlphaZero is based on python3.6

Suppose to build Connect6 at Rust 1.28.0 or later.

## Usage

To use lib `connect6`, first install rust compiler [rustup](https://rustup.rs).

```
python setup.py build; pyton setup.py install;
```

Then install connect6 with [setup.py](Connect6/setup.py).

```python
import pyconnect6
import numpy as np

board_size = 15
play_result = pyconnect6.self_play(
    lambda turn, board: np.random.rand(len(board)), np.random.rand(len(board), board_size ** 2)
    10,     # num_simulation
    1,      # num_expansion
    0.25,   # epsilon
    0.03,   # dirichlet_alpha
    1,      # c_puct
    True,   # debug
    1)      # num_game_thread

win, path = play_result
print(win)
```
