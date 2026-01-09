//! # Brainfuck Macro
//!
//! A procedural macro that executes Brainfuck code at compile time and produces a `&'static str`.
//!
//! ## Example
//!
//! ```rust
//! use brainfuck_macro::brainfuck;
//!
//! fn main() {
//!     // This Brainfuck code prints "Hello World!"
//!     let hello = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
//!     assert_eq!(hello, "Hello World!\n");
//! }
//! ```
//!
//! ## Brainfuck Language
//!
//! The macro supports all standard Brainfuck operations:
//! - `>` - Move the pointer to the right
//! - `<` - Move the pointer to the left
//! - `+` - Increment the memory cell at the pointer
//! - `-` - Decrement the memory cell at the pointer
//! - `.` - Output the character signified by the cell at the pointer
//! - `,` - Input a character and store it in the cell at the pointer (not supported at compile time)
//! - `[` - Jump past the matching `]` if the cell at the pointer is 0
//! - `]` - Jump back to the matching `[` if the cell at the pointer is nonzero
//!
//! ## Limitations
//!
//! - Input operations (`,`) are not supported at compile time and will cause a compilation error
//! - The tape size is limited to 30,000 cells
//! - Maximum execution steps is limited to 1,000,000 to prevent infinite loops at compile time

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// The maximum number of cells in the Brainfuck tape
const TAPE_SIZE: usize = 30_000;

/// The maximum number of execution steps to prevent infinite loops
const MAX_STEPS: usize = 1_000_000;

/// Error types for Brainfuck execution
#[derive(Debug)]
enum BrainfuckError {
    /// Unmatched opening bracket
    UnmatchedOpenBracket(usize),
    /// Unmatched closing bracket
    UnmatchedCloseBracket(usize),
    /// Pointer moved out of bounds (left)
    PointerUnderflow,
    /// Pointer moved out of bounds (right)
    PointerOverflow,
    /// Input operation not supported at compile time
    InputNotSupported,
    /// Execution exceeded maximum steps
    MaxStepsExceeded,
}

impl std::fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainfuckError::UnmatchedOpenBracket(pos) => {
                write!(f, "Unmatched '[' at position {}", pos)
            }
            BrainfuckError::UnmatchedCloseBracket(pos) => {
                write!(f, "Unmatched ']' at position {}", pos)
            }
            BrainfuckError::PointerUnderflow => {
                write!(f, "Pointer moved below zero")
            }
            BrainfuckError::PointerOverflow => {
                write!(f, "Pointer moved beyond tape size ({})", TAPE_SIZE)
            }
            BrainfuckError::InputNotSupported => {
                write!(f, "Input operation ',' is not supported at compile time")
            }
            BrainfuckError::MaxStepsExceeded => {
                write!(f, "Execution exceeded maximum steps ({})", MAX_STEPS)
            }
        }
    }
}

/// Brainfuck interpreter that executes code at compile time
struct BrainfuckInterpreter {
    tape: Vec<u8>,
    pointer: usize,
    output: String,
}

impl BrainfuckInterpreter {
    /// Create a new Brainfuck interpreter
    fn new() -> Self {
        Self {
            tape: vec![0; TAPE_SIZE],
            pointer: 0,
            output: String::new(),
        }
    }

    /// Find matching bracket positions for jump operations
    fn find_matching_brackets(code: &str) -> Result<Vec<Option<usize>>, BrainfuckError> {
        let mut jump_table = vec![None; code.len()];
        let mut stack = Vec::new();

        for (i, ch) in code.chars().enumerate() {
            match ch {
                '[' => {
                    stack.push(i);
                }
                ']' => {
                    if let Some(open_pos) = stack.pop() {
                        jump_table[open_pos] = Some(i);
                        jump_table[i] = Some(open_pos);
                    } else {
                        return Err(BrainfuckError::UnmatchedCloseBracket(i));
                    }
                }
                _ => {}
            }
        }

        if let Some(open_pos) = stack.pop() {
            return Err(BrainfuckError::UnmatchedOpenBracket(open_pos));
        }

        Ok(jump_table)
    }

