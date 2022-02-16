//! Solves sudoku boards.
//! ```
//! # use marshians_fn::sudoku::Board;
//! let mut board = Board::new(
//!     "120400586060201403040096000090000014081000360430000070000720030608903040372008051"
//! )
//! .unwrap();
//!
//! assert_eq!(board.solve(), true);
//! assert_eq!(board.to_string(), "129437586867251493543896127795362814281574369436189275914725638658913742372648951");
//! ```
use std::fmt;

use regex::Regex;

/// Error contains the errors this module returns.
#[derive(Debug)]
pub enum Error {
    InvalidBoard(String),
    Unsolvable,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidBoard(ref s) => write!(f, "Invalid board: {}", s),
            Error::Unsolvable => write!(f, "board is not solvable"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Solve is a helper functions that takes a board as a string and then
/// returns the solved board.
pub fn solve(board: &str) -> Result<String, Error> {
    let mut board = match Board::new(board) {
        Ok(board) => board,
        Err(e) => return Err(Error::InvalidBoard(e.to_string())),
    };
    if !board.solve() {
        return Err(Error::Unsolvable);
    }
    Ok(board.to_string())
}

/// A representation of a sudoku board.
#[derive(Debug, PartialEq)]
pub struct Board {
    grid: Vec<u8>,
}

impl Board {
    /// Create a new board. Will fail if the string isn't exactly 81
    /// characters long. '0' can be used for empty spaces.
    pub fn new(s: &str) -> Result<Board, Box<dyn std::error::Error>> {
        let re = Regex::new(r"^[0-9]*$")?;
        if s.len() != 81 {
            return Err(Box::new(Error::InvalidBoard(
                "string must be exactly 81 characters".to_string(),
            )));
        } else if !re.is_match(&s) {
            return Err(Box::new(Error::InvalidBoard(
                "string must contain only digits (zero for empty)".to_string(),
            )));
        }

        let mut b = Board { grid: Vec::new() };
        for c in s.chars() {
            b.grid.push(c.to_string().parse()?);
        }
        Ok(b)
    }

    fn valid(&self, p: usize, n: u8) -> bool {
        let x = p % 9;
        let y = p / 9;
        // Check the column and row but exclude the position being checked.
        for i in 0..9 {
            if i != x && self.grid[y * 9 + i] == n {
                return false;
            }
            if i != y && self.grid[i * 9 + x] == n {
                return false;
            }
        }

        // Check the containing box.
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;
        for dx in 0..3 {
            for dy in 0..3 {
                // Ignore the position being checked.
                if (y0 + dy) == y && (x0 + dx) == x {
                    continue;
                }
                if self.grid[(y0 + dy) * 9 + x0 + dx] == n {
                    return false;
                }
            }
        }
        return true;
    }

    fn solved(&self) -> bool {
        for (p, n) in self.grid.iter().enumerate() {
            if *n == 0 || !self.valid(p, *n) {
                return false;
            }
        }
        true
    }

    /// Solve the board. Returns true on success and false if no
    /// solution was found.
    pub fn solve(&mut self) -> bool {
        self.solve_helper(self.next_unsolved(0))
    }

    fn next_unsolved(&self, p: usize) -> usize {
        for i in p..81 {
            if self.grid[i] == 0 {
                return i;
            }
        }
        return 81;
    }

    fn solve_helper(&mut self, p: usize) -> bool {
        // Check to see if we have reached the end.
        if p == 81 {
            return self.solved();
        }

        // We are at an unsolved square. Let's try different values.
        for n in 1..10 {
            // Try all valid positions.
            if self.valid(p, n) {
                // Check to see if this was a solution.
                self.grid[p] = n;

                if self.solve_helper(self.next_unsolved(p + 1)) {
                    return true;
                }
            }
        }

        // If we've tried them all, this one isn't the solution.
        self.grid[p] = 0;
        return false;
    }
}

impl std::string::ToString for Board {
    fn to_string(&self) -> String {
        self.grid.iter().map(|p| p.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let tests = vec![
            (
                true,
                0,
                1,
                "023456789000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                2,
                "023456789000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000001000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000000100000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000000000000000000000000000000000000000000000000000000000000000000000100000000",
            ),
        ];
        for (n, test) in tests.iter().enumerate() {
            let b = Board::new(test.3).unwrap();
            assert_eq!(b.valid(test.1, test.2), test.0, "test {}", n);
        }
    }

    #[test]
    fn solvable() {
        let mut board = Board::new(
            "120400586060201403040096000090000014081000360430000070000720030608903040372008051",
        )
        .unwrap();

        assert_eq!(board.solve(), true);
    }

    #[test]
    fn to_string() {
        let mut board = Board::new(
            "120400586060201403040096000090000014081000360430000070000720030608903040372008051",
        )
        .unwrap();
        board.solve();
        assert_eq!(
            board.to_string(),
            "129437586867251493543896127795362814281574369436189275914725638658913742372648951"
        );
    }

    #[test]
    #[should_panic(expected = r#"string must contain only digits"#)]
    fn board_new_non_numeric() {
        Board::new(
            "240000789308000016001800023034502698 ._03007000006003008700090205300080160008435a",
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = r#"string must be exactly 81 characters"#)]
    fn board_new_too_short() {
        Board::new("24000078930800001600180002").unwrap();
    }

    #[test]
    #[should_panic(expected = r#"string must be exactly 81 characters"#)]
    fn board_new_too_long() {
        Board::new(
            "240000789308000016000000001800000000000000000000000000000000000000000000000000000000000002"
        ).unwrap();
    }
}
