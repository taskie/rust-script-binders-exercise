use rlua;
use std::error::Error;

fn lua_from_rust() -> rlua::Result<()> {
    use rlua::{Context, Function, Lua, Table};

    let lua = Lua::new();
    let result = lua.context(|ctx: Context| {
        ctx.load("function foo(x, y) return x + y end").exec()?;
        let globals: Table = ctx.globals();
        let foo: Function = globals.get("foo")?;
        let result: i32 = foo.call((5, 3))?;
        Ok(result)
    })?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_from_lua() -> rlua::Result<()> {
    use rlua::{Context, Function, Lua, Table};

    let lua = Lua::new();
    let result = lua.context(|ctx: Context| {
        let foo: Function = ctx.create_function(|_ctx: Context, (x, y): (i32, i32)| Ok(x + y))?;
        let globals: Table = ctx.globals();
        globals.set("foo", foo)?;
        let result: i32 = ctx.load("foo(5, 3)").eval()?;
        Ok(result)
    })?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_prng_from_lua() -> rlua::Result<()> {
    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use rlua::{Context, Function, Lua, Table, UserData, UserDataMethods};
    use std::cell::RefCell;

    struct PRNG {
        rng: Box<RefCell<XorShiftRng>>,
    }

    impl PRNG {
        fn new() -> Self {
            PRNG {
                rng: Box::new(RefCell::new(XorShiftRng::from_seed([0; 16]))),
            }
        }

        fn gen(&self) -> i32 {
            self.rng.as_ref().borrow_mut().gen::<i32>()
        }
    }

    impl UserData for PRNG {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("gen", |_, prng, ()| Ok(prng.gen()));
        }
    }

    impl Drop for PRNG {
        fn drop(&mut self) {
            println!("dropped!")
        }
    }

    let lua = Lua::new();
    let result = lua.context(|ctx: Context| {
        let prng: Function = ctx.create_function(|_, ()| Ok(PRNG::new()))?;
        let globals: Table = ctx.globals();
        globals.set("PRNG", prng)?;
        ctx.load("prng = PRNG()").exec()?;
        let result: i32 = ctx.load("prng:gen()").eval()?;
        println!("Result: {}", result);
        assert_eq!(1788228419, result);
        let result: i32 = ctx.load("prng:gen()").eval()?;
        ctx.load("prng = nil").exec()?;
        Ok(result)
    })?;
    lua.gc_collect()?; // dropped!
    println!("Result: {}", result);
    assert_eq!(195908298, result);
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("# rlua");
    lua_from_rust()?;
    rust_from_lua()?;
    rust_prng_from_lua()?;
    Ok(())
}
