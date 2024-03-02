
//단 하나의 가변 참조자만 갖거나, 여러 개의 불변 참조자를 가질 수 있습니다.
//참조자는 항상 유효해야 합니다.

pub fn var_2_22(){
    let mut s = String::from("hello"); 
    { let r1 = &mut s; } // 여기서 r1이 스코프 밖으로 벗어나며, 따라서 아무 문제없이 새 참조자를 만들 수 있습니다. 
    let r2 = &mut s;

    //println!("{}, {}", r1, r2); //r1에 문제가 있어서 사용하면 안되요.

    //어떤 값에 불변 참조자가 있는데 같은 값의 가변 참조자를 또 만들면 패닉!
    let mut s = String::from("hello"); 
    let r1 = &s; // 문제없음 
    let r2 = &s; // 문제없음 
    //let r3 = &mut s; // 패닉

    println!("{}, {}", r1, r2);
    //println!("{}, {}, and {}", r1, r2, r3);

}

