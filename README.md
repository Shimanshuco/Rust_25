# Rust_25 - Your Rust Learning Journey 🦀

Welcome to Rust_25! This repository is designed to help you learn Rust programming through hands-on examples and exercises.

## Prerequisites

Make sure you have Rust installed on your system:
- Install Rust from [rustup.rs](https://rustup.rs/)
- Verify installation: `rustc --version`

## Getting Started

1. **Clone this repository** (if you haven't already):
   ```bash
   git clone https://github.com/Shimanshuco/Rust_25.git
   cd Rust_25
   ```

2. **Run the basic examples**:
   ```bash
   cargo run
   ```

3. **Run the tests** (for exercises):
   ```bash
   cargo test
   ```

## What's Included

### 📖 Learning Examples (`src/main.rs`)
- **Variables and Data Types**: Learn about mutability, different data types
- **Functions**: How to define and use functions in Rust
- **Control Flow**: if-else statements, loops, and iteration
- **Ownership Basics**: Understanding Rust's unique ownership system

### 🏋️ Practice Exercises (`src/exercises.rs`)
Hands-on coding challenges to reinforce your learning:

1. **Basic Calculator**: Implement arithmetic operations with error handling
2. **String Manipulation**: Work with strings and text processing
3. **Vector Operations**: Learn about collections and finding elements
4. **Struct and Methods**: Object-oriented programming in Rust
5. **Error Handling**: Working with `Result` types and error management

### 🧪 Testing
All exercises come with unit tests. You can:
- Run all tests: `cargo test`
- Run specific test: `cargo test test_calculator`
- See test output: `cargo test -- --nocapture`

## Learning Path

1. **Start with the examples**: Run `cargo run` to see Rust concepts in action
2. **Try the exercises**: Open `src/exercises.rs` and replace `todo!()` with your implementations
3. **Test your solutions**: Use `cargo test` to verify your code works
4. **Experiment**: Modify the examples and create your own code

## Useful Commands

```bash
# Run the main program
cargo run

# Build the project (check for compilation errors)
cargo build

# Run tests
cargo test

# Check code without building
cargo check

# Format your code
cargo fmt

# Run clippy (Rust linter) for suggestions
cargo clippy
```

## Key Rust Concepts Covered

- **Variables and Mutability**: `let` vs `let mut`
- **Data Types**: integers, floats, booleans, characters, strings
- **Functions**: parameters, return values, expressions vs statements
- **Control Flow**: `if`, `loop`, `for`, `while`
- **Ownership**: moving, borrowing, references
- **Structs**: defining custom types and methods
- **Error Handling**: `Option` and `Result` types
- **Pattern Matching**: `match` statements
- **Collections**: vectors and basic operations

## Next Steps

After completing these exercises, consider exploring:
- Advanced ownership concepts (lifetimes)
- Traits and generics
- Error handling with custom error types
- Working with external crates
- Building larger projects

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - The official Rust programming guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust with examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises to get you used to reading and writing Rust code

Happy coding! 🎉