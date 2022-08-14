//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}
//
//struct Color(i32, i32, i32);
//struct Point(i32, i32, i32);
//
//struct AlwaysEqual;
//
//fn main() {
//    let mut user1 = User {
//        email: String::from("someone@example.com"),
//        username: String::from("someusername123"),
//        active: true,
//        sign_in_count: 1,
//    };
//
//    user1.email = String::from("hoge@example.com");
//
//    let user2 = User {
//        email: String::from("fuga@example.com"),
//        ..user1
//    };
//
//    let black = Color(0, 0, 0);
//    let origin = Point(0, 0, 0);
//
//    let subject = AlwaysEqual;
//}
//
//fn build_user(email: String, username: String) -> User {
//    User {
//        email,
//        username,
//        active: true,
//        sign_in_count: 1,
//    }
//}

//fn main() {
//    let width1 = 30;
//    let height1 = 50;
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(width1, height1)
//    );
//}
//
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
