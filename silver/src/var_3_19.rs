pub fn var_3_19() {
    // 문자열 리터럴 사용
    let s1 = "Hello, Rust!"; // &str 타입의 문자열 리터럴
    let s2: &'static str = "Hello, Rust!"; // 정적 수명을 가지는 문자열 리터럴

    // String 타입 사용
    let mut s = String::new(); // 빈 문자열 생성
    s.push_str("Hello, "); // 문자열 추가
    s.push_str("Rust!"); // 문자열 추가

    // from 메서드 사용
    let s3 = String::from("Hello, Rust!"); // 문자열 생성

    // 문자열 결합
    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let s3 = s1.clone() + &s2; // s1이 소유권을 잃지 않도록 복제하여 사용
    let s4 = format!("{} {}", s1, s2); // format! 매크로를 사용하여 결합

    // 인덱싱과 슬라이싱
    let s = String::from("hello");
    let slice = &s[1..3]; // 문자열 슬라이싱
    let ch = s.chars().nth(0); // 인덱싱을 통한 문자 접근

    // 불변 및 가변 참조 사용
    let immutable_ref = &s; // 불변 참조
    let mut s = String::from("hello"); // 가변성을 가지는 문자열
    let mutable_ref = &mut s; // 가변 참조

    // to_string 메서드 사용
    let s1 = "Hello, Rust!";
    let s2 = s1.to_string();

    // 벡터의 일부분에 대한 슬라이싱
    let v = vec![1, 2, 3, 4, 5];
    let slice = &v[1..3]; // 인덱스 1부터 2까지 (0-based 인덱스)
    println!("{:?}", slice); // 출력: [2, 3]

    // 문자열의 일부분에 대한 슬라이싱
    let slice = &s[1..3]; // 인덱스 1부터 2까지 (0-based 인덱스)
    println!("{}", slice); // 출력: "el"
}
