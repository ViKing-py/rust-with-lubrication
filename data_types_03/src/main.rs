fn main() {
    // ==========================================
    // 1. SCALAR TYPES (Represent a single value)
    // ==========================================

    // --- Integers ---
    // Numbers without fractional parts.
    // Default is 'i32' (32-bit signed integer).
    let default_int = 10; 
    
    // Explicit type annotation: 'u8' (unsigned 8-bit integer, 0 to 255).
    let small_number: u8 = 255;
    
    // You can use underscores for readability (1,000,000).
    let big_number = 1_000_000; 

    println!("Integers: {}, {}, {}", default_int, small_number, big_number);

    // --- Floating-Point Numbers ---
    // Numbers with decimal points.
    // Default is 'f64' (double precision) because it is faster on modern CPUs.
    let default_float = 2.5; 
    
    // Explicit 'f32' (single precision).
    let small_float: f32 = 3.14;

    println!("Floats: {} and {}", default_float, small_float);

    // --- Boolean ---
    // Represents true or false. Used in control flow.
    let is_rust_fun: bool = true;
    let is_hard = false; // Implicit type inference

    println!("Is Rust fun? {}", is_rust_fun);

    // --- Characters (char) ---
    // Specified with single quotes ''. 
    // Rust char is 4 bytes and represents a Unicode Scalar Value (supports emojis etc).
    let letter = 'z';
    let emoji = '🦀';

    println!("Char: {} {}", letter, emoji);

    // ==========================================
    // 2. COMPOUND TYPES (Group multiple values)
    // ==========================================

    // --- Tuples ---
    // Fixed length. Can contain different types.
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Accessing tuple elements using dot notation (index starts at 0).
    let five_hundred = my_tuple.0;
    let six_point_four = my_tuple.1;

    // Destructuring a tuple (unpacking values into variables).
    let (x, y, z) = my_tuple;

    println!("Tuple value y is: {}", y);

    // --- Arrays ---
    // Fixed length. MUST contain the SAME type.
    // Allocated on the Stack (very fast).
    // Syntax: [Type; Length]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing array elements.
    let first = numbers[0];
    let second = numbers[1];
    
    // Common mistake: accessing an index out of bounds will cause a runtime PANIC.
    // let error = numbers[10]; // This would crash the program.

    println!("Array first element: {}", first);

    // Short declaration for repeating values: [value; repeats]
    // Creates an array of 5 zeros.
    let zeros = [0; 5]; 
    println!("Array of zeros: {:?}", zeros); // {:?} is for debug printing

    // ==========================================
    // 3. TYPE INFERENCE VS EXPLICIT
    // ==========================================
    
    // Inference: Compiler guesses the type (i32).
    let guess = 42; 
    
    // Explicit: Required when many types are possible.
    // Here, "42" is parsed from a string, so we MUST tell Rust what type we want.
    let parsed_number: u32 = "42".parse().expect("Not a number!");
    
    println!("Parsed explicit type: {}", parsed_number);
}