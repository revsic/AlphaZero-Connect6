extern crate rand;

use super::*;

#[cfg(test)]
mod root_tests {
    use super::*;

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
mod simulate_test {
    use super::*;

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

    fn test_simulate() {
        let game = Game::new();
        let mut root = Root::from_game(&game);
        let mut simulate = root.to_simulate();

        {
            let sim_aA = simulate.simulate(0, 0);
            assert_eq!(sim_aA.board[0][0], Player::Black);

            let index = sim_aA.possible.iter().position(|x| *x == (0, 0));
            assert!(index.is_none());
        }
        assert_eq!(simulate.board[0][0], Player::None);

        let index = simulate.possible.iter().position(|x| *x == (0, 0));
        assert!(index.is_some());
    }
}