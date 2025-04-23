// enum의 메모리는 가장 큰 배리언트의 크기로 정렬된다.
// vector에 여러 배리언트를 저장해도 같은 크기로 정렬되기 때문에 문제가 되지 않는다.
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
//    let mut v = vec![1, 2, 3]; // Vec<i32> 타입의 객체 생성
    let mut v: Vec<i32> = Vec::new(); // 빈 벡터
    v.push(5); // push하는 데이터의 타입을 보고 벡터의 타입을 추론
    v.push(6); // v를 선언할 때 타입을 명시하지 않아도 됨
    v.push(7);
    v.push(8);

    println!("Indexing");
    for i in 0..(v.len()) {
        println!("v[{i}]: {}", v[i]);
    }

    println!("\nget()");
    for i in 0..10 {
        match v.get(i) {
            Some(x) => println!("v[{i}]: {x}"),
            None => println!("None"),
        }
    }

    println!("\n(Mutable) Referencing");
    // 벡터의 각 요소의 대한 참조자 불러오기
    // `for i in &mut v`로 가변 참조자를 불러올 수도 있음
    for i in &mut v {
        *i += 1; // 참조자의 값을 수정하려면 역참조를 해야 한다.
                 // 포인터네 포인터
        println!("{i}");
    }

    let first = &v[0]; // immutable borrow
    v.push(9); // mutable borrow
               // `first` not available anymore
//    println!("{first}"); // error!

    // enum으로 벡터에 다양한 타입 저장하기
    let v1 = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("42")),
        SpreadsheetCell::Float(4.2),
    ];

    println!("\nVarious types in vector");
    for i in v1 {
        match i {
            SpreadsheetCell::Int(int) => println!("Integer: {int}"),
            SpreadsheetCell::Float(float) => println!("Float: {float}"),
            SpreadsheetCell::Text(string) => println!("String: {string}"),
        }
    }
}
// 벡터가 버려질 때 내용물도 모두 버려진다.
