use super::*;
use rand;

#[test]
fn test_rotate_left() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    board[0][0] = Player::Black;
    board[0][1] = Player::Black;
    board[0][BOARD_SIZE-1] = Player::White;
    board[1][BOARD_SIZE-1] = Player::Black;
    board[BOARD_SIZE-1][BOARD_SIZE-1] = Player::White;
    board[BOARD_SIZE-1][BOARD_SIZE-2] = Player::White;
    board[BOARD_SIZE-1][0] = Player::Black;
    board[BOARD_SIZE-2][0] = Player::White;

    augment::rotate_left(&mut board);
    assert_eq!(board[BOARD_SIZE-1][0], Player::Black);
    assert_eq!(board[BOARD_SIZE-2][0], Player::Black);
    assert_eq!(board[0][0], Player::White);
    assert_eq!(board[0][1], Player::Black);
    assert_eq!(board[0][BOARD_SIZE-1], Player::White);
    assert_eq!(board[1][BOARD_SIZE-1], Player::White);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-1], Player::Black);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-2], Player::White);
}

#[test]
fn test_rotate_right() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    board[0][0] = Player::Black;
    board[0][1] = Player::Black;
    board[0][BOARD_SIZE-1] = Player::White;
    board[1][BOARD_SIZE-1] = Player::Black;
    board[BOARD_SIZE-1][BOARD_SIZE-1] = Player::White;
    board[BOARD_SIZE-1][BOARD_SIZE-2] = Player::White;
    board[BOARD_SIZE-1][0] = Player::Black;
    board[BOARD_SIZE-2][0] = Player::White;

    augment::rotate_right(&mut board);
    assert_eq!(board[BOARD_SIZE-1][0], Player::White);
    assert_eq!(board[BOARD_SIZE-2][0], Player::White);
    assert_eq!(board[0][0], Player::Black);
    assert_eq!(board[0][1], Player::White);
    assert_eq!(board[0][BOARD_SIZE-1], Player::Black);
    assert_eq!(board[1][BOARD_SIZE-1], Player::Black);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-1], Player::White);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-2], Player::Black);
}

#[test]
fn test_flip_vertical() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    board[0][0] = Player::Black;
    board[0][1] = Player::Black;
    board[0][BOARD_SIZE-1] = Player::White;
    board[1][BOARD_SIZE-1] = Player::Black;
    board[BOARD_SIZE-1][BOARD_SIZE-1] = Player::White;
    board[BOARD_SIZE-1][BOARD_SIZE-2] = Player::White;
    board[BOARD_SIZE-1][0] = Player::Black;
    board[BOARD_SIZE-2][0] = Player::White;

    augment::flip_vertical(&mut board);
    assert_eq!(board[BOARD_SIZE-1][0], Player::White);
    assert_eq!(board[BOARD_SIZE-1][1], Player::White);
    assert_eq!(board[0][0], Player::White);
    assert_eq!(board[1][0], Player::Black);
    assert_eq!(board[0][BOARD_SIZE-1], Player::Black);
    assert_eq!(board[0][BOARD_SIZE-2], Player::Black);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-1], Player::Black);
    assert_eq!(board[BOARD_SIZE-2][BOARD_SIZE-1], Player::White);
}

#[test]
fn test_flip_horizontal() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    board[0][0] = Player::Black;
    board[0][1] = Player::Black;
    board[0][BOARD_SIZE-1] = Player::White;
    board[1][BOARD_SIZE-1] = Player::Black;
    board[BOARD_SIZE-1][BOARD_SIZE-1] = Player::White;
    board[BOARD_SIZE-1][BOARD_SIZE-2] = Player::White;
    board[BOARD_SIZE-1][0] = Player::Black;
    board[BOARD_SIZE-2][0] = Player::White;

    augment::flip_horizontal(&mut board);
    assert_eq!(board[BOARD_SIZE-1][0], Player::Black);
    assert_eq!(board[BOARD_SIZE-1][1], Player::Black);
    assert_eq!(board[0][0], Player::Black);
    assert_eq!(board[1][0], Player::White);
    assert_eq!(board[0][BOARD_SIZE-1], Player::White);
    assert_eq!(board[0][BOARD_SIZE-2], Player::White);
    assert_eq!(board[BOARD_SIZE-1][BOARD_SIZE-1], Player::White);
    assert_eq!(board[BOARD_SIZE-2][BOARD_SIZE-1], Player::Black);
}

#[test]
fn test_sum_board() {
    let max = BOARD_SIZE * BOARD_SIZE;
    let mut board1 = [[0; BOARD_SIZE]; BOARD_SIZE];
    let mut board2 = [[0; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board1[i][j] = i * BOARD_SIZE + j;
            board2[i][j] = max - board1[i][j];
        }
    }

    augment::sum_board(&mut board1, &board2);
    board1.iter().for_each(|x|
         x.iter().for_each(|x| assert_eq!(*x, max)));
}

#[test]
fn test_augment_and_recover() {
    fn p2f(board: &Board) -> [[f32; BOARD_SIZE]; BOARD_SIZE] {
        let mut table = [[0.; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                table[i][j] = board[i][j] as i32 as f32;
            }
        }
        table
    }
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = Player::from(rand::random::<i32>() % 3 - 1);
        }
    }
    let augmented = augment::augment_way8(&board);
    assert_eq!(augmented.len(), 8);

    for i in 0..8 {
        for j in (i+1)..8 {
            assert_ne!(augmented[i], augmented[j]);
        }
    }
    let converted = augmented.iter()
        .map(|x| p2f(x))
        .collect::<Vec<_>>();
    let recovered = augment::recover_way8(converted);
    assert_eq!(recovered, p2f(&board));
}