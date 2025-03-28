use std::string;


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut  user1 = User {
        active: true,
        username: String::from("someusername123"), 
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotherexample.com");

    let mut user2 = User {
        active: user1.active,
        ..user1
    };

    user2.email = String::from("anotherexample.com");

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
