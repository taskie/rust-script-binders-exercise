use std::error::Error;

pub fn prepare() -> Result<(), Box<dyn Error>> {
    use rusty_v8::{new_default_platform, V8};
    let platform = new_default_platform().unwrap();
    V8::initialize_platform(platform);
    V8::initialize();
    Ok(())
}

fn js_from_rust() -> Result<(), Box<dyn Error>> {
    use rusty_v8::{
        self as v8, undefined, Context, ContextScope, Function, HandleScope, Integer, Isolate, Local, Object, Script,
    };
    use std::convert::TryInto;

    let isolate = &mut Isolate::new(Default::default());
    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let code = v8::String::new(scope, "function foo(x, y) { return x + y; }").unwrap();
    let script: Local<Script> = Script::compile(scope, code, None).unwrap();
    script.run(scope).unwrap();

    let global = context.global(scope);
    let foo_key = v8::String::new(scope, "foo").unwrap();
    let foo = global.get(scope, foo_key.into()).unwrap();

    let foo: Local<Object> = foo.to_object(scope).unwrap();
    let foo: Local<Function> = foo.try_into()?;
    let undefined = undefined(scope).into();
    let x = Integer::new(scope, 5);
    let y = Integer::new(scope, 3);
    let result = foo.call(scope, undefined, &[x.into(), y.into()]).unwrap();
    let result: i32 = result.to_int32(scope).unwrap().value() as i32;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_from_js() -> Result<(), Box<dyn Error>> {
    use rusty_v8::{
        self as v8, Context, ContextScope, Function, FunctionCallbackArguments, HandleScope, Integer, Isolate, Local,
        ReturnValue, Script,
    };

    fn foo(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        let x: i32 = args.get(0).to_int32(scope).unwrap().value() as i32;
        let y: i32 = args.get(1).to_int32(scope).unwrap().value() as i32;
        let result = Integer::new(scope, x + y);
        rv.set(result.into());
    }

    let isolate = &mut Isolate::new(Default::default());
    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let foo = Function::new(scope, foo).unwrap();

    let global = context.global(scope);
    let key = v8::String::new(scope, "foo").unwrap();
    global.create_data_property(scope, key.into(), foo.into()).unwrap();

    let code = v8::String::new(scope, "foo(5, 3)").unwrap();
    let script: Local<Script> = Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result: i32 = result.to_int32(scope).unwrap().value() as i32;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("# rusty_v8");
    prepare()?;
    js_from_rust()?;
    rust_from_js()?;
    Ok(())
}
