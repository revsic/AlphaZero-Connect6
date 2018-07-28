#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Black,
    White,
    None,
}

impl Player {
    pub fn switch(&self) -> Player {
        match self {
            &Player::Black => Player::White,
            &Player::White => Player::Black,
            &Player::None => Player::None
        }
    }

    pub fn mut_switch(&mut self) {
        *self = self.switch();
    }
}

#[derive(Copy, Clone)]
pub struct Row(char);

impl Row {
    pub fn to_char(&self) -> char {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize - 0x61
    }

    pub fn validate(&self) -> bool {
        self.0 >= 'a' && self.0 <= 's'
    }
}

#[derive(Copy, Clone)]
pub struct Col(char);

impl Col {
    pub fn to_char(&self) -> char {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize - 0x41
    }

    pub fn validate(&self) -> bool {
        self.0 >= 'A' && self.0 <= 'S'
    }
}

#[derive(Copy, Clone)]
pub struct Pos(Row, Col);

impl Pos {
    pub fn with_str(query: &String) -> Option<Pos> {
        if query.len() != 2 {
            return None;
        }

        let mut qchars = query.chars();
        let row = match qchars.next() {
            Some(c) => Row(c),
            None => return None
        };
        let col = match qchars.next() {
            Some(c) => Col(c),
            None => return None
        };
        let pos = Pos(row, col);
        if !pos.validate() {
            return None;
        }

        Some(pos)
    }

    pub fn to_char(&self) -> (char, char) {
        (self.0.to_char(), self.1.to_char())
    }

    pub fn to_usize(&self) -> (usize, usize) {
        (self.0.to_usize(), self.1.to_usize())
    }

    pub fn validate(&self) -> bool {
        self.0.validate() && self.1.validate()
    }
}

pub struct PlayResult {
    player: Player,
    num_remain: i32,
    position: Pos,
}

impl PlayResult {
    fn with_game(game: &Game, position: Pos) -> PlayResult {
        PlayResult {
            player: game.turn,
            num_remain: game.num_remain,
            position,
        }
    }
}

pub struct Game {
    turn: Player,
    num_remain: i32,
    board: [[Player; 19]; 19],
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Player::Black,
            num_remain: 1,
            board: [[Player::None; 19]; 19],
        }
    }

    pub fn play(&mut self, query: String) -> Result<PlayResult, &'static str> {
        let position = match Pos::with_str(&query) {
            Some(pos) => pos,
            None => return Err("Invalid Query")
        };

        let player = self.turn;
        let ok = self.set(position, player);
        if !ok {
            return Err("Already set position");
        }

        let result = PlayResult::with_game(self, position);
        self.num_remain -= 1;
        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }

        return Ok(result);
    }

    pub fn set(&mut self, pos: Pos, player: Player) -> bool {
        if !pos.validate() {
            return false;
        }

        let (row, col) = pos.to_usize();
        if self.board[row][col] != Player::None {
            return false;
        }

        self.board[row][col] = player;
        true
    }

    pub fn print(&self) {
        fn idx2alpha(idx: usize) -> char {
            return ('a' as u8 + idx as u8) as char;
        }

        println!("0 A B C D E F G H I J K L M N O P Q R S");
        for i in 0..19 {
            print!("{} ", idx2alpha(i));
            for j in 0..19 {
                match self.board[i][j] {
                    Player::Black => print!("X "),
                    Player::White => print!("O "),
                    Player::None => print!("_ "),
                }
            }
            println!();
        }
    }

    fn is_game_end(&self) -> bool {
        false
    }
}