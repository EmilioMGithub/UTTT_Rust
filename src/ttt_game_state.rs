use crate::tic_utils::{check_board_state, TicState};
use crate::ttt_board::TTTBoard;
use std::fmt;

#[derive(Clone, Copy)]
pub struct BoardLoc {
    pub outer_row: usize,
    pub outer_col: usize,
    pub inner_row: usize,
    pub inner_col: usize,
}

impl BoardLoc {
    pub fn new(outer_row: usize, outer_col: usize, inner_row: usize, inner_col: usize) -> Self {
        BoardLoc {
            outer_row,
            outer_col,
            inner_row,
            inner_col,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TTTGameState {
    pub board: [[TTTBoard; 3]; 3],
    pub current_player: TicState,
    pub board_state: TicState,
    pub last_row: usize,
    pub last_col: usize,
}

impl TTTGameState {
    pub fn new() -> Self {
        let board = [[TTTBoard::new(); 3]; 3];
        TTTGameState {
            board,
            current_player: TicState::X,
            board_state: TicState::N,
            last_row: 9,
            last_col: 9,
        }
    }

    pub fn is_legal_move(&self, loc: BoardLoc) -> bool {
        let is_open_square = self.board[loc.outer_row][loc.outer_col].board[loc.inner_row][loc.inner_col] == TicState::N;
        if !is_open_square {
            return false;
        }

        let move_in_next_board = loc.outer_row == self.last_row && loc.outer_col == self.last_col;
        if !move_in_next_board && !self.can_move_anywhere() {
            return false;
        }

        let board_clicked_won =  self.board[loc.outer_row][loc.outer_col].board_state != TicState::N;

        return !board_clicked_won;
    }

    pub fn can_move_anywhere(&self) -> bool {
        let is_first_move = self.last_row == 9 && self.last_col == 9;
        if is_first_move {
            return true;
        }

        let board_won = self.board[self.last_row][self.last_col].board_state != TicState::N;
        return board_won;
    }


    pub fn make_move(&self, loc: BoardLoc) -> TTTGameState {
        let mut new_state = self.clone();
        new_state.last_row = loc.inner_row;
        new_state.last_col = loc.inner_col;

        // Make the move on the selected board
        let board_on = &mut new_state.board[loc.outer_row][loc.outer_col];
        board_on.make_move(loc.inner_row, loc.inner_col, new_state.current_player);

        // Start by assuming it's a tie
        new_state.board_state = TicState::T;
        
        // Create a 3x3 grid to track the state of each small board
        let mut state_of_boards = [[TicState::N; 3]; 3];
        
        // Check each board's state
        for row in 0..3 {
            for col in 0..3 {
                let board_state = new_state.board[row][col].board_state;
                let board_won = board_state != TicState::N;
                                
                // If any board is still open to play, game is not tied
                if !board_won {
                    new_state.board_state = TicState::N;
                }
                
                state_of_boards[row][col] = board_state;
            }
        }

        // If game isn't tied, check for a win
        if new_state.board_state != TicState::T {
            new_state.board_state = check_board_state(&state_of_boards);
        }

        // Switch players
        new_state.current_player = match new_state.current_player {
            TicState::X => TicState::O,
            TicState::O => TicState::X,
            _ => TicState::N,
        };

        return new_state;
    }

    pub fn get_legal_moves(&self) -> Vec<BoardLoc> {
        let mut legal_moves = Vec::new();
        for outer_row in 0..3 {
            for outer_col in 0..3 {
                for inner_row in 0..3 {
                    for inner_col in 0..3 {
                        let loc = BoardLoc::new(outer_row, outer_col, inner_row, inner_col);
                        if self.is_legal_move(loc) {
                            legal_moves.push(loc);
                        }
                    }
                }
            }
        }
        return legal_moves;
    }
}

impl fmt::Display for TTTGameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  | 0 1 2 | 3 4 5 | 6 7 8")?;
        writeln!(f, "--------------------------")?;
        
        for outer_row in 0..3 {
            for row in 0..3 {
                let left = self.board[outer_row][0].get_row_as_string(row);
                let center = self.board[outer_row][1].get_row_as_string(row);
                let right = self.board[outer_row][2].get_row_as_string(row);
                writeln!(f, "{} |{}|{}|{}", outer_row * 3 + row, left, center, right)?;
            }
            writeln!(f, "--------------------------")?;
        }
        
        write!(f, "    Next Player is: {:?}", self.current_player)
    }
}