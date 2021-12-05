use advent_of_code_2021::read_lines;
use std::fs::File;
use std::io::{self, BufRead};

struct Board {
    board: Vec<Vec<(isize, bool)>>,
}

impl Board {
    fn mark_value(&mut self, value:isize) -> bool {
        let mut i_won_flag = false;
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].0 == value {
                    self.board[i][j].1 = true;
                    i_won_flag = self.check_bingo();
                }
            }
        }
        return i_won_flag;
    }

    fn check_bingo(&self) -> bool {
        // check rows and cols
        let mut bingo = false;
        for i in 0..5 {
            let mut row_flag = true;
            let mut col_flag = true;
            let mut diag_flag = true;
            for j in 0..5 {
                if !self.board[i][j].1 {
                    row_flag = false
                }
                if !self.board[j][i].1 {
                    col_flag = false
                }
                if !self.board[j][j].1 {
                    diag_flag = false
                }
                if !self.board[j][4 - j].1 {
                    diag_flag = false
                }
            }
            bingo = row_flag || col_flag || diag_flag;
            if bingo {
                break;
            }
        }
        return bingo;
    }

    fn score(&self, winning_num: isize) -> isize {
        return if self.check_bingo() {
            winning_num * self.sum_unmarked()
        } else {
            -1
        }
    }

    fn sum_unmarked(&self) -> isize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.board[i][j].1 {
                    sum += self.board[i][j].0
                }
            }
        }
        return sum;
    }
}

pub fn answer_part_1() -> isize {
    let mut score = 0;
    if let Ok(mut lines) = read_lines("src/inputs/input_4.txt") {
        let guesses = get_bingo_guesses(&mut lines);
        let mut boards:Vec<Board> = Vec::new();
        while let Some(l)= lines.next() {
            boards.push(get_bingo_board(&mut lines));
        }
        println!("All boards saved");
        'guessing_loop: for guess in guesses {
            'marking_loop: for mut board in boards.iter_mut() {
                if board.mark_value(guess) {
                    score = board.score(guess);
                    println!("I found a winner with score: {}", score);
                    break 'guessing_loop;
                }
            }
        }
    }
    return score;
}

fn get_bingo_guesses(mut lines: &mut io::Lines<io::BufReader<File>>) -> Vec<isize> {
    //The guesses are on the first line separated by commas
    let guesses: Vec<isize> = lines.next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    return guesses;
}

fn get_bingo_board(mut lines: &mut io::Lines<io::BufReader<File>>) -> Board {
    //Assume empty line before board, board is the next five lines
    let mut b: Vec<Vec<(isize, bool)>> = Vec::new();
    for _ in 0..5 {
        let mut row:Vec<(isize, bool)> = Vec::new();
        let line:Vec<isize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        for num in line {
            println!("num is {}", num);
            row.push((num, false))
        }
        b.push(row);
    }
    let mut board = Board {
        board:b
    };
    return board;
}

pub fn answer_part_2() -> isize {
    let mut score = 0;
    if let Ok(mut lines) = read_lines("src/inputs/input_4.txt") {
        let guesses = get_bingo_guesses(&mut lines);
        let mut boards:Vec<Board> = Vec::new();
        let mut board_count =
        while let Some(l)= lines.next() {
            boards.push(get_bingo_board(&mut lines));
        }
        println!("All boards saved");
        'guessing_loop: for guess in guesses {
            'marking_loop: for mut board in boards.iter_mut() {
                if board.mark_value(guess) {
                    score = board.score(guess);
                    println!("I found a winner with score: {}", score);
                    break 'guessing_loop;
                }
            }
        }
    }
    return score;
}