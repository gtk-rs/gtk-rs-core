use std::cell::{Cell, RefCell};
use std::panic;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use futures_executor::block_on;
use glib::clone_block;

struct State {
    count: i32,
    started: bool,
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            started: false,
        }
    }
}

#[test]
#[clone_block]
fn clone_and_references() {
    let state = Rc::new(RefCell::new(State::new()));
    let ref_state = &state;
    assert!(!ref_state.borrow().started);

    let closure = {
        move |#[weak] ref_state| {
            ref_state.unwrap().borrow_mut().started = true;
        }
    };

    closure();
    assert!(ref_state.borrow().started);
}

#[test]
#[clone_block]
fn subfields_renaming() {
    struct Foo {
        v: Rc<usize>,
    }

    impl Foo {
        fn foo(&self) {
            let state = Rc::new(RefCell::new(State::new()));

            let closure = move |#[strong(self.v)] v, #[weak(state)] hello, _| {
                println!("v: {v}");
                hello.unwrap().borrow_mut().started = true;
            };
            closure(2);
        }
    }

    Foo { v: Rc::new(0) }.foo();
}

#[test]
#[clone_block]
fn renaming() {
    let state = Rc::new(RefCell::new(State::new()));
    assert!(!state.borrow().started);

    let closure = {
        move |#[weak(state)] hello| {
            hello.unwrap().borrow_mut().started = true;
        }
    };

    closure();
    assert!(state.borrow().started);
}

#[test]
#[clone_block]
fn clone_closure() {
    let state = Rc::new(RefCell::new(State::new()));
    assert!(!state.borrow().started);

    let closure = {
        move |#[weak(or_return)] state| {
            state.borrow_mut().started = true;
        }
    };

    closure();

    assert!(state.borrow().started);
    assert_eq!(state.borrow().count, 0);

    let closure = {
        let state2 = Rc::new(RefCell::new(State::new()));
        assert!(state.borrow().started);

        move |#[weak(or_return)] state, #[strong] state2| {
            state.borrow_mut().count += 1;
            state.borrow_mut().started = true;
            state2.borrow_mut().started = true;
        }
    };

    closure();

    assert_eq!(state.borrow().count, 1);
    assert!(state.borrow().started);
}

#[test]
#[clone_block]
fn clone_default_value() {
    let closure = {
        let state = Rc::new(RefCell::new(State::new()));
        move |_, #[weak(or_return 42)] state| {
            state.borrow_mut().started = true;
            10
        }
    };

    assert_eq!(42, closure(50));
}

#[test]
#[clone_block]
fn clone_panic() {
    let state = Arc::new(Mutex::new(State::new()));
    state.lock().expect("Failed to lock state mutex").count = 20;

    let closure = {
        let state2 = Arc::new(Mutex::new(State::new()));
        move |#[weak(or_panic)] state2, #[strong] state, _| {
            state.lock().expect("Failed to lock state mutex").count = 21;
            state2.lock().expect("Failed to lock state2 mutex").started = true;
            10
        }
    };

    let result = panic::catch_unwind(|| {
        closure(50);
    });

    assert!(result.is_err());

    assert_eq!(state.lock().expect("Failed to lock state mutex").count, 20);
}

#[test]
fn clone_import_rename() {
    import_rename::test();
}

#[clone_block]
mod import_rename {
    use glib::clone_block as clone_block_g;

    #[allow(unused_macros)]
    macro_rules! clone_block {
        ($($anything:tt)*) => {
            |_, _| panic!("The clone_block macro doesn't support renaming")
        };
    }

    #[allow(unused_variables)]
    #[clone_block_g]
    pub fn test() {
        let n = 2;

        let closure: Box<dyn Fn(u32, u32)> = Box::new(move |#[strong] n, _, _| {
            println!("The clone_block macro does support renaming")
        });

        closure(0, 0);
    }
}

