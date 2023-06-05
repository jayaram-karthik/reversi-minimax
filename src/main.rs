mod ai;
mod reversi;


// adjust this if you want
const DEPTH: usize = 8;

fn main() {
    let mut board = reversi::ReversiBoard::new();
    
    loop {
        println!("P1's turn:");
        println!("{}", board);
        println!("valid moves: {:?}", board.get_possible_moves(board.current_player));
        println!("Enter a move (row, col):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut input = input.split_whitespace();
        let row = input.next().unwrap().parse::<usize>().unwrap();
        let col = input.next().unwrap().parse::<usize>().unwrap();
        
        let moved = board.make_move(row, col);

        if !moved {
            println!("Invalid move!");
            continue;
        }

        if board.is_board_terminal(board.current_player) {
            let winner = board.get_winner();

            match winner {
                reversi::CellState::Empty => println!("Tie!"),
                reversi::CellState::Black => println!("P1 wins!"),
                reversi::CellState::White => println!("P2 wins!"),
            }
            println!("{}", board);
            break;
        }

        let (ai_move_row, ai_move_col) = ai::get_best_move(board, DEPTH);
        board.make_move(ai_move_row, ai_move_col);

        if board.is_board_terminal(board.current_player) {
            let winner = board.get_winner();

            match winner {
                reversi::CellState::Empty => println!("Tie!"),
                reversi::CellState::Black => println!("P1 wins!"),
                reversi::CellState::White => println!("P2 wins!"),
            }
            println!("{}", board);
            break;
        }
    }
}
