struct User {
    active: bool,
    username: String, //&str 을 사용하면 안됨.
    email: String,//&str 을 사용하면 안됨.
    sign_in_count: Option<u64>,
}

pub fn var_2_25() {
    // build_user 함수를 호출하여 User 구조체를 생성하고 반환합니다.
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    //기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기
    let user2 = User { 
        active: user1.active, 
        username: user1.username, 
        email: String::from("another@example.com"), 
        sign_in_count: user1.sign_in_count, 
    }; 

}

// User 구조체를 반환하는 build_user 함수를 정의합니다.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: Some(1), // sign_in_count를 Some(1)로 설정하여 초기화합니다.
    }
}
