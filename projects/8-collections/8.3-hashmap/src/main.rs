// HashMap은 Vec, String에 비해 많이 사용되지 않아 프렐루드에 포함되지 않는다.
use std::collections::HashMap;

fn main() {
    // 해시맵을 생성하는 기본 제공 매크로가 없다.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
//    let score = scores
//        .get(&team_name) // team_name을 키로 사용하여 값의 레퍼런스를 불러온다.
//        .copied() // get()이 &Option<T>를 반환하기에 Option<T>로 복사한다.
//        .unwrap_or(0); // Some일 경우 값을 score에 저장하고 None일 경우 0을 저장한다.

    // 위와 동일.
    let score = match scores
        .get(&team_name)
        .copied() {
            Some(x) => x,
            None => 0,
    };
    println!("{score}");

    let ent = scores
        .entry(String::from("Red")) // enum Entry를 반환.
                                    // Entry는 OccupiedEntry, VacantEntry를 배리언트로 가짐.
        .or_insert(30);

    println!("{ent}");


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() { // split_whitespace는 text를 공백문자로 나눈 슬라이스에 대한 반복자를 반환.
        let count = map.entry(word).or_insert(0); // or_insert는 value의 &mut를 반환한다.
        *count += 1; // 이 레퍼런스는 곧 스코프를 벗어나고, 동시에 여러 &mut가 생기지 않는다.
    }

    println!("{:?}", map);
}
