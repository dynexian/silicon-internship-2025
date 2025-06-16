fn main() {
    // copy_example();
    // move_example()
    // demo_transfer();
    test_consume_return();
}

fn copy_example() {
    let number = 42; //define original number
    let number_copy = number; // defined number_copy, number is not moved to number_copy instead it is copied

    println!("Original Number {}", number);
    println!("Copied Number {}", number_copy);

    let flag = true;
    let flag_copy = flag; // value is copied not being moved

    // small data  or stackk data are copied instead of moved

    println!("Original Flag {}", flag);
    println!("Copied Flag {}", flag_copy);
}

fn move_example() {
    let text = String::from("Hello"); // text is feined
    let text_moved = text.clone();

    //. clone is never suggested to be used, untill you know the data length will
    // never be large

    println!("Original Text: {}", text); // value is already moved in line 28
    println!("Moved Text: {}", text_moved);

    let numbers = vec![1, 2, 3, 4, 5]; // vector is defined
    let numbers_moved = numbers; // vector is moved, not copied

    // println!("Original Numbers: {:?}", numbers);
    println!("Moved Numbers: {:?}", numbers_moved); // this will work because numbers has been moved to numbers_moved
}

fn demo_transfer() {
    let first = String::from("Hello"); // first is defined
    println!("First: {}", first); // prints "Hello"

    let second = first;
    println!("Second: {}", second); // prints "Hello"
    // println!("First: {}", first);

    let third = second;
    println!("Third: {}", third); // prints "Hello"
    // println!("Second: {}", second); // prints "Hello"
    // println!("First: {}", first);
}


fn test_consume_return(){
    // let data = String::from("hello");
    // consume_data(data);
    // println!("Trying to ptint data {}", data);

    let data = String::from("hello");
    let data = return_data(data); // shadowed
    println!("Original data is {}", data);
    // println!("Returned data is {}", returned_data);
}

fn consume_data(data: String) {
    println!("Only printing data {}", data);
}

fn return_data(data: String) -> String {
    println!("Returning data {}", data);
    // you can perform any action
    data // returning the data
}


// borrowing tomorrow