mod vars;

use std::thread;

fn main() {
    vars::add_one(2);
    vars::pluser();

    static X: [i32; 3] = [1, 2, 3];

    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}
