use super::*;

mod policy_test {
    use super::*;

    pub fn test_select(policy: &impl Policy) {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let sim = root.to_simulate();

        for (row, col) in policy.select(&sim) {
            assert!(sim.validate(row, col));

            let pos = sim.possible.iter().position(|x| *x == (row, col));
            assert!(pos.is_some());
        }
    }

    pub fn test_update(policy: &mut impl Policy) {
        let game = Game::new();
        let origin = Root::from_game(&game);
        let mut root = Root::from_game(&game);
        {
            let mut sim = root.to_simulate();
            let mut sim_aA = sim.simulate(0, 0);

            policy.update(&mut sim_aA);
        }
        assert_eq!(origin.turn, root.turn);
        assert_eq!(origin.num_remain, root.num_remain);
        assert_eq!(origin.board, root.board);

        let pos = root.possible.pop().unwrap();
        root.possible.insert(0, pos);
        assert_eq!(origin.possible, root.possible);
    }

    pub fn test_get_policy(policy: &impl Policy) {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let (row, col) = policy.get_policy(&root);

        let sim = root.to_simulate();
        assert!(sim.validate(row, col));

        let pos = sim.possible.iter().position(|x| *x == (row, col));
        assert!(pos.is_some());
    }
}

#[cfg(test)]
mod default_policy_test {
    use super::*;

    #[test]
    fn test_select() {
        let policy = DefaultPolicy::new();
        policy_test::test_select(&policy);
    }

    #[test]
    fn test_update() {
        let mut policy = DefaultPolicy::new();
        policy_test::test_update(&mut policy);

        fn num(board: &[[Player; 19]; 19]) -> usize {
            board.iter().flat_map(|x|
                x.iter().filter(|y| **y != Player::None )
            ).count()
        }

        let mut table: Vec<(&[[Player; 19]; 19], &Node)> = policy.map.iter().collect();
        table.sort_by(|(board1, _), (board2, _)| num(board1).cmp(&num(board2)));

        assert!(table.len() >= 10);
        assert_eq!(num(table[0].0), 2);

        let mut hasher = DefaultHasher::new();
        let mut board = [[Player::None; 19]; 19];
        board[0][0] = Player::Black;
        board.hash(&mut hasher);

        let mut prev_num = 1;
        let mut prev_hash = hasher.finish();
        for (board, node) in table {
            assert_eq!(prev_num + 1, num(board));
            assert!(node.prev.contains(&prev_hash));

            let mut hasher = DefaultHasher::new();
            board.hash(&mut hasher);

            prev_hash = hasher.finish();
            prev_num += 1;
        }
    }

    #[test]
    fn test_get_policy() {
        let policy = DefaultPolicy::new();
        policy_test::test_get_policy(&policy);
    }
}

#[cfg(test)]
mod tree_search_test {
    use super::*;

    #[test]
    fn test_new() {
        let mut policy = DefaultPolicy::new();
        let tree = TreeSearch::new(&mut policy);

        assert!(true);
    }

    #[test]
    fn test_from_game() {
        let game = Game::new();
        let mut policy = DefaultPolicy::new();
        let _tree = TreeSearch::from_game(&game, &mut policy);

        assert!(true);
    }

    #[test]
    fn test_self_play() {
//        let agent = Agent::with_start();
//        let game = agent.get_game();
//        let mut policy = DefaultPolicy::debug();
//
//        let mut stdout = io::stdout();
//        loop {
//            let (row, col) = {
//                let game = game.read().unwrap();
//                game.print(&mut stdout);
//                println!("-----------------{}-----------------", policy.map.len());
//
//                let mut tree = TreeSearch::from_game(&*game, &mut policy);
//                tree.search()
//            };
//
//            let row = (row as u8 + 0x61) as char;
//            let col = (col as u8 + 0x41) as char;
//            let query: String = vec![row, col].iter().collect();
//
//            match agent.play(&query) {
//                Ok(GameResult::GameEnd(player)) => {
//                    println!("{:?} win", player);
//                    break;
//                },
//                Ok(GameResult::Status(result)) => (),
//                Err(err) => {
//                    println!("err occured {}", err);
//                    assert!(false);
//                    break;
//                }
//            };
//        }

        assert!(true);
    }
}