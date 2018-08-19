# import os
# import platform
#
# if not os.path.exists('./pyconnect6/connect6.pyd'):
#     name = platform.system()
#     if name == 'Windows':
#         os.system('cd ..\\Connect6 &'
#                   'cargo build --release &'
#                   'move .\\target\\release\\connect6.dll ..\\AlphaZero\\pyconnect6\\connect6.pyd')
#     elif name == 'Linux':
#         pass
#     elif name == 'Darwin':
#         os.system('cd ..\\Connect6;'
#                   'cargo build --release;'
#                   'move .\\target\\release\connect6.dylib ..\\AlphaZero\\pyconnect6\\connect6.so')

import sys

from setuptools import setup

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print('Please install setuptools-rust package')
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup(
    name='connect6',
    version='0.1.0',
    packages=['pyconnect6'],
    rust_extensions=[RustExtension('pyconnect6.connect6',
                                   'Cargo.toml',
                                   binding=Binding.RustCPython)],
    zip_safe=False,
)