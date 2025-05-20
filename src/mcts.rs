use crate::tic_utils::TicState;
use crate::ttt_game_state::{TTTGameState, BoardLoc};

use rand::{prelude::*};

pub struct MCTS {
    nodes: Vec<Node>,
}

impl MCTS {
    pub fn new(initial_state: TTTGameState) -> Self {
        let root_node = Node {
            state: initial_state,
            parent: None,
            children: Vec::new(),
            move_made: None,
            visits: 0,
            score: 0.0,
        };
        MCTS {
            nodes: vec![root_node],
        }
    }

    pub fn find_move_sim(&mut self, simulations: usize) -> Option<BoardLoc> {
        let mut simulations_done = 0;
        
        'outer: loop {
            let node = self.traverse(0);
            let simulation_result = self.rollout(node);
            self.backpropagate(simulation_result, node);
            simulations_done += 1;
        
            if simulations_done >= simulations {
                break 'outer;
            }
        }

        self.get_best_move()
    }


    fn best_uct(&self, node_index: usize) -> usize {
        let parent_node = &self.nodes[node_index];
        let exploration_factor = f64::sqrt(2.0);
        let parent_visits_ln = f64::ln(parent_node.visits as f64);
        
        let mut best_child = parent_node.children[0];
        let mut best_uct = f64::NEG_INFINITY;

        for &child in parent_node.children.iter() {
            let child_node = &self.nodes[child];
            let visits = child_node.visits as f64 + f64::EPSILON;
            let exploitation = child_node.score / visits;
            let exploration = exploration_factor * f64::sqrt(parent_visits_ln / visits);
            let uct = exploitation + exploration;

            if uct > best_uct {
                best_child = child;
                best_uct = uct;
            }
        }
        best_child
    }

    fn expand(&mut self, node_index: usize) {
        let state = self.nodes[node_index].state.clone();
        let legal_moves = state.get_legal_moves();
        
        for &move_loc in &legal_moves {            
            let new_state = state.make_move(move_loc);
            let child_node = Node {
                state: new_state,
                parent: Some(node_index),
                children: Vec::new(),
                move_made: Some(move_loc),
                visits: 0,
                score: 0.0,
            };
            let child_index = self.nodes.len();
            self.nodes.push(child_node);
            self.nodes[node_index].children.push(child_index);
        }
    }    
    
    fn traverse(&mut self, node_index: usize) -> usize {
        let mut current_node = node_index;
        let mut rng = rand::rng();

        while !self.nodes[current_node].children.is_empty() {
            current_node = self.best_uct(current_node);
        }

        if self.nodes[current_node].state.board_state != TicState::N {
            return current_node;
        }
        
        self.expand(current_node);
        let children = &self.nodes[current_node].children;
        if children.is_empty() {
            current_node
        } else {
            let idx = rng.random_range(0..children.len());
            children[idx]
        }
    }    
    
    fn rollout(&self, node_index: usize) -> i8 {
        let mut current_state = self.nodes[node_index].state.clone();
        let current_player = current_state.current_player;
        let mut rng = rand::rng();

        loop {
            match current_state.board_state {
                TicState::N => (),
                _ => break,
            };

            let moves = current_state.get_legal_moves();
            if moves.is_empty() {
                break;
            }

            let idx = rng.random_range(0..moves.len());
            let random_move = moves[idx];
            current_state = current_state.make_move(random_move);
        }

        match current_state.board_state {
            TicState::T => 0,
            winner if winner == current_player => -1,
            _ => 1,
        }
    }

    fn backpropagate(&mut self, result: i8, node_index: usize) {
        self.nodes[node_index].score += result as f64;
        self.nodes[node_index].visits += 1;

        if let Some(parent_index) = self.nodes[node_index].parent {
            self.backpropagate(-result, parent_index);
        }
    }
    
    pub fn get_best_move(&self) -> Option<BoardLoc> {
        let root = &self.nodes[0];
        if root.children.is_empty() {
            return None;
        }

        let mut best_node = root.children[0];
        let mut best_score = self.nodes[best_node].score / (self.nodes[best_node].visits as f64);

        for &child in &root.children[1..] {
            let score = self.nodes[child].score / (self.nodes[child].visits as f64);
            if score > best_score {
                best_node = child;
                best_score = score;
            }
        }

        self.nodes[best_node].move_made
    }
}

struct Node {
    state: TTTGameState,
    parent: Option<usize>,
    children: Vec<usize>,
    move_made: Option<BoardLoc>,
    visits: i32,
    score: f64,
}