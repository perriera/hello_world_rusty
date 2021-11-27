//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
//

#[derive(Debug)]
enum UserType {
    Guest,
    Regular,
    Admin,
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u16,
    user_type: UserType,
}

impl User {
    pub fn print_user(self) {
        println!(
            "The name of the user is {}.\nHis email is: {}.\nHe is {} old\nType of user: {:?}",
            self.name, self.email, self.age, self.user_type
        )
    }
}

fn main() {
    let user_type = UserType::Regular;
    println!("{:?}", user_type);
    let new_user = User {
        name: "Dave".to_string(),
        email: "david@mail.com".to_string(),
        age: 32,
        user_type: UserType::Regular,
    };

    // println!("{:#?}", new_user);

    // print_user(new_user);
    new_user.print_user();
}

//
