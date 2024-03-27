pub fn var_2_4() {
    // tuple 알아보기
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 배열 (array) 알아보기
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    let b = [1, 2, 3, 4, 5];
    let c = [3; 5]; // a = [3, 3, 3, 3, 3] 과 동일한 방식
    println!("{}", b[0]); // 결과는? 맞춰보기!
}
