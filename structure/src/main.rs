struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email = String::from("wat");
    let username = String::from("someone");
    let user1 = build_user(email, username);
    println!("User1 email is: {}", user1.email);
    let user2 = User {
        email: String::from("another1@"),
        username: String::from("another1"),
        ..user1
    };
    println!("user2 email is: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}