use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    data: i32,
    next: Option<Weak<RefCell<Node>>>, // Weak<RefCell<T>>를 사용하여 순환 참조 방지
}

fn main() {
    // Rc<T>와 Weak<T>를 사용하여 순환 참조 방지하는 노드 생성
    let node1 = Rc::new(RefCell::new(Node {
        data: 1,
        next: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        data: 2,
        next: Some(Rc::downgrade(&node1)), // Rc<T>에서 Weak<T>로의 downgrade를 사용하여 순환 참조 방지
    }));

    // 노드 간의 연결 설정
    match &node1.borrow().next {
    Some(next) => {
        if let Some(next_strong_ref) = next.upgrade() {
            next_strong_ref.borrow_mut().next = Some(Rc::downgrade(&node2));
        }
    }
    None => {}
};


    // 순환 참조 방지로 메모리 누수 방지
}
