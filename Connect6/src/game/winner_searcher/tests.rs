extern crate rand;

use super::*;

#[cfg(test)]
mod block_tests {
    use super::*;

    fn rand_cumulative() -> Cumulative {
        Cumulative {
            right: rand::random::<i32>(),
            down: rand::random::<i32>(),
            right_down: rand::random::<i32>(),
            left_down: rand::random::<i32>(),
        }
    }

    fn rand_block() -> Block {
        Block {
            flag: 0,
            mem: [[rand_cumulative(); 21]; 2],
        }
    }

    fn get_tuple(block: &Block) -> ([Cumulative; 21], [Cumulative; 21]) {
        let (prev, now) = block.as_tuple();
        (*prev, *now)
    }

    #[test]
    fn test_block_new() {
        let block = Block::new();
        assert_eq!(block.flag, 0);
        assert_eq!(block.mem, [[Cumulative::new(); 21]; 2]);
    }

    #[test]
    fn test_as_tuple() {
        let mut block = Block::new();
        block.mem[0][0] = rand_cumulative();
        block.mem[1][0] = rand_cumulative();

        let (prev, now) = get_tuple(&block);

        block.flag = 1;
        let (s_prev, s_now) = block.as_tuple();
        assert_eq!(prev[0], s_now[0]);
        assert_eq!(s_prev[0], now[0]);
    }

    #[test]
    fn test_get_prev() {
        let block = rand_block();
        let (prev, now) = get_tuple(&block);

        let right = *block.get_prev(1, &Path::Right);
        assert_eq!(right, now[0]);

        let down = *block.get_prev(1, &Path::Down);
        assert_eq!(down, prev[1]);

        let right_down = *block.get_prev(1, &Path::RightDown);
        assert_eq!(right_down, prev[0]);

        let left_down = *block.get_prev(1, &Path::LeftDown);
        assert_eq!(left_down, prev[2]);
    }

    #[test]
    fn test_update_now() {
        let mut block = Block::new();
        let crand = rand_cumulative();

        block.update_now(|now| now[0] = crand.clone());

        let (_, now) = block.as_tuple();
        assert_eq!(now[0], crand);
    }

    #[test]
    fn test_update_row() {
        let mut block = Block::new();
        let crand = [rand_cumulative(); 21];

        block.mem[1] = crand.clone();
        block.update_row();

        let (prev, now) = block.as_tuple();
        assert_eq!(*prev, crand);
        assert_eq!(*now, [Cumulative::new(); 21]);
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;

    fn new_table() -> [[Player; 19]; 19] {
        [[Player::None; 19]; 19]
    }

    #[test]
    fn test_base_search() {
        let mut table = new_table();
        // Right
        for i in 0..5 {
            table[10][5+i] = Player::Black;
        }
        assert_eq!(search(&table), Player::None);

        table[10][10] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        // Down
        for i in 0..5 {
            table[5+i][10] = Player::White;
        }
        assert_eq!(search(&table), Player::Black);

        table[10][10] = Player::White;
        assert_eq!(search(&table), Player::White);

        // RightDown
        for i in 0..5 {
            table[5+i][5+i] = Player::Black;
        }
        assert_eq!(search(&table), Player::White);

        table[10][10] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        // LeftDown
        for i in 0..5 {
            table[5+i][15-i] = Player::White;
        }
        assert_eq!(search(&table), Player::Black);

        table[10][10] = Player::White;
        assert_eq!(search(&table), Player::White);
    }

    #[test]
    fn test_boundary_search() {
        // boundary test
        let mut table = new_table();
        for i in 0..5 {
            table[0][i] = Player::White; // 0 ~ 4
            table[0][i + 7] = Player::White; // 7 ~ 11
            table[0][18 - i] = Player::White; // 14 ~ 18
        }
        assert_eq!(search(&table), Player::None);

        table[0][5] = Player::White; // 0 ~ 5
        assert_eq!(search(&table), Player::White);

        table[0][5] = Player::None;
        table[0][12] = Player::White; // 7 ~ 12
        assert_eq!(search(&table), Player::White);

        table[0][12] = Player::None;
        table[0][13] = Player::White; // 13 ~ 18
        assert_eq!(search(&table), Player::White);

        let mut table = new_table();
        for i in 0..5 {
            table[18][i] = Player::White;
            table[18][i + 7] = Player::White;
            table[18][18 - i] = Player::White;
        }
        assert_eq!(search(&table), Player::None);

        table[18][5] = Player::White;
        assert_eq!(search(&table), Player::White);

        table[18][5] = Player::None;
        table[18][12] = Player::White;
        assert_eq!(search(&table), Player::White);

        table[18][12] = Player::None;
        table[18][13] = Player::White;
        assert_eq!(search(&table), Player::White);

        let mut table = new_table();
        for i in 0..5 {
            table[i][0] = Player::Black;
            table[i + 7][0] = Player::Black;
            table[18 - i][0] = Player::Black;
        }
        assert_eq!(search(&table), Player::None);

        table[5][0] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[5][0] = Player::None;
        table[12][0] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[12][0] = Player::None;
        table[13][0] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        let mut table = new_table();
        for i in 0..5 {
            table[i][18] = Player::Black;
            table[i + 7][18] = Player::Black;
            table[18 - i][18] = Player::Black;
        }
        assert_eq!(search(&table), Player::None);

        table[5][18] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[5][18] = Player::None;
        table[12][18] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[12][18] = Player::None;
        table[13][18] = Player::Black;
        assert_eq!(search(&table), Player::Black);
    }

    #[test]
    fn test_cross_search() {
        let mut table = new_table();
        for i in 0..5 {
            table[i][i] = Player::Black; // 0 ~ 4
            table[i + 7][i + 7] = Player::Black; // 7 ~ 11
            table[18 - i][18 - i] = Player::Black; // 14 ~ 18
        }
        assert_eq!(search(&table), Player::None);

        table[5][5] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[5][5] = Player::None;
        table[12][12] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[12][12] = Player::None;
        table[13][13] = Player::Black;
        assert_eq!(search(&table), Player::Black);

        table[13][13] = Player::None;
        for i in 0..5 {
            table[i][18 - i] = Player::White;
            table[i + 7][11 - i] = Player::White;
            table[18 - i][i] = Player::White;
        }
        assert_eq!(search(&table), Player::None);

        table[5][13] = Player::White;
        assert_eq!(search(&table), Player::White);

        table[5][13] = Player::None;
        table[12][6] = Player::White;
        assert_eq!(search(&table), Player::White);

        table[12][6] = Player::None;
        table[13][5] = Player::White;
        assert_eq!(search(&table), Player::White);
    }
}