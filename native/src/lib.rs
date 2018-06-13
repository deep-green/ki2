#[macro_use]
extern crate neon;
extern crate chess_move_gen;

use neon::vm::{Call, JsResult, This, FunctionCall};
use neon::js::{JsString, Value};
use chess_move_gen::*;


trait CheckArgument<'a> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
        self.arguments.require(self.scope, i)?.check::<V>()
    }
}

fn get_move(mut call: Call) -> JsResult<JsString> {
    let fen: String = call.check_argument::<JsString>(0)?.value();

    let mut list = MoveVec::new();
    let position = &Position::from_fen(&fen).unwrap();
    legal_moves::<MoveVec>(position, &mut list);

    for item in list.iter() {
        println!("{:#?}", item);
    }

    Ok(JsString::new(call.scope, "e2e4").unwrap())
}

register_module!(m, {
    m.export("getMove", get_move)
});
