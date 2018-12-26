# AlphaZero-Connect6
[![License](https://img.shields.io/badge/Licence-MIT-blue.svg)](https://github.com/revsic/AlphaZero-Connect6/blob/master/LICENSE)
[![Build Status](https://travis-ci.org/revsic/AlphaZero-Connect6.svg?branch=master)](https://travis-ci.org/revsic/AlphaZero-Connect6)
[![Build Status](https://dev.azure.com/revsic99/AlphaZero-Connect6/_apis/build/status/revsic.AlphaZero-Connect6)](https://dev.azure.com/revsic99/AlphaZero-Connect6/_build/latest?definitionId=1)
[![codecov](https://codecov.io/gh/revsic/AlphaZero-Connect6/branch/master/graph/badge.svg)](https://codecov.io/gh/revsic/AlphaZero-Connect6)

AlphaZero training framework for game Connect6 written in Rust with C++, Python interface.

- [Documentation](https://revsic.github.io/AlphaZero-Connect6)

Copyright (c) 2018 YoungJoong Kim.
AlphaZero-Connect6 is licensed under the [MIT license](http://opensource.org/licenses/MIT).

- Suppose to build Connect6 at Rust 1.28.0 or later.
- Supported C++ standard is C++14 or later.
- Supported Python version for bindings of libconnect6 is dependent on [rust-cpython](https://github.com/dgrunwald/rust-cpython).
- Tensorflow implementation of AlphaZero is based on python3.

## Rust Usage

Install Rust compiler. [rustup](https://rustup.rs)
```
curl https://sh.rustup.rs -sSf | sh  #for linux user
curl -sSf -o rustup-init.exe https://win.rustup.rs && rustup-init.exe  #for windows user
```

Sample program play connect6 with AlphaZero policy and RandomEvaluator.
```
cd Connect6 && cargo run -p sample
```

## Python Usage

Install connect6 with [setup.py](Connect6/pybind/setup.py) (rust compiler is required).
```
cd Connect6/pybind; python setup.py build && pyton setup.py install;
```

For testing python bindness, use `pytest` at [test_pybind](Connect6/pybind/test_pybind).
```
cd Connect6/pybind/test_pybind; pytest;
```

Sample program play connect6 with AlphaZero policy and random evaluator.

For more complicated example, reference [py_weighted](AlphaZero/py_weighted)
```
cd Connect6/sample && python -m python.random_policy
```

## C++ Usage

Before using libconnect6 with C++ interface, it should be compiled in release mode.
```
cd Connect6 && cargo build --release
```

Single header [connect6.hpp](Connect6/cppbind/connect6.hpp) and sample [CMakeLists.txt](Connect6/cppbind/test_cppbind/CMakeLists.txt) is supported.

For testing C++ bindness, build test_cppbind and run it.
```
# for linux
mkdir build && pushd build
cmake .. && make
./test_cppbind
popd
```

Sample program play connect6 with AlphaZero policy and random callback.

For more complicated example, reference [cpp_weighted](AlphaZero/cpp_weighted)
```
cd Connect6/sample/cpp; ./build.sh && ./build/sample_exe  #for linux
cd Connect6/sample/cpp; ./build.ps1 && ./build/Release/sample_exe.exe  #for win-x64-msbuild
```
