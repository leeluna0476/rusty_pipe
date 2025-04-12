fn main() {
    let s = String::from("hello");
    println!("{s}");

//    {
//        let s2 = &s;
//        let s3 = &s;
//        println!("{s2} and {s3} are pointing to {s}");
//
//        let s4 = &mut s;
//        //    println!("{s2} and {s3} are not available anymore");
//        println!("the values that {s4} is pointing can be changed");
//        s4.push_str(" world");
//    }
//    println!("{s} is changed");

//    println!("strlen: {}", strlen_move(s));
//    println!("string: {s}"); // no!

    let (s, len) = strlen_move_move(s);
    println!("strlen: {len}");
    println!("string: {s}");

    let len = strlen_ref(&s);
    println!("strlen: {len}");
    println!("string: {s}");
}

fn strlen_ref(s: & String) -> usize {
    s.len()
}

//fn strlen_move(s: String) -> usize {
//    s.len()
//} // s was moved to this function scope and is not available anymore

fn strlen_move_move(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
} // s was moved to this function scope and is returned again
