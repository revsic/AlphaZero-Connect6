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

To install lib `connect6`, rust compiler is required, [rustup](https://rustup.rs).
```
curl https://sh.rustup.rs -sSf | sh  #for linux user
```
Install connect6 with [setup.py](Connect6/setup.py).
```
cd Connect6; python setup.py build; pyton setup.py install;
```
Example program playing connect6 with random policy.

For more complicated example, reference [weighted](AlphaZero/weighted)
```python
import pyconnect6
import numpy as np

board_size = 15
param = pyconnect6.default_param()
param['num_simulation'] = 10
param['debug'] = True

policy = lambda turn, board: (np.random.rand(len(board)), np.random.rand(len(board), board_size ** 2))
play_result = pyconnect6.self_play(policy, param)

win, path = play_result
print(win)
```
