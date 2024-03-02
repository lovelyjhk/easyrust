pub fn var_3_9() { 
    panic!("crash and burn");  // 패닉 발생 
    let v = vec![1, 2, 3]; v[99]; // index 밖을 벗어나서 접근할 떄 패닉 발생 
}
