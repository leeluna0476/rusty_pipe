//#[derive(Debug)]
//enum UsState {
//    Alabama,
//    Alaska,
//}
//
//enum Coin {
//    Penny,
//    Nickel,
//    Dime,
//    Quarter(UsState),
//}

fn main() {
//    let coin: Coin = Coin::Quarter(UsState::Alaska);
//    let value: u8 = value_in_cents(coin);
//    println!("value: {value}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", none);
}

// if문은 조건문에 boolean 값만 포함할 수 있음
// match문은 어떤 값이든 포함할 수 있음
// match 갈래의 코드를 중괄호로 감싼다면 쉼표를 붙이지 않아도 됨
//fn value_in_cents(coin: Coin) -> u8 {
//    match coin {
//        Coin::Penny => 1,
//        Coin::Nickel => 5,
//        Coin::Dime => 10,
//        Coin::Quarter(state) => {
//            println!("State quarter from {:?}!", state);
//            25
//        },
//    }
//}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
