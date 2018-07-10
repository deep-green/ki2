extern crate shakmaty;

use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use shakmaty::{ Board, Piece, Square, Chess, MoveList, Position, Setup, Bitboard, Color, Move };

const PAWN_EVAL_WHITE: [[f64; 8]; 8] = [
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0],
    [1.0,  1.0,  2.0,  3.0,  3.0,  2.0,  1.0,  1.0],
    [0.5,  0.5,  1.0,  2.5,  2.5,  1.0,  0.5,  0.5],
    [0.0,  0.0,  0.0,  2.0,  2.0,  0.0,  0.0,  0.0],
    [0.5, -0.5, -1.0,  0.0,  0.0, -1.0, -0.5,  0.5],
    [0.5,  1.0, 1.0,  -2.0, -2.0,  1.0,  1.0,  0.5],
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0]
];

const PAWN_EVAL_BLACK: [[f64; 8]; 8] = [
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [0.5,  1.0, 1.0,  -2.0, -2.0,  1.0,  1.0,  0.5],
    [0.5, -0.5, -1.0,  0.0,  0.0, -1.0, -0.5,  0.5],
    [0.0,  0.0,  0.0,  2.0,  2.0,  0.0,  0.0,  0.0],
    [0.5,  0.5,  1.0,  2.5,  2.5,  1.0,  0.5,  0.5],
    [1.0,  1.0,  2.0,  3.0,  3.0,  2.0,  1.0,  1.0],
    [5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0],
    [0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0]
];

const KNIGHT_EVAL: [[f64; 8]; 8]  = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0,  0.0,  0.0,  0.0,  0.0, -2.0, -4.0],
    [-3.0,  0.0,  1.0,  1.5,  1.5,  1.0,  0.0, -3.0],
    [-3.0,  0.5,  1.5,  2.0,  2.0,  1.5,  0.5, -3.0],
    [-3.0,  0.0,  1.5,  2.0,  2.0,  1.5,  0.0, -3.0],
    [-3.0,  0.5,  1.0,  1.5,  1.5,  1.0,  0.5, -3.0],
    [-4.0, -2.0,  0.0,  0.5,  0.5,  0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0]
];

const BISHOP_EVAL_WHITE: [[f64; 8]; 8] = [
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [ -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  1.0,  1.0,  0.5,  0.0, -1.0],
    [ -1.0,  0.5,  0.5,  1.0,  1.0,  0.5,  0.5, -1.0],
    [ -1.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0, -1.0],
    [ -1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0, -1.0],
    [ -1.0,  0.5,  0.0,  0.0,  0.0,  0.0,  0.5, -1.0],
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0]
];

const BISHOP_EVAL_BLACK: [[f64; 8]; 8] = [
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [ -1.0,  0.5,  0.0,  0.0,  0.0,  0.0,  0.5, -1.0],
    [ -1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0, -1.0],
    [ -1.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0, -1.0],
    [ -1.0,  0.5,  0.5,  1.0,  1.0,  0.5,  0.5, -1.0],
    [ -1.0,  0.0,  0.5,  1.0,  1.0,  0.5,  0.0, -1.0],
    [ -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0]
];

