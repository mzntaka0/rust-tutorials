//fn main() {
//    let s = 1;
//    let t = s;
//    println!("{}", t);
//    println!("{}", s);
//}

//fn main() {
//    let s = "Hellow".to_string();
//    let t = s;
//    println!("{}", t);
//    println!("{}", s);
//}

//fn myprint<T: std::fmt::Display>(msg: T) {
//    println!("{}", msg)
//}
//
//fn main() {
//    let s = "Hello".to_string();
//    let ss = s.clone();
//    myprint(s);
//    myprint(ss);
//}

fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", *msg)
}
//
//fn main() {
//    let s = "Hello".to_string();
//    myprint(&s);
//    myprint(&s);
//}

//fn main() {
//    let s = "Hello".to_string();
//    let s_ref = &s;
//    let s_ref2 = &s;
//    myprint(s_ref);
//    myprint(s_ref);
//
//    myprint(s_ref2);
//    myprint(s_ref2);
//}

fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    println!("s={}", s);
    let s_ref = &mut s;
    let s_ref2 = &mut s;
    myclear(s_ref);
    println!("s={}", s);
}
