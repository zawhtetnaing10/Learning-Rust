fn main() {
    let mut user = User {
        is_active: true,
        username: String::from("Zaw Htet Naing"),
        email: String::from("zawgyi.gog@gmail.com"),
        sign_in_count: 3,
    };
    user.email = String::from("zawhtetnaing722@gmail.com");

    let changed_user = User {
        email: String::from("zawhtetnaing522@gmail.com"),
        ..user
    };
    println!("The old username is {}", changed_user.username);

    // Printing user 1's username will not work anymore becuase it's been moved to changed_user
    println!("Is user 1 still active? : {}", user.is_active);

    let mut user2 = build_user(
        String::from("zawhtetnaing822@gmail.com"),
        String::from("zawhtetnaing822"),
    );
    let r2 = &mut user2;
    r2.username = String::from("zawtetnaing823");
    println!("The new user name is {}", user2.username);

    let white = Color(255, 255, 255);
    let origin = Color(0, 0, 0);

    let Color(r, g, b) = white;
    println!("The colors are {}, {}, {}", r, g, b);

    let scale = 2;
    // Area of rectangle
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("Rect is {rectangle:#?}");

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };
    println!("Can rect1 hold rect2? {}", rectangle.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels",
        rectangle.area()
    );

    dbg!(&rectangle);

    let square = Rectangle::square(30);
    println!("Square : {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        is_active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
