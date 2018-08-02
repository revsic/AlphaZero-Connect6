use super::*;

#[test]
fn test_with_start() {
    let mut agent = Agent::with_start();
    agent.terminate();
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
    match agent.play(&query) {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!(err, "Already set position"),
    };
}

#[test]
fn test_play() {
    let agent = Agent::with_start();
    let play = |query: &str| agent.play(&String::from(query));

    let mut num_remain = 1;
    let mut player = Player::Black;

    let mut auto_player = |query: &str| {
        let mut chars = query.chars();
        let row = chars.next().unwrap();
        let col = chars.next().unwrap();

        num_remain -= 1;
        assert_eq!(play(query), Ok(
            GameResult::Status(PlayResult { player, num_remain, position: (row, col) })
        ));

        if num_remain <= 0 {
            num_remain = 2;
            player.mut_switch();
        }
    };

    // black
    auto_player("aA");

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
    assert_eq!(play("aG"), Ok(GameResult::GameEnd(Player::White)));
}