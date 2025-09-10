// Import the exercises module
mod exercises;
mod advanced_examples;

fn main() {
    // Welcome to Rust Learning!
    println!("Welcome to Rust_25 - Your Rust Learning Journey!");
    println!();

    // Basic variables and data types
    variables_and_types();
    println!();

    // Functions
    functions_example();
    println!();

    // Control flow
    control_flow();
    println!();

    // Ownership basics
    ownership_basics();
    println!();

    // Learning exercises
    println!("=== Practice Exercises ===");
    println!("Check out src/exercises.rs for hands-on practice!");
    println!("Run 'cargo test' to test your solutions.");
    println!();

    // Advanced examples
    advanced_examples_demo();
}

fn advanced_examples_demo() {
    println!("=== Advanced Examples ===");
    
    // Option example
    let words = ["hello", "world", "rust", "programming"];
    match advanced_examples::find_word_length(&words, "rust") {
        Some(length) => println!("The word 'rust' has {} letters", length),
        None => println!("Word not found"),
    }

    // Result example
    match advanced_examples::divide_numbers(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // HashMap example
    let text = "hello world hello rust programming";
    let word_counts = advanced_examples::count_words(text);
    println!("Word counts: {:?}", word_counts);

    // Generic function example
    let numbers = [1, 5, 3, 9, 2];
    if let Some(largest) = advanced_examples::find_largest(&numbers) {
        println!("Largest number: {}", largest);
    }

    // Struct example
    let rect = advanced_examples::Rectangle::new(10.0, 20.0);
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());

    // Enum example
    let messages = [
        advanced_examples::Message::Move { x: 10, y: 20 },
        advanced_examples::Message::Write("Hello, Rust!".to_string()),
        advanced_examples::Message::ChangeColor(255, 0, 0),
        advanced_examples::Message::Quit,
    ];

    println!("Processing messages:");
    for message in &messages {
        message.process();
    }
}

// Example 1: Variables and Data Types
fn variables_and_types() {
    println!("=== Variables and Data Types ===");
    
    // Immutable variable (default)
    let x = 5;
    println!("Immutable x: {}", x);
    
    // Mutable variable
    let mut y = 10;
    println!("Mutable y before: {}", y);
    y = 15;
    println!("Mutable y after: {}", y);
    
    // Different data types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '🦀';
    let string: String = String::from("Hello, Rust!");
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("String: {}", string);
}

// Example 2: Functions
fn functions_example() {
    println!("=== Functions ===");
    
    let sum = add_numbers(10, 20);
    println!("Sum of 10 and 20: {}", sum);
    
    let greeting = create_greeting("Rust Learner");
    println!("{}", greeting);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // No semicolon - this is the return value
}

fn create_greeting(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust!", name)
}

// Example 3: Control Flow
fn control_flow() {
    println!("=== Control Flow ===");
    
    // If-else
    let number = 6;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
    
    // Loop with break
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Loop count reached: {}", count);
            break;
        }
    }
    
    // For loop
    println!("Counting from 1 to 5:");
    for i in 1..=5 {
        println!("  {}", i);
    }
}

// Example 4: Basic Ownership Concepts
fn ownership_basics() {
    println!("=== Ownership Basics ===");
    
    // String ownership
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Clone to avoid move
    println!("s1: {}, s2: {}", s1, s2);
    
    // Borrowing
    let message = String::from("Rust is awesome!");
    print_message(&message); // Borrowing with &
    println!("Original message still available: {}", message);
}

fn print_message(msg: &String) {
    println!("Borrowed message: {}", msg);
}
