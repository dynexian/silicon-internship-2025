fn main() {
    // let variable_name: type = "value";
    
    // i8
    let a: i8 = 100;
    // println!("a: {}", a);

    let x: i8 = 127;
    // println!("x: {}", x);

    // u128
    let b: u128 = 9999999999999999999999999999999999999;

    // Type Inference
    let c = -1;
    let c = -1.968;

    // Floats
    let floating_num = 3.14;
    // println!("floating_num: {}", floating_num);

    let f1: f32 = 0.1;
    let f2: f32 = 0.2;
    let f3: f64 = (f1 + f2).into();
    // println!("f3: {}", f3);

    // Booleans
    let is_true = true;
    let is_false = false;

    // println!("is_true: {}, is_false: {}", !is_true, !is_false);

    // Characters
    let char_a = 'A';
    // println!("char_a: {}", char_a);

    let char_n = 'ñ';
    // println!("char_a: {}", char_n);

    let emoji = '😊';
    // println!("emoji: {}", emoji);

    let char_a_bytes = "A".as_bytes();
    let char_n_bytes = "ñ".as_bytes();
    let emoji_bytes = "😊".as_bytes();

    println!("Char a bytes: {:?}", char_a_bytes);
    println!("Char n bytes: {:?}", char_n_bytes);
    println!("Emoji bytes: {:?}", emoji_bytes);
}