// See also https://docs.rs/ref_thread_local/0.0.0/ref_thread_local/

use std::cell::{RefCell, RefMut};

use once_cell::sync::OnceCell;

type Global<T> = OnceCell<RefCell<T>>;

thread_local! {
    static X: Global<S> = OnceCell::new();
}

struct S {
    x: usize,
    y: usize,
}

fn get<T>(x: &Global<T>) -> RefMut<T> {
    x.get().unwrap().borrow_mut()
}

fn demo() {
    X.with(|s| {
        let mut s = get(s);
        s.x += 1;
        s.y += 2;
        let result = s.x + s.y;
        println!("{} {} {}", s.x, s.y, result);
    });
}

fn init<T>(x: &Global<T>, v: T) {
    if let Err(_) = x.set(RefCell::new(v)) {
        panic!("oops");
    }
}

fn main() {
    X.with(|s| init(s, S{x: 0, y: 0}));
    demo();
    demo();
}
