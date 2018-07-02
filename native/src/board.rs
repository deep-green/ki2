extern crate shakmaty;

use shakmaty::{ Board, Piece, Square, Chess, MoveList, Position, Setup };

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

const KNIGHT_EVAL_WHITE: [[f64; 8]; 8]  = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0,  0.0,  0.0,  0.0,  0.0, -2.0, -4.0],
    [-3.0,  0.0,  1.0,  1.5,  1.5,  1.0,  0.0, -3.0],
    [-3.0,  0.5,  1.5,  2.0,  2.0,  1.5,  0.5, -3.0],
    [-3.0,  0.0,  1.5,  2.0,  2.0,  1.5,  0.0, -3.0],
    [-3.0,  0.5,  1.0,  1.5,  1.5,  1.0,  0.5, -3.0],
    [-4.0, -2.0,  0.0,  0.5,  0.5,  0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0]
];

const KNIGHT_EVAL_BLACK: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0,  0.0,  0.5,  0.5,  0.0, -2.0, -4.0],
    [-3.0,  0.5,  1.0,  1.5,  1.5,  1.0,  0.5, -3.0],
    [-3.0,  0.0,  1.5,  2.0,  2.0,  1.5,  0.0, -3.0],
    [-3.0,  0.5,  1.5,  2.0,  2.0,  1.5,  0.5, -3.0],
    [-3.0,  0.0,  1.0,  1.5,  1.5,  1.0,  0.0, -3.0],
    [-4.0, -2.0,  0.0,  0.0,  0.0,  0.0, -2.0, -4.0],
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
    let is_white: bool = piece.color.char().to_string().eq(&"White".to_string());

    if piece.role.char().to_string().eq(&"p".to_string()) {
        ret = 10.0 + if is_white { PAWN_EVAL_WHITE[x][y] } else { PAWN_EVAL_BLACK[x][y] };
    } else if piece.role.char().to_string().eq(&"b".to_string()) {
        ret = 30.0 + if is_white { BISHOP_EVAL_WHITE[x][y] } else { BISHOP_EVAL_BLACK[x][y] };
    } else if piece.role.char().to_string().eq(&"n".to_string()) {
        ret = 30.0 + if is_white { KNIGHT_EVAL_WHITE[x][y] } else { KNIGHT_EVAL_BLACK[x][y] };
    } else if piece.role.char().to_string().eq(&"r".to_string()) {
        ret = 50.0 + if is_white { ROOK_EVAL_WHITE[x][y] } else { ROOK_EVAL_BLACK[x][y] };
    } else if piece.role.char().to_string().eq(&"q".to_string()) {
        ret = 90.0 + QUEEN_EVAL[x][y];
    } else if piece.role.char().to_string().eq(&"k".to_string()) {
        ret = 900.0 + if is_white { KING_EVAL_WHITE[x][y] } else { KING_EVAL_BLACK[x][y] };
    }

    return ret;
}

pub fn evaluate_board(board: &Board, self_color: Color) -> f64 {
    let mut totalvalue = 0.0;
    for x in 0..8 {
        for y in 0..8 {
            let square = Square::from_coords(x, y).unwrap();
            if board.piece_at(square) != None {
                if {}
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

pub fn minimax(depth: i8, final_chess: Chess, mut alpha: f64, mut beta: f64, is_maximising_player: bool, self_color: Color) -> f64 {
    let mut best_move: f64 = 0.0;
    let board: &Board = Setup::board(&final_chess);

    if depth == 0 {
        return -evaluate_board(board, self_color);
    }

    let moves: MoveList = Position::legals(&final_chess);

    if is_maximising_player {
        best_move = -9999.9;
        for mov in moves {
            let mut chess = final_chess.clone();
            chess = Position::play(chess, &mov).unwrap();
            best_move = max(best_move, minimax(depth - 1, chess, alpha, beta, !is_maximising_player));
            alpha = max(alpha, best_move);
            if beta <= alpha {
                return best_move;
            }
        }
    } else {
        best_move = 9999.9;
        for mov in moves {
            let mut chess = final_chess.clone();
            chess = Position::play(chess, &mov).unwrap();
            best_move = min(best_move, minimax(depth - 1, chess, alpha, beta, !is_maximising_player));
            beta = min(beta, best_move);
            if beta <= alpha {
                return best_move;
            }
        }
    }
    return best_move;
}
