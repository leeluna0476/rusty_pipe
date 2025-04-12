fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s[..]); // immutable borrow.
    println!("{word}"); // word still available.

    s.clear(); // mutable borrow. clear(&mut self)... word not available from now.
//    println!("{word}"); // compile error.

    let word = first_word("42seoul 42seoul"); // string literal은 &'static str
                                              // 바이너리에 하드코딩된 리터럴 문자열을 참조한다.
                                              // &'static str은 &str으로 자동 캐스팅?된다.
    println!("{word}");

    let array = [1, 2, 3, 4, 5];
    let aslice = &array[3..];
    println!("{}", aslice[0]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
