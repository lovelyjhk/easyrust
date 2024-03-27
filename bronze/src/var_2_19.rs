pub fn var_2_19(){
    
    //소유권개념 - 얕은복사 
    let s1 = String::from("hello"); 
    let s2 = s1.clone(); //let s2 = s1; 
    println!("{}, world!", s1);
}