use std::error::Error;

fn js_from_rust() -> Result<(), Box<dyn Error>> {
    use quick_js::{Context, JsValue};
    use std::convert::TryInto;

    let ctx: Context = Context::new()?;
    ctx.eval("function foo(x, y) { return x + y; }")?;
    let result: JsValue = ctx.call_function("foo", vec![5, 3])?;
    let result: i32 = result.try_into()?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_from_js() -> Result<(), Box<dyn Error>> {
    use quick_js::Context;

    let ctx: Context = Context::new()?;
    ctx.add_callback("foo", |x: i32, y: i32| x + y)?;
    let result: i32 = ctx.eval_as("foo(5, 3)")?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("# quick_js");
    js_from_rust()?;
    rust_from_js()?;
    Ok(())
}
