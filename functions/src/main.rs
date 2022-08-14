//fn main() {
//    println!("Hello, world!");
//    another_function(5, 'h');
//}
//
//fn another_function(x: i32, unit_label: char) {
//    println!("The measurement is: {x}{unit_label}");
//}

//fn main() {
//    //let y = 6;
//    let y = {
//        let x = 3;
//        x + 1
//    };
//
//    println!("The value of y is: {y}");
//}

//fn five() -> i32 {
//    5
//}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}
