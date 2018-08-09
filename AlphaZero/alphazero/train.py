import os
import tensorflow as tf

flags = tf.app.flags
flags.DEFINE_float('learning_rate', 1e-3, 'float, learning rate, default 1e-3.')
flags.DEFINE_float('momentum', 0.9, 'float, beta1 value in Adam, default 0.9.')
flags.DEFINE_integer('epoch', 20, 'Integer, number of epochs, default 20.')
flags.DEFINE_integer('batch_size', 2048, 'Integer, size of batch, default 128.')
flags.DEFINE_integer('ckpt_interval', 5, 'Integer, interval for writing checkpoint, default 5')
flags.DEFINE_string('name', 'default', 'String, name of model, default `default`.')
flags.DEFINE_string('summary_dir', './summary', 'String, dir name for saving tensor summary, default `./summary`.')
flags.DEFINE_string('ckpt_dir', './ckpt', 'String, dir name for saving checkpoint, default `./ckpt_dir`.')
FLAGS = flags.FLAGS

def main(_):
    # total_x, total_y, x_dim, y_dim
    ckpt_path = os.path.join(FLAGS.ckpt_dir, FLAGS.name)

    batch = model.Batch(total_x, total_y, 128)

    with tf.Session() as sess:
        basic_model = model.BasicModel(x_dim, y_dim, FLAGS.learning_rate, FLAGS.beta1)
        writer = tf.summary.FileWriter(os.path.join(FLAGS.summary_dir, FLAGS.name), sess.graph)

        sess.run(tf.global_variables_initializer())
        for i in range(FLAGS.epoch):
            for n in range(batch.iter_per_epoch):
                batch_x, batch_y = batch()
                basic_model.train(sess, batch_x, batch_y)

                summary = basic_model.inference(sess, basic_model.summary, batch_x, batch_y)
                writer.add_summary(summary)

            if (i + 1) % FLAGS.ckpt_interval == 0:
                basic_model.dump(sess, ckpt_path)

if __name__ == '__main__':
    tf.app.run()