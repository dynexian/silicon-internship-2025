use std::vec;

fn main() {
    //    initilise_manish();

    // Example of a named-field struct
    // named_field_structs_example();

    // Example of a tuple struct
    // tuple_structs_example();

    // Example of a unit-like struct
    // unit_like_structs_example();
    // check_clone();

    // impl_examples();
    calculator_example();
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

    let emp2 = Employee { emp_id, name, dept }; // Short hand notation

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

// impl Block in Struct

// let vector = vec![1, 2, 3, 4, 5];
// vector.push(6);

// push(vector, 6);

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    phone_number: String,
}

impl Student {
    // associated function
    fn new(name: String, age: u8, phone_number: String) -> Student {
        Student {
            name,
            age,
            phone_number,
        }
    }

    // methods
    fn display_name(&self) {
        println!("Student Name: {}", self.name);
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_age(&self, age: u8) -> Self {
        Student {
            name: self.name.clone(), // keep the same name
            age,
            phone_number: self.phone_number.clone(), // keep the same phone number
        }
    }

    fn set_phone_number(self, ph_num: String) -> Self {
        Student {
            phone_number: ph_num,
            ..self // instance of the type s1: Student
        }
    }

    // self - instance of the type
    // Self - type itself
    // fn display_name_associated(name: String) {
    //     println!("Associated Function - Student Name: {}", name);
    // }
}

fn impl_examples() {
    // let s1 = Student {
    //     name: String::from("Manish"),
    //     age: 21,
    //     phone_number: String::from("123456789"),
    // };

    let mut s1 = Student::new(
        String::from("Manish"), 
        21, 
        String::from("123456789")
    );

    s1.display_name();
    // Student::display_name_associated(String::from("Manish"));

    // s1 = self
    // Student = Self;

    s1.set_name("John Doe".to_string());
    println!("Updated Student Name: {}", s1.name);

    let s2 = s1.set_phone_number(String::from("1234"));
    println!("Updated Student: {:?}", s2);

}

// fn check_clone() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();

//     println!("s1: {}, s2: {}", s1, s2);
// }


#[derive(Debug)]
struct Calculator {
    current_value: f64,
    history: Vec<f64>
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            current_value: 0.0,
            history: vec![],
        }
    }

    fn with_initial_value(initial_value: f64) -> Self {
        Calculator {
            current_value: initial_value,
            history: vec![initial_value],
        }
    }

    fn add(&mut self, a: f64, b: f64) -> f64 {
        let res = a + b;
        self.current_value = res;
        self.history.push(self.current_value);
        res
    }

    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("Cannot divide by zero");
        }
        a / b
    }

    fn get_current_value(&self) -> f64 {
        self.current_value
    }

    fn get_history(&self) -> &Vec<f64> {
        &self.history
    }

    fn clear(&mut self) {
        self.current_value = 0.0;
        self.history = vec![];
    }
}

fn calculator_example() {
    let mut calculator_one = Calculator::new();
    let mut calculator_two = Calculator::with_initial_value(53.67);
    println!("Initial Values 1: {:?}", calculator_one);
    println!("Initial Values 2: {:?}", calculator_two);

    let sum = calculator_one.add(3.0, 4.0);
    println!("Values 1: {:?}", calculator_one);
    println!("Sum: {}", sum);

    let sum = calculator_one.add(13.0, 14.0);
    println!("Values Again: {:?}", calculator_one);
    println!("Sum: {}", sum);

    calculator_one.clear();
    println!("Values After Clear: {:?}", calculator_one);
}

// self
// Self
// methods
// associated functions