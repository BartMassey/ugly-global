use ugly_global::*;

global_vars! { sync,
    GLOBAL_S: S;
}

struct S {
    x: usize,
    y: usize,
}

fn demo() {
    fetch!(s = GLOBAL_S);
    s.x += 1;
    s.y += 2;
    let result = s.x + s.y;
    println!("{} {} {}", s.x, s.y, result);
}

fn main() {
    init!(GLOBAL_S = S { x: 0, y: 0 });
    let t = std::thread::spawn(|| demo());
    demo();
    demo();
    t.join().unwrap();
}
