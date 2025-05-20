# Ultimate Tic-Tac-Toe with MCTS AI

A Rust implementation of Ultimate Tic-Tac-Toe featuring a Monte Carlo Tree Search (MCTS) artificial intelligence player. This project was created as part of my journey to learn Rust.

## Game Description

Ultimate Tic-Tac-Toe is a complex variant of the classic game where each cell of the traditional 3×3 board contains another smaller 3×3 board. To win, you need to win three small boards in a row. The twist is that your opponent must play in the board corresponding to your last move's position.

## Key Features

- Three game modes:
  - Human vs Human
  - Human vs AI
  - AI vs AI
- Configurable AI strength (simulation count)
- Different AI strengths for X and O in AI vs AI mode
- Clean command-line interface
- Efficient MCTS implementation

## How to Play

1. Clone the repository
2. Run with cargo:
```powershell
cargo run --release
```

Or simply download and run the UTTTRust.exe file from the releases page

### Game Controls

- Choose coordinates using numbers 0-8:
```
  | 0 1 2 | 3 4 5 | 6 7 8
--------------------------
0 | N N N | N N N | N N N 
1 | N N N | N N N | N N N 
2 | N N N | N N N | N N N 
--------------------------
3 | N N N | N N N | N N N 
4 | N N N | N N N | N N N 
5 | N N N | N N N | N N N 
--------------------------
6 | N N N | N N N | N N N 
7 | N N N | N N N | N N N 
8 | N N N | N N N | N N N 
--------------------------
```
The numbers represent the row and column

### AI Configuration

- AI thinking power is configured in thousands of simulations
- Higher simulation counts (e.g., 50,000) make the AI play stronger
- In AI vs AI mode, you can set different strengths for X and O

## Technical Details

### Implementation

- Written in pure Rust
- Uses Monte Carlo Tree Search (MCTS) for AI
- No external dependencies except for random number generation
- Memory-efficient game state representation

### Project Structure

- `src/main.rs` - Game loop and user interface
- `src/mcts.rs` - MCTS AI implementation
- `src/ttt_game_state.rs` - Game state management
- `src/ttt_board.rs` - Board representation
- `src/tic_utils.rs` - Utility functions and enums

## Performance

The MCTS AI is optimized for:
- Fast simulation speed
- Efficient memory usage
- Quick move generation
- Intelligent tree exploration

## License

MIT License - See LICENSE file for details

## Acknowledgments
- MCTS implementation based on standard algorithms
- This was a port of my older C# implementation, and I was surprised by the extreme increase in performance