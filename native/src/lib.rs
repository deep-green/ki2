#[macro_use]
extern crate neon;
extern crate shakmaty;

use neon::vm::{ Call, JsResult, This, FunctionCall };
use neon::js::{ JsString, Value };

use shakmaty::{ Position, Chess, Bitboard, Setup, Color };
use shakmaty::fen::Fen;

mod board;
mod tests;

pub trait CheckArgument<'a> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
        self.arguments.require(self.scope, i)?.check::<V>()
    }
}

fn get_move(mut call: Call) -> JsResult<JsString> {
    let fen: String = call.check_argument::<JsString>(0)?.value();

    let setup: Fen = fen.parse().unwrap();
    let chess: Chess = setup.position().unwrap();
    let self_color: Color = setup.turn;

    println!("{:?}", board::minimax(2, chess, -10000.0, 10000.0, true, self_color));

    Ok(JsString::new(call.scope, "e2e4").unwrap())
}

register_module!(m, {
    m.export("getMove", get_move);
    m.export("testEvaluateBoard", tests::test_evaluate_board)
});
