extern crate rand;

use super::*;

#[cfg(test)]
mod root_tests {
    use super::*;

    #[test]
    fn test_new() {
        let root = Root::new();
        assert_eq!(root.turn, Player::Black);
        assert_eq!(root.num_remain, 1);
        assert_eq!(root.board, [[Player::None; 19]; 19]);
        assert_eq!(root.possible.len(), 19 * 19);
    }

    #[test]
    fn test_from_game() {
        let game = Game::new();
        let root = Root::from_game(&game);

        assert_eq!(game.get_turn(), root.turn);
        assert_eq!(game.get_remain(), root.num_remain);
        assert_eq!(*game.get_board(), root.board);
    }

    #[test]
    fn test_to_simulate() {
        let game = Game::new();
        let mut root = Root::from_game(&game);

        let turn = root.turn;
        let num_remain = root.num_remain;
        {
            let simulate = root.to_simulate();
            assert_eq!(turn, simulate.turn);
            assert_eq!(num_remain, simulate.num_remain);

            simulate.board[0][0] = Player::White;
        }
        assert_eq!(root.board[0][0], Player::White);
    }
}

#[cfg(test)]
mod simulate_tests {
    use super::*;

    #[test]
    fn test_validate() {
        let game = Game::new();
        let mut root = Root::from_game(&game);

        let simulate = root.to_simulate();
        assert!(simulate.validate(0, 0));

        simulate.board[0][0] = Player::Black;
        assert!(!simulate.validate(0, 0));

        let row = rand::random::<usize>();
        let col = rand::random::<usize>();

        simulate.board[0][0] = Player::None;
        assert_eq!(simulate.validate(row, col), row < 19 && col < 19);
    }

    #[test]
    fn test_simulate() {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let mut simulate = root.to_simulate();
        {
            let sim_aA = simulate.simulate(0, 0);
            assert_eq!(sim_aA.board[0][0], Player::Black);
            assert_eq!(sim_aA.turn, Player::White);
            assert_eq!(sim_aA.num_remain, 2);

            let index = sim_aA.possible.iter().position(|x| *x == (0, 0));
            assert!(index.is_none());
        }
        assert_eq!(simulate.board[0][0], Player::None);

        let index = simulate.possible.iter().position(|x| *x == (0, 0));
        assert!(index.is_some());
    }

    #[test]
    fn test_simulate_mut() {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let mut simulate = root.to_simulate();

        simulate.simulate_mut(0, 0);
        assert_eq!(simulate.board[0][0], Player::Black);
        assert_eq!(simulate.turn, Player::White);
        assert_eq!(simulate.num_remain, 2);

        let index = simulate.possible.iter().position(|x| *x == (0, 0));
        assert!(index.is_none());
    }

    #[test]
    fn test_back_recover() {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let mut simulate = root.to_simulate();

        let backup = simulate.backup();
        simulate.simulate_mut(0, 0);

        simulate.recover(backup);
        assert_eq!(simulate.board[0][0], Player::None);
        assert_eq!(simulate.turn, Player::Black);
        assert_eq!(simulate.num_remain, 1);
        assert!(simulate.pos.is_none());
        assert_eq!(simulate.possible.len(), 19 * 19);
    }
}