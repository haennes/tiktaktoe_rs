#[cfg(test)]
mod tests;
// wird nur mit compiliert falls wir die tests mit cargo test ausführen

use core::{fmt::Display, ops::Not};
use rand::prelude::*;
use std::io::{self};

fn main() {
    let ttt = &mut TikTakToe::default();

    //random wer anfängt
    if rand::thread_rng().gen_bool(0.5) {
        //bot move
        ttt.turn(Index::One, Index::One);
    }

    println!("{}", ttt);

    loop {
        let (x, y) = input_ttt();
        let win = ttt.turn(x, y);
        println!("{}", ttt);
        if let Some(w) = win {
            println!("{} has won", w);
            return;
        }

        //bot -> smart move
        //bot turn
        //println!("{}", ttt);
    }

    // let mut ttt = &mut TikTakToe::default();
    // ttt.turn(Index::Two, Index::Two);
    // ttt.turn(Index::One, Index::Two);
    // let winner_one = ttt.turn(Index::One, Index::Three);
    // let winner_two = ttt.turn(Index::One, Index::One);
    // let winner_three = ttt.turn(Index::Three, Index::One);
    // println!("{}", ttt);
    // println!("{:?} {:?} {:?}", winner_one, winner_two, winner_three);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellState {
    X,
    O,
}

impl Not for CellState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            CellState::X => CellState::O,
            CellState::O => CellState::X,
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
            }
        )
    }
}

enum Index {
    One,
    Two,
    Three,
}

struct TikTakToe {
    matrix: (
        (Option<CellState>, Option<CellState>, Option<CellState>),
        (Option<CellState>, Option<CellState>, Option<CellState>),
        (Option<CellState>, Option<CellState>, Option<CellState>),
    ),
    current: CellState,
}

impl Default for TikTakToe {
    fn default() -> Self {
        Self {
            matrix: Default::default(),
            current: CellState::X,
        }
    }
}

impl TikTakToe {
    fn turn(&mut self, x: Index, y: Index) -> Option<CellState> {
        let new = match x {
            Index::One => (
                Self::line_index(self.matrix.0, y, self.current),
                self.matrix.1,
                self.matrix.2,
            ),
            Index::Two => (
                self.matrix.0,
                Self::line_index(self.matrix.1, y, self.current),
                self.matrix.2,
            ),
            Index::Three => (
                self.matrix.0,
                self.matrix.1,
                Self::line_index(self.matrix.2, y, self.current),
            ),
        };
        self.matrix = new;
        self.current = !self.current;

        self.determination()
    }

    fn line_index(
        line: (Option<CellState>, Option<CellState>, Option<CellState>),
        y: Index,
        current: CellState,
    ) -> (Option<CellState>, Option<CellState>, Option<CellState>) {
        match y {
            Index::One => (Self::set(line.0, current), line.1, line.2),
            Index::Two => (line.0, Self::set(line.1, current), line.2),
            Index::Three => (line.0, line.1, Self::set(line.2, current)),
        }
    }

    fn set(cell: Option<CellState>, current: CellState) -> Option<CellState> {
        match cell {
            Some(_) => panic!("Illegal Move"),
            None => Some(current),
        }
    }

    fn determination(&self) -> Option<CellState> {
        fn line(
            line: (Option<CellState>, Option<CellState>, Option<CellState>),
        ) -> Option<CellState> {
            if line.0 == line.1 && line.0 == line.2 {
                line.0
            } else {
                None
            }
        }
        let vec = vec![
            //line
            line((self.matrix.0 .0, self.matrix.0 .1, self.matrix.0 .2)),
            line((self.matrix.1 .0, self.matrix.1 .1, self.matrix.1 .2)),
            line((self.matrix.2 .0, self.matrix.2 .1, self.matrix.2 .2)),
            //column
            line((self.matrix.0 .0, self.matrix.1 .0, self.matrix.2 .0)),
            line((self.matrix.0 .1, self.matrix.1 .1, self.matrix.2 .1)),
            line((self.matrix.0 .2, self.matrix.1 .2, self.matrix.2 .2)),
            //diagonal
            line((self.matrix.0 .0, self.matrix.1 .1, self.matrix.2 .2)),
            line((self.matrix.0 .2, self.matrix.1 .1, self.matrix.2 .0)),
        ];

        vec.iter().filter_map(|x| *x).next()
    }
}

impl Display for TikTakToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_inner(var: Option<CellState>) -> String {
            format!("{}", {
                match var {
                    Some(var) => match var {
                        CellState::O => "O",
                        CellState::X => "X",
                    },
                    None => "-",
                }
            })
        }
        writeln!(
            f,
            "{}{}{}",
            fmt_inner(self.matrix.0 .0),
            fmt_inner(self.matrix.0 .1),
            fmt_inner(self.matrix.0 .2)
        )?;
        writeln!(
            f,
            "{}{}{}",
            fmt_inner(self.matrix.1 .0),
            fmt_inner(self.matrix.1 .1),
            fmt_inner(self.matrix.1 .2)
        )?;
        writeln!(
            f,
            "{}{}{}",
            fmt_inner(self.matrix.2 .0),
            fmt_inner(self.matrix.2 .1),
            fmt_inner(self.matrix.2 .2)
        )?;
        Ok(())
    }
}

fn input_ttt() -> (Index, Index) {
    println!("Input all numbers top left to top right to bottom");
    println!("press ENTER to proceed");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let x = match chars.next().unwrap() {
        '1' => Index::One,
        '2' => Index::Two,
        '3' => Index::Three,
        _ => panic!("illegal index"),
    };
    let y = match chars.next().unwrap() {
        '1' => Index::One,
        '2' => Index::Two,
        '3' => Index::Three,
        _ => panic!("illegal index"),
    };
    (x, y)
}
