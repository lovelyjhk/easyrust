pub fn deliver_order() {}

pub mod var_3_2 {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    pub fn cook_order() {}
}
