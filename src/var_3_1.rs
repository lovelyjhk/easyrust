pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


pub fn var_3_1() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
