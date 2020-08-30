use std::error::Error;

mod pyo3;
mod quick_js;
mod rlua;
mod rusty_v8;

fn main() -> Result<(), Box<dyn Error>> {
    rlua::run()?;
    pyo3::run()?;
    quick_js::run()?;
    rusty_v8::run()?;
    Ok(())
}
