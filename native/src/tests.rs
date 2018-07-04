use neon::vm::{ Call, JsResult, This, FunctionCall };
use neon::js::{ JsString, JsNumber, Value };

use shakmaty::{ Board, Position, Chess, Bitboard, Setup, Color };
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
