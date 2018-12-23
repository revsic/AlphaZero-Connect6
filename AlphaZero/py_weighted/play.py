import os
import tensorflow as tf
from datetime import datetime

import pyconnect6
from py_weighted.model import WeightedPolicy

flags = tf.app.flags
flags.DEFINE_integer('board_size', 15, 'int, size of the board, default 15')
flags.DEFINE_integer('load_ckpt', 0, 'int, load ckpt with given epoch, if zero, initialize new, default 0')
flags.DEFINE_string('name', 'default', 'String, name of model, default `default`.')
flags.DEFINE_string('ckpt_dir', '.\weighted\ckpt', 'String, dir name for saving checkpoint, default `./ckpt_dir`.')
FLAGS = flags.FLAGS


def log(msg):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print('[{}] {}'.format(timestamp, msg))


def main(_):
    ckpt_path = os.path.join(FLAGS.ckpt_dir, FLAGS.name)
    with tf.Session() as sess:
        if FLAGS.load_ckpt != 0:
            # load ckpt
            policy = WeightedPolicy.load(sess, ckpt_path + str(FLAGS.load_ckpt))
            param = pyconnect6.load_param(ckpt_path)
        else:
            # initialize new
            policy = WeightedPolicy(sess, FLAGS.board_size, 0, 0)
            param = pyconnect6.default_param()
            sess.run(tf.global_variables_initializer())

        pyconnect6.play_with(policy, param)


if __name__ == '__main__':
    tf.app.run()
