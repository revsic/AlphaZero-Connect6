#[derive(Copy, Clone)]
pub enum Player {
    Black,
    White,
    None,
}

pub struct Game {
    board: [[Player; 19]; 19],
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[Player::None; 19]; 19],
        }
    }

    pub fn play(&mut self) -> Player {
        Player::None
    }

    pub fn set(&mut self, row: char, column: char, player: Player) -> bool {
        if row >= 'a' && row <= 's'
            && column >= 'A' && column <= 'S' {
            let row_c = row as usize - 0x61;
            let col_c = column as usize - 0x41;

            self.board[row_c][col_c] = player;
            true
        }
        else {
            false
        }
    }

    pub fn print(&self) {
        println!("0 A B C D E F G H I J K L M N O P Q R S");
        for i in 0..19 {
            let alpha: char = ('a' as u8 + i as u8) as char;
            print!("{} ", alpha);
            for j in 0..19 {
                match self.board[i][j] {
                    Player::Black => print!("X "),
                    Player::White => print!("O "),
                    Player::None  => print!("_ "),
                }
            }
            println!();
        }
    }
}