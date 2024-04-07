
fn main() {
    let my_array = [1, 2, 3, 4, 5];
    let index = 10; // 인덱스가 범위를 벗어난 경우

    // 배열 길이를 확인하여 인덱스가 유효한지 검사하고 패닉 발생
    if index >= my_array.len() {
        panic!("Index out of bounds!");
    }

    // 인덱스가 유효한 경우 배열에 접근
    let element = my_array[index];
    println!("Element at index {}: {}", index, element);
}
