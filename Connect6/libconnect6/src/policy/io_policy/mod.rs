//! Policy for in-game user selection with stdio
//!
//! Input format is "{row lowercase}{col uppercase}" such as "sS" or "aC".
//! If invalid format or position is given, policy will write retrying message to stdout.
//! *Note* if given position raise Err from `Game` like `already set position`, game will be terminated by `Agent`.
//!
//! *Note* We suggest that if you use IoPolicy, construct `Agent` with `Agent::debug`.
//! IoPolicy doesn't displaying the board when you make the choice,
//! so that if you want to confirm the board, you have to construct `Agent` in debug mode.
//!
//! # Examples
//! ```ignore
//! # extern crate connect6;
//! # use connect6::{policy::IoPolicy, agent::Agent};
//! let mut stdin = std::io::stdin();
//! let mut stdout = std::io::stdout();
//! let mut io_policy = IoPolicy::new(&mut stdin, &mut stdout);
//! Agent::debug(&mut io_policy).play().unwrap();
//! ```
use game::Game;
use policy::Policy;
use BOARD_SIZE;

use std::io;

#[cfg(test)]
mod tests;

/// Policy for in-game user selection with io
///
/// Input format is {row lowercase}{col uppercase} such as "sS" or "aC".
/// If invalid format or position is given, policy will write retrying message to stdout.
///
/// *Note* if given position raise Err from Game, game will be terminated by `Agent`.
///
/// *Note* We suggest that if you use IoPolicy, construct `Agent` with `Agent::debug`.
/// IoPolicy doesn't displaying the board when you make the choice,
/// so that if you want to confirm the board, you have to construct `Agent` in debug mode.
///
/// # Examples
/// ```ignore
/// # #[macro_use] extern crate connect6;
/// # use connect6::agent::Agent;
/// io_policy_stdio!(io_policy);
/// Agent::debug(&mut io_policy).play().unwrap();
/// ```
pub struct IoPolicy<'a, 'b> {
    reader: &'a mut io::Read,
    writer: &'b mut io::Write,
}

impl<'a, 'b> IoPolicy<'a, 'b> {
    /// Construct a new IoPolicy
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::policy::IoPolicy;
    /// let mut stdin = std::io::stdin();
    /// let mut stdout = std::io::stdout();
    /// let mut io_policy = IoPolicy::new(&mut stdin, &mut stdout);
    /// ```
    pub fn new(reader: &'a mut io::Read, writer: &'b mut io::Write) -> IoPolicy<'a, 'b> {
        IoPolicy { reader, writer }
    }
}

impl<'a, 'b> Policy for IoPolicy<'a, 'b> {
    /// validate user input from stdin and passing it to `Agent`
    fn next(&mut self, _game: &Game) -> Option<(usize, usize)> {
        let mut pos = None;
        // until make the possible selection
        loop {
            // get from buffer
            let mut buffer = [0; 10];
            self.reader
                .read(&mut buffer)
                .expect("io_policy::next - couldn't read from self.reader");

            let query: String = buffer
                .iter()
                .filter(|x| x.is_ascii_alphabetic())
                .map(|x| *x as char)
                .collect();

            if query.len() == 2 {
                // parse position
                let mut chars = query.chars();
                let row = chars.next();
                let col = chars.next();

                if row.is_some() || col.is_some() {
                    // validation
                    let row = row.unwrap() as usize - 0x61;
                    let col = col.unwrap() as usize - 0x41;
                    if row < BOARD_SIZE && col < BOARD_SIZE {
                        pos = Some((row, col));
                        break;
                    }
                }
            }
            self.writer
                .write(b"invalid input, retry\n")
                .expect("agent_io::play - write invalid query msg fail");
        }
        pos
    }
}
