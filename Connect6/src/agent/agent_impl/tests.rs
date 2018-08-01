use super::*;

#[test]
fn test_with_start() {
    let mut agent = Agent::with_start();

    agent.to_thread.send(Query::Terminate)
        .expect("test_with_start : couldn't terminate");

    agent.is_game_end.set(true);
    if let Some(handle) = agent.main_thread.take() {
        handle.join()
            .expect("test_with_start : handle join fail");
    }

    assert!(true);
}

#[test]
fn test_get_game() {
    let agent = Agent::with_start();
    {
        let game = agent.get_game();
        let mut game = game.write()
            .expect("test_get_game : agent.game is poisoned");
        game.play("aA").unwrap();
    }

    let query = String::from("aA");
    let result = match agent.play(&query) {
        Ok(result) => result,
        Err(_) => {
            assert!(false);
            GameResult::GameEnd(Player::None)
        },
    };

    let result = match result {
        GameResult::Status(result) => result,
        GameResult::GameEnd(_) => {
            assert!(false);
            Err("test_get_game : assertion fail")
        },
    };

    match result {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!(err, "Already set position"),
    };
}

#[test]
fn test_play() {
    let agent = Agent::with_start();

    let play = |query: &str| -> Result<PlayResult, &'static str> {
        let play_result = match agent.play(&String::from(query)) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        match play_result {
            GameResult::GameEnd(_) => Err("get_result, already finished"),
            GameResult::Status(result) => result
        }
    };

    let mut num_remain = 1;
    let mut player = Player::White;

    let mut auto_player = |query: &str| {
        let mut chars = query.chars();
        let row = chars.next().unwrap();
        let col = chars.next().unwrap();

        assert_eq!(play(query), Ok(
            PlayResult { player, num_remain, position: (row, col) }
        ));

        num_remain -= 1;
        if num_remain < 0 {
            num_remain = 1;
            player.mut_switch();
        }
    };

    // black
    assert_eq!(play("aA"), Ok(
        PlayResult { player: Player::Black, num_remain: 0, position: ('a', 'A') }
    ));

    // white
    assert_eq!(play("aA"), Err("Already set position"));
    assert_eq!(play("zZ"), Err("Invalid Query"));

    let record = [
        ("aB", "aC"), // white
        ("bA", "cA"), // black
        ("aD", "aE"), // white
        ("dA", "eA"), // black
    ];

    for (turn1, turn2) in record.iter() {
        auto_player(turn1);
        auto_player(turn2);
    }

    auto_player("aF"); // white
    assert_eq!(play("aG"), Err("get_result, already finished"));
}