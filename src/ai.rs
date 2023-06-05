use crate::reversi::{ReversiBoard, CellState};
use std::cmp::{max, min};


#[inline]
fn get_max_util(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn get_min_util(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn alpha_beta(node: ReversiBoard, depth: usize, mut alpha: f32, mut beta: f32, maximizingPlayer: bool) -> f32 {
    if depth == 0 || node.is_board_terminal(node.current_player) {
        return node.get_board_eval(node.current_player);
    }

    if maximizingPlayer {
        let mut value = f32::NEG_INFINITY;
        let children = node.get_children_nodes();
        
        for child in children.into_iter() {
            value = get_max_util(value, alpha_beta(child, depth - 1, alpha, beta, false));
            if value > beta {
                break;
            }
            alpha = get_max_util(alpha, value);
        }
        return value;
    } else {
        let mut value = f32::INFINITY;
        let children = node.get_children_nodes();
        
        for child in children.into_iter() {
            value = get_min_util(value, alpha_beta(child, depth - 1, alpha, beta, true));
            if value < alpha {
                break;
            }
            beta = get_min_util(beta, value);
        }
        return value;
    }
}

pub fn get_best_move(node: ReversiBoard, depth: usize) -> (usize, usize) {
    let mut best_val = f32::NEG_INFINITY;
    let mut best_move = (0, 0);
    let children = node.get_children_nodes();
    for child in children.into_iter() {
        let move_val = alpha_beta(child, depth - 1, f32::NEG_INFINITY, f32::INFINITY, false);
        if move_val > best_val {
            best_move = child.last_move.unwrap();
            best_val = move_val;
        }
    }
    return best_move;
}

