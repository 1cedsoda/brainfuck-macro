//! This crate demonstrates the usage of the brainfuck-macro.
//!
//! The brainfuck! macro allows you to execute Brainfuck code at compile time
//! and embed the result as a static string in your binary.

pub use brainfuck_macro::brainfuck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let result = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
        assert_eq!(result, "Hello World!\n");
    }

    #[test]
    fn test_simple_chars() {
        // Output 'HI' - H=72 I=73
        let result = brainfuck!("
            ++++++++ ++++++++ ++++++++ ++++++++ ++++++++ 
            ++++++++ ++++++++ ++++++++ ++++++++ .    
            +.                                  
        ");
        assert_eq!(result, "HI");
    }

    #[test]
    fn test_alphabet() {
        // Print A-E
        let result = brainfuck!("
            ++++++++ ++++++++ ++++++++ ++++++++ ++++++++ 
            ++++++++ ++++++++ ++++++++ +.       # A (65)
            +.                                  # B (66)
            +.                                  # C (67)
            +.                                  # D (68)
            +.                                  # E (69)
        ");
        assert_eq!(result, "ABCDE");
    }
}
