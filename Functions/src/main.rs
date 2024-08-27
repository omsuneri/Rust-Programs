fn main() {
    println!("Hello, world!");
    // calling functions 
    prints();
    add(10, 90);
    let sub = subtract(100, 90);
    println!("Subtracted value is {}", sub);
    let sne = SnE(7, 8);
    println!("SnE value is {}", sub);
    // The End  !!
    
}

// Functions are the block code that can be reused as many time youy want 
// Simple fuction
fn prints(){
    println!("Hello Guyzzz !!!!");
}

// Function with parameter 
fn add(x: i32, y: i32) {
    println!("sum is {}", (x + y));
}

// Function with return value 
fn subtract(x: i32, y: i32) -> i32 {
    return (x-y);
}

// Statement : the line which do not return any value like assinging a var 
// Expression : the part of code which return a value 

fn SnE(x: i32, y: i32) -> i32 {
    let sum = {
        x+y 
    };
    // here the block of code that returns a value x+y is assinging the returned value to the sum var so here 
    // assining the sum is statement and the block of code is expression 
    return sum;
}