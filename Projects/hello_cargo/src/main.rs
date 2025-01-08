struct User {
    active : bool,
    username: String,
    email : String,
    sign_in_count : u64,
}

struct Color (i32, i32, i32);


fn build_user(email : String, username : String) -> User {
    User {
        active : true,
        username,
        email,
        sign_in_count : 1,
    }
}

fn main() {

    let black = Color (0,0,10);
    let mut user1 = User {
        email : String::from ("someone@example.com"),
        username : String::from ("someusername123"),
        active : true,
        sign_in_count : 1,
    };
    user1.email = String::from ("yukiamano221@gmail.com");
    println!("Hello, world!");
    println!("{}",user1.email);

    let mut user2 : User = build_user ( "asd_mail".to_string(), "my_name".to_string());
    println!("{}",user2.email);
    println!("{}", black.2);
    println!("{}", black.0);

}
