use super::Player;

#[cfg(test)]
mod tests;

enum Path {
    Right,
    Down,
    RightDown,
    LeftDown,
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

    fn get(&self, path: &Path) -> i32 {
        match path {
            &Path::Right => self.right,
            &Path::Down => self.down,
            &Path::RightDown => self.right_down,
            &Path::LeftDown => self.left_down,
        }
    }

    fn get_mut(&mut self, path: &Path) -> &mut i32 {
        match path {
            &Path::Right => &mut self.right,
            &Path::Down => &mut self.down,
            &Path::RightDown => &mut self.right_down,
            &Path::LeftDown => &mut self.left_down,
        }
    }
}

struct Block {
    flag: usize,
    mem: [[Cumulative; 21]; 2],
}

impl Block {
    fn new() -> Block {
        Block {
            flag: 0,
            mem: [[Cumulative::new(); 21]; 2],
        }
    }

    fn as_tuple(&self) -> (&[Cumulative; 21], &[Cumulative; 21]) {
        let f = self.flag;
        (&self.mem[f], &self.mem[1 - f])
    }

    fn get_prev(&self, col: usize, path: &Path) -> &Cumulative {
        let (prev, now) = self.as_tuple();
        match path {
            &Path::Right => &now[col - 1],
            &Path::Down => &prev[col],
            &Path::RightDown => &prev[col - 1],
            &Path::LeftDown => &prev[col + 1],
        }
    }

    fn update_now<F>(&mut self, update: F)
        where F: Fn(&mut [Cumulative; 21])
    {
        let f = self.flag;
        let now = &mut self.mem[1 - f];
        update(now);
    }

    fn update_row(&mut self) {
        self.flag = 1 - self.flag;
        let now = &mut self.mem[1 - self.flag];

        for i in 0..21 {
            now[i] = Cumulative::new();
        }
    }
}

pub fn search(table: &[[Player; 19]; 19]) -> Player {
    let mut black = Block::new();
    let mut white = Block::new();

    fn path_iter(block: &mut Block, col: usize) -> bool {
        let col = col + 1;
        let paths = [Path::Right, Path::Down, Path::RightDown, Path::LeftDown];

        for path in paths.iter() {
            let updated = block.get_prev(col, path).get(path) + 1;
            if updated >= 6 { return true; }

            block.update_now(
                |now| *now[col].get_mut(path) = updated);
        }
        false
    }

    for row in 0..19 {
        black.update_row();
        white.update_row();

        for col in 0..19 {
            match table[row][col] {
                Player::None => continue,
                Player::Black =>
                    if path_iter(&mut black, col) { return Player::Black; },
                Player::White =>
                    if path_iter(&mut white, col) { return Player::White; },
            };
        }
    }

    Player::None
}
