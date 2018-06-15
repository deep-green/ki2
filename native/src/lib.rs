#[macro_use]
extern crate neon;
extern crate chess;

use neon::vm::{Call, JsResult, This, FunctionCall};
use neon::js::{JsString, Value};

use chess::{ MoveGen, Board };


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

    let board = Board::from_fen(fen.to_owned()).unwrap();
    let iterable = MoveGen::new(board, true);

    let mut ret: String = "[".to_string();
    let mut count: usize = 0;
    let size: usize = iterable.len();

    for item in iterable {
        ret.push_str(&item.to_string());

        if count < size - 1 {
            ret.push_str(&", ");
        }

        count += 1;
    }

    ret.push_str(&"]");

    Ok(JsString::new(call.scope, &ret).unwrap())
}

register_module!(m, {
    m.export("getMove", get_move)
});
