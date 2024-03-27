
//참조와 대여 (가변/불변)
pub fn var_2_21() {
    let mut value = 5; // 가변 변수 선언

    // 가변 대여
    let ref_mut = &mut value; // 가변 참조자 생성
    *ref_mut += 1; // 참조자를 통해 값을 변경
    println!("Value after mutable borrowing: {}", value); // 원본 값 출력

    // 불변 대여
    let ref_imm = &value; // 불변 참조자 생성
    // *ref_imm += 1; // 에러! 불변 참조자를 통해 값을 변경할 수 없음
    println!("Value after immutable borrowing: {}", value); // 원본 값 출력
}

pub fn var_2_21_panic() {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String) {
    // some_string.push_str(", world"); // 컴파일 에러! 불변 참조자로부터 값을 변경할 수 없음
}
