use super::{Player, Game};

#[cfg(test)]
mod tests;

enum Path {
    Right,
    Down,
    RightDown,
    LeftDown,
}

impl Path {
    fn delta(&self) -> (i32, i32) {
        match self {
            &Path::Right => (0, -1),
            &Path::Down => (-1, 0),
            &Path::RightDown => (-1, -1),
            &Path::LeftDown => (-1, 1),
        }
    }

    fn validate(&self, row: usize, col: usize) -> bool {
        match self {
            &Path::Right => col > 0,
            &Path::Down => row > 0,
            &Path::RightDown => row > 0 && col > 0,
            &Path::LeftDown => row > 0 && col < 18,
        }
    }

    fn apply(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let (dr, dc) = self.delta();
        let row = (row as i32 + dr) as usize;
        let col = (col as i32 + dc) as usize;

        if self.validate(row, col) {
            Some((row, col))
        } else {
            None
        }
    }

    fn at_least(&self, row: usize, col: usize) -> bool {
        match self {
            &Path::Right => col >= 5,
            &Path::Down => row >= 5,
            &Path::RightDown => row >= 5 && col >= 5,
            &Path::LeftDown => row >= 5 && col <= 13,
        }
    }
}

#[derive(Copy, Clone)]
struct Cumulative {
    right: i32,
    down: i32,
    right_down: i32,
    left_down: i32,
}

impl Cumulative {
    fn new() -> Cumulative {
        Cumulative {
            right: 0,
            down: 0,
            right_down: 0,
            left_down: 0,
        }
    }

    fn one() -> Cumulative {
        Cumulative {
            right: 1,
            down: 1,
            right_down: 1,
            left_down: 1,
        }
    }

    fn get(&self, path: &Path) -> i32 {
        match path {
            &Path::Right => self.right,
            &Path::Down => self.down,
            &Path::RightDown => self.right_down,
            &Path::LeftDown => self.left_down,
        }
    }

    fn mut_get(&mut self, path: &Path) -> &mut i32 {
        match path {
            &Path::Right => &mut self.right,
            &Path::Down => &mut self.down,
            &Path::RightDown => &mut self.right_down,
            &Path::LeftDown => &mut self.left_down,
        }
    }
}

struct Status<'a> {
    table: &'a [[Player; 19]; 19],
    row: usize,
    col: usize,
}

fn search<'a>(status: Status<'a>) -> Player {
    let path = [Path::Right, Path::Down, Path::RightDown, Path::LeftDown];
    let mut black = [[Cumulative::new(); 19]; 19];
    let mut white = [[Cumulative::new(); 19]; 19];

    match status.table[0][0] {
        Player::None => (),
        Player::White => white[0][0] = Cumulative::one(),
        Player::Black => black[0][0] = Cumulative::one(),
    }

    for row in 0..19 {
        for col in 0..19 {
            if status.table[row][col] == Player::None {
                continue;
            }

            for p in path.iter() {
                let (r, c) = match p.apply(row, col) {
                    None => continue,
                    Some(coord) => coord
                };

                let applier =
                    |board: &mut [[Cumulative; 19]; 19]| -> bool {
                        *board[row][col].mut_get(p) = board[r][c].get(p) + 1;
                        p.at_least(row, col) && board[row][col].get(p) >= 6
                    };

                match status.table[row][col] {
                    Player::None => (),
                    Player::White => if applier(&mut white) { return Player::White },
                    Player::Black => if applier(&mut black) { return Player::Black },
                }
            }
        }
    }

    Player::None
}

pub fn run(game: &Game) -> Player {
    let status = Status {
        table: game.get_board(),
        row: 0,
        col: 0,
    };

    search(status)
}