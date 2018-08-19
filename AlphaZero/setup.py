import os
import platform

if not os.path.exists('./pyconnect6/connect6.pyd'):
    name = platform.system()
    if name == 'Windows':
        os.system('cd ..\\Connect6 &'
                  'cargo build --release &'
                  'move .\\target\\release\\connect6.dll ..\\AlphaZero\\pyconnect6\\connect6.pyd')
    elif name == 'Linux':
        pass
    elif name == 'Darwin':
        pass