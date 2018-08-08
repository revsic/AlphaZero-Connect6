import json
import tensorflow as tf

class AlphaZero(object):
    ''' BasicModel
    Attributes:
        board_size: int, size of the board
        l2_level: float, parameter controlling the level of L2 weight regluarization
        learning_rate: Float, learning rate of Adam for optimizing loss function.
        beta1: Float, beta1 value of Adam for optimizing loss function.
        plc_x: tf.placeholder, input vector.
        plc_training: tf.placeholder, boolean, batch normalization mode either train or inference.
        plc_dropout: tf.placeholder, probability for dropout layer.
        model: tf.Tensor, model.
        loss: tf.Tensor, loss function.
        metric: tf.Tensor, metric for evaluating model.
        optimize: tf.Tensor, optimize object.
        summary: tf.Tensor, tensor summary of the metric.
    '''
    def __init__(self,
                 board_size,
                 l2_level=1e-4,
                 learning_rate=1e-3,
                 momentum=0.9):
        ''' Initializer
        Args:
            board_size: int, size of the board
            l2_level: float, parameter controlling the level of L2 weight regluarization
            learning_rate: Float, learning rate.
            momentum: Float, momentum parameter of the optimizer
        '''
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
        pass

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
                    'learning_rate': self.learning_rate,
                    'beta1': self.beta1
                }
            )
            f.write(dump)

    @classmethod
    def load(cls, path):
        with open(path + '.json') as f:
            param = json.loads(f.read())
        model = cls(param['board_size'], param['learning_rate'], param['beta1'])
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
        value_loss = tf.square(self.plc_z - self.value)
        policy_loss = -tf.matmul(tf.transpose(self.pi), self.policy)

        vars = tf.trainable_variables()
        l2_norm = self.l2_level * tf.add_n([tf.nn.l2_loss(v) for v in vars])

        loss = value_loss + policy_loss + l2_norm
        return loss

    def _get_metric(self):
        return self.plc_metric


class Batch(object):
    def __init__(self, x, y, batch_size):
        self.total_x = x
        self.total_y = y
        self.batch_size = batch_size

        self.iter_per_epoch = len(x) // batch_size
        self.epochs_completed = 0

        self._iter = 0

    def __call__(self):
        start = self._iter * self.batch_size
        end = (self._iter + 1) * self.batch_size

        batch_x = self.total_x[start:end]
        batch_y = self.total_y[start:end]

        self._iter += 1
        if self._iter == self.iter_per_epoch:
            self.epochs_completed += 1
            self._iter = 0

        return batch_x, batch_y