use neon::prelude::*;

use crate::Expression;

#[neon::main]
fn export(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("evaluate", evaluate)?;
    Ok(())
}

fn evaluate(mut cx: FunctionContext) -> JsResult<JsArray> {
    let expr = cx.argument::<JsString>(0)?.value(&mut cx);
    let result = Expression::new(&expr).unwrap().evaluate().unwrap();
    let out_arr = cx.empty_array();
    result.iter().enumerate().for_each(|(i, val)| {
        if let crate::fhirpath::Value::String(str) = val {
            let js_str = cx.string(str);
            out_arr.set(&mut cx, i as u32, js_str).unwrap();
        } else {
            todo!()
        }
    });
    Ok(out_arr)
}
