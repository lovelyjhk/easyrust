mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn var_3_4() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
