//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
// https://github.com/reem/stainless
//

fn increase_by_five<'a>(x: &'a u16) -> u16 {
    x + 6
}

#[derive(Debug)]
struct TestStruct<'a> {
    x: &'a u32,
}

fn main() {
    let mut x = 5;
    x = increase_by_five(&x);
    println!("{}", x);
}

//
