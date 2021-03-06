use super::*;
use agent::Agent;

use std::time::Instant;

#[test]
fn test_select() {
    let game = Game::new();
    let mut sim = Simulate::from_game(&game);

    let rand_eval = Box::new(RandomEvaluator {});
    let mut policy = AlphaZero::new(rand_eval);
    policy.init(&sim);

    let mut path = Vec::new();
    while let Some((row, col)) = policy.select(&sim) {
        {
            // borrow sim: Simulate
            let node = sim.node.borrow();
            let pos = node.possible.iter().position(|x| *x == (row, col));
            assert!(pos.is_some());
            assert!(sim.validate(row, col));
        }
        path.push((row, col));
        sim.simulate_in(row, col);
    }

    policy.expand(&sim);
    policy.update(&sim, &path);

    let pos = policy.select(&sim);
    assert!(pos.is_some());

    let (row, col) = pos.unwrap();
    assert!(sim.validate(row, col));

    let node = sim.node.borrow();
    let pos = node.possible.iter().position(|x| *x == (row, col));
    assert!(pos.is_some());
}

#[test]
fn test_expand() {
    let game = Game::new();
    let mut sim = Simulate::from_game(&game);

    let rand_eval = Box::new(RandomEvaluator {});
    let mut policy = AlphaZero::new(rand_eval);
    policy.init(&sim);

    while let Some((row, col)) = policy.select(&sim) {
        sim.simulate_in(row, col);
    }
    policy.expand(&sim);

    let sim = Simulate::new();
    let root = policy.map.get(&hash(&sim.board()));
    assert!(root.is_some());

    let root = root.unwrap();
    assert_eq!(root.visit, 1);
    assert_ne!(root.value, 0.);
    assert_eq!(root.q_value, 0.);
    assert_eq!(root.n_prob, 0.);
    assert_ne!(root.prob, [[0.; BOARD_SIZE]; BOARD_SIZE]);
    assert_eq!(root.num_player, 0);
    assert_eq!(root.board, sim.board());
    assert_eq!(root.next_node.len(), BOARD_CAPACITY);

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let sim = sim.simulate(i, j);
            let board = sim.board();
            let hashed = hash(&board);

            assert!(policy.map.contains_key(&hashed));
            assert!(root.next_node.contains(&hashed));

            let node = policy.map.get(&hashed).unwrap();
            assert_eq!(node.visit, 0);
            assert_eq!(node.value, 0.);
            assert_eq!(node.n_prob, root.prob[i][j]);
            assert_eq!(node.prob, [[0.; BOARD_SIZE]; BOARD_SIZE]);
            assert_eq!(node.num_player, 1);
            assert_eq!(node.next_node.len(), 0);
        }
    }
}

#[test]
fn test_update() {
    let game = Game::new();
    let sim = Simulate::from_game(&game);

    let rand_eval = Box::new(RandomEvaluator {});
    let mut policy = AlphaZero::new(rand_eval);
    policy.init(&sim);

    for _ in 0..2 {
        let mut sim = sim.deep_clone();
        let mut path = Vec::new();
        while let Some((row, col)) = policy.select(&sim) {
            sim.simulate_in(row, col);
            path.push((row, col));
        }
        policy.expand(&sim);
        policy.update(&sim, &path);
    }
    let node = policy.map.get(&hash(game.get_board()));
    assert!(node.is_some());

    let node = node.unwrap();
    assert_ne!(node.q_value, 0.);
    assert_eq!(node.visit, 2);
    assert_eq!(node.next_node.len(), BOARD_CAPACITY);

    let child_hashed = node
        .next_node
        .iter()
        .filter(|x| policy.map.get(x).unwrap().value != 0.)
        .collect::<Vec<_>>();
    assert_eq!(child_hashed.len(), 1);

    let child = policy.map.get(child_hashed[0]);
    assert!(child.is_some());

    let child = child.unwrap();
    assert_ne!(child.n_prob, 0.);
    assert_eq!(node.q_value * 2., child.value);

    let diff = diff_board(&node.board, &child.board);
    assert!(diff.is_some());

    let (row, col) = diff.unwrap();
    assert_eq!(node.prob[row][col], child.n_prob)
}

#[test]
fn test_policy() {
    let game = Game::new();
    let sim = Simulate::from_game(&game);

    let rand_eval = Box::new(RandomEvaluator {});
    let mut policy = AlphaZero::new(rand_eval);
    policy.init(&sim);

    for _ in 0..2 {
        let mut sim = sim.deep_clone();
        let mut path = Vec::new();
        while let Some((row, col)) = policy.select(&sim) {
            sim.simulate_in(row, col);
            path.push((row, col));
        }
        policy.expand(&sim);
        policy.update(&sim, &path);
    }

    let pos = policy.policy(&sim);
    assert!(pos.is_some());

    // validation
    let (row, col) = pos.unwrap();
    assert!(sim.validate(row, col));

    let pos = sim.possible().iter().position(|x| *x == (row, col));
    assert!(pos.is_some());

    let node = policy.map.get(&hash(&sim.board()));
    assert!(node.is_some());

    let child = node
        .unwrap()
        .next_node
        .iter()
        .filter(|x| policy.map.get(x).unwrap().value != 0.)
        .collect::<Vec<_>>();
    assert_eq!(child.len(), 1);

    let child_node = policy.map.get(child[0]).unwrap();
    let diff = diff_board(game.get_board(), &child_node.board);
    assert!(diff.is_some());
    assert_eq!((row, col), diff.unwrap());
}

#[test]
fn test_self_play() {
    let param = HyperParameter::light_weight();
    let rand_eval = Box::new(RandomEvaluator {});
    let mut policy = AlphaZero::with_param(rand_eval, param);
    let mut mcts = Agent::new(&mut policy);

    let now = Instant::now();
    let result = mcts.play();
    let done = now.elapsed();

    println!("{}.{}s elapsed", done.as_secs(), done.subsec_millis());
    let result = result.map_err(|_| assert!(false)).unwrap();
    if let Some(last) = result.path.last() {
        if result.winner != Player::None {
            assert_eq!(last.turn, result.winner);
        }
    }
    assert!(true);
}
