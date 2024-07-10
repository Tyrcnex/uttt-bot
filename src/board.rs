pub const WIN_STATES: [u16; 8] = [
    0b100100100,
    0b010010010,
    0b001001001,
    0b111000000,
    0b000111000,
    0b000000111,
    0b100010001,
    0b001010100,
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    X,
    O,
}

impl Side {
    pub fn swap(self) -> Self {
        match self {
            Side::X => Side::O,
            Side::O => Side::X,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Outcome {
    X,
    O,
    Draw,
    Undecided,
}

impl std::cmp::PartialEq<Side> for Outcome {
    fn eq(&self, other: &Side) -> bool {
        match self {
            Outcome::X => *other == Side::X,
            Outcome::O => *other == Side::O,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub tile: u8,
    pub square: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    pub x_board: [u16; 9],
    pub o_board: [u16; 9],
    pub side: Side,
    pub current_square: Option<u8>,
    pub square_states: [Outcome; 9],
}

fn is_winning(square: u16) -> bool {
    WIN_STATES
        .iter()
        .any(|win_state| (square & win_state) == *win_state)
}

fn squares_to_move(s_idx: u8, square: u16) -> Vec<Move> {
    (0u8..=8)
        .filter(|x| (square & (1 << x)) == 0)
        .map(|x| Move {
            tile: x,
            square: s_idx,
        })
        .collect()
}

impl Board {
    pub fn get_s_idx(&self, s_idx: u8) -> u8 {
        match self.current_square {
            None => s_idx,
            Some(i) => i,
        }
    }

    pub fn check_square_outcome(&self, s_idx: u8) -> Outcome {
        let x_square = self.x_board[s_idx as usize];
        let o_square = self.o_board[s_idx as usize];
        if is_winning(x_square) {
            Outcome::X
        } else if is_winning(o_square) {
            Outcome::O
        } else if x_square | o_square == 0b111111111 {
            Outcome::Draw
        } else {
            Outcome::Undecided
        }
    }

    pub fn place(&mut self, mov: Move) {
        let s_idx = self.get_s_idx(mov.square);

        let current_square = match self.side {
            Side::X => &mut self.x_board[s_idx as usize],
            Side::O => &mut self.o_board[s_idx as usize],
        };

        *current_square |= 1 << mov.tile;

        let outcome = self.check_square_outcome(s_idx);
        if outcome != Outcome::Undecided {
            self.square_states[s_idx as usize] = outcome;
        }

        self.current_square = match self.check_square_outcome(mov.tile) {
            Outcome::Undecided => Some(mov.tile),
            _ => None,
        };
        self.side = self.side.swap();
    }

    pub fn is_legal(&self, mov: Move) -> bool {
        let s_idx = self.get_s_idx(mov.square);
        self.check_square_outcome(s_idx) == Outcome::Undecided
            && (self.x_board[s_idx as usize] & (1 << mov.tile)) == 0
            && (self.o_board[s_idx as usize] & (1 << mov.tile)) == 0
    }

    fn states_to_u16(&self, outcome: Outcome) -> u16 {
        self.square_states
            .into_iter()
            .enumerate()
            .fold(0u16, |a, (idx, val)| a | ((val == outcome) as u16) << idx)
    }

    pub fn check_board_outcome(&self) -> Outcome {
        if is_winning(self.states_to_u16(Outcome::X)) {
            Outcome::X
        } else if is_winning(self.states_to_u16(Outcome::O)) {
            Outcome::O
        } else if self.square_states.iter().all(|x| *x != Outcome::Undecided) {
            Outcome::Draw
        } else {
            Outcome::Undecided
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        if self.check_board_outcome() != Outcome::Undecided {
            return vec![];
        }

        match self.current_square {
            None => self
                .x_board
                .iter()
                .zip(&self.o_board)
                .map(|(a, b)| a | b)
                .enumerate()
                .filter(|&(i, _)| self.square_states[i] == Outcome::Undecided)
                .flat_map(|(s_idx, square)| squares_to_move(s_idx as u8, square))
                .collect(),
            Some(i) => squares_to_move(i, self.x_board[i as usize] | self.o_board[i as usize]),
        }
    }

    pub fn draw_board(&self) {
        println!("___ ___ ___");
        for row in 0..9 {
            let mut my_str = "".to_owned();
            for col in 0..9 {
                let s_idx = 3f32 * ((row as f32) / 3f32).floor() + ((col as f32) / 3f32).floor();
                let t_idx = 3f32 * ((row as f32) % 3f32) + ((col as f32) % 3f32);
                let x_tile = self.x_board[s_idx as usize] & (1 << (t_idx as u8));
                let o_tile = self.o_board[s_idx as usize] & (1 << (t_idx as u8));
                my_str.push_str(if x_tile > 0 {
                    "X"
                } else if o_tile > 0 {
                    "O"
                } else {
                    " "
                });
                if col % 3 == 2 {
                    my_str.push('|');
                }
            }
            println!("{my_str}");
            if row % 3 == 2 {
                println!("___ ___ ___");
            }
        }
    }
}
