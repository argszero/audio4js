use std::cell::RefCell;

use controller::NodeRodioController;
use neon::prelude::*;
pub mod controller;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}
impl Finalize for NodeRodioController {}

impl NodeRodioController {
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<RefCell<NodeRodioController>>> {
        let controller = NodeRodioController::new();
        Ok(cx.boxed(RefCell::new(controller)))
    }
    fn js_play(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let path = cx.argument::<JsString>(0)?.value(&mut cx);
        let skip_secs = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;
        let controller = cx
            .this()
            .downcast_or_throw::<JsBox<RefCell<NodeRodioController>>, _>(&mut cx)?;
        controller.borrow_mut().play(path, skip_secs).unwrap();
        Ok(cx.undefined())
    }
    fn js_stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let controller = cx
            .this()
            .downcast_or_throw::<JsBox<RefCell<NodeRodioController>>, _>(&mut cx)?;
        controller.borrow_mut().stop();
        Ok(cx.undefined())
    }
    fn js_set_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let volume = cx.argument::<JsNumber>(0)?.value(&mut cx) as f32;
        let controller = cx
            .this()
            .downcast_or_throw::<JsBox<RefCell<NodeRodioController>>, _>(&mut cx)?;
        controller.borrow_mut().set_volume(volume);
        Ok(cx.undefined())
    }
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("js_new", NodeRodioController::js_new)?;
    cx.export_function("js_play", NodeRodioController::js_play)?;
    cx.export_function("js_stop", NodeRodioController::js_stop)?;
    cx.export_function("js_set_volume", NodeRodioController::js_set_volume)?;
    Ok(())
}
