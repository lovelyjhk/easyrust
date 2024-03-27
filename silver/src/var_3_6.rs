mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub mod var_3_6 {
    pub fn eat_at_restaurant() {
        super::front_of_house::hosting::add_to_waitlist(); // super::hosting을 사용하여 부모 모듈로부터의 단축 경로를 참조합니다.
    }
}
