//
// https://levelup.gitconnected.com/rust-with-visual-studio-code-46404befed8
//

extern crate regex;
extern crate some_crate;

//
// https://github.com/reem/stainless
//
#[test]
fn test_user_structure() {
    let new_user = some_crate::User {
        name: "Dave".to_string(),
        email: "david@mail.com".to_string(),
        age: 32,
        user_type: some_crate::UserType::Regular,
    };
    assert_eq!("Dave".to_string(), new_user.name);
}

fn main() {
    let user_type = some_crate::UserType::Regular;
    println!("{:?}", user_type);
    let new_user = some_crate::User {
        name: "Dave".to_string(),
        email: "david@mail.com".to_string(),
        age: 32,
        user_type: some_crate::UserType::Regular,
    };

    // println!("{:#?}", new_user);

    // print_user(new_user);
    new_user.print_user();
}

//
