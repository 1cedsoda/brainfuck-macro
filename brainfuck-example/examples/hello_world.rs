//! Hello World example using the brainfuck! macro
//!
//! Run with: cargo run --example hello_world

use brainfuck_macro::brainfuck;

fn main() {
    println!("=== Brainfuck Macro Examples ===\n");

    // Example 1: Classic Hello World
    println!("Example 1: Hello World");
    let hello = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    println!("Output: {}", hello);
    println!();

    // Example 2: Simple message
    println!("Example 2: Simple 'Hi'");
    let hi = brainfuck!("
        ++++++++ ++++++++ ++++++++ ++++++++ ++++++++ 
        ++++++++ ++++++++ ++++++++ ++++.    
        +.                                  
    ");
    println!("Output: {}", hi);
    println!();

    // Example 3: Using loops to create multiplication
    println!("Example 3: Multiplication (5 x 13 = 65 = 'A')");
    let multiplication = brainfuck!("
        +++++           Set cell 0 to 5
        [               Loop while cell 0 is not zero
            >+++++++++++++  Add 13 to cell 1
            <-              Decrement cell 0
        ]
        >.              Output cell 1 (65 = 'A')
    ");
    println!("Output: {}", multiplication);
    println!();

    // Example 4: Alphabet sequence
    println!("Example 4: First 5 letters of the alphabet");
    let alphabet = brainfuck!("
        ++++++++ ++++++++ ++++++++ ++++++++ ++++++++ 
        ++++++++ ++++++++ ++++++++ +.
        +.+.+.+.
    ");
    println!("Output: {}", alphabet);
    println!();

    // Example 5: ROT13-style transformation (conceptual)
    println!("Example 5: Number sequence");
    let numbers = brainfuck!("
        ++++++[>++++++++<-]>+.  Output 49 ('1')
        +.                       Output 50 ('2')
        +.                       Output 51 ('3')
    ");
    println!("Output: {}", numbers);
    println!();

    println!("All examples completed successfully!");
}
