fn main() {
    // init_explicit();
    // init_repeated();
    // init_mutation();
    // accessing_elements();
    // get_length();
    // iterate_simple();
    // iterate_indices();
    // iterate_mutably();
    // array_slicing();
    // find_max();
    // init_tuples();
    // access_elements();
    // destructure_tuple();
    // let ret = return_tuple();
    // println!("Returned Tuple: {:?}", ret);
    // println!("First element of returned tuple: {}", ret.0);
    // println!("Second element of returned tuple: {}", ret.1);
    // match_tuple();
}

// ARRAY EXAMPLES
fn init_explicit() {
    let arr = [1, 2, 3, 4, 5];
    println!("ARRAY: {:?}", arr);
}

fn init_repeated() {
    let arr = [24; 20];
    println!("ARRAY: {:?}", arr);
}

fn init_mutation() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("ARRAY BEFORE: {:?}", arr);

    arr[2] = 10;

    println!("ARRAY AFTER: {:?}", arr);
}

fn accessing_elements() {
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);
    println!("Third element: {}", arr[2]);
    println!("Fourth element: {}", arr[3]);
    println!("Fifth element: {}", arr[4]);
    // println!("Error! {}", arr[200]); // This will cause an error at compile time.
}

fn get_length() {
    let arr = ['1', '2', '3', 'ñ', '😎'];
    let length = arr.len();
    println!("Length of the array: {}", length);
}

fn iterate_simple() {
    // let arr = ['1', '2', '3', 'ñ', '😎'];
    // for element in arr {
    //     println!("Element: {}", element);
    // }

    // println!("Array[2] = {}", arr[2]);

    // for element in &arr {
    //     println!("Element (by reference): {}", element);
    // }

    let string_array: [String; 3] = [String::from("Hello"), String::from("World"), String::from("Rust")];
    for element in string_array {
        println!("String Element: {}", element);
    }

    // println!("String Array[2] = {}", string_array[2]); // will throw error

    // for element in &string_array {
    //     println!("String Element: {}", element);
    // }
}

fn iterate_indices() {
    let arr = ['1', '2', '3', 'ñ', '😎'];
    for i in 0..arr.len() {
        println!("Element at index {}: {}", i, arr[i]);
    }
}

fn iterate_mutably() {
    let mut arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        arr[i] = arr[i] * arr[i];
    }

    println!("Mutated Array: {:?}", arr);
}

fn array_slicing() {
    let arr = [1, 2, 3, 4, 5];
    let slc = &arr[1..4]; // Type -> [i32] -> &[i32]
    println!("Sliced Array: {:?}", slc);
}

fn find_max() {
    let arr = [1, 2, 3, 4, 5];
    let mut max = 0;
    for num in arr {
        if num > max {
            max = num;
        }
    }
    println!("Max value in the array: {}", max);
}

fn init_tuples() {
    let tup = (1, 2, 3, 4, 5); // -> (i32, i32, i32, i32, i32)
    println!("TUPLE: {:?}", tup);

    let tup = (1, 2.6, true, 'f'); // -> (i32, f64, bool, char)
    println!("TUPLE: {:?}", tup);

    let tup = ();
    println!("TUPLE: {:?}", tup);

    let tup = ((1, 't'), ([1, 1, 1], false, 3.92), [1]);
    println!("TUPLE: {:?}", tup);
}

fn access_elements() {
    let tup = (1, 20, 13);
    println!("First element: {}", tup.0);
    println!("Second element: {}", tup.1);
    // println!("Third element: {}", tup.12); // Homework
}

fn destructure_tuple() {
    let tup = (1, 20, 13);
    let (.., a) = (12, 34, 80, 12, 123, 14, 35, 6, 57, 68, 79);
    println!("Destructured elements: a = {}", a);
}

fn return_tuple() -> (i32, bool) {
    // Computation GO!
    (30, false)
}

fn match_tuple() {
    let point = (-2, 10);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On X-axis at {}", x),
        (0, y) => println!("On Y-axis at {}", y),
        (-2, 10) => println!("Special point at (-2, 10)"),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}