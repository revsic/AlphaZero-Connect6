extern crate connect6;
extern crate rand;

#[cfg(test)]
mod player_tests {
    use connect6::game::Player;

    #[test]
    fn test_switch() {
        let none = Player::None;
        assert_eq!(none.switch(), Player::None);

        let black = Player::Black;
        assert_eq!(black.switch(), Player::White);

        let white = Player::White;
        assert_eq!(white.switch(), Player::Black);
    }

    #[test]
    fn test_mut_switch() {
        let mut player = Player::None;

        player.mut_switch();
        assert_eq!(player, Player::None);

        player = Player::Black;
        player.mut_switch();
        assert_eq!(player, Player::White);

        player.mut_switch();
        assert_eq!(player, Player::Black);
    }
}

#[cfg(test)]
mod row_tests {
    use rand;
    use connect6::game::Row;

    #[test]
    fn test_to_char() {
        let rnd_char = rand::random::<char>();
        let row = Row(rnd_char);

        assert_eq!(rnd_char, row.to_char());
    }

    #[test]
    fn test_to_usize() {
        let rnd_char = rand::random::<char>();
        let row = Row(rnd_char);

        let idx = rnd_char as usize - 0x61;
        assert_eq!(idx, row.to_usize());
    }

    #[test]
    fn test_validate() {
        let rnd_char = rand::random::<char>();
        let row = Row(rnd_char);

        let is_valid = rnd_char >= 'a' && rnd_char <= 's';
        assert_eq!(is_valid, row.validate());
    }
}

#[cfg(test)]
mod col_tests {
    use rand;
    use connect6::game::Col;

    #[test]
    fn test_to_char() {
        let rnd_char = rand::random::<char>();
        let col = Col(rnd_char);

        assert_eq!(rnd_char, col.to_char());
    }

    #[test]
    fn test_to_usize() {
        let rnd_char = rand::random::<char>();
        let col = Col(rnd_char);

        let idx = rnd_char as usize - 0x41;
        assert_eq!(idx, col.to_usize());
    }

    #[test]
    fn test_validate() {
        let rnd_char = rand::random::<char>();
        let col = Col(rnd_char);

        let is_valid = rnd_char >= 'A' && rnd_char <= 'S';
        assert_eq!(is_valid, col.validate());
    }
}

#[cfg(test)]
mod pos_tests {
    use rand::prelude::*;
    use connect6::game::{Row, Col, Pos};

    macro_rules! concat_char {
        ( $( $x:expr ), * ) => {{
            let mut base = String::new();
            $(
                base.push($x);
            )*
            base
        }}
    }

    pub fn gen_pos() -> (Pos, char, char) {
        let mut rng = thread_rng();
        let mut gen_char = |base: u8| -> char {
            let idx: u8 = rng.gen_range(0, 19);
            (idx + base) as char
        };

        let rnd_row = gen_char(0x61);
        let rnd_col = gen_char(0x41);
        let query = concat_char!(rnd_row, rnd_col);

        let pos = match Pos::from(query.as_str()) {
            None => panic!(format!("internal exception at pos_tests::gen_pos : ({}, {})", rnd_row, rnd_col)),
            Some(pos) => pos
        };

        (pos, rnd_row, rnd_col)
    }

    #[test]
    fn test_from() {
        fn validate(query: &str, row: char, col: char) {
            match Pos::from(query) {
                None => assert!(false),
                Some(pos) => assert_eq!(pos.to_char(), (row, col))
            }
        }

        validate("aA", 'a', 'A');
        validate("sS", 's', 'S');

        assert!(Pos::from("zZ").is_none());
        assert!(Pos::from("Aa").is_none());
        assert!(Pos::from("11").is_none());
        assert!(Pos::from("aAaA").is_none());
    }

    #[test]
    fn test_to_char() {
        let (pos, row, col) = gen_pos();
        assert_eq!(pos.to_char(), (row, col));
    }

    #[test]
    fn test_to_usize() {
        let (pos, chr_row, chr_col) = gen_pos();
        let row = Row(chr_row);
        let col = Col(chr_col);

        assert_eq!(pos.to_usize(), (row.to_usize(), col.to_usize()));
    }

    #[test]
    fn test_validate() {
        let rnd_row = random::<char>();
        let row = Row(rnd_row);

        let rnd_col = random::<char>();
        let col = Col(rnd_col);

        let pos = Pos(row, col);
        assert_eq!(pos.validate(), row.validate() && col.validate());
    }
}

#[cfg(test)]
mod game_tests {
    use pos_tests::gen_pos;
    use connect6::game::*;

    #[test]
    fn test_set() {
        let mut game = Game::new();

        let (pos, _, _) = gen_pos();
        let (row, col) = pos.to_usize();

        assert_eq!(game.board[row][col], Player::None);
        assert!(game.set(pos, Player::White));
        assert_eq!(game.board[row][col], Player::White);
        assert!(! game.set(pos, Player::Black));

        let pos = Pos(Row('z'), Col('Z'));
        assert!(! game.set(pos, Player::Black));
    }

    #[test]
    fn test_play() {
        let mut game = Game::new();
        let result = match game.play("aA") {
            Ok(res) => res,
            Err(_) => {
                assert!(false);
                PlayResult::new()
            },
        };

        let expected = PlayResult {
            player: Player::Black,
            num_remain: 0,
            position: Pos::from("aA").unwrap(),
        };

        assert_eq!(result, expected);
        assert_eq!(game.turn, Player::White);
        assert_eq!(game.num_remain, 2);
        assert_eq!(game.board[0][0], Player::Black);

        match game.play("aA") {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "Already set position"),
        };

        match game.play("AA") {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "Invalid Query"),
        };

    }
}