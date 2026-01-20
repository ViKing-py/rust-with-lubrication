fn main() {
    // ==========================================
    // 1. Variables and Immutability by default
    // ==========================================
    
    // By default, variables in Rust are immutable. 
    // This means once a value is bound to a name, you cannot change it.
    let x = 5;
    println!("The value of x is: {}", x);

    // UNCOMMENT the line below to see the compilation error:
    // x = 6; 
    // Error: cannot assign twice to immutable variable `x`


    // ==========================================
    // 2. Mutable Variables (mut)
    // ==========================================
    
    // We can make a variable mutable by adding 'mut' in front of the variable name.
    let mut y = 10;
    println!("The value of y is: {}", y);

    y = 20; // This is allowed because y is mutable
    println!("The value of y is now: {}", y);


    // ==========================================
    // 3. Constants (const)
    // ==========================================
    
    // Constants are ALWAYS immutable. You cannot use 'mut' with them.
    // The type of the value must be annotated explicitly (e.g., : u32).
    // Naming convention: UPPER_CASE_WITH_UNDERSCORES.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    println!("Constant value: {}", THREE_HOURS_IN_SECONDS);


    // ==========================================
    // 4. Shadowing
    // ==========================================
    
    // Shadowing is different from mutability.
    // It allows you to declare a new variable with the same name as a previous one.
    
    let z = 5;
    let z = z + 1; // This is a new variable 'z' that shadows the previous one.
    
    {
        // Shadowing works within inner scopes too
        let z = z * 2;
        println!("The value of z in the inner scope is: {}", z); // Prints 12
    }

    println!("The value of z in the outer scope is: {}", z); // Prints 6

    // POWERFUL FEATURE OF SHADOWING: Changing types
    // Since we are creating a new variable using 'let', we can change the value's type.
    
    let spaces = "   "; // Type: &str (string slice)
    let spaces = spaces.len(); // Type: usize (number)
    
    println!("Number of spaces: {}", spaces);

    // Compare with 'mut':
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len(); // ERROR: expected `&str`, found `usize`
    // You cannot mutate a variable's type, but you can shadow it to a new type.
}