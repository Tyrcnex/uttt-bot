use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ultimate_ttt_bot::board::*;
// use ultimate_ttt_bot::mcts::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("uttt_main", |b| b.iter(|| {
        let board = black_box(Board { x_board: [3, 64, 66, 135, 80, 136, 400, 64, 129], o_board: [352, 162, 8, 8, 12, 0, 77, 281, 64], side: Side::X, current_square: None, square_states: [Outcome::Undecided, Outcome::Undecided, Outcome::Undecided, Outcome::X, Outcome::Undecided, Outcome::Undecided, Outcome::O, Outcome::O, Outcome::Undecided] });
        board.get_legal_moves();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);