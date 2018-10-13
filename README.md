# AlphaZero-Connect6
[![License](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://github.com/revsic/AlphaZero-Connect6/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/revsic/AlphaZero-Connect6.svg?branch=master)](https://travis-ci.org/revsic/AlphaZero-Connect6)
[![Build Status](https://dev.azure.com/revsic99/AlphaZero-Connect6/_apis/build/status/revsic.AlphaZero-Connect6)](https://dev.azure.com/revsic99/AlphaZero-Connect6/_build/latest?definitionId=1)
[![codecov](https://codecov.io/gh/revsic/AlphaZero-Connect6/branch/master/graph/badge.svg)](https://codecov.io/gh/revsic/AlphaZero-Connect6)

Tensorflow implementation of AlphaZero for Connect6 written in Rust, Python

- [Documentation](https://revsic.github.io/AlphaZero-Connect6)

Copyright (c) 2018 YoungJoong Kim.
AlphaZero-Connect6 is licensed under the [MIT license](http://opensource.org/licenses/MIT).

- Supported Python version for lib connect6 is dependent on [rust-cpython](https://github.com/dgrunwald/rust-cpython).
- Tensorflow impl of AlphaZero is based on python3.6

Suppose to build Connect6 at Rust 1.28.0 or later.

## Usage

Rust compiler is required to install lib pyconnect6. [rustup](https://rustup.rs)
```
curl https://sh.rustup.rs -sSf | sh  #for linux user
```
Install connect6 with [setup.py](Connect6/pybind/setup.py).
```
cd Connect6/pybind; python setup.py build; pyton setup.py install;
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
