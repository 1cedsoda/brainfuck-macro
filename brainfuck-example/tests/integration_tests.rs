//! Integration tests for the brainfuck! macro

use brainfuck_macro::brainfuck;

#[test]
fn test_hello_world() {
    let result = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    assert_eq!(result, "Hello World!\n");
}

#[test]
fn test_empty_code() {
    let result = brainfuck!("");
    assert_eq!(result, "");
}

#[test]
fn test_comments_ignored() {
    let result = brainfuck!("This is a comment +++ Add three . Output More comments here!");
    assert_eq!(result, "\u{03}");
}

#[test]
fn test_single_char_output() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.");
    assert_eq!(result, "A");
}

#[test]
fn test_multiple_chars() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.+.+.");
    assert_eq!(result, "ABC");
}

#[test]
fn test_simple_loop() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.");
    assert_eq!(result, "A");
}

#[test]
fn test_nested_loops() {
    let result = brainfuck!("++[>++[>++<-]<-]>>.");
    assert_eq!(result, "\u{08}");
}

#[test]
fn test_cell_wrapping() {
    let result = brainfuck!("-.");
    assert_eq!(result, "\u{ff}");
}

#[test]
fn test_zero_loop_skip() {
    let result = brainfuck!("[++++++++++++++]+++++[>+++++++++++++<-]>.");
    assert_eq!(result, "A");
}

#[test]
fn test_loop_until_zero() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.");
    assert_eq!(result, "A");
}

#[test]
fn test_multiple_outputs() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.++++.");
    assert_eq!(result, "AE");
}

#[test]
fn test_whitespace_handling() {
    let result = brainfuck!("+ + + + + [ > + + + + + + + + + + + + + < - ] > .");
    assert_eq!(result, "A");
}

#[test] 
fn test_sequential_output() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.+.+.+.+.");
    assert_eq!(result, "ABCDE");
}

#[test]
fn test_calculation() {
    let result = brainfuck!("+++++[>+++++++++++++<-]>.");
    assert_eq!(result, "A");
}
