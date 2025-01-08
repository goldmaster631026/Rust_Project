struct User {
    active : bool,
    username: String,
    email : String,
    sign_in_count : u64,
}

#[derive(Debug)]
struct Reactangle {
    width : u32,
    height : u32,
}

impl Reactangle {
    fn can_hold(&self, other: &Reactangle) ->bool {
        self.width> other.width && self.height > other.height
    }
}
struct Color (i32, i32, i32);

fn area (dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn build_user(email : String, username : String) -> User {
    User {
        active : true,
        username,
        email,
        sign_in_count : 1,
    }
}
fn area_struct ( react : & Reactangle) ->u32 {
    react.width * react.height
}

fn main() {
    let rect1 = (30,50);

    let rect2 = Reactangle {
        width : 30,
        height : 60,
    };
    let rect3 = Reactangle {
        width: 30,
        height: 50,
    };
    let rect4 = Reactangle {
        width: 10,
        height: 60,
    };
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
    println! ("{}", area(rect1));
    println! ("{}", area_struct(&rect2));
    // println!("{}", rect1.height);
    println!("{}", rect2.height);
    println! ("{rect2:?}");
    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
}
