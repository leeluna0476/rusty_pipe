struct Color(u8, u8, u8); // tuple struct: 튜플형 타입을 선언하고 싶지만... 요소에 이름을 붙이고 싶지 않을 때 쓴다.

fn main() {
    let cyan = Color(0, 255, 255);
    let magenta = Color(255, 0, 255);
    let yellow = Color(255, 255, 0);
    let black = Color(0, 0, 0);

    let Color(r, g, b) = cyan;

    println!("{r}, {g}, {b}");
}
