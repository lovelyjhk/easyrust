pub fn var_2_2() {
    let x: f64 = 10.0; // 숫자 타입을 명시

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결과값은 -1입니다
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Subtraction: {}", difference);
    println!("Division: {}", quotient);
    println!("Truncated: {}", truncated);
    println!("Remainder: {}", remainder);

    let abs_x = x.abs();
    let sin_x = x.sin();
    let cos_x = x.cos();
    let tan_x = x.tan();
    let asin_x = x.asin();
    let acos_x = x.acos();
    let atan_x = x.atan();
    let sqrt_x = x.sqrt();
    let cbrt_x = x.cbrt();
    let exp_x = x.exp();
    let ln_x = x.ln();
    let log10_x = x.log10();
    let pow_x_n = x.powi(2);

    println!("절댓값: {}", abs_x);
    println!("사인 값: {}", sin_x);
    println!("코사인 값: {}", cos_x);
    println!("탄젠트 값: {}", tan_x);
    println!("아크 사인 값: {}", asin_x);
    println!("아크 코사인 값: {}", acos_x);
    println!("아크 탄젠트 값: {}", atan_x);
    println!("제곱근 값: {}", sqrt_x);
    println!("세제곱근 값: {}", cbrt_x);
    println!("자연로그 값: {}", exp_x);
    println!("상용로그 값: {}", ln_x);
    println!("10을 밑으로 하는 로그 값: {}", log10_x);
    println!("2 제곱 값: {}", pow_x_n);
}
