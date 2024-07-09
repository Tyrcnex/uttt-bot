use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::ops::{Index, IndexMut};

use crate::board::*;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub side: Side,
    pub node_move: Move,
    pub wins: u32,
    pub visits: u32,
    pub children: Option<(usize, usize)>,
}

#[derive(Debug)]
pub struct Tree(Vec<Node>);

impl Tree {
    fn extend_nodes(&mut self, iter: impl Iterator<Item = Node>) -> (usize, usize) {
        let slen = self.0.len();
        self.0.extend(iter);
        (slen, self.0.len())
    }

    pub fn expand(&mut self, node_idx: usize, board: &Board) {
        let node = self[node_idx];
        let legal_moves = board.get_legal_moves();
        if legal_moves.is_empty() {
            return;
        }
        let extended_nodes = self.extend_nodes(legal_moves.iter().map(|x| Node {
            side: node.side.swap(),
            node_move: *x,
            wins: 0,
            visits: 0,
            children: None,
        }));
        self[node_idx].children = Some(extended_nodes);
    }
}

impl Index<usize> for Tree {
    type Output = Node;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Tree {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

pub fn uct_policy(wins: u32, visits: u32, parent_visits: u32) -> f32 {
    if visits == 0 {
        999999999999f32
    } else {
        let mean_action_value = (wins as f32) / (visits as f32);
        let explore_factor = ((parent_visits as f32).ln() / (visits as f32)).sqrt();
        mean_action_value + 1.4 * explore_factor
    }
}

impl Node {
    pub fn select(&self, tree: &Tree) -> usize {
        let tup_range = self.children.unwrap();
        let mut v = Vec::with_capacity(tup_range.1 - tup_range.0);
        for (idx, node) in tree.0[tup_range.0..tup_range.1].iter().enumerate() {
            if node.visits == 0 {
                return idx;
            } else {
                v.push(uct_policy(node.wins, node.visits, self.visits))
            }
        }
        let dist = WeightedIndex::new(&v).unwrap();
        let mut rng = thread_rng();
        self.children.unwrap().0 + dist.sample(&mut rng)
    }
}

pub fn bot_move(board: &Board, last_move: Move) -> Move {
    let mut mcts_tree = Tree(vec![Node {
        side: board.side.swap(),
        node_move: last_move,
        wins: 0,
        visits: 0,
        children: None,
    }]);

    for _ in 0..1000 {
        let mut new_board = *board;

        // selection
        let mut node_path: Vec<usize> = vec![0];
        for _ in 0..81 {
            // length of a game is at most 81 moves
            let last_node = &mcts_tree[node_path[node_path.len() - 1]];
            let is_leaf = match last_node.children {
                None => true,
                Some(i) => i.1 - i.0 == 0,
            };
            if is_leaf {
                break;
            }
            node_path.push(last_node.select(&mcts_tree));
            new_board.place(last_node.node_move);
        }

        if new_board.check_board_outcome() != Outcome::Undecided {
            continue;
        }

        // expansion
        let leaf_idx = node_path[node_path.len() - 1];
        mcts_tree.expand(leaf_idx, &new_board);

        node_path.push(mcts_tree[leaf_idx].children.unwrap().0);
        new_board.place(mcts_tree[leaf_idx].node_move);

        // rollouts
        for _ in 0..81 {
            let mut rng = thread_rng();

            let outcome = new_board.check_board_outcome();
            if outcome != Outcome::Undecided {
                break;
            }

            let legal_moves = new_board.get_legal_moves();
            if legal_moves.is_empty() {
                panic!("nooooo no legal moves noooooooo");
            }
            let rng_index = rng.gen_range(0..legal_moves.len());
            new_board.place(legal_moves[rng_index]);
        }

        let outcome = new_board.check_board_outcome();

        if outcome == Outcome::Undecided {
            new_board.draw_board();
            println!("{:?}", new_board);
            panic!("oh shit what did you DO");
        }

        // backpropagation
        let this_side = mcts_tree[leaf_idx].side;
        let this_side_score: u32 = if outcome == Outcome::Draw {
            1
        } else if outcome == this_side {
            2
        } else {
            0
        };
        let opponent_score: u32 = if outcome == Outcome::Draw {
            1
        } else if outcome == this_side.swap() {
            2
        } else {
            0
        };
        let toggle = true;
        for &i in node_path.iter() {
            let node = &mut mcts_tree[i];
            node.visits += 1;
            node.wins += if toggle {
                this_side_score
            } else {
                opponent_score
            };
        }
    }

    let tup_range = mcts_tree[0].children.unwrap();
    let mut node_max_visits: Node = mcts_tree[1];
    for &node in mcts_tree.0[tup_range.0..=tup_range.1].iter() {
        if node.visits > node_max_visits.visits {
            node_max_visits = node;
        }
    }
    node_max_visits.node_move
}
