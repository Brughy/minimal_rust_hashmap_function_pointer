use std::collections::HashMap;
use lazy_static::lazy_static;

fn a() { println!("function a"); }

lazy_static! {
    static ref MAP: HashMap<&'static str, fn()> = {
        let mut t = HashMap::new();
        t.insert("aa", a as fn());
        t
    };
}

fn main() {
    let run = *(MAP.get("aa").unwrap());
    run();
}
