/*
 14-1. 클로저: 익명 함수
 */
pub fn var_3_29() {
    // 두 개의 파라미터를 받아서 합을 반환하는 클로저
    let add = |x, y| {
        let result = x + y;
        result // 명시적으로 반환할 수 있음
    };

    let a = 5;
    let b = 10;
    let sum = add(a, b);
    println!("Sum: {}", sum); // 출력: Sum: 15
}
