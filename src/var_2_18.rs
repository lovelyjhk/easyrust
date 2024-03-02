pub fn var_2_18() {
    // 벡터 생성과 값 초기화
    let mut dynamic_array = Vec::new();
    for i in 0..6 {
        dynamic_array.push(i);
    }
    
    // 벡터의 길이에 맞지 않는 인덱스에 접근하면 런타임 에러가 발생
    // panic!("index out of bounds: the len is 6 but the index is 6");
    
    // 벡터의 길이에 맞는 인덱스에 접근
    //println!("{}", dynamic_array[8]); //패닉 발생 

}
    