// 컴파일러가 모듈의 내용을 찾는 경로:
// 1. 모듈 옆 중괄호 안
// 2. src/lib/front_of_house.rs
// 3. src/lib/front_of_house/mod.rs

mod front_of_house;

mod back_of_house;

use crate::front_of_house::hosting;

fn deliver_order() {}

mod customer {
    use super::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
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
