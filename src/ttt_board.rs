use crate::tic_utils::{TicState, check_board_state};

#[derive(Debug, Clone, Copy)]
pub struct TTTBoard {
    pub board: [[TicState; 3]; 3],
    pub board_state: TicState,
}

impl TTTBoard {
    pub fn new() -> Self {
        let board = [[TicState::N; 3]; 3];
        TTTBoard {
            board,
            board_state: TicState::N,
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: TicState) -> TicState {
        self.board[row][col] = player;
        self.board_state = check_board_state(&self.board);
        self.board_state
    }

    pub fn get_row_as_string(&self, row: usize) -> String {
        let mut row_str = String::new();

        if self.board_state != TicState::N {
            if row == 1 {
                return format!("   {}   ", match self.board_state {
                    TicState::X => "X",
                    TicState::O => "O",
                    TicState::N => "N",
                    TicState::T => "-"
                });
            }
            return "       ".to_string();
        }
        
        for col in 0..3 {
            if col == 0 {
                row_str.push(' ');
            }
            let symbol = match self.board[row][col] {
                TicState::X => "X",
                TicState::O => "O",
                TicState::N => "N",
                TicState::T => "-"
            };
            row_str.push_str(symbol);
            row_str.push(' ');
        }
        row_str
    }
}
