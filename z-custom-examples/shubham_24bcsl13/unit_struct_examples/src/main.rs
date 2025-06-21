//! # Simple Unit Struct Examples
//! This file shows how to use unit structs in Rust.
//!
//! Unit structs are structs without any fields. They are often used as simple tags,
//! markers, or to group related functions.
//! 
//! Written by Shubham Vishwakarma, 24BCSL13


/// A struct with no fields — just a tag or a marker.
struct Welcome;

/// Another empty struct to represent a state.
struct Start;

/// A struct that can be used to print messages.
struct Printer;

/// A struct that does a simple math operation.
struct Math;

/// Entry point of the program.
fn main() {
    println!("== Simple Unit Struct Examples ==\n");

    // Example 1: Creating a unit struct and calling a function
    print_welcome(Welcome);

    // Example 2: Representing a system state
    print_state(Start);

    // Example 3: Struct calling its own function
    Printer::print_message();

    // Example 4: Struct doing a math task
    Math::add(3, 4);
}

/// This function uses a unit struct to print a welcome message.
fn print_welcome(_w: Welcome) {
    println!("Welcome to Rust programming!");
}

/// This function prints what state the program is in.
fn print_state(_s: Start) {
    println!("State: The program is starting up...");
}

impl Printer {
    /// A simple print function inside a struct.
    fn print_message() {
        println!("Printer: This is a message from a unit struct.");
    }
}

impl Math {
    /// Adds two numbers and prints the result.
    fn add(a: i32, b: i32) {
        let result = a + b;
        println!("Math: {} + {} = {}", a, b, result);
    }
}
