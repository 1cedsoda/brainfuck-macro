//! Advanced Brainfuck examples demonstrating complex patterns
//!
//! Run with: cargo run --example advanced

use brainfuck_macro::brainfuck;

fn main() {
    println!("=== Advanced Brainfuck Examples ===\n");

    // Example 1: Nested loops
    println!("Example 1: Nested loops creating a pattern");
    let nested = brainfuck!("
        ++++[              Cell 0 = 4
            >++++[         Cell 1 = 4
                >++++      Cell 2 = 4
                <-         Decrement cell 1
            ]              Cell 1 is now 0. cell 2 is 16
            <-             Decrement cell 0
        ]                  Cell 0 is now 0. cell 2 is 64 (4*16)
        >>+.               Cell 2 = 65 - output 'A'
    ");
    println!("Output: {}", nested);
    println!();

    // Example 2: Complex string generation
    println!("Example 2: Multiple character output");
    let complex = brainfuck!("
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    ");
    println!("Output: {}", complex);
    println!();

    // Example 3: Creating specific ASCII values
    println!("Example 3: Simple multiplication");
    let multiplication = brainfuck!("
        ++++[>+++++<-]>+.      Output 21 
        [>+<-]                 Move value
    ");
    println!("Output (with control char): {:?}", multiplication);
    println!();

    // Example 4: Building letters efficiently  
    println!("Example 4: Building 'ABC'");
    let abc = brainfuck!("
        +++++ +++++              10
        [
            >+++++ +             6 per loop = 60
            <-
        ]
        >+++++.                  65 = A
        +.                       66 = B
        +.                       67 = C
    ");
    println!("Output: {}", abc);
    println!();

    // Example 5: Using cell manipulation
    println!("Example 5: Building 'OK!'");
    let ok = brainfuck!("
        +++++ +++++
        [
            >+++++ ++             7 per loop = 70
            >++++                 4 per loop = 40  
            <<-
        ]
        >++++++++.                70+8=78=N then +1=79=O ;
        >+++++ +++++
        +++++ +++++
        +++++ +++++
        +.                        40+31=71=G then +4=75=K ;
        <<
        +++++ +++
        [
            >>++++                4*8=32
            <<-
        ]
        >>+.                      33=!
    ");
    println!("Output: {}", ok);
    println!();

    println!("Advanced examples completed!");
}
