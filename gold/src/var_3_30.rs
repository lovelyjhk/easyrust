/*  
14-1. 클로저: 익명 함수

*/
pub fn var_3_30() {
    let add = |x, y| x + y; // 클로저 정의
    let result = add(3, 5); // 클로저 호출
    println!("Result: {}", result); // 출력: Result: 8
}
