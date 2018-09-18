use super::*;

#[test]
fn test_new() {
    let game = Game::new();

    assert_eq!(game.get_turn(), Player::Black);
    assert_eq!(game.get_remain(), 1);

    let sample_board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    assert_eq!(*game.get_board(), sample_board);
}

#[test]
fn test_play() {
    let mut game = Game::new();
    let result = game.play((0, 0)).map_err(|_| assert!(false)).unwrap();

    let expected = PlayResult {
        player: Player::Black,
        num_remain: 0,
        position: (0, 0),
    };

    assert_eq!(result, expected);
    assert_eq!(game.turn, Player::White);
    assert_eq!(game.num_remain, 2);
    assert_eq!(game.board[0][0], Player::Black);

    match game.play((0, 0)) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "game::play already set position"),
    };

    match game.play((BOARD_SIZE, BOARD_SIZE)) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "game::play invalid position"),
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
