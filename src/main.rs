#[cfg(test)]
mod tests;
// wird nur mit compiliert falls wir die tests mit cargo test ausführen

use core::{cell::Cell, fmt::Display, ops::Not};
use enum_iterator::{all, Sequence};
use rand::prelude::*;
use std::io::{self};

fn main() {
    let ttt = &mut TikTakToe::default();

    // //random wer anfängt
    // if rand::thread_rng().gen_bool(0.5) {
    //     //bot move
    //     ttt.turn(Index::One, Index::One);
    // }

    println!("{}", ttt);

    loop {
        let (x, y) = input_ttt();
        let win = ttt.turn(x, y).unwrap();
        println!("{}", ttt);
        if let Some(w) = win {
            println!("{} has won", w);
            return;
        }

        let bot_move = ttt.best_bot_move();
        let win = ttt.turn(bot_move.0, bot_move.1);

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

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
enum Index {
    One,
    Two,
    Three,
}

#[derive(Copy, Clone, Debug)]
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
    fn turn(&mut self, x: Index, y: Index) -> Result<Option<CellState>, ()> {
        let new = match x {
            Index::One => Ok((
                Self::line_index(self.matrix.0, y, self.current)?,
                self.matrix.1,
                self.matrix.2,
            )),
            Index::Two => Ok((
                self.matrix.0,
                Self::line_index(self.matrix.1, y, self.current)?,
                self.matrix.2,
            )),
            Index::Three => Ok((
                self.matrix.0,
                self.matrix.1,
                Self::line_index(self.matrix.2, y, self.current)?,
            )),
        };
        self.matrix = new?;
        self.current = !self.current;

        Ok(self.winner_determination())
    }

    fn line_index(
        line: (Option<CellState>, Option<CellState>, Option<CellState>),
        y: Index,
        current: CellState,
    ) -> Result<(Option<CellState>, Option<CellState>, Option<CellState>), ()> {
        match y {
            Index::One => Ok((Self::set(line.0, current)?, line.1, line.2)),
            Index::Two => Ok((line.0, Self::set(line.1, current)?, line.2)),
            Index::Three => Ok((line.0, line.1, Self::set(line.2, current)?)),
        }
    }

    fn set(cell: Option<CellState>, current: CellState) -> Result<Option<CellState>, ()> {
        match cell {
            Some(_) => Err(()),
            None => Ok(Some(current)),
        }
    }

    fn winner_determination(&self) -> Option<CellState> {
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

    fn possible_moves(self) -> Vec<(Index, Index)> {
        let mut working_self = self;
        let mut out = vec![];
        for x in all::<Index>() {
            for y in all::<Index>() {
                if working_self.turn(x, y).is_ok() {
                    out.push((x, y));
                }
            }
        }
        out
    }

    // move Some - End game None
    fn best_bot_move(&self) -> Option<(Index, Index)> {
        let who = self.current;
        for moves in self.possible_moves().iter() {
            let mut game = self.clone();
            // move is definetifly possible -> unwrap
            let winner = game.turn(moves.0, moves.1).unwrap();
            match winner {
                Some(winner) => {
                    if winner == who {
                        return Some((moves.0, moves.1));
                    }
                }
                None => {
                    let best_enemy_move = game.best_bot_move();
                    match best_enemy_move {
                        None => return Some((moves.0, moves.1)),
                        Some(best_enemy_move) => match game
                            .turn(best_enemy_move.0, best_enemy_move.1)
                            .unwrap()
                        {
                            Some(winner) => {
                                if winner == who {
                                    return Some((moves.0, moves.1));
                                }
                            }
                            //Draw
                            None => return Some(self.possible_moves().first().unwrap().clone()),
                        },
                    }
                }
            }
        }
        return None;
    }

    // returns the best move for a bot_player in the szenario
    // fn best_bot_move(&self, who: CellState) -> Option<(Index, Index)> {
    //     let mut to_return = vec![];
    //     for moves in self.possible_moves().iter() {
    //         let mut game_cp = self.clone();
    //         let winner = game_cp.turn(moves.0, moves.1).unwrap();
    //         match winner {
    //             None => {
    //                 //best move for enemy
    //                 let best_enemy = game_cp.best_bot_move(!who);
    //                 let winner = game_cp.turn(best_enemy.0, best_enemy.1).unwrap();
    //                 match winner {
    //                     Some(winner) => {
    //                         if winner == who {
    //                             to_return.push((moves.0, moves.1));
    //                         }
    //                         //otherwise discard it
    //                     }
    //                     None => {
    //                         let move_self = game_cp.best_bot_move(who);
    //                         to_return.push((move_self.0, move_self.1));
    //                     }
    //                 }
    //             }
    //             Some(winner) => {
    //                 if winner == who {
    //                     to_return.push((moves.0, moves.1));
    //                 }
    //             } //otherwise discard it
    //         }
    //     }
    //     let first = to_return.first().cloned();
    //     match first {
    //         None => self.possible_moves().iter().next().cloned(),
    //         Some(first) => Some(first.clone()),
    //     }
    //}
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
