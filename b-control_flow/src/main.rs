fn main() {
    greet();
    greet_person("Abhilash");
    // add function
    let result = add(10, 15);
    println!("The result of the addition is: {}", result);

    // division function
    let dividion_result = divide(10.0, 0.0);
    println!("The result of the division is: {}", dividion_result);
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

// +=
// -=