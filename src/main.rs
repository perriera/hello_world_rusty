//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
// https://github.com/reem/stainless
//

use std::thread;

fn main() {
    let thread_1 = thread::spawn(|| "Hello");
    thread::sleep(std::time::Duration::from_millis((100)));
    let thread_2 = thread::spawn(|| "Rust");
    println!("{:?}", thread_2.join().unwrap());
    println!("{:?}", thread_1.join().unwrap());
}

//
