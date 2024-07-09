use std::env;

use ultimate_ttt_bot::board::*;
use ultimate_ttt_bot::mcts::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board {
        x_board: [0; 9],
        o_board: [0; 9],
        side: Side::X,
        current_square: None,
        square_states: [Outcome::Undecided; 9],
    };
    
    let mut last_move = Move { tile: 4, square: 4 };
    board.place(last_move);

    while board.check_board_outcome() == Outcome::Undecided {
        last_move = bot_move(&board, last_move);
        board.place(last_move);
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        board.draw_board();
    }

    println!("Outcome: {:?}", board.check_board_outcome());
}
