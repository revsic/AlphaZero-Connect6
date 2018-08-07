use super::*;

use std::io;
use super::super::super::agent::*;

#[cfg(test)]
mod default_policy_test {
    use super::*;

    #[test]
    fn test_select() {
        let mut policy = DefaultPolicy::new();
        policy_tests::test_select(&mut policy);
    }

    #[test]
    fn test_expand() {
        let mut policy = DefaultPolicy::new();
        policy_tests::test_expand(&mut policy);
        assert_eq!(policy.map.len(), 2);

        let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
        let node = policy.map.get(&hash(&board));
        assert!(node.is_some());

        let hashed = &node.unwrap().next_node;
        assert_eq!(hashed.len(), 1);

        let node = policy.map.get(&hashed[0]);
        assert!(node.is_some());
    }

    #[test]
    fn test_update() {
        let mut policy = DefaultPolicy::new();
        policy_tests::test_update(&mut policy);
        assert_eq!(policy.map.len(), 2);

        let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
        let node = policy.map.get(&hash(&board));
        assert!(node.is_some());

        let parent = node.unwrap();
        let hashed = &parent.next_node;
        assert_eq!(hashed.len(), 1);
        assert_eq!(parent.visit, 1);

        let node = policy.map.get(&hashed[0]);
        assert!(node.is_some());

        let child = node.unwrap();
        assert_eq!(child.next_node.len(), 0);
        assert_eq!(child.visit, 1);
        assert_eq!(child.black_win, parent.black_win);

        let num = child.board.iter()
            .flat_map(|x| x.iter()
                .filter(|y| **y != Player::None))
            .count();
        assert_eq!(num, 1);
    }

    #[test]
    fn test_get_policy() {
        let mut policy = DefaultPolicy::new();
        policy_tests::test_get_policy(&mut policy);
    }

    #[test]
    #[ignore]
    fn test_self_play() {
        let agent = Agent::with_start();
        let game = agent.get_game();
        let mut black_policy = DefaultPolicy::new();
        let mut white_policy = DefaultPolicy::new();

        let mut stdout = io::stdout();
        loop {
            let (row, col) = {
                let game = game.read().unwrap();
                game.print(&mut stdout).unwrap();
                println!("-----------------{}-{}-----------------",
                         black_policy.map.len(), white_policy.map.len());

                if game.get_turn() == Player::Black {
                    black_policy.get_policy(&*game)
                } else {
                    white_policy.get_policy(&*game)
                }
            };

            let row = (row as u8 + 0x61) as char;
            let col = (col as u8 + 0x41) as char;
            let query: String = vec![row, col].iter().collect();

            match agent.play(&query) {
                Ok(GameResult::GameEnd(player)) => {
                    println!("{:?} win", player);
                    break;
                },
                Ok(GameResult::Status(_)) => (),
                Err(err) => {
                    println!("err occured {}", err);
                    assert!(false);
                    break;
                }
            };
        }

        let game = game.read().unwrap();
        game.print(&mut stdout).unwrap();
    }
}
