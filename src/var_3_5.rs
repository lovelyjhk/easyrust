pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub mod var_3_5 {
    use super::front_of_house::hosting; // use 문을 customer 모듈 내부로 옮깁니다.

    pub fn var_3_5() {
        hosting::add_to_waitlist();
    }
}
