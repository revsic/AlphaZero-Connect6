import json
import tensorflow as tf


class WeightedPolicy(object):
    """ Weighted Policy consists of single fully connected layer

    Attributes:
        sess: tf.Session, sesion for inference policy
        board_size: int, size of the board
        learning_rate: float, learning rate.
        momentum: float, momentum for the optimizer.
        plc_player: tf.placeholder, player of the given board status.
        plc_board: tf.placeholder, input vector, flattened board.
        plc_value: tf.placeholder, expected value vector.
        plc_policy: tf.placeholder, policy based on monte carlo tree search
        value: tf.Tensor, value head of the model
        policy: tf.Tensor, policy head of the model
        loss: tf.Tensor, loss function.
        optimize: tf.Tensor, optimize object.
        summary: tf.Tensor, tensor summary.
    """
    def __init__(self, sess, board_size, learning_rate, momentum):
        """ Initializer
        Args:
            sess: tf.Session, session for inference policy
            board_size: int, size of the board.
            learning_rate: Float, learning rate.
            momentum: float, momentum for the optimizer.
        """
        self.sess = sess
        self.board_size = board_size
        self.learning_rate = learning_rate
        self.momentum = momentum

        board_capacity = board_size ** 2
        self.plc_player = tf.placeholder(tf.float32, [None], name='plc_player')
        self.plc_board = tf.placeholder(tf.float32, [None, board_capacity], name='plc_board')

        self.plc_value = tf.placeholder(tf.float32, [None], name='plc_value')
        self.plc_policy = tf.placeholder(tf.int32, [None], name='plc_policy')

        self.value, self.policy = self._get_model()
        self.value = tf.identity(self.value, name='value')
        self.output_policy = tf.nn.softmax(self.policy, name='policy')
        self.value_loss, self.policy_loss = self._get_loss()
        self.loss = self.value_loss + self.policy_loss

        self.optimize = tf.train.MomentumOptimizer(self.learning_rate, self.momentum).minimize(self.loss)
        self.summary = tf.summary.merge([
            tf.summary.scalar('value_mse', self.value_loss),
            tf.summary.scalar('policy_ce', self.policy_loss),
            tf.summary.scalar('loss', self.loss)
        ])
        self.ckpt = tf.train.Saver()

    def __call__(self, turn, board, **kwargs):
        return self.sess.run((self.value, self.output_policy),
                             feed_dict={self.plc_player: [turn] * len(board),
                                        self.plc_board: board})

    def train(self, turn, board, value, policy):
        self.sess.run(self.optimize,
                      feed_dict={self.plc_player: turn,
                                 self.plc_board: board,
                                 self.plc_value: value,
                                 self.plc_policy: policy})

    def inference(self, obj, turn, board, value, policy):
        return self.sess.run(obj,
                             feed_dict={self.plc_player: turn,
                                        self.plc_board: board,
                                        self.plc_value: value,
                                        self.plc_policy: policy})

    def write_graph(self, path):
        init = tf.global_variables_initializer()
        saver = self.ckpt.as_saver_def()

        print('Initializer : ', init.name)
        print('Train : ', self.optimize.name)
        print('Tensor ckpt filename : ', saver.filename_tensor_name)
        print('Save ckpt : ', saver.save_tensor_name)
        print('Restore ckpt : ', saver.restore_op_name)

        # Initializer :  init
        # Train :  Momentum
        # Tensor ckpt filename :  save/Const:0
        # Save ckpt :  save/control_dependency:0
        # Restore ckpt :  save/restore_all

        with open(path + '.pb', 'wb') as f:
            f.write(tf.get_default_graph().as_graph_def().SerializeToString())

    def dump(self, path):
        self.ckpt.save(self.sess, path + '.ckpt')
        with open(path + '.json', 'w') as f:
            dump = json.dumps(
                {
                    'board_size': self.board_size,
                    'learning_rate': self.learning_rate,
                    'momentum': self.momentum
                }
            )
            f.write(dump)

    @classmethod
    def load(cls, sess, path):
        with open(path + '.json') as f:
            param = json.loads(f.read())

        model = cls(sess, param['board_size'], param['learning_rate'], param['momentum'])
        model.ckpt.restore(sess, path + '.ckpt')

        return model

    def _get_model(self):
        player = tf.reshape(self.plc_player, (-1, 1))
        repr = tf.concat([player, self.plc_board], axis=1)

        policy = tf.layers.dense(repr, self.board_size ** 2)
        value = tf.layers.dense(repr, 1, activation=tf.nn.tanh)
        return tf.reshape(value, (-1, )), policy  # value, policy

    def _get_loss(self):
        value_loss = tf.reduce_mean(tf.square(self.plc_value - self.value))
        policy = tf.one_hot(self.plc_policy, self.board_size ** 2)
        policy_loss = tf.reduce_mean(
            tf.nn.softmax_cross_entropy_with_logits_v2(labels=policy, logits=self.policy))
        return value_loss, policy_loss
