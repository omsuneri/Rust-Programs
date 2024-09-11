fn main() {
    println!("Variables and Mutability");

    // const are the variables which cant be changed throughout the program used with const keyword
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Variables is defined by let keyword "mut" is used to define its mutalbility
    let mut x  = 10;
    // Cause mutable -->
    x = 8;
    let y = 20;
    
    // let keyword use karke we can change the value of immutable var 
    // but wo value valid hogi sirf pehli wali change hojayegi
    let y = 40;

    println!("The value of x is {}", x);

    // also we can write {x} to get varible in println
    println!("The value of y is {y}");

    // Shadowing : to define a scope in which you change the immutable variable value 
    // also you can change the data type of the var in the new scope using let keyword
    {
        let y = 30;
        println!("The value of y is {y}");
    }

    println!("The value of y is {y}");

}
