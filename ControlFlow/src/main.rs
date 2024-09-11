fn main() {
    println!("Hello, world!");
    // In this program we gonna have the examples of Loop and If-else conditions and there working
    // In if and else statement the control flow works with true false condition only 
    // condition must always be of boolean datatype
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // type of loops 
    // 1. loop 

    let mut var = 0;
    loop {
        println!("hello");
        //this runs infinite iterations and needs a conditionn to stop 
        var = var + 1;
        if var == 4 {
            break ;
        }
    }


    // unique concept of loop 
    let result = loop{
        var += 1;
        
        if var == 5 {
           break var * 4;
        }
    };
    println!("{result}");
    // 2. While 
    // iss me ek limit described hoti hai pehle hi  

    while var<6 {
        println!("{var}");
        var += 1;
    }

    // 3. for 
    // in this there is limit in loop 
    let arr = [10, 20 ,30 ,40 ,50];

    for x in arr {
        println!("{x}");
    }
    // you can give the range for x in as 1..4 in which 1 to 3 except 4 will be count
    for x in 1..4{
         println!("{x}");
    }

    // naming of nested loop for using break and continue as if break and continue works only on innermost loop 
    // naming start with 'name: loop.....

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // to print the range in reverse order use .rev() 
    for number in (1..4).rev() {
        println!("{number}!");
    }
    
}