    /// Execute Brainfuck code and return the output
    fn execute(&mut self, code: &str) -> Result<String, BrainfuckError> {
        let jump_table = Self::find_matching_brackets(code)?;
        let chars: Vec<char> = code.chars().collect();
        
        let mut ip = 0; // instruction pointer
        let mut steps = 0;

        while ip < chars.len() {
            if steps >= MAX_STEPS {
                return Err(BrainfuckError::MaxStepsExceeded);
            }
            steps += 1;

            match chars[ip] {
                '>' => {
                    if self.pointer >= TAPE_SIZE - 1 {
                        return Err(BrainfuckError::PointerOverflow);
                    }
                    self.pointer += 1;
                }
                '<' => {
                    if self.pointer == 0 {
                        return Err(BrainfuckError::PointerUnderflow);
                    }
                    self.pointer -= 1;
                }
                '+' => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
                }
                '-' => {
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
                }
                '.' => {
                    self.output.push(self.tape[self.pointer] as char);
                }
                ',' => {
                    return Err(BrainfuckError::InputNotSupported);
                }
                '[' => {
                    if self.tape[self.pointer] == 0 {
                        if let Some(matching) = jump_table[ip] {
                            ip = matching;
                        }
                    }
                }
                ']' => {
                    if self.tape[self.pointer] != 0 {
                        if let Some(matching) = jump_table[ip] {
                            ip = matching;
                        }
                    }
                }
                _ => {
                    // Ignore non-Brainfuck characters (comments)
                }
            }

            ip += 1;
        }

        Ok(self.output.clone())
    }
}

/// Execute Brainfuck code at compile time and produce a `&'static str`.
///
/// # Example
///
/// ```rust
/// use brainfuck_macro::brainfuck;
///
/// let hello = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
/// assert_eq!(hello, "Hello World!\n");
/// ```
///
/// # Errors
///
/// The macro will produce a compile-time error if:
/// - The Brainfuck code has unmatched brackets
/// - The code attempts to use input operations (`,`)
/// - The pointer moves out of bounds
/// - Execution exceeds the maximum step limit
///
/// # Supported Operations
///
/// - `>` - Move pointer right
/// - `<` - Move pointer left
/// - `+` - Increment cell
/// - `-` - Decrement cell
/// - `.` - Output cell as character
/// - `[` - Loop start (jump to matching `]` if cell is 0)
/// - `]` - Loop end (jump to matching `[` if cell is nonzero)
///
/// All other characters are treated as comments and ignored.
#[proc_macro]
pub fn brainfuck(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let code = input_str.value();

    let mut interpreter = BrainfuckInterpreter::new();
    
    match interpreter.execute(&code) {
        Ok(output) => {
            let expanded = quote! {
                #output
            };
            TokenStream::from(expanded)
        }
        Err(e) => {
            let error_msg = format!("Brainfuck execution error: {}", e);
            let expanded = quote! {
                compile_error!(#error_msg)
            };
            TokenStream::from(expanded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "Hello World!\n");
    }

    #[test]
    fn test_simple_output() {
        // 5 * 13 = 65 = 'A'
        let code = "+++++[>+++++++++++++<-]>.";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "A");
    }

    #[test]
    fn test_loop() {
        let code = "+++++[>++++<-]>."; // 5 * 4 = 20
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "\u{14}"); // ASCII 20
    }

    #[test]
    fn test_unmatched_open_bracket() {
        let code = "[++";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code);
        assert!(matches!(result, Err(BrainfuckError::UnmatchedOpenBracket(_))));
    }

    #[test]
    fn test_unmatched_close_bracket() {
        let code = "++]";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code);
        assert!(matches!(result, Err(BrainfuckError::UnmatchedCloseBracket(_))));
    }

    #[test]
    fn test_input_not_supported() {
        let code = ",";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code);
        assert!(matches!(result, Err(BrainfuckError::InputNotSupported)));
    }

    #[test]
    fn test_pointer_underflow() {
        let code = "<";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code);
        assert!(matches!(result, Err(BrainfuckError::PointerUnderflow)));
    }

    #[test]
    fn test_nested_loops() {
        // 2 outer * 2 inner * 2 innermost = 8 in cell 2
        let code = "++[>++[>++<-]<-]>>.";
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "\u{08}"); // ASCII 8
    }

    #[test]
    fn test_comments() {
        let code = "This is a comment +++ with text . interspersed"; // Should output ASCII 3
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "\u{03}");
    }

    #[test]
    fn test_wrapping() {
        // Test that cells wrap at 256
        let code = "--------."; // 0 - 8 = 248 (wrapping)
        let mut interpreter = BrainfuckInterpreter::new();
        let result = interpreter.execute(code).unwrap();
        assert_eq!(result, "\u{f8}");
    }
}
