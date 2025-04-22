//enum IpAddrKind {
//    V4,
//    V6,
//}
//
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

//enum IpAddr {
//    V4(String),
//    V6(String),
//}

enum IpAddr { // IpAddr 열거형은 V4, V6라는 variant(변형)를 가지고 있다.
    V4(u8, u8, u8, u8), // 각 variant는 서로 다른 타입의 데이터를 저장할 수 있다.
    V6(String),
}

// 열거형은 다른 종류의 구조체들을 하나의 타입으로 묶는 것과 비슷하다.
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 구조체처럼 이름이 있는 필드를 정의할 때 중괄호를 사용한다.
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move { x, y } => {
                println!("x: {x}, y: {y}");
            },
            _ => println!("else"),
        }
    }
}

fn main() {
//    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;
//
//    route(IpAddrKind::V4);
//    route(IpAddrKind::V6);
//
//    let home = IpAddr {
//        kind: IpAddrKind::V4,
//        address: String::from("127.0.0.1"),
//    };
//
//    let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1"),
//    };

//    let home = IpAddr::V4(String::from("127.0.0.1"));
//
//    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let address: [IpAddr; 2] = [
        home,
        loopback,
    ];

    for i in 0..2 {
        match &address[i] {
            IpAddr::V4(a, b, c, d) => {
                println!("{a},{b}.{c}.{d}");
            },
            IpAddr::V6(s) => {
                println!("{s}");
            },
        }
    }

    let msg = Message::Move { x: 6, y: 3 };
    msg.call();

//    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // None과 NULL의 다른 점은...
    // None은 값의 부재를 나타내는 '경우'.
    // NULL은 특정 값을 부재라고 정의하고 사용하는 것이다.
    // 예: 빈 리스트
    //  NULL: struct List *list = NULL;
    //        printf("%s\n", list->s); // 역참조 오류!
    //  None: let list: Option<List> = None;
    //        match list {
    //            Some(list) => { // 값이 존재할 때만 접근
    //                println!("{}", list.s);
    //            },
    //            None => {
    //            },
    //        }
    //  컴파일 단계에서 안전한 접근을 강제한다.
    match y {
        Some(d) => {
            println!("{d}");
        },
        None => {
            println!("None");
        }
    }

    // i8과 Option<i8>은 서로 다른 타입... 더할 수 없다.
//    let sum = x + y;
}

//fn route(_ip_kind: IpAddrKind) {}
