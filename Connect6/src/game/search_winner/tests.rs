//extern crate rand;
//
//use super::*;
//
//#[cfg(test)]
//mod block_tests {
//    use super::*;
//
//    fn rand_cumulative() -> Cumulative {
//        Cumulative {
//            right: rand::random::<i32>(),
//            down: rand::random::<i32>(),
//            right_down: rand::random::<i32>(),
//            left_down: rand::random::<i32>(),
//        }
//    }
//
//    fn rand_block() -> Block {
//        Block {
//            flag: 0,
//            mem: [[rand_cumulative(); BOARD_SIZE+2]; 2],
//        }
//    }
//
//    fn get_tuple(block: &Block) -> ([Cumulative; BOARD_SIZE+2], [Cumulative; BOARD_SIZE+2]) {
//        let (prev, now) = block.as_tuple();
//        (*prev, *now)
//    }
//
//    #[test]
//    fn test_block_new() {
//        let block = Block::new();
//        assert_eq!(block.flag, 0);
//        assert_eq!(block.mem, [[Cumulative::new(); BOARD_SIZE+2]; 2]);
//    }
//
//    #[test]
//    fn test_as_tuple() {
//        let mut block = Block::new();
//        block.mem[0][0] = rand_cumulative();
//        block.mem[1][0] = rand_cumulative();
//
//        let (prev, now) = get_tuple(&block);
//
//        block.flag = 1;
//        let (s_prev, s_now) = block.as_tuple();
//        assert_eq!(prev[0], s_now[0]);
//        assert_eq!(s_prev[0], now[0]);
//    }
//
//    #[test]
//    fn test_get_prev() {
//        let block = rand_block();
//        let (prev, now) = get_tuple(&block);
//
//        let right = *block.get_prev(1, &Path::Right);
//        assert_eq!(right, now[0]);
//
//        let down = *block.get_prev(1, &Path::Down);
//        assert_eq!(down, prev[1]);
//
//        let right_down = *block.get_prev(1, &Path::RightDown);
//        assert_eq!(right_down, prev[0]);
//
//        let left_down = *block.get_prev(1, &Path::LeftDown);
//        assert_eq!(left_down, prev[2]);
//    }
//
//    #[test]
//    fn test_update_now() {
//        let mut block = Block::new();
//        let crand = rand_cumulative();
//
//        block.update_now(|now| now[0] = crand.clone());
//
//        let (_, now) = block.as_tuple();
//        assert_eq!(now[0], crand);
//    }
//
//    #[test]
//    fn test_update_row() {
//        let mut block = Block::new();
//        let crand = [rand_cumulative(); BOARD_SIZE+2];
//
//        block.mem[1] = crand.clone();
//        block.update_row();
//
//        let (prev, now) = block.as_tuple();
//        assert_eq!(*prev, crand);
//        assert_eq!(*now, [Cumulative::new(); BOARD_SIZE+2]);
//    }
//}
//
//#[cfg(test)]
//mod search_tests {
//    use super::*;
//
//    fn new_table() -> Board {
//        [[Player::None; BOARD_SIZE]; BOARD_SIZE]
//    }
//
//    #[test]
//    fn test_base_search() {
//        let mut table = new_table();
//        // Right
//        for i in 0..5 {
//            table[0][i] = Player::Black;
//        }
//        assert_eq!(search(&table), Player::None);
//
//        table[0][5] = Player::Black;
//        assert_eq!(search(&table), Player::Black);
//
//        // Down
//        for i in 0..5 {
//            table[5-i][0] = Player::White;
//        }
//        assert_eq!(search(&table), Player::Black);
//
//        table[0][0] = Player::White;
//        assert_eq!(search(&table), Player::White);
//
//        // RightDown
//        for i in 0..5 {
//            table[5-i][5-i] = Player::Black;
//        }
//        assert_eq!(search(&table), Player::White);
//
//        table[0][0] = Player::Black;
//        assert_eq!(search(&table), Player::Black);
//
//        // LeftDown
//        for i in 0..5 {
//            table[5-i][i] = Player::White;
//        }
//        assert_eq!(search(&table), Player::Black);
//
//        table[0][5] = Player::White;
//        assert_eq!(search(&table), Player::White);
//    }
//
//    macro_rules! boundary_test {
//        (row => $table:ident, $base:expr, $init:expr) => {
//            let mut $table = new_table();
//            for i in 0..5 {
//                $table[$base][$init + i] = Player::White;
//            }
//            assert_eq!(search(&$table), Player::None);
//
//            $table[$base][$init + 5] = Player::White;
//            assert_eq!(search(&$table), Player::White);
//        };
//        (col => $table:ident, $base:expr, $init:expr) => {
//            let mut $table = new_table();
//            for i in 0..5 {
//                $table[$init + i][$base] = Player::Black;
//            }
//            assert_eq!(search(&$table), Player::None);
//
//            $table[$init + 5][$base] = Player::Black;
//            assert_eq!(search(&$table), Player::Black);
//        };
//    }
//
//    #[test]
//    fn test_variadic_boundary_search() {
//        // boundary test
//        boundary_test!(row => table, 0, 0);
//        boundary_test!(row => table, 0, 1);
//        boundary_test!(row => table, 0, BOARD_SIZE-6);
//
//        boundary_test!(row => table, BOARD_SIZE-1, 0);
//        boundary_test!(row => table, BOARD_SIZE-1, 1);
//        boundary_test!(row => table, BOARD_SIZE-1, BOARD_SIZE-6);
//
//        boundary_test!(col => table, 0, 0);
//        boundary_test!(col => table, 0, 1);
//        boundary_test!(col => table, 0, BOARD_SIZE-6);
//
//        boundary_test!(col => table, BOARD_SIZE-1, 0);
//        boundary_test!(col => table, BOARD_SIZE-1, 1);
//        boundary_test!(col => table, BOARD_SIZE-1, BOARD_SIZE-6);
//    }
//
//    macro_rules! cross_test {
//        (right_down => $table:ident, $init:expr) => {
//            let mut $table = new_table();
//            for i in 0..5 {
//                $table[$init + i][$init + i] = Player::White;
//            }
//            assert_eq!(search(&$table), Player::None);
//
//            $table[$init + 5][$init + 5] = Player::White;
//            assert_eq!(search(&$table), Player::White);
//        };
//        (left_down => $table:ident, $init:expr) => {
//            let mut $table = new_table();
//            for i in 0..5 {
//                $table[$init + i][BOARD_SIZE - 1 - $init - i] = Player::Black;
//            }
//            assert_eq!(search(&$table), Player::None);
//
//            $table[$init + 5][BOARD_SIZE - $init - 6] = Player::Black;
//            assert_eq!(search(&$table), Player::Black);
//        };
//    }
//
//    #[test]
//    fn test_variadic_cross_search() {
//        cross_test!(right_down => table, 0);
//        cross_test!(right_down => table, 1);
//        cross_test!(right_down => table, BOARD_SIZE-6);
//
//        cross_test!(left_down => table, 0);
//        cross_test!(left_down => table, 1);
//        cross_test!(left_down => table, BOARD_SIZE-6);
//    }
//}