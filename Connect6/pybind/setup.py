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
    rust_extensions=[RustExtension('pyconnect6.pyconnect6',
                                   './libpyconnect6/Cargo.toml',
                                   binding=Binding.RustCPython)],
    zip_safe=False,
)