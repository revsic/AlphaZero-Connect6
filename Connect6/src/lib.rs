pub mod game;
pub mod agent;
pub mod mcts;
pub mod pybind;

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

py_module_initializer!(libconnect6, initlibconnect6, PyInit_connect6, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust, for Simulating Connect6"));
    try!(m.add(py, "test", py_fn!(py, test(a: i64, b: i64))));
    Ok(())
});

fn test(_: Python, a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

fn print_board(board: &[[game::Player; 19]; 19]) {
    println!("0 A B C D E F G H I J K L M N O P Q R S");
    for i in 0..19 {
        print!("{} ", (0x61 + i as u8) as char);
        for j in 0..19 {
            match board[i][j] {
                game::Player::Black => print!("X "),
                game::Player::White => print!("O "),
                game::Player::None => print!("_ "),
            }
        }
        println!();
    }
}