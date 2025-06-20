fn main() {
//    initilise_manish();

    // Example of a named-field struct
    // named_field_structs_example();

    // Example of a tuple struct
    // tuple_structs_example();

    // Example of a unit-like struct
    unit_like_structs_example();
}


// Structs let you group together related data types into one single type.

// Example: 
// Student -> name, age, grades, phone_number, roll_number

// {
//     let manish_name = "Manish";
//     let manish_age = 21;
//     let manish_grades = [40, 35, 24];
//     let manish_phone_number = "123456789";
//     let manish_roll_number = 1;
// }

// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u8,
//     grades: [u8; 3],
//     phone_number: String,
//     roll_number: u32,
// } // field of a struct

// fn initilise_manish() {
//     let manish = Student {
//         name: String::from("Manish"),
//         age: 21,
//         grades: [40, 35, 24],
//         phone_number: String::from("123456789"),
//         roll_number: 1,
//     };

//     println!("Manish: {:?}", manish);

//     let name = String::from("Manish");
//     let age = 21;
//     let grades = [40, 35, 24];
//     let phone_number = String::from("123456789");
//     let roll_number = 1;

//     let manish = Student {
//         name,
//         age,
//         grades,
//         phone_number,
//         roll_number,
//     }; // Short hand notation
// }

// Types Of Structs:
// 1. Named-field Structs
#[derive(Debug)]
struct Employee {
    emp_id: u32,
    name: String,
    dept: String,
}

fn named_field_structs_example() {
    let emp1 = Employee {
        emp_id: 101,
        name: String::from("Alice"),
        dept: String::from("Engineering"),
    };

    println!("EMPLOYEE 1 = {:?}", emp1);

    let emp_id = 102;
    let name = String::from("Bob");
    let dept = String::from("Marketing");

    let emp2 = Employee {
        emp_id,
        name,
        dept,
    }; // Short hand notation

    println!("EMPLOYEE 2 = {:?}", emp2);

    // Accessing fields
    println!("Employee 1 ID: {}", emp1.emp_id);
    println!("Employee 2 Name: {}", emp2.name);
    println!("Employee 2 Department: {}", emp2.dept);
}

// 2. Tuple Structs
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(i32, i32);

fn tuple_structs_example() {
    let white = Color(255, 255, 255);
    let black = Color(0, 0, 0);

    println!("White: {:?}", white);
    println!("Black: {:?}", black);

    let origin = Point(0, 0);
    println!("Origin: {:?}", origin); 

    println!("Red component of White: {}", white.0);
    println!("Green component of White: {}", white.1);
    println!("Blue component of White: {}", white.2);
}

// 3. Unit-like Structs
#[derive(Debug)]
struct Meter;
#[derive(Debug)]
struct Uranium;

fn unit_like_structs_example() {
    let meter = Meter;
    let uranium = Uranium;

    println!("Unit-like structs created: Meter and Uranium");
    println!("Meter: {:?}, Uranium: {:?}", meter, uranium);

    // Homework - Usecase for the Unit Structs
}