import json
import tensorflow as tf


class AlphaZeroBaseLine(object):
    """ AlphaZeroBaseLine
    Attributes:
        board_size: int, size of the board
        l2_level: float, parameter controlling the level of L2 weight regluarization
        learning_rate: Float, learning rate of Adam for optimizing loss function.
        momentum: Float, momentum parameter of the optimizer
        sess: tf.Session, tf session for inferencing and training model
        plc_x: tf.placeholder, input vector shaped (batch_size, board_size ** 2).
        plc_z: tf.placeholder, game results as expected value vectors
        plc_pi: tf.placeholder, real policy from combined mcts
        plc_metric: tf.placeholder, metric value
        plc_training: tf.placeholder, boolean, batch normalization mode either train or inference.
        policy: tf.Tensor, policy head results of model.
        value: tf.Tensor, value head results of model
        loss: tf.Tensor, loss function.
        metric: tf.Tensor, metric for evaluating model.
        optimize: tf.Tensor, optimize object.
        summary: tf.Tensor, tensor summary of the metric.
    """
    def __init__(self,
                 board_size,
                 l2_level=1e-4,
                 learning_rate=1e-3,
                 momentum=0.9):
        """ Initializer
        Args:
            board_size: int, size of the board
            l2_level: float, parameter controlling the level of L2 weight regluarization
            learning_rate: Float, learning rate.
            momentum: Float, momentum parameter of the optimizer
        """
        self.board_size = board_size
        self.l2_level = l2_level
        self.learning_rate = learning_rate
        self.momentum = momentum

        self.sess = None

        board_capacity = self.board_size ** 2
        self.plc_x = tf.placeholder(tf.float32, [None, board_capacity])

        self.plc_z = tf.placeholder(tf.float32, [None])
        self.plc_pi = tf.placeholder(tf.float32, [None, board_capacity])

        self.plc_metric = tf.placeholder(tf.float32)
        self.plc_training = tf.placeholder(tf.bool)

        self.policy, self.value = self._get_model()
        self.loss = self._get_loss()
        self.metric = self._get_metric()

        self.optimize = tf.train.MomentumOptimizer(self.learning_rate, self.momentum).minimize(self.loss)
        self.summary = tf.summary.scalar('Metric', self.metric)
        self.ckpt = tf.train.Saver()

    def __call__(self, boards, **kwargs):
        return self.sess.run((self.value, self.policy), feed_dict={self.plc_x: x, self.plc_training: False})

    def init(self, sess=None):
        if sess is None:
            self.sess = tf.Session()
            self.sess.run(tf.global_variables_initializer())
        else:
            self.sess = sess

    def train(self, x, z, pi):
        self.sess.run(self.optimize, feed_dict={self.plc_x: x, self.plc_z: z, self.plc_pi: pi, self.plc_training: True})

    def inference(self, obj, x):
        return self.sess.run(obj, feed_dict={self.plc_x: x, self.plc_training: False})

    def dump(self, path):
        self.ckpt.save(self.sess, path + '.ckpt')
        with open(path + '.json', 'w') as f:
            dump = json.dumps(
                {
                    'board_size': self.board_size,
                    'l2_level': self.l2_level,
                    'learning_rate': self.learning_rate,
                    'momentum': self.momentum,
                }
            )
            f.write(dump)

    @classmethod
    def load(cls, path):
        with open(path + '.json') as f:
            param = json.loads(f.read())
        model = cls(param['board_size'], param['l2_level'], param['learning_rate'], param['momentum'])
        model.init()
        model.ckpt.restore(model.sess, path + '.ckpt')

        return model

    def _residual_block(self, input_):
        conv = tf.layers.conv2d(input_, 256, (3, 3), (1, 1), padding='same')
        norm = tf.layers.batch_normalization(conv, training=self.plc_training)
        relu = tf.nn.relu(norm)

        res_conv = tf.layers.conv2d(relu, 256, (3, 3), (1, 1), padding='same')
        res_norm = tf.layers.batch_normalization(res_conv, training=self.plc_training)
        residual = res_norm + input_
        res_relu = tf.nn.relu(residual)

        return res_relu

    def _get_model(self):
        size = self.board_size
        boards = tf.reshape(self.plc_x, (-1, size, size, 1))

        l1_conv = tf.layers.conv2d(boards, 256, (3, 3), (1, 1), padding='same')
        l1_norm = tf.layers.batch_normalization(l1_conv, training=self.plc_training)
        l1_relu = tf.nn.relu(l1_norm)

        input_ = l1_relu
        for _ in range(20):
            input_ = self._residual_block(input_)

        policy_conv = tf.layers.conv2d(input_, 1, (1, 1), (1, 1), padding='same')
        policy_norm = tf.layers.batch_normalization(policy_conv, training=self.plc_training)
        reshaped = tf.reshape(policy_norm, (-1, size * size))

        policy_fc = tf.layers.dense(reshaped, size * size)
        policy = tf.nn.sigmoid(policy_fc)

        value_conv = tf.layers.conv2d(input_, 1, (1, 1), (1, 1), padding='same')
        value_norm = tf.layers.batch_normalization(value_conv, training=self.plc_training)
        value_relu = tf.nn.relu(value_norm)

        value_fc = tf.layers.dense(value_relu, 256)
        value_relu = tf.nn.relu(value_fc)

        value_fc = tf.layers.dense(value_relu, 1)
        value_tanh = tf.nn.tanh(value_fc)
        value = tf.reshape(value_tanh, (-1, ))

        return policy, value

    def _get_loss(self):
        value_loss = tf.reduce_sum(tf.square(self.plc_z - self.value))
        policy_loss = tf.reduce_sum(tf.multiply(self.pi, tf.log(self.policy)))

        vars = tf.trainable_variables()
        l2_norm = self.l2_level * tf.reduce_sum(tf.nn.l2_loss(v) for v in vars)

        return value_loss - policy_loss + l2_norm

    def _get_metric(self):
        return self.plc_metric
