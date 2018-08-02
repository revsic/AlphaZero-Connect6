use super::super::game::*;

#[cfg(test)]
mod tests;

pub struct Root {
    turn: Player,
    num_remain: i32,
    board: [[Player; 19]; 19],
    possible: Vec<(usize, usize)>,
}

pub struct Simulate<'a> {
    turn: Player,
    num_remain: i32,
    pos: Option<(usize, usize)>,
    board: &'a mut [[Player; 19]; 19],
    possible: &'a mut Vec<(usize, usize)>,
}

impl Root {
    pub fn from_game(game: &Game) -> Root {
        let possible = {
            let lower: Vec<usize> = (0..19).map(|x| x + 0x61).collect();
            let upper: Vec<usize> = (0..19).map(|x| x + 0x41).collect();

            lower.iter().cloned()
                 .flat_map(
                     |x| upper.iter().map(move |y| (x, *y)))
                 .collect()
        };

        Root {
            turn: game.get_turn(),
            num_remain: game.get_remain(),
            board: *game.get_board(),
            possible: possible,
        }
    }

    pub fn to_simulate(&mut self) -> Simulate {
        Simulate {
            turn: self.turn,
            num_remain: self.num_remain,
            pos: None,
            board: &mut self.board,
            possible: &mut self.possible,
        }
    }
}

impl<'a> Simulate<'a> {
    pub fn validate(&self, row: usize, col: usize) -> bool {
        if row >= 19 || col >= 19 {
            return false;
        }
        if self.board[row][col] == Player::None {
            return false;
        }
        true
    }

    pub fn simulate(&mut self, row: usize, col: usize) -> Simulate {
        let pos = (row, col);
        let item = self.possible.iter()
            .position(|x| *x == pos);
        self.possible.remove(item.unwrap());

        self.board[row][col] = self.turn;

        let (turn, num_remain) = {
            if self.num_remain <= 1 {
                (self.turn.switch(), 2)
            } else {
                (self.turn, 1)
            }
        };

        Simulate {
            turn,
            num_remain,
            pos: Some(pos),
            board: self.board,
            possible: self.possible,
        }
    }
}

impl<'a> Drop for Simulate<'a> {
    fn drop(&mut self) {
        if let Some((row, col)) = self.pos {
            self.possible.push((row, col));
            self.board[row][col] = Player::None;
        }
    }
}