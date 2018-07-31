extern crate rand;

use super::*;
use super::super::position::tests::gen_pos;

#[test]
fn test_new() {
    let game = Game::new();

    assert_eq!(game.get_turn(), Player::Black);
    assert_eq!(game.get_remain(), 1);

    let sample_board = [[Player::None; 19]; 19];
    assert_eq!(*game.get_board(), sample_board);
}

#[test]
fn test_set() {
    let mut game = Game::new();

    let (pos, _, _) = gen_pos();
    let (row, col) = pos.to_usize();

    assert_eq!(game.board[row][col], Player::None);
    assert!(game.set(pos, Player::White));
    assert_eq!(game.board[row][col], Player::White);
    assert!(!game.set(pos, Player::Black));
}

#[test]
fn test_play() {
    let mut game = Game::new();
    let result = match game.play("aA") {
        Ok(res) => res,
        Err(_) => {
            assert!(false);
            PlayResult::new()
        }
    };

    let expected = PlayResult {
        player: Player::Black,
        num_remain: 0,
        position: ('a', 'A'),
    };

    assert_eq!(result, expected);
    assert_eq!(game.turn, Player::White);
    assert_eq!(game.num_remain, 2);
    assert_eq!(game.board[0][0], Player::Black);

    match game.play("aA") {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "Already set position"),
    };

    match game.play("AA") {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "Invalid Query"),
    };
}

#[test]
fn test_print() {
//        let game = Game::new();
//        game.print();
    assert!(true);
}