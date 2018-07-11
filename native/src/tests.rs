use neon::vm::{ Call, JsResult, This, FunctionCall };
use neon::js::{ JsString, JsNumber, Value };

use shakmaty::{ Board, Position, Chess, Bitboard, Setup, Color, Piece, Role };
use shakmaty::fen::Fen;

use board;

pub trait CheckArgument<'a> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
        self.arguments.require(self.scope, i)?.check::<V>()
    }
}

pub fn test_evaluate_board(mut call: Call) -> JsResult<JsNumber> {
    let chess: Chess = Chess::default();

    Ok(JsNumber::new(call.scope, board::evaluate_board(chess)))
}

pub fn test_get_piece_value_of_pawn(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::Pawn;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 6, 1)))
}

pub fn test_get_piece_value_of_knight(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::Knight;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 5, 2)))
}

pub fn test_get_piece_value_of_bishop(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::Bishop;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 5, 2)))
}

pub fn test_get_piece_value_of_rook(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::Rook;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 7, 0)))
}

pub fn test_get_piece_value_of_queen(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::Queen;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 6, 3)))
}

pub fn test_get_piece_value_of_king(mut call: Call) -> JsResult<JsNumber> {
    let role: Role = Role::King;
    let color: Color = Color::White;
    let piece: Piece = Piece{ role, color };

    Ok(JsNumber::new(call.scope, board::get_piece_value(&piece, 6, 2)))
}