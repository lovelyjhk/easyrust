/*
cargo.toml 에 종속성 추가 
[dependencies]
rand = "0.8.5"
*/

use rand::Rng;

pub fn var_3_7() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
