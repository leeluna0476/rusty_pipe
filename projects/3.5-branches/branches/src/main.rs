fn main() {
    let x = 5;
    let greater = is_greater_than_zero(x);
    println!("{x} is greater than zero: {greater}");

    let ten = count_till_ten(x);
    println!("{ten}");
}

fn is_greater_than_zero(x: u32) -> bool {
//    if x > 0 {
//        return true
//    }
//    false
    
    if x > 0 {
        return true;
    } else {
        return false
    }
}

// 테스트용 아무 함수...
fn count_till_ten(x: u32) -> u32 {
    let mut v = x;
    let ret;
    if v < 10 {
        ret = loop {
            v += 1;
            if v == 10 {
                break 42;
            }
        };
    } else {
        ret = loop {
            v -= 1;
            if v == 10 {
                break 24
            }
        };
    }

    ret
}

// 값을 반환하는 것 -> expression
// 값을 반환하지 않는 것 -> statement
//
// 스코프도 expression.
// 스코프 끝에 둔 expression의 반환값은 그 스코프의 반환값이 된다.
//
// expression; -> statement
//
// Q:
//  return, break는 왜 semicolon이 붙어도 값을 반환하는가?
