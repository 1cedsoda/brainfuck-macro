# Brainfuck Macro

A Rust procedural macro that executes [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) code at compile time and embeds the result as a `&'static str` in your binary.

## Features

- ✨ **Compile-time execution**: Brainfuck code is executed during compilation
- 🚀 **Zero runtime overhead**: Results are embedded as static strings
- 🛡️ **Safe and validated**: Comprehensive error checking for invalid code
- 📝 **Well-documented**: Full documentation and examples
- 🧪 **Thoroughly tested**: Extensive test coverage including edge cases
- 🎯 **Easy to use**: Simple macro interface

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
brainfuck-macro = "0.1.0"
```

## Quick Start

```rust
use brainfuck_macro::brainfuck;

fn main() {
    // Execute Brainfuck code at compile time
    let hello = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    
    println!("{}", hello); // Prints: Hello World!
}
```

## How It Works

The `brainfuck!` macro takes Brainfuck source code as a string literal, executes it at compile time using a built-in interpreter, and replaces the macro invocation with the resulting output as a static string.

This means:
1. ✅ No runtime Brainfuck interpreter needed
2. ✅ The output is baked into your binary
3. ✅ Compilation fails if the Brainfuck code is invalid
4. ✅ Zero performance cost at runtime

## Brainfuck Language

The macro supports all standard Brainfuck operations:

| Operation | Description |
|-----------|-------------|
| `>` | Move the pointer to the right |
| `<` | Move the pointer to the left |
| `+` | Increment the memory cell at the pointer |
| `-` | Decrement the memory cell at the pointer |
| `.` | Output the character signified by the cell at the pointer |
| `,` | Input a character (⚠️ not supported at compile time) |
| `[` | Jump past the matching `]` if the cell at the pointer is 0 |
| `]` | Jump back to the matching `[` if the cell at the pointer is nonzero |

All other characters are treated as comments and ignored.

## Examples

### Hello World

The classic Brainfuck "Hello World!" program:

```rust
use brainfuck_macro::brainfuck;

let hello = brainfuck!("
    ++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.
    >.+++.------.--------.>+.>.
");

assert_eq!(hello, "Hello World!\n");
```

### Simple Calculations

Use loops for multiplication:

```rust
use brainfuck_macro::brainfuck;

// 5 * 13 = 65 = ASCII 'A'
let letter_a = brainfuck!("
    +++++           Set cell 0 to 5
    [               Loop 5 times
        >+++++++++++++  Add 13 to cell 1
        <-              Decrement cell 0
    ]
    >.              Output cell 1 (65 = 'A')
");

assert_eq!(letter_a, "A");
```

### Comments

You can add comments to make your Brainfuck code more readable:

```rust
use brainfuck_macro::brainfuck;

let result = brainfuck!("
    This is a comment and will be ignored
    +++ Add three to cell 0
    . Output the character
    More comments here!
");
```

### Nested Loops

Complex patterns with nested loops:

```rust
use brainfuck_macro::brainfuck;

let nested = brainfuck!("
    ++++[              Cell 0 = 4
        >++++[         Cell 1 = 4 (per outer loop iteration)
            >++++      Cell 2 += 4 (per inner loop iteration)
            <-         Decrement cell 1
        ]
        <-             Decrement cell 0
    ]
    >>+.               Cell 2 = 65, output 'A'
");

assert_eq!(nested, "A");
```

## Error Handling

The macro provides compile-time errors for invalid Brainfuck code:

### Unmatched Brackets

```rust
// This will fail to compile:
let invalid = brainfuck!("[++");
// Error: Unmatched '[' at position 0
```

### Input Operation

```rust
// This will fail to compile:
let invalid = brainfuck!(",");
// Error: Input operation ',' is not supported at compile time
```

### Pointer Underflow

```rust
// This will fail to compile:
let invalid = brainfuck!("<");
// Error: Pointer moved below zero
```

## Limitations

- **No input operations**: The `,` (input) operation is not supported at compile time
- **Tape size**: Limited to 30,000 cells (standard Brainfuck tape size)
- **Execution limit**: Maximum 1,000,000 steps to prevent infinite loops during compilation
- **Cell values**: Cells are 8-bit unsigned integers (0-255) with wrapping arithmetic

## Running Examples

```bash
# Run the hello world example
cargo run --example hello_world

# Run the advanced examples
cargo run --example advanced
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run only integration tests
cargo test --test integration_tests
```

## Project Structure

```
brainfuck-project/
├── brainfuck-macro/           # The procedural macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs             # Macro implementation and interpreter
├── brainfuck-example/         # Example usage crate
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── examples/
│   │   ├── hello_world.rs     # Basic examples
│   │   └── advanced.rs        # Advanced examples
│   └── tests/
│       └── integration_tests.rs
├── Cargo.toml                 # Workspace configuration
└── README.md
```

## How the Macro Works Internally

1. **Parsing**: The macro receives a string literal containing Brainfuck code
2. **Validation**: Brackets are matched and validated
3. **Execution**: A compile-time interpreter executes the code:
   - Maintains a tape of 30,000 cells
   - Tracks a pointer position
   - Executes each operation
   - Collects output characters
4. **Code Generation**: The output is embedded as a string literal in the compiled binary

## Performance

Since the Brainfuck code is executed at compile time:
- ✅ **Zero runtime overhead** - The result is just a string constant
- ✅ **No interpreter needed** - No Brainfuck interpreter in your binary
- ✅ **Fast execution** - Results are immediately available
- ⚠️ **Compilation time** - Complex Brainfuck programs may increase compile time

## Use Cases

While this is primarily an educational and fun project, potential use cases include:

- 🎓 **Learning**: Understanding procedural macros and compile-time execution
- 🎨 **Code art**: Creating interesting compile-time computed strings
- 🧪 **Testing**: Testing macro and const evaluation capabilities
- 🎮 **Obfuscation**: Embedding strings in an unusual way (though not secure)
- 🏆 **Code golf**: Embedding complex string generation logic compactly

## Contributing

Contributions are welcome! Some ideas:

- Optimize the interpreter for faster compile times
- Add more examples and documentation
- Improve error messages
- Add support for different tape sizes or cell types

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) - Created by Urban Müller in 1993
- The Rust procedural macro system for making this possible

## Learn More

- [Brainfuck on Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
- [Brainfuck Tutorial](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
- [Rust Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)

---

**Made with ❤️ by Philipp**
