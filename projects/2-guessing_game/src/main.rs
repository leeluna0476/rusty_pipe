use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range(start..=end) -> [start, end]

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut는 값이 변경될 수 있다는 뜻.
        io::stdin() // std::io::Stdin 반환
            .read_line(&mut guess) // & 참조자를 뜻함. 참조자는 기본적으로 불변. &mut는 가변 참조자를 나타냄.
                                   // read_line은 열거형 Result를 반환.
                                   // Result의 상태값(variant)는 Ok와 Err가 있다.
            .expect("Failed to read line."); // Result의 variant가 Err일 때 expect message 출력.
                                             // 프로그램 종료.

        // shadowing: 변수를 다른 타입으로 덮어씌우는 것.
        // trim: 문자열의 양끝 공백 및 개행 삭제.
        // parse: let guess: u32로 명시한 u32 타입으로 변환.
        // match ... { ... } Result Ok, Err에 대한 동작을 명시.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // parse가 Ok(42) 따위를 반환하면 42를 꺼내서 guess에 넣는다.
            Err(_) => continue, // _: 에러값 무시. 이름으로 꺼내 쓸 수 없다.
//            Err(e) => {
//                println!("{e}");
//                continue;
//            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
