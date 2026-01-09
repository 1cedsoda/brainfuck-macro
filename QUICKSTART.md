# Quick Start Guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
brainfuck-macro = { path = "brainfuck-macro" }
```

Or if published to crates.io:

```toml
[dependencies]
brainfuck-macro = "0.1.0"
```

## Basic Usage

```rust
use brainfuck_macro::brainfuck;

fn main() {
    // Hello World
    let hello = brainfuck!("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
    println!("{}", hello);
    
    // Simple 'A'
    let a = brainfuck!("+++++[>+++++++++++++<-]>.");
    println!("{}", a);
}
```

## Running the Project

```bash
# Clone or navigate to the project
cd brainfuck-project

# Build
cargo build --all

# Run tests
cargo test --all

# Run examples
cargo run --example hello_world
cargo run --example advanced
```

## Common Patterns

### Multiplication
```rust
// 5 * 13 = 65 = 'A'
let a = brainfuck!("+++++[>+++++++++++++<-]>.");
```

### Sequential Output
```rust
// ABCDE
let letters = brainfuck!("+++++[>+++++++++++++<-]>.+.+.+.+.");
```

### With Comments
```rust
let result = brainfuck!("
    Set cell to 65
    +++++[>+++++++++++++<-]>
    
    Output it
    .
");
```

## Testing Your Code

```bash
# Run all tests
cargo test --all

# Run specific tests
cargo test test_hello_world

# Run with output
cargo test -- --nocapture
```

## Next Steps

1. Check out `examples/hello_world.rs` for more examples
2. Read the full `README.md` for detailed documentation
3. Explore `brainfuck-macro/src/lib.rs` for implementation details
4. Write your own Brainfuck programs!

## Troubleshooting

### Compile Error: Unmatched Brackets
Make sure all `[` have matching `]`:
```rust
// Wrong
brainfuck!("[++");

// Right
brainfuck!("[++]");
```

### Compile Error: Input Not Supported
Remove `,` operations:
```rust
// Wrong
brainfuck!(",");

// Right - use hardcoded values instead
brainfuck!("+++.");
```

### Compile Error: Pointer Underflow
Don't move left from position 0:
```rust
// Wrong
brainfuck!("<");

// Right
brainfuck!(">.<");
```

## Resources

- [Brainfuck on Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
- [Brainfuck Tutorial](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)
- [Project README](README.md)
- [Implementation Details](PROJECT_SUMMARY.md)
