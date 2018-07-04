extern crate shakmaty;

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


fn get_piece_value(piece: &Piece, x: usize, y: usize) -> f64 {
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

fn max(x: f64, y:f64) -> f64 {
    if x > y {
        return x;
    } else {
        return y;
    }
}

fn min(x: f64, y: f64) -> f64 {
    if x < y {
        return x;
    } else {
        return y;
    }
}

pub fn minimax_root(depth: i8, chess: Chess, is_maximising_player: bool) -> Move {
    let moves: MoveList = Position::legals(&chess);
    let mut best_move_value: f64 = -9000.0;
    let mut value: f64;
    let mut best_move: Move = moves[0].clone();

    for mov in moves {
        let chess_copy: Chess = chess.clone();
        let undo_chess: Chess = chess_copy.play(&mov).unwrap();

        value = minimax(depth - 1, undo_chess, -10000.0, 10000.0, !is_maximising_player);

        if value >= best_move_value {
            best_move_value = value;
            best_move = mov;
        }
    }
    println!("{:?}", best_move_value);
    return best_move;
}

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
            beta = min(alpha, best_move);
        }

        if beta <= alpha {
            return best_move;
        }
    }

    return best_move;
}
