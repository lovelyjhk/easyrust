use std::io;

//계산기 프로그램을 만들어보겠습니다.
pub fn var_2_9() {
    println!("첫번째 숫자를 입력하세요!: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("첫번째 값을 읽는데 실패!");

    let num1: f64 = input1.trim().parse().expect("유효하지 않은 입력!");

    println!("두번째 숫자를 입력하세요!: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("두번째 값을 읽는데 실패!");

    let num2: f64 = input2.trim().parse().expect("유효하지 않은 입력!");

    let result = num1 + num2;

    println!("결과는…다음과 같습니다! {} + {} = {}", num1, num2, result);
}
