struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("seojilee"),
        email: String::from("seojilee@student.42seoul.kr"),
        sign_in_count: 1
    };

    user1.email = String::from("leeluna0476@daum.net"); // 가변성은 구조체 멤버 전체가 가진다.

    let user2 = build_user(String::from("leeluna0476"), String::from("leeluna0476@daum.net"));

    let user3 = User {
        email: String::from("soljeong@student.42seoul.kr"),
        ..user1 // username의 소유권이 user3로 이동.
                // active, sign_in_count는 복사됨.
    };

    println!("{}, {}, {}", user1.email, user1.active, user1.sign_in_count); // email의 소유권은 이동되지 않았다.
//    println!("{}", user1.username); // compile error!
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
