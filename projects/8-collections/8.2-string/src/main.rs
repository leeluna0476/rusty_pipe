// 러스트 언어에서 제공하는 문자열 타입은 str뿐이다.
// str: 문자열 슬라이스 (UTF-8로 인코딩)
// 문자열 슬라이스는 어딘가에 저장된 문자열 데이터의 참조자
// 그러므로 "string"과 같은 문자열 리터럴도 문자열 슬라이스
fn main() {
    // String은 러스트 표준 라이브러리가 제공한다.
    // String은 Vec<u8>에 여러 기능을 추가한 wrapper로 구현되어 있다.
    // 그러므로 Vec<u8>에서 사용할 수 있는 기능들을 String에서도 전부 사용할 수 있다.
//    let mut s = String::new();
//
//    // String의 고유 기능
//    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    s1.push_str("bar");

    // UTF-8 인코딩...
    let s2 = String::from("السلام عليكم");
    println!("{s2}");

    for c in s2.chars() {
        print!("{c} ");
    }
    println!();

    let s3 = s2 + " hello"; // + 연산자를 사용하면 add 메서드가 호출된다.
                            // 이는 s2의 호출을 가져가서 두번째 피연산자를 더하고
                            // 다시 반환한다.
                            // s2의 소유권은 이제 s3로 이동했기 때문에 s2를 더 이상 사용할 수 없다.

    // + 연산자로 여러 문자열을 나열하는 것을
    // 아래처럼 좀 더 편리하게 포맷팅을 할 수 있다.
    let s4 = format!("{s3}-{}", "hello world");
    println!("{s4}");

    println!("{}", &s4[0..2]); // 문자의 일부를 슬라이싱하면 패닉이 일어난다.
}

// 러스트 문자열을 해석하는 세가지 방법:
// 1. byte: 컴퓨터가 데이터를 궁극적으로 저장하는 방법
// 2. char: 유니코드 스칼라 값
// 3. grapheme: 문자소(의미를 구별할 수 있는 글자의 최소 단위) 클러스터
// 유니코드는 1~4바이트 크기의 가변 길이 인코딩 방식이다.
// String의 바이트 배열 크기와 char 개수 * char 크기가 일치하지 않을 수 있다.
