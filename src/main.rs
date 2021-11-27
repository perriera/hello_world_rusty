//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
//

fn multiplication(a: i32, b: i32) -> i32 {
    a*b
}

fn multiplication_print_result(a:i32,b:i32) {
    println!("{:?}",a*b);
}

fn main() {
    let x = 2;
    let y = 3;
    println!("{:?}",multiplication(x,y));
    multiplication_print_result(x,y);

}

//
