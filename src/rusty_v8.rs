use std::error::Error;

pub fn prepare() {
    use rusty_v8::{new_default_platform, Platform, UniqueRef, V8};
    let platform: UniqueRef<Platform> = new_default_platform().unwrap();
    V8::initialize_platform(platform);
    V8::initialize();
}

fn js_from_rust() -> Option<()> {
    use rusty_v8::{
        self as v8, undefined, Context, ContextScope, Function, HandleScope, Integer, Isolate, Local, Object, Script,
    };
    use std::convert::TryInto;

    let isolate = &mut Isolate::new(Default::default());
    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let code = v8::String::new(scope, "function foo(x, y) { return x + y; }")?;
    let script: Local<Script> = Script::compile(scope, code, None)?;
    script.run(scope)?;

    let global = context.global(scope);
    let foo_key = v8::String::new(scope, "foo")?;
    let foo = global.get(scope, foo_key.into())?;

    let foo: Local<Object> = foo.to_object(scope)?;
    let foo: Local<Function> = foo.try_into().ok()?;
    let undefined = undefined(scope).into();
    let x = Integer::new(scope, 5);
    let y = Integer::new(scope, 3);
    let result = foo.call(scope, undefined, &[x.into(), y.into()])?;
    let result: i32 = result.to_int32(scope)?.value() as i32;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Some(())
}

fn rust_from_js() -> Option<()> {
    use rusty_v8::{
        self as v8, Context, ContextScope, Function, FunctionCallbackArguments, HandleScope, Integer, Isolate, Local,
        ReturnValue, Script,
    };

    fn foo(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
        let x = args.get(0).to_int32(scope).unwrap().value() as i32;
        let y = args.get(1).to_int32(scope).unwrap().value() as i32;
        let result = Integer::new(scope, x + y);
        rv.set(result.into());
    }

    let isolate = &mut Isolate::new(Default::default());
    let scope = &mut HandleScope::new(isolate);
    let context = Context::new(scope);
    let scope = &mut ContextScope::new(scope, context);

    let foo = Function::new(scope, foo)?;

    let global = context.global(scope);
    let key = v8::String::new(scope, "foo")?;
    global.create_data_property(scope, key.into(), foo.into())?;

    let code = v8::String::new(scope, "foo(5, 3)")?;
    let script: Local<Script> = Script::compile(scope, code, None)?;
    let result = script.run(scope)?;
    let result = result.to_int32(scope)?.value() as i32;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Some(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    use crate::util::SimpleError;
    println!("# rusty_v8");
    prepare();
    js_from_rust().ok_or(SimpleError("js_from_rust".to_owned()))?;
    rust_from_js().ok_or(SimpleError("rust_from_js".to_owned()))?;
    Ok(())
}
