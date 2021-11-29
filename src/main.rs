//
// https://www.how2shout.com/linux/how-to-install-brew-ubuntu-20-04-lts-linux/
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
// https://github.com/reem/stainless
//

mod structs;
extern crate rand;
use rand::Rng;
use std::io;
use structs::MineField;

fn main() {
    let mut mine_field = MineField {
        size: (3, 3),
        mines: Vec::new(),
    };
    let _ = &mine_field.generate_mines();
    let _ = &mine_field.print_fields();
    let mut x = 0;
    let mut y = 0;
    println!("You have 5 tries");
    let mut counter = 1;
    loop {
        if counter == 5 {
            println!("Congrats")
        }
        println!("Enter coordinate x:");
        let mut input_x = String::new();
        let _ = io::stdin().read_line(&mut input_x);
        let trimmed_x = input_x.trim();
        match trimmed_x.parse::<u32>() {
            Ok(i) => x = i,
            Err(..) => println!("This was not an integer: {}", trimmed_x),
        }
        println!("Enter coordinate y:");
        let mut input_y = String::new();
        let _ = io::stdin().read_line(&mut input_y);
        let trimmed_y = input_y.trim();
        match trimmed_y.parse::<u32>() {
            Ok(i) => y = i,
            Err(..) => println!("This was not an integer: {}", trimmed_y),
        }

        if mine_field.find_by_coordinates(x, y) {
            println!("The game is over :(");
            break;
        }
        counter = counter + 1;
    }
    println!("The mines are marked with o: ");
    mine_field.print_fields_solved();
}

//
