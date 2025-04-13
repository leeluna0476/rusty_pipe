#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// impl 블럭은 여러 개로 나누어 쓸 수 있음.
impl Rectangle { // 연관함수 정의.
                 // self를 매개변수로 갖는 함수는 메서드.
    fn area(&self) -> u32 { // &self는 self: &Self를 줄인 것.
        self.width * self.height
    }

    fn can_hold(&self, another_rect: &Rectangle) -> bool {
        self.width > another_rect.width && self.height > another_rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 20
    };

    let r2 = Rectangle {
        width: 5,
        height: 10
    };

    println!("The area of r1 is {} square pixels", r1.area());
    println!("r1 can hold r2: {}", r1.can_hold(&r2));

    dbg!(r1); // 소유권 이동되지 않음. r1 유효.
    dbg!(r2); // 소유권 이동되지 않음. r2 유효.

    let square1 = Rectangle::square(10);
    dbg!(square1);
}
