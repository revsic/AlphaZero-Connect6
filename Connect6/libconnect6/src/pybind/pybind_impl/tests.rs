use super::*;
use {game::*, BOARD_CAPACITY};

#[test]
fn test_pyseq_to_vec() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let data = [1, 2, 3, 4, 5]
        .iter()
        .map(|x: &i32| x.to_py_object(py).into_object())
        .collect::<Vec<PyObject>>();

    let list = PyList::new(py, data.as_slice()).into_object();
    let vec = pyseq_to_vec(py, list);
    assert!(vec.is_some());
    assert_eq!(vec.unwrap(), [1., 2., 3., 4., 5.]);

    let tuple = PyTuple::new(py, data.as_slice()).into_object();
    let vec = pyseq_to_vec(py, tuple);
    assert!(vec.is_some());
    assert_eq!(vec.unwrap(), [1., 2., 3., 4., 5.]);
}

#[test]
fn test_pylist_from_board() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    board[0][0] = Player::White;
    board[1][0] = Player::Black;

    let pylist = pylist_from_board(py, &board);
    let seq = pyseq_to_vec(py, pylist);
    assert!(seq.is_some());

    let seq = seq.unwrap();
    let mut recovered = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let player = match seq[i * BOARD_SIZE + j] as i32 {
                -1 => Player::Black,
                0 => Player::None,
                1 => Player::White,
                _ => {
                    assert!(false);
                    Player::None
                }
            };
            recovered[i][j] = player;
        }
    }
    assert_eq!(board, recovered);
}

#[test]
fn test_pylist_from_multiple() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let board = vec![
        [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        [[Player::Black; BOARD_SIZE]; BOARD_SIZE],
    ];
    let list = pylist_from_multiple(py, &board);

    let res = list.cast_into::<PySequence>(py).ok();
    assert!(res.is_some());

    let res = res.unwrap().iter(py).ok();
    assert!(res.is_some());

    let vec = res.unwrap()
        .filter_map(|x| x.ok())
        .filter_map(|x| pyseq_to_vec(py, x))
        .collect::<Vec<_>>();

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0].len(), BOARD_CAPACITY);
    assert_eq!(vec[1].len(), BOARD_CAPACITY);

    assert_eq!(vec[0], vec![Player::None as i32 as f32; BOARD_CAPACITY]);
    assert_eq!(vec[1], vec![Player::Black as i32 as f32; BOARD_CAPACITY]);
}
