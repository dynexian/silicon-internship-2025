use std::alloc;

fn main() {
    // greet();
    // greet_person("Abhilash");
    // // add function
    // let result = add(10, 15);
    // println!("The result of the addition is: {}", result);

    // // division function
    // let dividion_result = divide(10.0, 0.0);
    // println!("The result of the division is: {}", dividion_result);

    // custom scope example
    // custom_scope2();
    // if_else_expression();
    // match_example();
    // loop_example();
    // while_example();
    for_example();
}

// simple function without any imput or output
fn greet() {
    println!("Hello, siliconites!");
}


// Function with paramters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}


// Function with input and output
fn add(a: i32, b: i32) -> i32 {
    // let result = a + b;
    // println!("The sum of {} and {} is {}", a, b, result);
    // result

    a + b
}

// Early exit using return 
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Zero division error");
        return 0.0; // early return
    }
    a/b // normal return 
}

// scope 
// scope determines where the variable is visible and accessible


fn scope_example() {
    let x = 10;

    println!(" In this function value of  x is {}", x);
    // println!("In this function value of y is {}", y);
}

fn other_scope_example() {
    let y = 10;

    println!("In this function value of y is {}", y);
    // println!("In this function value of x is {}", x); 
}

fn custom_scope(){
    let a = 5;

    {
        let b = 10;
        {
            let c = 15;
            println!("In this inner scope, the value of a is {}, b is {}, and c is {}", a, b, c);
        }

        println!("In this inner scope, the value of a is {}, and b is {}", a, b);
        // println!("In this inner scope, the value of c is {}", c);
    }

    println!("In this function the value of a is {}", a);
    // println!("in this function scope the value of b is {}", b)

}


fn custom_scope2() {
    let x = 10;

    {
        let x = 20; // shadowed
        println!("In this inner scope, the value of x is {}", x);
    }

    {
        let y = 30;
        println!("In this inner scope, the value of x is {}, and y is {}", x, y);
    }

    println!("In this function scope, the value of x is {}", x);
}


fn if_example() {
     let number = 5;


     if number > 0 {
        println!("The number is positive.");
     } else {
        println!("The number is negative.");
     }


     if number > 0 {
        println!("The number is positive")
     } else if number < 0 {
        println!("The number is negative");
     } else {
        println!("The number is zero");
     }
}

fn if_else_expression() {
    let condition = true ;
    let number = if condition  {5} else {6};
    println!("The value of number is: {}", number);


    let age = 25;
    let age_group = if age >= 18 {"Adult"}  else {"Minor"};
    println!("The age group is: {}", age_group);
    let has_license = true;

    if age >=18 && has_license{
        println!("You are eligible to drive.");
    } else if age >= 18 && !has_license {
        println!("You are an adult but not eligible to drive but you can get a license.");
    } else {
        println!("You are not eligible to drive.");
    }
}

fn match_example() {
    let number = 10;

    match number {
        1  => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Wildcard in Action!"),
    }

    match number {
         1 | 2 | 3 => println!("One, Two or Three"),
         4 | 5 | 6 => println!("Four, Five or Six"),
         7 | 8 | 9 => println!("Seven, Eight or Nine"),
            _ => println!("Wildcard in Action!"),
    }

    match number {
        1..10 => println!("One to ten"),
        11..=50 => println!("Eleven to fifty"),
        51..=100 => println!("Fifty one to one hundred"),
           _ => println!("Wildcard in Action!"),
   }
}

fn loop_example() {
    //loop
    //while
    //for

    let mut counter = 0;

    loop {
        counter += 1;

        if  counter == 10 {
            println!("Counter reached 10, exiting loop.");
            break; 
        }

        println!("counter {}", counter );
    }

}   

fn while_example() {
    let mut counter = 0;

    while counter <= 10 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}

fn for_example() {
    // iterate over a range
    // iteratte over a collection

    // for i in 0..10{
    //     println!("Current number: {}", i);
    // }

    // println!("______________________________________");


    // for i in 0..=10 {
    //     println!("Current number: {}", i);
    // }

    // // 0  1 2 3 4 5 6 7 8 9 10

    // println!("______________________________________");

    // for i in (0..=10).rev() {
    //     println!("Current number in reverse: {}", i);
    // }

    // println!("______________________________________");

    // for i in (0..=10).rev().step_by(2) {
    //     println!("Current number {}", i)
    // }


    for i in 1..=15{
        if i % 2 == 0 {
            continue;
        }

        println!("Current number: {}", i);
        // break
        // continue
        // return
    }
}



