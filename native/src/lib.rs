#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, This, FunctionCall};
use neon::js::{JsString, Value};

trait CheckArgument<'a> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
    fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
        self.arguments.require(self.scope, i)?.check::<V>()
    }
}

fn get_move(mut call: Call) -> JsResult<JsString> {
    let fen = call.check_argument::<JsString>(0)?.value();
    Ok(JsString::new(call.scope, &fen).unwrap())
}

register_module!(m, {
    m.export("getMove", get_move)
});
