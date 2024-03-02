use std::collections::HashMap;

pub fn var_3_18() {
    // 해시맵 생성
    let mut map: HashMap<&str, i32> = HashMap::new();
    
    // 값 추가
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    // 값 가져오기
    match map.get("two") {
        Some(&n) => println!("Value of 'two': {}", n),
        None => println!("Key 'two' not found"),
    }
}
