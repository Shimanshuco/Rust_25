// advanced_examples.rs - More advanced Rust concepts

use std::collections::HashMap;

/// Example of using Option<T> for handling nullable values
pub fn find_word_length(words: &[&str], word: &str) -> Option<usize> {
    words.iter()
        .find(|&&w| w == word)
        .map(|w| w.len())
}

/// Example of using Result<T, E> for error handling
pub fn divide_numbers(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Example of working with collections (HashMap)
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    
    counts
}

/// Example of a generic function
pub fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

/// Example of a struct with methods
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Example of an enum with associated values
#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn process(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_length() {
        let words = ["hello", "world", "rust", "programming"];
        assert_eq!(find_word_length(&words, "rust"), Some(4));
        assert_eq!(find_word_length(&words, "python"), None);
    }

    #[test]
    fn test_divide_numbers() {
        assert_eq!(divide_numbers(10.0, 2.0), Ok(5.0));
        assert!(divide_numbers(10.0, 0.0).is_err());
    }

    #[test]
    fn test_count_words() {
        let text = "hello world hello rust";
        let counts = count_words(text);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&[1, 3, 2, 5, 4]), Some(5));
        assert_eq!(find_largest(&[-1, -3, -2]), Some(-1));
        assert_eq!(find_largest::<i32>(&[]), None);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
        assert_eq!(rect.perimeter(), 60.0);

        let small_rect = Rectangle::new(5.0, 10.0);
        assert!(rect.can_hold(&small_rect));
        assert!(!small_rect.can_hold(&rect));
    }
}