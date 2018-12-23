# Wieghted Policy for Combined MCTS

Baseline of the weighted value, policy evaluator.

## Example

Train the model, reference [train.py](train.py).
```
python -m weighted.train --name=[model_name]
```
Play game with trained model, black for weighted policy, white for user input.
```
python -m weighted.play --load_ckpt=[n_epoch] --name=[model_name]
```
pyconnect6 will display the board status and you can input like 'aD' or 'sB'.
```
White (a, C), remain 1, 7.276 elapsed
0 A B C D E F G H I J K L M N O
a O O O _ _ _ X _ _ _ _ _ _ _ _
b _ _ X _ _ _ _ _ _ _ _ _ _ _ _
c _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
d _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
e _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
f _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
g _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
h _ _ _ _ _ _ _ _ _ _ _ _ _ X _
i _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
j _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
k _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
l _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
m _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
n _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
o _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
aD
```
If you want to modify the neural net archihtecture, reference [WeightedPolicy](model.py).
```python
def _get_model(self):
    player = tf.reshape(self.plc_player, (-1, 1))
    repr = tf.concat([player, self.plc_board], axis=1)

    policy = tf.layers.dense(repr, self.board_size ** 2)
    value = tf.layers.dense(repr, 1, activation=tf.nn.tanh)
    return tf.reshape(value, (-1, )), policy  # value, policy
```