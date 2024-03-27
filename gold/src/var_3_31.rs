
/*
 14-1. 클로저: 특정한 유형 (Fn, FnMut, FnOnce)
*/
pub fn var_3_31() {
    let x = 5;
    
    // Fn: 불변 참조를 통해 x를 사용하는 클로저
    let closure_fn = || println!("x is: {}", x);
    closure_fn();

    // FnMut: 가변 참조를 통해 x를 사용하는 클로저
    let mut y = 10;
    let mut closure_fn_mut = || {
        y += 1;
        println!("y is: {}", y);
    };
    closure_fn_mut();

    // FnOnce: 소유권을 통해 x를 사용하는 클로저
    let closure_fn_once = || println!("x is: {}", x);
    closure_fn_once();
}