#[test]
#[clone_block]
fn test_clone_macro_self_rename() {
    #[derive(Debug)]
    struct Foo {
        v: u8,
    }

    impl Foo {
        #[allow(dead_code)]
        fn foo(&self) {
            let closure = move |_x, #[strong(self)] this| {
                println!("v: {this:?}");
            };
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = move |#[strong(self)] this| {
                println!("v: {this:?}");
            };
            let closure = move |_x, #[strong(self)] this| println!("v: {this:?}");
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = move |#[strong(self)] this| println!("v: {this:?}");

            // Fields now!
            let closure = move |_x, #[strong(self.v)] v| {
                println!("v: {v:?}");
            };
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = move |#[strong(self.v)] v| println!("v: {v:?}");

            // With default_panic
            let closure = #[default_panic]
            move |#[strong(self.v)] v, _x| {
                println!("v: {v:?}");
            };
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = #[default_panic]
            move |#[strong(self.v)] v| println!("v: {v:?}");

            // With default_return
            let closure = #[default_return(true)]
            move |#[strong(self.v)] _v, _x| false;
            closure(0i8); // to prevent compiler error for unknown `x` type.
            let _ = #[default_return(true)]
            move |#[strong(self.v)] _v| false;
        }
    }
}

#[test]
#[clone_block]
fn test_clone_macro_rename() {
    let v = Rc::new(1);

    let closure = move |_x, #[weak(v or_panic)] y| {
        println!("v: {y}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak(v or_panic)] y| println!("v: {y}");

    let closure = move |_x, #[strong(v)] y| {
        println!("v: {y}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong(v)] y| println!("v: {y}");

    let closure = move |_x, #[weak(v or_return)] y| {
        println!("v: {y}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak(v or_return)] y| println!("v: {y}");

    let closure = move |_x, #[strong(v)] y| {
        println!("v: {y}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong(v)] y| println!("v: {y}");

    let closure = move |_x, #[weak(v or_return true)] _y| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak(v or_return true)] _y| false;

    let closure = move |_x, #[strong(v)] _y| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong(v)] _y| false;
}

#[test]
#[clone_block]
#[allow(unused_variables)]
fn test_clone_macro_simple() {
    let v = Rc::new(1);

    let closure = move |_x, #[weak(or_panic)] v| {
        println!("v: {v}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak(or_panic)] v| println!("v: {v}");

    let closure = move |_x, #[strong] v| {
        println!("v: {v}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong] v| println!("v: {v}");

    let closure = move |#[weak] v, _x| {
        println!("v: {}", v.unwrap());
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak] v| println!("v: {}", v.unwrap());

    let closure = move |_x, #[strong] v| {
        println!("v: {v}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong] v| println!("v: {v}");

    let closure = move |_x, #[weak(or_return true)] v| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[weak(or_return true)] v| false;

    let closure = move |_x, #[strong] v| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong] v| false;
}

#[test]
#[clone_block]
#[allow(unused_variables)]
fn test_clone_macro_double_simple() {
    let v = Rc::new(1);
    let w = Rc::new(2);

    let closure = move |#[weak(or_panic)] v, #[weak(or_panic)] w, _x| {
        println!("v: {v}, w: {w}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = #[default_panic]
    move |#[weak] v, #[weak] w| println!("v: {v}, w: {w}");

    let closure = #[default_panic]
    move |#[strong] v, #[strong] w, _x| {
        println!("v: {v}, w: {w}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = #[default_panic]
    move |#[strong] v, #[strong] w| println!("v: {v}, w: {w}");

    let closure = #[default_return]
    move |_x, #[weak] v, #[weak] w| {
        println!("v: {v}, w: {w}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = #[default_return]
    move |#[weak] v, #[weak] w| println!("v: {v}, w: {w}");

    let closure = move |_x, #[strong] v, #[strong] w| {
        println!("v: {v}, w: {w}");
    };
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = move |#[strong] v, #[strong] w| println!("v: {v}, w: {w}");

    let closure = #[default_return(true)]
    move |_x, #[weak] v, #[weak] w| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = #[default_return(true)]
    move |#[weak] v, #[weak] w| false;

    let closure = #[default_return(true)]
    move |#[strong] v, #[strong] w, _x| false;
    closure(0i8); // to prevent compiler error for unknown `x` type.
    let _ = #[default_return(true)]
    move |#[strong] v, #[strong] w| false;
}

#[test]
#[clone_block]
#[allow(unused_variables)]
fn test_clone_macro_double_rename() {
    let v = Rc::new(1);
    let w = Rc::new(2);
    let done = Rc::new(RefCell::new(0));

    let closure = #[default_panic]
    move |z, #[weak(v)] x, #[weak] w| z + *x + *w;
    assert_eq!(closure(1i8), 4i8);
    let closure = #[default_panic]
    move |#[weak(v)] x, #[weak] w| 1;
    assert_eq!(closure(), 1);

    let closure = #[default_panic]
    move |z, #[weak] v, #[weak(w)] x| z + *v + *x;
    assert_eq!(closure(10i8), 13i8);
    let closure = #[default_panic]
    move |#[weak] v, #[weak(w)] x| 2 + *x;
    assert_eq!(closure(), 4);

    let closure = #[default_panic]
    move |#[strong(v)] x, #[strong] w, z| z + *x + *w;
    assert_eq!(closure(3i8), 6i8);
    let closure = #[default_panic]
    move |#[strong(v)] x, #[strong] w| 4 + *w;
    assert_eq!(closure(), 6);

    let closure = #[default_panic]
    move |#[strong] v, #[strong(w)] x, z| z + *v + *x;
    assert_eq!(closure(0i8), 3i8);
    let closure = #[default_panic]
    move |#[strong] v, #[strong(w)] x| 5;
    assert_eq!(closure(), 5);

    let t_done = done.clone();
    let closure = #[default_return]
    move |z, #[weak(v)] x, #[weak] w| {
        *t_done.borrow_mut() = z + *x + *w;
    };
    closure(4i8);
    assert_eq!(*done.borrow(), 7);
    let t_done = done.clone();
    let closure = #[default_return]
    move |#[weak(v)] x, #[weak] w| *t_done.borrow_mut() = *x + *w;
    closure();
    assert_eq!(*done.borrow(), 3);

    let t_done = done.clone();
    let closure = #[default_return]
    move |z, #[weak] v, #[weak(w)] x| {
        *t_done.borrow_mut() = z + *v + *x;
    };
    closure(8i8);
    assert_eq!(*done.borrow(), 11i8);
    let t_done = done.clone();
    let closure = #[default_return]
    move |#[weak] v, #[weak(w)] x| *t_done.borrow_mut() = *v * *x;
    closure();
    assert_eq!(*done.borrow(), 2);

    let t_done = done.clone();
    let closure = move |z, #[strong(v)] x, #[strong] w| {
        *t_done.borrow_mut() = z + *x + *w;
    };
    closure(9i8);
    assert_eq!(*done.borrow(), 12i8);
    let t_done = done.clone();
    let closure = move |#[strong(v)] x, #[strong] w| *t_done.borrow_mut() = *x - *w;
    closure();
    assert_eq!(*done.borrow(), -1);

    let t_done = done.clone();
    let closure = move |z, #[strong] v, #[strong(w)] x| {
        *t_done.borrow_mut() = *v + *x * z;
    };
    closure(2i8);
    assert_eq!(*done.borrow(), 5);
    let t_done = done.clone();
    let closure = move |#[strong] v, #[strong(w)] x| *t_done.borrow_mut() = *x - *v;
    closure();
    assert_eq!(*done.borrow(), 1);

    let closure = #[default_return(true)]
    move |_, #[weak(v)] _x, #[weak] w| false;
    assert!(!closure(0u8));
    let closure = #[default_return(true)]
    move |#[weak(v)] _x, #[weak] w| false;
    assert!(!closure());

    let closure = #[default_return(true)]
    move |_, #[weak] v, #[weak(w)] _x| false;
    assert!(!closure("a"));
    let closure = #[default_return(true)]
    move |#[weak] v, #[weak(w)] _x| false;
    assert!(!closure());

    let closure = #[default_return(true)]
    move |#[strong(v)] _x, #[strong] w, _| false;
    assert!(!closure('a'));
    let closure = #[default_return(true)]
    move |#[strong(v)] _x, #[strong] w| false;
    assert!(!closure());

    let closure = #[default_return(true)]
    move |#[strong] v, #[strong(w)] _x, _| false;
    assert!(!closure(12.));
    let closure = #[default_return(true)]
    move |#[strong] v, #[strong(w)] _x| false;
    assert!(!closure());
}

macro_rules! test_arc_closure {
    ($name:ident, $kind:tt, $($fail:tt)+) => {
        #[test]
        #[clone_block]
        fn $name() {
            // We need Arc and Mutex to use them below in the thread.
            let check = Arc::new(Mutex::new(0));
            let v = Arc::new(Mutex::new(1));
            let w = Arc::new(Mutex::new(1));

            let closure = #[$($fail)+] move |#[$kind(v)] x, #[$kind] w, #[weak] check, arg: i8| {
                *x.lock().unwrap() += arg;
                *w.lock().unwrap() += arg;
                *check.lock().unwrap() += 1;
            };
            closure(1);
            assert_eq!(2, *v.lock().unwrap());
            assert_eq!(2, *w.lock().unwrap());
            assert_eq!(1, *check.lock().unwrap());

            let closure2 = #[$($fail)+] move |#[$kind] v, #[$kind(w)] x, #[weak] check, arg: i8| {
                *v.lock().unwrap() += arg;
                *x.lock().unwrap() += arg;
                *check.lock().unwrap() += 1;
            };
            closure2(1);
            assert_eq!(3, *v.lock().unwrap());
            assert_eq!(3, *w.lock().unwrap());
            assert_eq!(2, *check.lock().unwrap());

            macro_rules! inner {
                (strong) => {{}};
                (weak) => {{
                    std::mem::drop(v);
                    std::mem::drop(w);

                    // We use the threads to ensure that the closure panics as expected.
                    assert!(thread::spawn(move || {
                        closure(1);
                    }).join().is_err());
                    assert_eq!(2, *check.lock().unwrap());
                    assert!(thread::spawn(move || {
                        closure2(1);
                    }).join().is_err());
                    assert_eq!(2, *check.lock().unwrap());
                }}
            }

            inner!($kind);
        }
    };
}

macro_rules! test_rc_closure {
    ($name:ident, $kind:tt, $($fail:tt)+) => {
        #[test]
        #[clone_block]
        fn $name() {
            let check = Rc::new(RefCell::new(0));
            let v = Rc::new(RefCell::new(1));
            let w = Rc::new(RefCell::new(1));

            let closure = #[$($fail)+] move |#[$kind(v)] x, #[$kind] w, #[weak] check, arg: i8| {
                *x.borrow_mut() += arg;
                *w.borrow_mut() += arg;
                *check.borrow_mut() += 1;
            };
            closure(1);
            assert_eq!(2, *v.borrow());
            assert_eq!(2, *w.borrow());
            assert_eq!(1, *check.borrow());

            let closure2 = #[$($fail)+] move |#[$kind] v, #[$kind(w)] x, #[weak] check, arg: i8| {
                *v.borrow_mut() += arg;
                *x.borrow_mut() += arg;
                *check.borrow_mut() += 1;
            };
            closure2(1);
            assert_eq!(3, *v.borrow());
            assert_eq!(3, *w.borrow());
            assert_eq!(2, *check.borrow());

            macro_rules! inner {
                (strong) => {{}};
                (weak) => {{
                    std::mem::drop(v);
                    std::mem::drop(w);

                    closure(1);
                    assert_eq!(2, *check.borrow());
                    closure2(1);
                    assert_eq!(2, *check.borrow());
                }}
            }

            inner!($kind);
        }
    };
}

