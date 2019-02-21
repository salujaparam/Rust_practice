fn main() {
    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        password: String,
    };

    let user1 = User{
        name: String::from("param"),
        email: String::from("salujaparam@gmail.com"),
        password: String::from("abc123"),
    };

    println!("my e-mail is {}", user1.email);
}
