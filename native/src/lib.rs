#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn get_move(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "e2e4").unwrap())
}

register_module!(m, {
    m.export("getMove", get_move)
});
