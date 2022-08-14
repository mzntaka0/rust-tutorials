//fn main() {
//    let mut s = String::from("hello");
//
//    s.push_str(", world");
//
//    println!("{}", s);
//}

//fn main() {
//    let mut s1 = String::from("hello");
//    let s2 = s1;
//
//    println!("{}", s1);
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let s2 = s1.clone();
//
//    println!("x = {s1}, y = {s2}");
//}

//fn main() {
//    let x = 5;
//    let y = x;
//
//    println!("x = {x}, y = {y}");
//}

//fn main() {
//    let s = String::from("hello");
//
//    takes_ownership(s);
//
//    let x = 5;
//
//    makes_copy(x);
//
//    println!("{x}");
//}
//
//fn takes_ownership(some_string: String) {
//    println!("{}", some_string);
//}
//
//fn makes_copy(some_integer: i32) {
//    println!("{}", some_integer);
//}

//fn main() {
//    let s1 = gives_ownership();
//
//    let s2 = String::from("hello");
//
//    let s3 = takes_and_gives_back(s2);
//}
//
//fn gives_ownership() -> String {
//    let some_string = String::from("yours");
//    some_string
//}
//
//fn takes_and_gives_back(a_string: String) -> String {
//    a_string
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let (s2, len) = calculate_length(s1);
//
//    println!("The length of '{}' is {}.", s2, len);
//}
//
//fn calculate_length(s: String) -> (String, usize) {
//    let length = s.len();
//
//    (s, length)
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of '{}' is {}.", s1, len);
//}
//
//fn calculate_length(s: &String) -> usize {
//    s.len()
//}

//fn main() {
//    let mut s = String::from("hello");
//
//    change(&mut s);
//}
//
//fn change(some_string: &mut String) {
//    some_string.push_str(", world")
//}

//fn main() {
//    let mut s = String::from("hello");
//
//    let r1 = &mut s;
//    let r2 = &mut s;
//
//    println!("{}, {}", r1, r2);
//}

//fn main() {
//    let mut s = String::from("hello");
//
//    let r1 = &s;
//    let r2 = &s;
//    let r3 = &mut s;
//
//    println!("{}, {}, and {}", r1, r2, r3);
//}

//fn main() {
//    let mut s = String::from("hello");
//
//    let r1 = &s;
//    let r2 = &s;
//    println!("{} and {}", r1, r2);
//
//    let r3 = &mut s;
//    println!("{}", r3);
//}

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
