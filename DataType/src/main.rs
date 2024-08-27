fn main() {
    println!("Hello, world!");
    // Datatype in Rust are 1. Scalar 2. Compound 
    // The rust compiler infer some primitive datatype like int, float, string, char...
    let integer = 34;
    let float = 3.4;
    let bool = true;
    let char = 'o';
    let heart_eyed_cat = '😻'; // char me you can have emojis too
    println!("the value of integer is {integer}");
    println!("the value of integer is {float}");
    println!("the value of integer is {bool}");
    println!("the value of integer is {char}");
    println!("the value of integer is {heart_eyed_cat}");

    // Compound datatype are tuple and array
    // Tuple
    // In tuple you can store value of different datatype
    // tuple is not dynamic 
    // structuring of the tuple 
    let tuple = (1, 2.4, 'c', true, '😻' );
    // Destructuring of tuple
    let (v, w, x, y, z) = tuple;
    // obtaining the value of elements from a tuple 
    println!("the 1st element is {}", tuple.0);
    println!("the 2nd element is {}", tuple.1);
    println!("the 3rd element is {}", tuple.2);
    println!("the 4th element is {}", tuple.3);
    println!("the 5th element is {}", z);

    // Array
    // in array you can only store the elements of same datatype
    let array = [1, 2, 3, 4, 5];
    let array1 = [3; 5];
    // accessing the elements of array
    println!("the 1st element is {}", array[0]);
    println!("the 2nd element is {}", array[1]);
    println!("the 3rd element is {}", array[2]);
    println!("the 4th element is {}", array[3]);
    println!("the 5th element is {}", array[4]);

}
