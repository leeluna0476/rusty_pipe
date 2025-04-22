// 모듈은 구조체, 열거형, 상수, 트레이트, 함수, 그리고 다른 모듈을 가질 수 있다.
// 모듈은 기본적으로 부모 모듈에 대해서 비공개다.
// 자식 모듈은 부모 모듈을 볼 수 있다.
// 모듈의 내용도 마찬가지로, 기본적으로 비공개다.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//mod front_of_house;

// 컴파일러가 모듈의 내용을 찾는 경로:
// 1. 모듈 옆 중괄호 안
// 2. src/lib/front_of_house.rs
// 3. src/lib/front_of_house/mod.rs

mod back_of_house {
    // 구조체의 필드도 기본적으로 비공개다.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // super: 자기 부모로부터 시작하는 상대 경로
    // 여기서는 루트
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// eat_at_restaurant와 front_of_house는 같은 모듈 안에 있으므로 서로 형제 관계이다.
// 그러므로 eat_at_restaurant는 front_of_house에 접근할 수 있다.
pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
//    meal.seasonal_fruit = String::from("peaches"); // error! private field
}

// 위의 구조:
// crate
//  ├── mod front_of_house
//  │   ├── pub mod hosting
//  │   │   ├── pub fn add_to_waitlist
//  │   │   └── fn seat_at_table
//  │   └── mod serving
//  │       ├── fn take_order
//  │       ├── fn serve_order
//  │       └── fn take_payment
//  └── mod back_of_house
//      ├── pub struct Breakfast
//      │   ├── pub toast
//      │   ├── seasonal_fruit
//      │   └── pub fn summer
//      ├── fn fix_incorrect_order
//      └── fn cook_order
//
