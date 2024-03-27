pub fn var_2_18() {
    // 벡터 생성과 값 초기화
    let mut dynamic_array = Vec::new();
    for i in 0..6 {
        dynamic_array.push(i);
    }
    
    // 벡터의 길이에 맞지 않는 인덱스에 접근하면 런타임 에러가 발생
     panic!("index out of bounds: the len is 6 but the index is 6");
    
    // 벡터의 길이에 맞는 인덱스에 접근
    //println!("{}", dynamic_array[8]); //패닉 발생 

}
    
/*
주의사항 : cargo.toml 을 사용해서 panic 을 만나면 , 자동종로되게끔 합니다!
# panic 을 만나면 바로 프로그램 종료 
# panic = 'abort' 설정은 패닉이 발생하면 강제로 종료되므로, 
# 프로그램이 코어 덤프나 스택 트레이스 등의 추가 정보를 생성하지 않습니다. 
#이는 디버깅을 위한 정보를 최소화하여 프로그램의 크기를 줄이고 실행 속도를 높이는 데 도움이 됩니다.

[profile.release] 
panic = 'abort'

*/