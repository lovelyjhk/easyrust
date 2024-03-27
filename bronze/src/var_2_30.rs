#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn var_2_30() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
