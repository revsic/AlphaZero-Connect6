//use super::*;
//use super::super::super::BOARD_CAPACITY;
//
//use std::time::Instant;
//
//py_class!(class PyPolicy |py| {
//    def __call__(&self, arg: PyObject) -> PyResult<PyObject> {
//        let len = arg.cast_into::<PyList>(py)?.len(py);
//
//        let value = (0..len)
//            .map(|_| rand::random::<f32>().to_py_object(py).into_object())
//            .collect::<Vec<PyObject>>();
//        let value = PyList::new(py, value.as_slice()).into_object();
//
//        let policy = (0..len).map(|_| {
//            let rand_policy = (0..BOARD_CAPACITY)
//                .map(|_| rand::random::<f32>().to_py_object(py).into_object())
//                .collect::<Vec<PyObject>>();
//            PyList::new(py, rand_policy.as_slice()).into_object()
//        }).collect::<Vec<PyObject>>();
//
//        let policy = PyList::new(py, policy.as_slice()).into_object();
//        Ok(PyTuple::new(py, &[value, policy]).into_object())
//    }
//});
//
//macro_rules! py_policy {
//    ($py:ident, $py_policy:ident) => {
//        let gil = Python::acquire_gil();
//        let $py = gil.python();
//        let $py_policy = PyPolicy::create_instance($py).unwrap().into_object();
//    }
//}
//
//#[test]
//fn test_select() {
//    py_policy!(py, py_policy);
//    let game = Game::new();
//    let mut sim = Simulate::from_game(&game);
//
//    let mut policy = AlphaZero::new(py, py_policy);
//    policy.init(&sim);
//
//    let mut path = Vec::new();
//    while let Some((row, col)) = policy.select(&sim) {
//        { // borrow sim: mut Simulate
//           let node = sim.node.borrow();
//            let pos = node.possible.iter().position(|x| *x == (row, col));
//            assert!(pos.is_some());
//            assert!(sim.validate(row, col));
//        }
//        path.push((row, col));
//        sim.simulate_in(row, col);
//    }
//
//    policy.expand(&sim);
//    policy.update(&sim, &path);
//
//    let pos = policy.select(&sim);
//    assert!(pos.is_some());
//
//    let (row, col) = pos.unwrap();
//    assert!(sim.validate(row, col));
//
//    let node = sim.node.borrow();
//    let pos = node.possible.iter().position(|x| *x == (row, col));
//    assert!(pos.is_some());
//}
//
//#[test]
//fn test_expand() {
//    py_policy!(py, py_policy);
//    let mut param = HyperParameter::default();
//    param.num_expansion = BOARD_CAPACITY;
//
//    let game = Game::new();
//    let mut sim = Simulate::from_game(&game);
//
//    let mut policy = AlphaZero::with_param(py, py_policy, param);
//    policy.init(&sim);
//
//    while let Some((row, col)) = policy.select(&sim) {
//        sim.simulate_in(row, col);
//    }
//    policy.expand(&sim);
//
//    let sim = Simulate::new();
//    let root = policy.map.get(&hash(&sim.board()));
//    assert!(root.is_some());
//
//    let root = root.unwrap();
//    assert_eq!(root.visit, 0);
//    assert_ne!(root.value, 0.);
//    assert_eq!(root.q_value, 0.);
//    assert_eq!(root.n_prob, 0.);
//    assert_ne!(root.prob, [[0.; BOARD_SIZE]; BOARD_SIZE]);
//    assert_eq!(root.num_player, 0);
//    assert_eq!(root.board, sim.board());
//    assert_eq!(root.next_node.len(), BOARD_CAPACITY);
//
//    for i in 0..BOARD_SIZE {
//        for j in 0..BOARD_SIZE {
//            let sim = sim.simulate(i, j);
//            let board = sim.board();
//            let hashed = hash(&board);
//
//            assert!(policy.map.contains_key(&hashed));
//            assert!(root.next_node.contains(&hashed));
//
//            let node = policy.map.get(&hashed).unwrap();
//            assert_eq!(node.visit, 1);
//            assert_ne!(node.value, 0.);
//            assert_eq!(node.n_prob, root.prob[i][j]);
//            assert_ne!(node.prob, [[0.; BOARD_SIZE]; BOARD_SIZE]);
//            assert_eq!(node.num_player, 1);
//            assert_eq!(node.next_node.len(), 0);
//        }
//    }
//}
//
//#[test]
//fn test_update() {
//    py_policy!(py, py_policy);
//    let game = Game::new();
//    let mut sim = Simulate::from_game(&game);
//    let mut policy = AlphaZero::new(py, py_policy);
//
//    policy.init(&sim);
//    let mut path = Vec::new();
//    while let Some((row, col)) = policy.select(&sim) {
//        sim.simulate_in(row, col);
//        path.push((row, col));
//    }
//    policy.expand(&sim);
//    policy.update(&sim, &path);
//
//    let node = policy.map.get(&hash(&[[Player::None; BOARD_SIZE]; BOARD_SIZE]));
//    assert!(node.is_some());
//
//    let node = node.unwrap();
//    assert_ne!(node.q_value, 0.);
//    assert_eq!(node.visit, 1);
//}
//
//#[test]
//fn test_get_policy() {
//    py_policy!(py, py_policy);
//    let game = Game::new();
//    let mut sim = Simulate::from_game(&game);
//    let mut policy = AlphaZero::new(py, py_policy);
//
//    policy.init(&sim);
//    let mut path = Vec::new();
//    while let Some((row, col)) = policy.select(&sim) {
//        sim.simulate_in(row, col);
//        path.push((row, col));
//    }
//    policy.expand(&sim);
//    policy.update(&sim, &path);
//
//    let sim = Simulate::from_game(&game);
//    // let (row, col) = policy.get_policy(&game);
//    let (row, col) = policy.policy(&sim).unwrap();
//    assert!(sim.validate(row, col));
//
//    let node = sim.node.borrow();
//    let pos = node.possible.iter().position(|x| *x == (row, col));
//    assert!(pos.is_some());
//}
//
//#[test]
//fn test_self_play() {
//    py_policy!(py, py_policy);
//    let mut param = HyperParameter::default();
//    param.num_simulation = 10;
//    param.num_expansion = 1;
//
//    let mut policy = AlphaZero::with_param(py, py_policy, param);
//    let mut mcts = SinglePolicyMCTS::new(&mut policy);
//
//    let now = Instant::now();
//    let result = mcts.run();
//    let done = now.elapsed().as_secs();
//
//    println!("{} elapsed", done);
//    if let Some(last) = result.path.last() {
//        if result.winner != Player::None {
//            assert_eq!(last.turn, result.winner);
//        }
//    }
//    assert!(true);
//}