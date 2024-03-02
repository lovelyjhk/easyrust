//슬라이스 배우기 

pub fn var_2_24() {
    // 첫 번째 문자열 생성
    let s = String::from("hello world");

    // 첫 번째 문자열에서 부분 문자열 추출
    let hello = &s[0..5]; // "hello"
    let world = &s[6..11]; // "world"

    // 문자열과 부분 문자열 출력
    println!("s: {}", s);
    println!("hello: {}", hello);
    println!("world: {}", world);

    // 두 번째 문자열 생성
    let s = String::from("hello");

    // 두 번째 문자열에서 부분 문자열 추출 (slice)
    let slice = &s[0..2]; // "he"

    // 부분 문자열 출력
    println!("slice: {}", slice);

    // 세 번째 문자열 생성
    let s = String::from("hello");

    // 세 번째 문자열의 길이 확인
    let len = s.len(); // 문자열의 길이는 5

    // 문자열에서 부분 문자열 추출 (slice)
    let slice = &s[3..len]; // 문자열의 네 번째 문자부터 마지막 문자까지의 슬라이스

    // 길이와 슬라이스 출력
    println!("length of s: {}", len);
    println!("slice: {}", slice);
}
