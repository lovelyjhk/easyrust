struct Rectangle { width: u32, height: u32, } 

pub fn var_2_28() { 
	let rect1 = Rectangle { width: 30, height: 50, }; 
	println!( "The area of the rectangle is {} square pixels.", area(&rect1) ); 

}  //함수 호출 이후에 더 이상 rect1 을 사용할 수 없으므로 불변 참조자로 해서 계속 사용하도록 

fn area(rectangle: &Rectangle) -> u32 { rectangle.width *rectangle.height } //가독성이 좋음
