//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
//

fn main() {
    let mut end = false;
    let mut counter = 1;
    while end==false {
        println!("Counter is: {}", counter);
        if counter==10 {
            end = true;
        }
        counter = counter + 1;
    }
}

//
