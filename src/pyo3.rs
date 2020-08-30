use pyo3;
use std::error::Error;

fn py_from_rust() -> pyo3::PyResult<()> {
    use pyo3::{
        types::{PyAny, PyDict, PyModule},
        GILGuard, Python,
    };

    let gil: GILGuard = Python::acquire_gil();
    let py: Python = gil.python();
    let globals: &PyDict = PyModule::import(py, "__main__")?.dict();
    py.run(
        r#"
def foo(x, y):
    return x + y
"#,
        Some(globals),
        None,
    )?;
    let foo: &PyAny = globals.get_item("foo").unwrap();
    let result: &PyAny = foo.call1((5, 3))?;
    let result: i32 = result.extract()?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_from_py() -> pyo3::PyResult<()> {
    use pyo3::{
        prelude::pyfunction,
        types::{PyAny, PyDict, PyModule},
        wrap_pyfunction, GILGuard, Python,
    };

    let gil: GILGuard = Python::acquire_gil();
    let py: Python = gil.python();
    let globals: &PyDict = PyModule::import(py, "__main__")?.dict();

    #[pyfunction]
    fn foo(x: i32, y: i32) -> i32 {
        x + y
    }

    globals.set_item("foo", wrap_pyfunction!(foo)(py))?;
    let result: &PyAny = py.eval("foo(5, 3)", Some(globals), None)?;
    let result: i32 = result.extract()?;
    println!("Result: {}", result);
    assert_eq!(8, result);
    Ok(())
}

fn rust_prng_from_py() -> pyo3::PyResult<()> {
    use pyo3::{
        prelude::{pyclass, pymethods},
        types::{PyDict, PyModule, PyType},
        GILGuard, PyObject, Python,
    };
    use rand::{Rng, SeedableRng};
    use rand_xorshift::XorShiftRng;
    use std::cell::RefCell;

    #[pyclass]
    struct PRNG {
        rng: Box<RefCell<XorShiftRng>>,
    }

    #[pymethods]
    impl PRNG {
        #[new]
        fn new() -> Self {
            PRNG {
                rng: Box::new(RefCell::new(XorShiftRng::from_seed([0; 16]))),
            }
        }

        fn gen(&self) -> i32 {
            self.rng.as_ref().borrow_mut().gen::<i32>()
        }
    }

    impl Drop for PRNG {
        fn drop(&mut self) {
            println!("dropped!")
        }
    }

    let gil: GILGuard = Python::acquire_gil();
    let py: Python = gil.python();
    let prng_cls: &PyType = py.get_type::<PRNG>();
    let globals: &PyDict = PyModule::import(py, "__main__")?.dict();
    globals.set_item("PRNG", prng_cls)?;

    py.run("prng = PRNG()", Some(globals), None)?;
    let result: i32 = py.eval("prng.gen()", Some(globals), None)?.extract()?;
    println!("Result: {}", result);
    assert_eq!(1788228419, result);
    let result: i32 = py.eval("prng.gen()", Some(globals), None)?.extract()?;
    py.run("prng = None", Some(globals), None)?; // dropped!
    println!("Result: {}", result);
    assert_eq!(195908298, result);
    Ok(())
}

pub fn convert_err(e: pyo3::PyErr) -> Box<dyn Error> {
    use crate::util::SimpleError;
    use pyo3::Python;

    let gil = Python::acquire_gil();
    let py = gil.python();
    let dbg = format!("{:?}", &e);
    e.print(py); // PyErr moved...

    Box::new(SimpleError(dbg))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("# pyo3");
    py_from_rust().map_err(convert_err)?;
    rust_from_py().map_err(convert_err)?;
    rust_prng_from_py().map_err(convert_err)?;
    Ok(())
}
