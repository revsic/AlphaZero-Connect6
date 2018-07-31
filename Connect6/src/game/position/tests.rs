extern crate rand;

use super::*;

pub fn gen_pos() -> (Pos, char, char) {
    use self::rand::prelude::*;

    let mut rng = thread_rng();
    let mut gen_char = |base: u8| -> char {
        let idx: u8 = rng.gen_range(0, 19);
        (idx + base) as char
    };

    let rnd_row = gen_char(0x61);
    let rnd_col = gen_char(0x41);
    let query = {
        let mut base = String::new();
        base.push(rnd_row);
        base.push(rnd_col);
        base
    };

    let pos = match Pos::from(query.as_str()) {
        None => panic!(format!("internal exception at pos_tests::gen_pos : ({}, {})", rnd_row, rnd_col)),
        Some(pos) => pos
    };

    (pos, rnd_row, rnd_col)
}

#[cfg(test)]
mod row_tests {
    use super::rand;
    use super::Row;

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
    use super::rand;
    use super::Col;

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
    use super::{rand::random, gen_pos};
    use super::{Row, Col, Pos};

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