use super::Policy;
use super::Game;
use super::super::BOARD_SIZE;

use std::io;

#[cfg(test)]
mod tests;

pub struct IoPolicy<'a, 'b> {
    reader: &'a mut io::Read,
    writer: &'b mut io::Write,
}

impl<'a, 'b> IoPolicy<'a, 'b> {
    pub fn new(reader: &'a mut io::Read, writer: &'b mut io::Write) -> IoPolicy<'a, 'b> {
        IoPolicy { reader, writer }
    }
}

impl<'a, 'b> Policy for IoPolicy<'a, 'b> {
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        game.print(self.writer).unwrap();

        let mut pos = None;
        loop {
            let mut buffer = [0; 10];
            self.reader.read(&mut buffer)
                .expect("io_policy::next - couldn't read from self.reader");

            let query: String = buffer.iter()
                .filter(|x| x.is_ascii_alphabetic())
                .map(|x| *x as char).collect();

            if query.len() == 2 {
                let mut chars = query.chars();
                let row = chars.next();
                let col = chars.next();

                if row.is_some() || col.is_some() {
                    let row = row.unwrap() as usize - 0x61;
                    let col = col.unwrap() as usize - 0x41;
                    if row < BOARD_SIZE && col < BOARD_SIZE {
                        pos = Some((row, col));
                        break;
                    }
                }
            }
            self.writer.write(b"invalid input, retry\n")
                .expect("agent_io::play - write invalid query msg fail");
        }
        pos
    }
}
