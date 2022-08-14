//fn main() {
//    let mut s = String::from("hoge aa");
//    let word = first_word(&s);
//
//    s.clear();
//}
//
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}

//fn main() {
//    let s = String::from("hellow world");
//
//    let hello = &s[0..5];
//    let world = &s[6..11];
//}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hoge");

    let res = first_word(&s);

    println!("{res}")
}
