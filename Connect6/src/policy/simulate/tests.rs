use super::*;
use super::super::super::BOARD_CAPACITY;

use rand;

#[cfg(test)]
mod simulate_tests {
    use super::*;

    #[test]
    fn test_new() {
        let simulate = Simulate::new();
        assert_eq!(simulate.turn, Player::Black);
        assert_eq!(simulate.num_remain, 1);
        assert_eq!(simulate.pos, None);

        let node = simulate.node.borrow();
        assert_eq!(node.board, [[Player::None; BOARD_SIZE]; BOARD_SIZE]);
        assert_eq!(node.possible.len(), BOARD_CAPACITY);
    }

    #[test]
    fn test_from_game() {
        let game = Game::new();
        let simulate = Simulate::from_game(&game);

        assert_eq!(simulate.turn, Player::Black);
        assert_eq!(simulate.num_remain, 1);
        assert_eq!(simulate.pos, None);

        let node = simulate.node.borrow();
        assert_eq!(node.board, [[Player::None; BOARD_SIZE]; BOARD_SIZE]);
        assert_eq!(node.possible.len(), BOARD_CAPACITY);
    }

    #[test]
    fn test_deep_clone() {
        let simulate = Simulate::new();
        let cloned = simulate.deep_clone();

        { // borrow_mut simulate.node: Rc<RefCell<Node>>
            let mut node = simulate.node.borrow_mut();
            node.board[0][0] = Player::Black;
        }
        assert_eq!(simulate.board()[0][0], Player::Black);
        assert_eq!(cloned.board()[0][0], Player::None);
    }

    #[test]
    fn test_validate() {
        let game = Game::new();
        let simulate = Simulate::from_game(&game);
        assert!(simulate.validate(0, 0));

        { // borrow_mut simulate.node: Rc<RefCell<Node>>
            let mut node = simulate.node.borrow_mut();
            node.board[0][0] = Player::Black;
        }
        assert!(!simulate.validate(0, 0));

        let row = rand::random::<usize>() % 40 + 1;
        let col = rand::random::<usize>() % 40 + 1;
        assert_eq!(simulate.validate(row, col), row < BOARD_SIZE && col < BOARD_SIZE);
    }

    #[test]
    fn next_turn() {
        let mut simulate = Simulate::new();
        assert_eq!(simulate.next_turn(), Player::White);

        simulate.simulate_in(0, 0);
        assert_eq!(simulate.next_turn(), Player::White);

        simulate.simulate_in(0, 1);
        assert_eq!(simulate.next_turn(), Player::Black);
    }

    #[test]
    fn test_simulate() {
        let game = Game::new();
        let simulate = Simulate::from_game(&game);
        {
            let sim_aa = simulate.simulate(0, 0);
            let node = sim_aa.node.borrow();
            assert_eq!(node.board[0][0], Player::Black);
            assert_eq!(sim_aa.turn, Player::White);
            assert_eq!(sim_aa.num_remain, 2);

            let index = node.possible.iter().position(|x| *x == (0, 0));
            assert!(index.is_none());
        }
        let node = simulate.node.borrow();
        assert_eq!(node.board[0][0], Player::None);

        let index = node.possible.iter().position(|x| *x == (0, 0));
        assert!(index.is_some());
    }

    #[test]
    fn test_simulate_in() {
        let game = Game::new();
        let mut simulate = Simulate::from_game(&game);
        simulate.simulate_in(0, 0);

        let node = simulate.node.borrow();
        assert_eq!(node.board[0][0], Player::Black);
        assert_eq!(simulate.turn, Player::White);
        assert_eq!(simulate.num_remain, 2);

        let index = node.possible.iter().position(|x| *x == (0, 0));
        assert!(index.is_none());
    }

    #[test]
    fn test_rollback_in() {
        let game = Game::new();
        let mut simulate = Simulate::from_game(&game);

        simulate.simulate_in(0, 0);
        {
            let node = simulate.node.borrow();
            assert_eq!(node.board[0][0], Player::Black);
        }

        simulate.rollback_in(0, 0);
        assert_eq!(simulate.turn, Player::Black);
        assert_eq!(simulate.num_remain, 1);
        assert_eq!(simulate.pos, None);

        let node = simulate.node.borrow();
        assert_eq!(node.board, [[Player::None; BOARD_SIZE]; BOARD_SIZE]);
        assert_eq!(node.possible.len(), BOARD_CAPACITY);
    }
}