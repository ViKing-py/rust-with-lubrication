// This is the entry point of the program.
// Rust looks for a function named 'main' to start execution.
fn main() {
    // Standard output printing.
    // 'println!' is a MACRO, not a function. We know this because of the '!'.
    // Macros are code that writes other code (metaprogramming).
    println!("Hello, world!");

    // You can print multiple values using placeholders "{}".
    println!("This is lesson number: {}", 1);

    /* This is a block comment.
       It spans multiple lines.
       
       Formatting note: 
       Rust prefers 4 spaces for indentation, not tabs.
       Most editors (VS Code) handle this automatically if you have the Rust extension.
    */
}