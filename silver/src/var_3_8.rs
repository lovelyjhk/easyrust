mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // 대기 목록에 추가하는 로직
        }
    }
}

pub use crate::var_3_5::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
