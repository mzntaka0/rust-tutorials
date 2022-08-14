//fn main() {
//    let x = 1;
//    println!("x = {}", x);
//
//    let y = 2;
//    println!("y = {}", y)
//}

//fn main() {
//    let mut x = 1;
//    x = x + 1;
//    println!("x = {}", x);
//}

fn main() {
    let x = 5;
    let x = x + 1;
    println!("{x}");

    {
        let x = x * 2;
        println!("{x}")
    }

    println!("{x}")
}
