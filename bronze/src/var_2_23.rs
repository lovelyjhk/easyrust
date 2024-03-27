pub fn var_2_23() {
    let s = String::from("hello");

    // 문자열 s를 참조로 넘깁니다.
    change(&s);

    // 여기서도 s를 사용할 수 있습니다.
    println!("s: {}", s);
}

// 'a는 'b보다 더 오래 살아야 합니다.
fn change<'a>(some_string: &'a String) {
    // 오류! some_string은 변경할 수 없는 참조입니다.
    // some_string.push_str(", world");

    // 수정된 함수 시그니처에서 some_string은 변경할 수 없는 참조입니다.
    // 새로운 String을 생성하고 반환합니다.
    let _ = String::from("modified");
}
