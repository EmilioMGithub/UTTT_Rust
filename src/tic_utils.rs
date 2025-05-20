#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TicState {
    X,
    O,
    N,
    T
}

pub fn check_board_state(board: &[[TicState; 3]; 3]) -> TicState {
    if check_win_for_player(board, TicState::X) {
        return TicState::X;
    }
    
    if check_win_for_player(board, TicState::O) {
        return TicState::O;
    }

    if board.iter().all(|row| row.iter().all(|&cell| cell != TicState::N)) {
        return TicState::T;
    }

    return  TicState::N;
}

fn check_win_for_player(board: &[[TicState; 3]; 3], state: TicState) -> bool {
    for row_col in 0..3 {
        let win_in_row = board[row_col][0] == state && board[row_col][1] == state && board[row_col][2] == state;
        let win_in_col = board[0][row_col] == state && board[1][row_col] == state && board[2][row_col] == state;
        if win_in_row || win_in_col {
            return true;
        }
    }

    let win_in_top_left_diagonal = board[0][0] == state && board[1][1] == state && board[2][2] == state;
    let win_in_top_right_diagonal: bool = board[0][2] == state && board[1][1] == state && board[2][0] == state;

    return win_in_top_left_diagonal || win_in_top_right_diagonal;
}