test_arc_closure!(test_clone_macro_typed_arc_weak, weak, default_panic);
test_arc_closure!(test_clone_macro_typed_arc_strong, strong, default_panic);
test_rc_closure!(test_clone_macro_typed_rc_weak, weak, default_return);
test_rc_closure!(test_clone_macro_typed_rc_strong, strong, default_return);

#[test]
#[clone_block]
fn test_clone_macro_typed_args() {
    let check = Rc::new(RefCell::new(0));
    let v = Rc::new(RefCell::new(1));
    let w = Rc::new(RefCell::new(1));
    let closure = move |#[weak(or_return)] v,
                        #[weak(w or_return )] x,
                        #[weak(or_return)] check,
                        arg: i8,
                        arg2| {
        *v.borrow_mut() = arg;
        *x.borrow_mut() = arg2;
        *check.borrow_mut() += 1;
    };
    closure(0, 9);
    assert_eq!(0, *v.borrow());
    assert_eq!(9, *w.borrow());
    assert_eq!(1, *check.borrow());

    std::mem::drop(v);
    std::mem::drop(w);
    assert_eq!(1, *check.borrow());
}

macro_rules! test_default {
    ($name:ident, $ret:expr, $($closure_body:tt)*) => {
        #[test]
        #[clone_block]
        #[allow(clippy::bool_assert_comparison)]
        #[allow(clippy::nonminimal_bool)]
        fn $name() {
            let v = Rc::new(1);
            let tmp = move |#[weak(v or_return $ret)] _v| $($closure_body)*;
            assert_eq!(tmp(), $($closure_body)*, "shouldn't use or_return value!");
            ::std::mem::drop(v);
            assert_eq!(tmp(), $ret, "should use or_return value!");
        }
    }
}

