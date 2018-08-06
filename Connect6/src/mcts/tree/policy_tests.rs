use super::*;

pub fn test_select(policy: &mut impl Policy) {
    let game = Game::new();
    let sim = Simulate::from_game(&game);
    policy.init(&sim);

    loop {
        if let Some((row, col)) = policy.select(&sim) {
            let node = sim.node.borrow();
            let pos = node.possible.iter().position(|x| *x == (row, col));

            assert!(pos.is_some());
            assert!(sim.validate(row, col));
            break;
        }
            else {
                let pos = policy.select(&sim);
                assert!(pos.is_none());

                let (row, col) = policy.expand(&sim);
                let expanded = sim.simulate(row, col);
                let path = vec![(row, col)];
                policy.update(&expanded, &path);
            }
    }
}

pub fn test_expand(policy: &mut impl Policy) {
    let game = Game::new();
    let mut sim = Simulate::from_game(&game);
    policy.init(&sim);

    while let Some((row, col)) = policy.select(&sim) {
        sim.simulate_in(row, col);
    }
    let (row, col) = policy.expand(&sim);

    let node = sim.node.borrow();
    let pos = node.possible.iter().position(|x| *x == (row, col));

    assert!(pos.is_some());
    assert!(sim.validate(row, col));
}

pub fn test_update(policy: &mut impl Policy) {
    let game = Game::new();
    let mut sim = Simulate::from_game(&game);
    policy.init(&sim);

    let mut path = Vec::new();
    while let Some((row, col)) = policy.select(&sim) {
        sim.simulate_in(row, col);
        path.push((row, col));
    }
    let (row, col) = policy.expand(&sim);
    let expanded = sim.simulate(row, col);
    path.push((row, col));

    policy.update(&expanded, &path);
    assert!(true);
}

pub fn test_get_policy(policy: &mut impl Policy) {
    let game = Game::new();
    let sim = Simulate::from_game(&game);
    policy.init(&sim);

    let (row, col) = policy.policy(&sim);
    assert!(sim.validate(row, col));

    let node = sim.node.borrow();
    let pos = node.possible.iter().position(|x| *x == (row, col));
    assert!(pos.is_some());
}