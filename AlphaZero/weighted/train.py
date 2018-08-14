import os
import tensorflow as tf
from datetime import datetime

import pyconnect6
from pyconnect6.buffer import Buffer
from weighted.model import WeightedPolicy

flags = tf.app.flags
flags.DEFINE_float('learning_rate', 1e-3, 'float, learning rate, default 1e-3.')
flags.DEFINE_float('momentum', 0.9, 'float, beta1 value in Adam, default 0.9.')
flags.DEFINE_integer('board_size', 15, 'int, size of the board, default 15')
flags.DEFINE_integer('max_buffer', 100000, 'int, max size of buffer, default 100000')
flags.DEFINE_integer('start_train', 40000, 'int, start train when the size of buffer over given, default 40000')
flags.DEFINE_integer('batch_size', 1024, 'int, size of batch, default 1024')
flags.DEFINE_integer('mini_batch', 2048, 'int, size of mini-batch, default 2048.')
flags.DEFINE_integer('ckpt_interval', 10, 'int, interval for writing checkpoint, default 10')
flags.DEFINE_integer('load_ckpt', 0, 'int, load ckpt with given epoch, if zero, train new, default 0')
flags.DEFINE_string('name', 'default', 'String, name of model, default `default`.')
flags.DEFINE_string('summary_dir', '.\weighted\summary', 'String, dir name for saving summary, default `./summary`.')
flags.DEFINE_string('ckpt_dir', '.\weighted\ckpt', 'String, dir name for saving checkpoint, default `./ckpt_dir`.')
FLAGS = flags.FLAGS


def log(msg):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    print('[{}] {}'.format(timestamp, msg))


def main(_):
    ckpt_path = os.path.join(FLAGS.ckpt_dir, FLAGS.name)
    buffer = Buffer(FLAGS.max_buffer, FLAGS.board_size, FLAGS.mini_batch)
    with tf.Session() as sess:
        if FLAGS.load_ckpt != 0:
            policy = WeightedPolicy.load(sess, ckpt_path + str(FLAGS.load_ckpt))
        else:
            policy = WeightedPolicy(sess, FLAGS.board_size, FLAGS.learning_rate, FLAGS.momentum)
            sess.run(tf.global_variables_initializer())

        writer = tf.summary.FileWriter(os.path.join(FLAGS.summary_dir, FLAGS.name), sess.graph)

        param = pyconnect6.default_param()
        param['num_simulation'] = 500
        param['num_game_thread'] = 12
        param['debug'] = True

        num_game = 0
        epoch = FLAGS.load_ckpt
        while True:
            num_game += 1
            result = pyconnect6.with_param(policy, param)
            for game_result in result:
                buffer.push_game(game_result)
            log('self-play async game#{}'.format(num_game))

            if len(buffer) > FLAGS.start_train:
                epoch += 1
                for _ in range(FLAGS.batch_size):
                    value, board, pos = buffer.sample()
                    policy.train(board, value, pos)

                value, board, pos = buffer.sample()
                summary = policy.inference(policy.summary, board, value, pos)
                writer.add_summary(summary, global_step=epoch)

                if epoch % FLAGS.ckpt_interval == 0:
                    policy.dump(ckpt_path + str(epoch))
                    log('ckpt saved')

                log('epoch#{}'.format(epoch))
                buffer.clear_half()


if __name__ == '__main__':
    tf.app.run()
