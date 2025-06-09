# Rust Data Types, Variables, and Mutability - Complete Guide

## Table of Contents
1. [Data Types](#data-types)
   - [Integer Types](#integer-types)
   - [Floating Point Types](#floating-point-types)
   - [Boolean Type](#boolean-type)
   - [Character Type](#character-type)
   - [Type Literals and Suffixes](#type-literals-and-suffixes)
2. [Variables and Type Inference](#variables-and-type-inference)
   - [Variable Declaration](#variable-declaration)
   - [Type Inference Rules](#type-inference-rules)
   - [Explicit Type Annotations](#explicit-type-annotations)
3. [Mutability](#mutability)
   - [Immutable by Default](#immutable-by-default)
   - [Explicit Mutability](#explicit-mutability)
   - [Mutable References](#mutable-references)
4. [Variable Shadowing](#variable-shadowing)
   - [Basic Shadowing](#basic-shadowing)
   - [Shadowing vs Mutability](#shadowing-vs-mutability)
   - [Scope and Shadowing](#scope-and-shadowing)
5. [Constants and Static Variables](#constants-and-static-variables)
   - [Constants](#constants)
   - [Static Variables](#static-variables)
   - [When to Use What](#when-to-use-what)
6. [Memory and Performance](#memory-and-performance)

---

## Data Types

Rust is a statically typed language, meaning every variable must have a known type at compile time. Rust provides several scalar (single value) data types.

### Integer Types

Rust provides precise control over integer sizes and whether they can represent negative values.

#### Signed Integers (can hold negative values)
```rust
let tiny: i8 = -42;             // 1 byte: -128 to 127
let small: i16 = -1000;         // 2 bytes: -32,768 to 32,767
let normal: i32 = -50_000;      // 4 bytes: ~-2.1 billion to 2.1 billion (DEFAULT)
let big: i64 = -5_000_000_000;  // 8 bytes: ~-9.2 quintillion to 9.2 quintillion
let huge: i128 = -1000000000000000000000000000; // 16 bytes: extremely large range
```

#### Unsigned Integers (only positive values)
```rust
let byte: u8 = 255;             // 1 byte: 0 to 255
let port: u16 = 8080;           // 2 bytes: 0 to 65,535
let count: u32 = 1_000_000;     // 4 bytes: 0 to ~4.3 billion
let file_size: u64 = 1_073_741_824; // 8 bytes: 0 to ~18.4 quintillion
let crypto: u128 = 340282366920938463463374607431768211455; // 16 bytes: massive range
```

#### Architecture-Dependent Integers
```rust
let index: usize = 42;          // Pointer-sized (32 or 64 bit)
let offset: isize = -100;       // Signed pointer-sized

// usize is required for array indexing
let data = [1, 2, 3, 4, 5];
let element = data[index];      // Must use usize, not u32 or i32
```

#### Why Different Sizes Matter
- **Memory efficiency**: Use `u8` for small values instead of wasting 4 bytes with `i32`
- **Performance**: Smaller types can be faster for bulk operations
- **Correctness**: `usize` prevents array indexing bugs on different architectures
- **Interoperability**: Match expectations of external libraries and protocols

#### Integer Ranges Formula
- **Signed n-bit**: -2^(n-1) to 2^(n-1) - 1
- **Unsigned n-bit**: 0 to 2^n - 1

### Type Range Tables

#### Integer Type Ranges

| Type | Size | Minimum Value | Maximum Value |
|------|------|---------------|---------------|
| `i8` | 1 byte | -128 | 127 |
| `i16` | 2 bytes | -32,768 | 32,767 |
| `i32` | 4 bytes | -2,147,483,648 | 2,147,483,647 |
| `i64` | 8 bytes | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
| `i128` | 16 bytes | -170,141,183,460,469,231,731,687,303,715,884,105,728 | 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `u8` | 1 byte | 0 | 255 |
| `u16` | 2 bytes | 0 | 65,535 |
| `u32` | 4 bytes | 0 | 4,294,967,295 |
| `u64` | 8 bytes | 0 | 18,446,744,073,709,551,615 |
| `u128` | 16 bytes | 0 | 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| `isize` | arch-dependent | same as `i32` or `i64` | same as `i32` or `i64` |
| `usize` | arch-dependent | 0 | same as `u32` or `u64` |

#### Floating Point Type Ranges

| Type | Size | Smallest Positive | Largest Finite | Precision (decimal digits) | Epsilon |
|------|------|-------------------|----------------|---------------------------|---------|
| `f32` | 4 bytes | ~1.18 × 10^-38 | ~3.40 × 10^38 | ~7 | ~1.19 × 10^-7 |
| `f64` | 8 bytes | ~2.23 × 10^-308 | ~1.80 × 10^308 | ~15-17 | ~2.22 × 10^-16 |

```rust
// Accessing these values in code
println!("Integer ranges:");
println!("i8:  {} to {}", i8::MIN, i8::MAX);
println!("i32: {} to {}", i32::MIN, i32::MAX);
println!("u8:  {} to {}", u8::MIN, u8::MAX);
println!("u32: {} to {}", u32::MIN, u32::MAX);

println!("\nFloating point ranges:");
println!("f32 min positive: {:.2e}", f32::MIN_POSITIVE);
println!("f32 max:          {:.2e}", f32::MAX);
println!("f32 epsilon:      {:.2e}", f32::EPSILON);

println!("f64 min positive: {:.2e}", f64::MIN_POSITIVE);
println!("f64 max:          {:.2e}", f64::MAX);
println!("f64 epsilon:      {:.2e}", f64::EPSILON);
```

#### What is Epsilon?
Epsilon represents the machine epsilon - the smallest positive floating-point number that, when added to 1.0, produces a result different from 1.0. It's a measure of floating-point precision.

Think of it as the "granularity" or "precision limit" of floating-point numbers near 1.0.

It's essentially the _"gap"_ between 1.0 and the next representable floating-point number.

### Floating Point Types

Rust follows the IEEE 754 standard for floating-point representation.

```rust
let precise: f64 = 3.141592653589793;  // 64-bit (DEFAULT) - double precision
let graphics: f32 = 3.14159;           // 32-bit - single precision

// Scientific notation
let avogadro: f64 = 6.022e23;          // 6.022 × 10²³
let planck: f64 = 6.626e-34;           // 6.626 × 10⁻³⁴

// Special values
let infinity = f64::INFINITY;
let neg_infinity = f64::NEG_INFINITY;
let not_a_number = f64::NAN;

// Precision demonstration
println!("f32 π: {:.10}", std::f32::consts::PI);  // 3.1415927410
println!("f64 π: {:.17}", std::f64::consts::PI);  // 3.14159265358979324
```

#### IEEE 754 Standard Deep Dive

The IEEE 754 standard defines how floating-point numbers are represented in binary. Understanding this helps explain floating-point behavior:

**Binary Representation Format:**
```
[Sign Bit][Exponent][Mantissa/Significand]
```

**f32 (32-bit) breakdown:**
- 1 bit for sign (positive/negative)
- 8 bits for exponent (allows range of ~10^-38 to 10^38)
- 23 bits for mantissa (provides ~7 decimal digits of precision)

**f64 (64-bit) breakdown:**
- 1 bit for sign
- 11 bits for exponent (allows range of ~10^-308 to 10^308)
- 52 bits for mantissa (provides ~15-17 decimal digits of precision)

#### When to Use f32 vs f64
- **Use f64 (default)**: Scientific calculations, financial data, general purpose
- **Use f32**: Graphics programming, audio processing, large arrays where memory matters, interfacing with C libraries

#### Floating Point Precision Issues
```rust
let a: f64 = 0.1;
let b: f64 = 0.2;
let sum = a + b;
let expected = 0.3;

println!("0.1 + 0.2 = {:.17}", sum);     // 0.30000000000000004
println!("Expected = {:.17}", expected); // 0.30000000000000000
println!("Equal? {}", sum == expected);  // false!

// Safe comparison
const EPSILON: f64 = 1e-10;
if (sum - expected).abs() < EPSILON {
    println!("Approximately equal");
}
```

### Boolean Type

```rust
let is_learning: bool = true;
let is_difficult: bool = false;

// Size and memory
use std::mem;
println!("bool size: {} byte", mem::size_of::<bool>()); // 1 byte

// Boolean operations
let a = true;
let b = false;
let and_result = a && b;        // false (both must be true)
let or_result = a || b;         // true (at least one must be true)
let not_result = !a;            // false (flip the value)
let xor_result = a ^ b;         // true (exactly one must be true)
```

#### Why 1 byte instead of 1 bit?
- **Memory addressability**: CPUs access memory in byte-sized chunks
- **Performance**: Single byte operations are very fast
- **Simplicity**: No bit manipulation needed

#### Boolean in Control Flow
```rust
let age = 18;
let can_vote = age >= 18;
let status = if can_vote { "eligible" } else { "not eligible" };

// Short-circuit evaluation
let result = false && expensive_function(); // expensive_function() never called
let result = true || expensive_function();  // expensive_function() never called
```

### Character Type

Rust's `char` type represents a Unicode Scalar Value, not just ASCII.

```rust
let ascii: char = 'A';          // ASCII character
let accented: char = 'é';       // Accented character
let chinese: char = '中';        // Chinese character
let emoji: char = '🦀';         // Emoji (Rust crab!)
let math: char = '∑';           // Mathematical symbol

// Size: always 4 bytes
println!("char size: {} bytes", mem::size_of::<char>()); // 4

// Unicode code points
let letter = 'A';
println!("'A' Unicode: U+{:04X}", letter as u32); // U+0041

// Character properties
let ch = 'A';
println!("Is alphabetic: {}", ch.is_alphabetic());  // true
println!("Is numeric: {}", ch.is_numeric());        // false
println!("Is uppercase: {}", ch.is_uppercase());    // true
println!("Is ASCII: {}", ch.is_ascii());            // true

// Unicode vs UTF-8 difference
let emoji_char: char = '🦀';     // 4 bytes as char
let emoji_str = "🦀";            // 4 bytes as UTF-8 string
```

#### char bytes example
```rust
// ASCII character - 1 byte
let ascii = "A";
println!("'A' bytes: {:?}", ascii.as_bytes());  // [65]
println!("Length: {}", ascii.len());            // 1

// Latin extended - 2 bytes  
let latin = "ñ";
println!("'ñ' bytes: {:?}", latin.as_bytes());  // [195, 177]
println!("Length: {}", latin.len());            // 2

// Chinese character - 3 bytes
let chinese = "中";
println!("'中' bytes: {:?}", chinese.as_bytes()); // [228, 184, 173]
println!("Length: {}", chinese.len());           // 3

// Emoji - 4 bytes
let emoji = "🦀";
println!("'🦀' bytes: {:?}", emoji.as_bytes());  // [240, 159, 166, 128]
println!("Length: {}", emoji.len());             // 4
```

#### char vs String Relationship
```rust
// A char is a single Unicode scalar value
let single_char: char = '🦀';
let char_as_string: String = single_char.to_string();

// Strings are collections of UTF-8 bytes, not char arrays
let text = "Hello 🦀";
// let first = text[0];         // Error! Can't index strings
let first_char = text.chars().next().unwrap(); // Correct way

// Why no string indexing?
// UTF-8 encoding means characters can be 1-4 bytes
// Indexing would be O(n) operation, not O(1)
```

### Type Literals and Suffixes

```rust
// Number base representations
let decimal = 98_222;           // Underscore for readability
let hex = 0xFF;                 // Hexadecimal
let octal = 0o77;               // Octal  
let binary = 0b1111_0000;       // Binary

// Type suffixes
let explicit_u8 = 42u8;         // Force u8 type
let explicit_i64 = -100i64;     // Force i64 type
let explicit_f32 = 3.14f32;     // Force f32 type

// Byte literals (u8 only)
let byte_a = b'A';              // 65u8 (ASCII 'A')
let newline = b'\n';            // 10u8 (newline byte)

// When to use suffixes
let max_u8 = 255u8;             // When type can't be inferred
let data: Vec<u16> = vec![1u16, 2u16, 3u16]; // When working with specific types
```

---

## Variables and Type Inference

### Variable Declaration

```rust
// Basic variable declaration with let
let name = "Alice";             // Immutable by default
let age = 30;                   // Type inferred as i32
let height = 5.9;               // Type inferred as f64

// Variables are immutable by default
// age = 31;                    // Compile error!

// Explicit mutability
let mut score = 100;            // Mutable variable
score = 95;                     // Now this works
score += 5;                     // Compound assignment also works
```

### Type Inference Rules

Rust's type inference is sophisticated but follows predictable rules:

```rust
// 1. Default types for literals
let integer = 42;               // Defaults to i32
let float = 3.14;              // Defaults to f64
let text = "hello";            // Defaults to &str

// 2. Inference from function returns
fn get_age() -> u8 { 25 }
let age = get_age();           // age is u8

// 3. Inference from method calls
let text = "hello world";
let length = text.len();       // usize (str::len returns usize)
let upper = text.to_uppercase(); // String

// 4. Inference from usage context
let mut numbers = Vec::new();   // Type unknown initially
numbers.push(42);              // Now it's Vec<i32>

// 5. Inference through operations
let a = 10;                    // i32
let b = 20i64;                 // i64  
// let sum = a + b;            // Error: can't add i32 and i64
let sum = a as i64 + b;        // Must be explicit
```

### Explicit Type Annotations

Sometimes you need to specify types explicitly:

```rust
// When inference fails
let numbers: Vec<i32> = Vec::new();     // Empty collection
let parsed: i32 = "42".parse().unwrap(); // Multiple possible types

// When you need specific sizes
let small_number: u8 = 255;     // Instead of default i32
let precise_float: f32 = 3.14;  // Instead of default f64

// Complex type annotations
let coordinates: (f64, f64) = (10.5, 20.3);
let scores: [i32; 5] = [95, 87, 92, 78, 88];
let users: Vec<String> = vec!["Alice".to_string(), "Bob".to_string()];

// Tuple destructuring with types
let (x, y): (i32, f64) = (10, 3.14);

// Function parameters always need types
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}
```

---

## Mutability

### Immutable by Default

Rust variables are immutable by default, which prevents many common bugs:

```rust
let balance = 1000;             // Cannot be changed
let account_number = 12345;     // Cannot be changed

// Benefits of immutability:
// 1. Prevents accidental modifications
// 2. Enables safe concurrency
// 3. Makes code easier to reason about
// 4. Compiler optimizations

// This won't compile:
// balance = 900;               // Error: cannot assign twice to immutable variable
```

### Explicit Mutability

When you need to change a variable, use the `mut` keyword:

```rust
let mut counter = 0;            // Explicitly mutable
counter += 1;                   // OK
counter = 100;                  // OK

let mut name = String::from("Alice");
name.push_str(" Smith");        // OK
name = String::from("Bob");     // OK

// Mutability is part of the variable binding, not the type
let x = 5;                      // x is immutable
let mut x = x;                  // New mutable variable, shadows the old one
x += 1;                         // Now OK
```

### Mutable References

Rust's borrowing system controls how mutability works with references:

```rust
let mut data = vec![1, 2, 3];

// Immutable reference
let immutable_ref = &data;
println!("Data: {:?}", immutable_ref);

// Mutable reference (after immutable reference is done)
let mutable_ref = &mut data;
mutable_ref.push(4);

// Borrowing rules:
// 1. You can have multiple immutable references OR one mutable reference
// 2. References must always be valid (no dangling pointers)

// This won't compile:
let mut data = vec![1, 2, 3];
let r1 = &data;                 // Immutable reference
// let r2 = &mut data;          // Error: can't have mutable while immutable exists
// println!("{:?}", r1);        // r1 is still being used
```

#### Mutable Function Parameters

```rust
// Taking ownership and returning modified value
fn append_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

// Modifying through mutable reference
fn append_exclamation_ref(text: &mut String) {
    text.push('!');
}

let mut message = String::from("Hello");
message = append_exclamation(message);  // message moved and returned

let mut message2 = String::from("World");
append_exclamation_ref(&mut message2);  // message2 modified in place
```

---

## Variable Shadowing

Shadowing allows you to reuse variable names while creating new variables.

### Basic Shadowing

```rust
let x = 5;                      // x is i32 with value 5
let x = x + 1;                  // x is i32 with value 6 (new variable)
let x = x * 2;                  // x is i32 with value 12 (new variable)

println!("x = {}", x);          // Prints: x = 12

// Each `let` creates a completely new variable that "shadows" the previous one
// The old variables still exist in memory but are inaccessible by name
```

### Shadowing vs Mutability

```rust
// Shadowing can change types
let spaces = "   ";             // spaces is &str
let spaces = spaces.len();      // spaces is now usize (3)

// Mutability cannot change types
let mut spaces = "   ";         // spaces is &str
// spaces = spaces.len();       // Error: cannot change type

// Shadowing creates new variables
let data = "hello";             // Immutable
let data = data.to_uppercase(); // New immutable variable with different type

// Mutability modifies existing variable
let mut data = String::from("hello");
data = data.to_uppercase();     // Same variable, same type, modified value
```

### Scope and Shadowing

```rust
let x = 1;                      // Outer scope

{
    let x = 2;                  // Inner scope, shadows outer x
    println!("Inner x: {}", x); // Prints: Inner x: 2
    
    {
        let x = x * 3;          // Deeper scope, shadows inner x
        println!("Deeper x: {}", x); // Prints: Deeper x: 6
    }
    
    println!("Back to inner: {}", x); // Prints: Back to inner: 2
}

println!("Back to outer: {}", x);     // Prints: Back to outer: 1
```

#### Practical Shadowing Example

```rust
// Data processing pipeline with shadowing
let input = "  42  ";           // Raw input
let input = input.trim();       // Cleaned input
let input: i32 = input.parse().expect("Not a number"); // Parsed input
let input = input * 2;          // Processed result

println!("Final result: {}", input); // 84

// This is cleaner than:
// let raw_input = "  42  ";
// let cleaned_input = raw_input.trim();
// let parsed_input: i32 = cleaned_input.parse().expect("Not a number");
// let final_result = parsed_input * 2;
```

---

## Constants and Static Variables

### Constants

Constants are always immutable and must be compile-time computable:

```rust
// Constants use SCREAMING_SNAKE_CASE
const MAX_USERS: u32 = 100_000;
const PI: f64 = 3.14159265359;
const APP_NAME: &str = "MyRustApp";
const BUFFER_SIZE: usize = 1024 * 8;    // Compile-time arithmetic OK

// Constants must have explicit types
// const INVALID = 42;          // Error: missing type annotation

// Can use const fn in constants
const fn square(x: u32) -> u32 {
    x * x
}
const AREA: u32 = square(10);   // 100

// Constants can be declared in any scope
fn some_function() {
    const LOCAL_CONSTANT: i32 = 42;
    println!("Local constant: {}", LOCAL_CONSTANT);
}
```

#### Constant Requirements
- Must use `SCREAMING_SNAKE_CASE`
- Must have explicit type annotation
- Must be compile-time computable
- Can be declared in any scope (global, function, block)
- No function calls except `const fn`

### Static Variables

Static variables live for the entire program duration:

```rust
// Static variables also use SCREAMING_SNAKE_CASE
static GLOBAL_CONFIG: &str = "production";
static VISITOR_COUNT: std::sync::atomic::AtomicUsize = 
    std::sync::atomic::AtomicUsize::new(0);

// Static variables have memory addresses (unlike constants)
fn get_config_reference() -> &'static str {
    &GLOBAL_CONFIG              // Can return reference to static
}

// Mutable statics (unsafe and rare)
static mut DANGER_ZONE: i32 = 0;

unsafe fn increment_danger() {
    DANGER_ZONE += 1;           // Requires unsafe block
}
```

#### Constants vs Static Variables
| Constants | Static Variables |
|-----------|------------------|
| Inlined at each usage | Fixed memory location |
| Compile-time values only | Runtime values allowed |
| Cannot be referenced | Can be referenced |
| No memory overhead | Uses memory |
| Fast access | Memory access required |

### When to Use What

```rust
// Use constants for:
const HTTP_OK: u16 = 200;               // Small frequently-used values
const MAX_RETRIES: u32 = 3;             // Configuration constants
const MATH_PI: f64 = 3.14159265359;     // Mathematical constants

// Use static for:
static LOOKUP_TABLE: [u8; 256] = [0; 256];     // Large data structures
static ERROR_MESSAGES: &[&str] = &[             // Collections of data
    "File not found",
    "Permission denied", 
    "Network error"
];

// Use regular variables for:
let user_input = get_input();           // Runtime values
let calculated = expensive_function();  // Function results
let current_time = std::time::Instant::now(); // Time-dependent values
```

---

## Memory and Performance

### Type Sizes

```rust
use std::mem;

// Scalar type sizes
println!("Type sizes:");
println!("i8:   {} bytes", mem::size_of::<i8>());    // 1
println!("i16:  {} bytes", mem::size_of::<i16>());   // 2
println!("i32:  {} bytes", mem::size_of::<i32>());   // 4
println!("i64:  {} bytes", mem::size_of::<i64>());   // 8
println!("i128: {} bytes", mem::size_of::<i128>());  // 16

println!("u8:   {} bytes", mem::size_of::<u8>());    // 1
println!("u32:  {} bytes", mem::size_of::<u32>());   // 4
println!("u64:  {} bytes", mem::size_of::<u64>());   // 8

println!("f32:  {} bytes", mem::size_of::<f32>());   // 4
println!("f64:  {} bytes", mem::size_of::<f64>());   // 8

println!("bool: {} bytes", mem::size_of::<bool>());  // 1
println!("char: {} bytes", mem::size_of::<char>());  // 4

println!("usize: {} bytes", mem::size_of::<usize>()); // 8 on 64-bit, 4 on 32-bit
```

### Memory Layout Considerations

```rust
// Struct padding example
#[repr(C)]
struct BadLayout {
    a: u8,      // 1 byte
    b: u64,     // 8 bytes, needs 8-byte alignment
    c: u8,      // 1 byte
}

#[repr(C)]
struct GoodLayout {
    b: u64,     // 8 bytes
    a: u8,      // 1 byte  
    c: u8,      // 1 byte
}

println!("BadLayout: {} bytes", mem::size_of::<BadLayout>());   // 24 bytes
println!("GoodLayout: {} bytes", mem::size_of::<GoodLayout>()); // 16 bytes
```

### Performance Tips

1. **Choose appropriate integer sizes**: Don't always use `i32` if `u8` suffices
2. **Consider memory layout**: Group similar-sized fields in structs
3. **Use constants for repeated values**: They're inlined for better performance
4. **Prefer immutability**: Enables compiler optimizations
5. **Be mindful of floating-point precision**: Use `f32` when `f64` precision isn't needed

### Key Takeaways

- **Strong typing**: Rust prevents type-related bugs at compile time
- **Immutable by default**: Safer code and better performance
- **Explicit mutability**: Clear intent when data needs to change
- **Flexible shadowing**: Reuse names while preserving immutability
- **Smart inference**: Less verbosity without sacrificing safety
- **Memory efficient**: Choose the right type for the job