const ROOK_EVAL_WHITE: [[f64; 8]; 8] = [
    [  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
    [  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [  0.0,   0.0, 0.0,  0.5,  0.5,  0.0,  0.0,  0.0]
];

const ROOK_EVAL_BLACK: [[f64; 8]; 8] = [
    [  0.0,   0.0, 0.0,  0.5,  0.5,  0.0,  0.0,  0.0],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [ -0.5,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -0.5],
    [  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.5],
    [  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0],
];

const QUEEN_EVAL: [[f64; 8]; 8] = [
    [ -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [ -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0],
    [ -0.5,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5],
    [  0.0,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5],
    [ -1.0,  0.5,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0],
    [ -1.0,  0.0,  0.5,  0.0,  0.0,  0.0,  0.0, -1.0],
    [ -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0]
];

const KING_EVAL_WHITE: [[f64; 8]; 8] = [
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [ -1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [  2.0,  2.0,  0.0,  0.0,  0.0,  0.0,  2.0,  2.0],
    [  2.0,  3.0,  1.0,  0.0,  0.0,  1.0,  3.0,  2.0]
];

const KING_EVAL_BLACK: [[f64; 8]; 8] = [
    [  2.0,  3.0,  1.0,  0.0,  0.0,  1.0,  3.0,  2.0],
    [  2.0,  2.0,  0.0,  0.0,  0.0,  0.0,  2.0,  2.0],
    [ -1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [ -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [ -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0]
];

/// Returns the value of the piece based on the role and position on the board.
///
/// # Arguments
///
/// * `piece` - The piece to check
/// * `x` - Rank of the piece
/// * `y` - File of the piece
///
pub fn get_piece_value(piece: &Piece, x: usize, y: usize) -> f64 {
    let mut ret: f64 = 0.0;
    let is_white: bool = Color::is_white(piece.color);

    if piece.role.char().to_string().eq(&"p".to_string()) {
        ret = 10.0;

        if is_white {
            ret += PAWN_EVAL_WHITE[x][y];
        } else {
            ret += PAWN_EVAL_BLACK[x][y];
        }
    } else if piece.role.char().to_string().eq(&"b".to_string()) {
        ret = 30.0;

        if is_white {
            ret += BISHOP_EVAL_WHITE[x][y];
        } else {
            ret += BISHOP_EVAL_BLACK[x][y];
        }
    } else if piece.role.char().to_string().eq(&"n".to_string()) {
        ret = 30.0 + KNIGHT_EVAL[x][y];
    } else if piece.role.char().to_string().eq(&"r".to_string()) {
        ret = 50.0;

        if is_white {
            ret += ROOK_EVAL_WHITE[x][y];
        } else {
            ret += ROOK_EVAL_BLACK[x][y];
        }
    } else if piece.role.char().to_string().eq(&"q".to_string()) {
        ret = 90.0 + QUEEN_EVAL[x][y];
    } else if piece.role.char().to_string().eq(&"k".to_string()) {
        ret = 900.0;

        if is_white {
            ret += KING_EVAL_WHITE[x][y];
        } else {
            ret += KING_EVAL_BLACK[x][y];
        }
    }
    if is_white {
        return ret;
    } else {
        return -ret;
    }
}

/// Evaluates the board and returns it's value.
///
/// # Arguments
///
/// * `chess` - Chessboard
///
pub fn evaluate_board(chess: Chess) -> f64 {
    let mut totalvalue = 0.0;
    for x in 0..8 {
        for y in 0..8 {
            let board = Setup::board(&chess);
            let mut square = Square::from_coords(y, x).unwrap();
            square = square.flip_vertical();

            if board.piece_at(square) != None {
                totalvalue += get_piece_value(&board.piece_at(square).unwrap(), x as usize, y as usize);
            }
        }
    }

    return totalvalue;
}

/// Returns the higher value
///
/// # Arguments
///
/// * `x` - First value to check
/// * `y` - second value to check
///
pub fn max(x: f64, y:f64) -> f64 {
    if x > y {
        return x;
    } else {
        return y;
    }
}

/// Returns the lower value
///
/// # Arguments
///
/// * `x` - First value to check
/// * `y` - second value to check
///
pub fn min(x: f64, y: f64) -> f64 {
    if x < y {
        return x;
    } else {
        return y;
    }
}

/// Rootfunction of minimax. Iterates through all moves of the current board and returns the most promising as `String`.
///
/// # Arguments
///
/// * `depth` - Depth for building the decisiontree
/// * `chess` - `Chess of current board`
/// * `is_maximising_player` - Maximise or minimise player?
///
pub fn minimax_root(depth: i8, chess: Chess, is_maximising_player: bool) -> String {
    let mut hc = vec![];
    let moves: MoveList = Position::legals(&chess);
    let best_move_value = Arc::new(Mutex::new(-9000.0));
    let best_move =  Arc::new(Mutex::new(moves[0].clone()));;

    // Benchmark
    let start = SystemTime::now();
    for mov in moves {
        let best_move_value = Arc::clone(&best_move_value);
        let best_move = Arc::clone(&best_move);
        let chess_copy: Chess = chess.clone();
        let h = thread::spawn(move || {
            let undo_chess: Chess = chess_copy.play(&mov).unwrap();

            let value = minimax(depth, undo_chess, -10000.0, 10000.0, is_maximising_player);

            let mut best_move_value = best_move_value.lock().unwrap();
            if value >= *best_move_value {
                *best_move_value = value;

                let mut best_move = best_move.lock().unwrap();
                *best_move = mov;
            }
        });

        hc.push(h);
    }

    for h in hc {
        h.join().unwrap();
    }

    let mut best_move = best_move.lock().unwrap();
    let ret = format!("{}{}", (&*best_move).from().unwrap().to_string(), (&*best_move).to().to_string());

    println!("{}", *best_move_value.lock().unwrap());

    //Benchmark
    let end = SystemTime::now();
    let difference = end.duration_since(start);
    println!("{:?}", difference);

    return ret;
}

/// Minimax function builds the tree, evaluates it and returns it's value as `f64`
///
/// # Arguments
///
/// * `depth` - Depth for building the decisiontree
/// * `chess` - `Chess` to evaluate
/// * `alpha` - current value of best move
/// * `beta` - current value of worst move
/// * `is_maximising_player` - Maximise or minimise player?
///
pub fn minimax(depth: i8, chess: Chess, mut alpha: f64, mut beta: f64, is_maximising_player: bool) -> f64 {
    let mut best_move: f64 = 0.0;
    let moves: MoveList = Position::legals(&chess);

    if depth == 0 {
        return -evaluate_board(chess);
    }

    for mov in moves {
        let chess_copy: Chess = chess.clone();
        let undo_chess = chess_copy.play(&mov).unwrap();

        if is_maximising_player == true {
            best_move = -9999.0;
            best_move = max(best_move, minimax(depth - 1, undo_chess, alpha, beta, !is_maximising_player));
            alpha = max(alpha, best_move);
        } else {
            best_move = 9999.0;
            best_move = min(best_move, minimax(depth - 1, undo_chess, alpha, beta, !is_maximising_player));
            beta = min(beta, best_move);
        }

        if beta <= alpha {
            return best_move;
        }
    }

    return best_move;
}
