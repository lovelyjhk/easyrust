use std::fmt::Debug;

// 제네릭 함수 (Generic Functions)
fn print_vector<T>(vector: &[T])
where
    T: Debug,
{
    for item in vector {
        println!("{:?}", item);
    }
}

// 제네릭 구조체 (Generic Structs)
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// 제네릭 메서드 (Generic Methods)
struct MyVector<T> {
    items: Vec<T>,
}

impl<T> MyVector<T> {
    fn print_items(&self)
    where
        T: Debug,
    {
        for item in &self.items {
            println!("{:?}", item);
        }
    }
}

pub fn var_3_14() {
    // 제네릭 함수 호출
    let numbers = vec![1, 2, 3, 4, 5];
    let names = vec!["Alice", "Bob", "Charlie"];
    print_vector(&numbers);
    print_vector(&names);

    // 제네릭 구조체 사용
    let mut stack = Stack { items: vec![] };
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(item) = stack.pop() {
        println!("{}", item);
    }

    // 제네릭 메서드 사용
    let numbers = MyVector { items: vec![1, 2, 3, 4, 5] };
    let names = MyVector { items: vec!["Alice", "Bob", "Charlie"] };
    numbers.print_items();
    names.print_items();
}
