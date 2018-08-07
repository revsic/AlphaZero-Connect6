use super::*;
use super::super::position::tests::gen_pos;

#[test]
fn test_new() {
    let game = Game::new();

    assert_eq!(game.get_turn(), Player::Black);
    assert_eq!(game.get_remain(), 1);

    let sample_board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    assert_eq!(*game.get_board(), sample_board);
}

#[test]
fn test_set() {
    let mut game = Game::new();

    let (pos, row, col) = gen_pos();
    let (urow, ucol) = pos.to_usize();

    let query: String = vec![row, col].iter().collect();

    assert_eq!(game.board[urow][ucol], Player::None);
    match Game::set(&mut game.board, query.as_str(), Player::White) {
        Ok(set_pos) => assert_eq!(pos, set_pos),
        Err(_) => assert!(false),
    }
    assert_eq!(game.board[urow][ucol], Player::White);
    match Game::set(&mut game.board, query.as_str(), Player::Black) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!(err, "Already set position"),
    }
}

#[test]
fn test_simulate() {
    let game = Game::new();
    let game = match game.simulate("aA") {
        Ok(game) => game,
        Err(_) => {
            assert!(false);
            Game::new()
        }
    };

    assert_eq!(game.turn, Player::White);
    assert_eq!(game.num_remain, 2);
    assert_eq!(game.board[0][0], Player::Black);

    match game.simulate("aA") {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "Already set position"),
    };

    match game.simulate("AA") {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "Invalid Query"),
    };
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

#[test]
fn test_is_game_end() {
    let game = Game::new();
    assert_eq!(game.is_game_end(), Player::None);
}