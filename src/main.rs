//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
//

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u16,
}

impl User {
    pub fn print_user(self) {
        println!(
            "The name of the user is {}.\nHis email is: {}.\nHe is {} old",
            self.name, self.email, self.age
        )
    }
}
fn main() {
    let new_user = User {
        name: "Dave".to_string(),
        email: "david@mail.com".to_string(),
        age: 32,
    };

    // println!("{:#?}", new_user);

    // print_user(new_user);
    new_user.print_user();
}

//
