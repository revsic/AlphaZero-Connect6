import pyconnect6
import multiprocessing as mp
from datetime import datetime


def _player(proc_id, queue: mp.Queue, output: mp.Queue, policy, param):
    while True:
        msg = queue.get()
        if msg is None:
            output.put(proc_id)
            break

        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        print('[{}] player#{} run {}th game'.format(timestamp, proc_id, msg))

        result = pyconnect6.with_param(policy, param)
        output.put(result)


class AsyncConnect6(object):
    def __init__(self, n_proc, policy, param=None):
        self.n_proc = n_proc
        self.policy = policy
        if param is None:
            self.param = pyconnect6.default_param()
        else:
            self.param = param

        self.queue = mp.Queue()
        self.output = mp.Queue()

    def start(self):
        for proc_id in range(self.n_proc):
            proc = mp.Process(target=_player,
                              args=(proc_id, self.queue, self.output, self.policy, self.param))
            proc.daemon = True
            proc.start()

    def play(self, num, buffer):
        for i in range(num):
            self.queue.put(i)

        for i in range(num):
            result = self.output.get()
            buffer.push_game(result)

    def terminate(self):
        for i in range(self.n_proc):
            self.queue.put(None)

        for i in range(self.n_proc):
            self.output.get()