#[derive(PartialEq, Debug)]
struct Foo(i32);

test_default!(test_clone_macro_default_return_newtype, Foo(0), Foo(1));

#[derive(PartialEq, Debug)]
struct Bar {
    x: i32,
}

test_default!(
    test_clone_macro_default_return_struct,
    Bar { x: 0 },
    Bar { x: 1 }
);

#[derive(PartialEq, Debug)]
enum Enum {
    A,
    B(i32),
    C { x: i32 },
}
test_default!(
    test_clone_macro_default_return_enum_unit,
    Enum::A,
    Enum::B(0)
);
test_default!(
    test_clone_macro_default_return_enum_tuple,
    Enum::B(0),
    Enum::A
);
test_default!(
    test_clone_macro_default_return_enum_struct,
    Enum::C { x: 0 },
    Enum::A
);
test_default!(
    test_clone_macro_default_return_expr,
    {
        let x = 12;
        x + 2
    },
    19
);
// This one is simply to check that we wait for the comma for the default-return value.
test_default!(
    test_clone_macro_default_return_bool,
    Enum::A == Enum::B(0) || false,
    true
);

#[test]
#[clone_block]
fn test_clone_macro_body() {
    let v = Arc::new(Mutex::new(0));

    let closure = #[default_return]
    move |#[weak] v| {
        std::thread::spawn(move || {
            let mut lock = v.lock().expect("failed to lock");
            for _ in 1..=10 {
                *lock += 1;
            }
        })
        .join()
        .expect("thread::spawn failed");
    };
    closure();
    assert_eq!(10, *v.lock().expect("failed to lock"));
}

#[test]
#[clone_block]
fn test_clone_macro_async_kinds() {
    let v = Rc::new(RefCell::new(1));

    let closure = move |#[weak(or_return)] v| async move {
        *v.borrow_mut() += 1;
    };
    block_on(closure());
    assert_eq!(*v.borrow(), 2);
    block_on(
        #[clone(weak(or_return) v)]
        async move {
            *v.borrow_mut() += 1;
        },
    );
    assert_eq!(*v.borrow(), 3);
}

#[test]
#[clone_block]
fn test_clone_attr() {
    let func = {
        let a = Rc::new(Cell::new(1));
        let b = Rc::new(Cell::new(2));
        let c = Rc::new(Cell::new(3));
        let func = #[clone(strong a, weak b, weak(c or_return 2) d, default_return(1))]
        move |x| a.get() + b.get() + d.get() + x;
        assert_eq!(func(4), 10);
        func
    };
    assert_eq!(func(500), 1);
}
