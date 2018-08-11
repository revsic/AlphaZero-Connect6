//use super::*;
//
//use std::io;
//use super::super::super::agent::*;
//
//#[cfg(test)]
//mod default_policy_test {
//    use super::*;
//
//    #[test]
//    fn test_select() {
//        let game = Game::new();
//        let sim = Simulate::from_game(&game);
//
//        let mut policy = DefaultPolicy::new();
//        policy.init(&sim);
//
//        loop {
//            if let Some((row, col)) = policy.select(&sim) {
//                let node = sim.node.borrow();
//                let pos = node.possible.iter().position(|x| *x == (row, col));
//
//                assert!(pos.is_some());
//                assert!(sim.validate(row, col));
//                break;
//            } else {
//                let pos = policy.select(&sim);
//                assert!(pos.is_none());
//
//                let (row, col) = policy.expand(&sim);
//                let expanded = sim.simulate(row, col);
//                let path = vec![(row, col)];
//                policy.update(&expanded, &path);
//            }
//        }
//    }
//
//    #[test]
//    fn test_expand() {
//        let game = Game::new();
//        let mut sim = Simulate::from_game(&game);
//
//        let mut policy = DefaultPolicy::new();
//        policy.init(&sim);
//
//        while let Some((row, col)) = policy.select(&sim) {
//            sim.simulate_in(row, col);
//        }
//        let (row, col) = policy.expand(&sim);
//
//        let node = sim.node.borrow();
//        let pos = node.possible.iter().position(|x| *x == (row, col));
//
//        assert!(pos.is_some());
//        assert!(sim.validate(row, col));
//        assert_eq!(policy.map.len(), 2);
//
//        let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
//        let node = policy.map.get(&hash(&board));
//        assert!(node.is_some());
//
//        let hashed = &node.unwrap().next_node;
//        assert_eq!(hashed.len(), 1);
//
//        let node = policy.map.get(&hashed[0]);
//        assert!(node.is_some());
//    }
//
//    #[test]
//    fn test_update() {
//        let game = Game::new();
//        let mut sim = Simulate::from_game(&game);
//
//        let mut policy = DefaultPolicy::new();
//        policy.init(&sim);
//
//        let mut path = Vec::new();
//        while let Some((row, col)) = policy.select(&sim) {
//            sim.simulate_in(row, col);
//            path.push((row, col));
//        }
//        let (row, col) = policy.expand(&sim);
//        let expanded = sim.simulate(row, col);
//        path.push((row, col));
//
//        policy.update(&expanded, &path);
//        assert!(true);
//        assert_eq!(policy.map.len(), 2);
//
//        let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
//        let node = policy.map.get(&hash(&board));
//        assert!(node.is_some());
//
//        let parent = node.unwrap();
//        let hashed = &parent.next_node;
//        assert_eq!(hashed.len(), 1);
//        assert_eq!(parent.visit, 1);
//
//        let node = policy.map.get(&hashed[0]);
//        assert!(node.is_some());
//
//        let child = node.unwrap();
//        assert_eq!(child.next_node.len(), 0);
//        assert_eq!(child.visit, 1);
//        assert_eq!(child.black_win, parent.black_win);
//
//        let num = child.board.iter()
//            .flat_map(|x| x.iter()
//                .filter(|y| **y != Player::None))
//            .count();
//        assert_eq!(num, 1);
//    }
//
//    #[test]
//    fn test_next() {
//        let game = Game::new();
//        let sim = Simulate::from_game(&game);
//
//        let mut policy = DefaultPolicy::new();
//        policy.init(&sim);
//
//        let pos = policy.policy(&sim);
//        assert!(pos.is_some());
//
//        let (row, col) = pos.unwrap();
//        assert!(sim.validate(row, col));
//
//        let node = sim.node.borrow();
//        let pos = node.possible.iter().position(|x| *x == (row, col));
//        assert!(pos.is_some());
//    }
//
//    #[test]
//    #[ignore]
//    fn test_self_play() {
//        let agent = Agent::with_start();
//        let game = agent.get_game();
//        let mut black_policy = DefaultPolicy::new();
//        let mut white_policy = DefaultPolicy::new();
//
//        let mut stdout = io::stdout();
//        loop {
//            let (row, col) = {
//                let game = game.read().unwrap();
//                game.print(&mut stdout).unwrap();
//                println!("-----------------{}-{}-----------------",
//                         black_policy.map.len(), white_policy.map.len());
//
//                if game.get_turn() == Player::Black {
//                    black_policy.next(&*game).unwrap()
//                } else {
//                    white_policy.next(&*game).unwrap()
//                }
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
//                }
//                Ok(GameResult::Status(_)) => (),
//                Err(err) => {
//                    println!("err occured {}", err);
//                    assert!(false);
//                    break;
//                }
//            };
//        }
//
//        let game = game.read().unwrap();
//        game.print(&mut stdout).unwrap();
//    }
//}
