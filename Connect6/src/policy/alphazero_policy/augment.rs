use super::*;
use std::default::Default;
use std::ops::Add;

type GenericBoard<T> = [[T; BOARD_SIZE]; BOARD_SIZE];

fn rotate_left<T: Copy + Default>(board: &mut GenericBoard<T>) {
    let mut rotate = [[Default::default(); BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            rotate[BOARD_SIZE - j - 1][i] = board[i][j];
        }
    }
    *board = rotate;
}

fn rotate_right<T: Copy + Default>(board: &mut GenericBoard<T>) {
    let mut rotate = [[Default::default(); BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            rotate[j][BOARD_SIZE - i - 1] = board[i][j];
        }
    }
    *board = rotate;
}

fn flip_vertical<T>(board: &mut GenericBoard<T>) { // axis |
    for i in 0..BOARD_SIZE / 2 {
        for j in 0..BOARD_SIZE {
            // swap(board[i][BOARD_SIZE - j], board[i][j]);
            board[i].swap(BOARD_SIZE - j, j);
        }
    }
}

fn flip_horizontal<T>(board: &mut GenericBoard<T>) { // axis --
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE / 2 {
            // swap(board[BOARD_SIZE - j][i], board[j][i]);
            let (front, back) = board.split_at_mut(j);
            swap(&mut front[0][i], &mut back[back.len() - j][i]);
        }
    }
}

fn sum_board<T>(board1: &mut GenericBoard<T>, board2: &GenericBoard<T>)
    where T: Add<T, Output = T> + Copy + Default
{
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board1[i][j] = board1[i][j] + board2[i][j];
        }
    }
}

pub fn augment_way8(board: &Board) -> Vec<Board> {
    let mut vec = Vec::new();
    let mut board = *board;

    for _ in 0..4 {
        rotate_left(&mut board);
        vec.push(board);

        let mut copied = board;
        flip_vertical(&mut copied);
        vec.push(copied);
    }
    vec
}

pub fn recover_way8(mut probs: Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>) -> [[f32; BOARD_SIZE]; BOARD_SIZE] {
    let mut total = [[0.; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..4 {
        flip_vertical(&mut probs[i * 2 + 1]);
        { // borrow mut probs: Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>
            let (o, f) = probs.split_at_mut(i * 2);
            sum_board(&mut o[0], &f[0]);
        }
        for _ in 0..(i+1) {
            rotate_right(&mut probs[i * 2]);
        }
        sum_board(&mut total, &probs[i * 2]);
    }
    total.iter_mut().for_each(|x|
        x.iter_mut().for_each(|x| *x /= 8.));
    total
}