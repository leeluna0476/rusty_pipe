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

    // 열거형을 공개하면 그 배리언트도 전부 공개된다.
    pub enum Appetizer {
        Soup,
        Salad,
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

// hosting 모듈을 이 스코프 안으로 가져오기
// 이 단축 경로는 이 스코프 안에서만 유효하다
use crate::front_of_house::hosting;
// use에 pub를 붙이면 외부에서 hosting에 crate::hosting으로 접근할 수 있게 된다.
//pub use crate::front_of_house::hosting;

fn deliver_order() {}

// eat_at_restaurant와 front_of_house는 같은 모듈 안에 있으므로 서로 형제 관계이다.
// 그러므로 eat_at_restaurant는 front_of_house에 접근할 수 있다.
mod customer {
    // 위에서 가져온 모듈은 이 스코프에서 유효하지 않다. 다시 가져와야 함.
    // use crate::hosting과 동일.
    use super::hosting;
    pub fn eat_at_restaurant() {
//        let mut meal = back_of_house::Breakfast::summer("Rye");
//        meal.toast = String::from("Wheat");
//        println!("I'd like {} toast please", meal.toast);
////        meal.seasonal_fruit = String::from("blueberries"); // error! private field
        hosting::add_to_waitlist();
        // 어느 모듈의 함수를 불러올 때는 use로 해당 모듈 이름까지만 불러오는 것이 일반적이다.
        // 함수가 다른 모듈에 정의되어 있음을 분명하게 나타내기 위함이다.
        // 구조체나 열거형 등의 아이템을 불러올 때는 전체 경로를 불러오는 것이 일반적이다.
        // 그냥 관용이다.
        // ::를 두 번 쓰기 싫어서 그런가 보다. ㅋㅋㅋ
        // 그러나 Result처럼 서로 다른 모듈에 동일한 이름의 아이템이 있을 경우 모듈까지 불러와서 아이템을 구분해야 한다.
    }
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
