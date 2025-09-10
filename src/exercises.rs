// exercises.rs - Practice exercises for Rust learners

/// Exercise 1: Basic Calculator
/// TODO: Implement a function that takes two numbers and an operation (+, -, *, /)
/// and returns the result
pub fn calculator(a: f64, b: f64, operation: char) -> Option<f64> {
    // Your code here
    // Hint: Use a match statement to handle different operations
    // Return None for invalid operations or division by zero
    todo!("Implement the calculator function")
}

/// Exercise 2: String Manipulation
/// TODO: Implement a function that takes a string and returns it in reverse
pub fn reverse_string(input: &str) -> String {
    // Your code here
    // Hint: You can collect characters into a Vec and reverse it
    todo!("Implement the reverse_string function")
}

/// Exercise 3: Vector Operations
/// TODO: Implement a function that finds the maximum value in a vector
pub fn find_max(numbers: &[i32]) -> Option<i32> {
    // Your code here
    // Hint: Handle the case when the vector is empty
    todo!("Implement the find_max function")
}

/// Exercise 4: Struct and Implementation
/// TODO: Define a Person struct with name and age fields
/// TODO: Implement a method that returns a greeting message
pub struct Person {
    // Define fields here
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        // Your code here
        todo!("Implement the new function")
    }

    pub fn greet(&self) -> String {
        // Your code here
        todo!("Implement the greet method")
    }
}

/// Exercise 5: Error Handling
/// TODO: Implement a function that parses a string to an integer
/// Return a Result type - Ok(number) if successful, Err(message) if failed
pub fn parse_number(input: &str) -> Result<i32, String> {
    // Your code here
    // Hint: Use the parse() method and handle the error
    todo!("Implement the parse_number function")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        assert_eq!(calculator(5.0, 3.0, '+'), Some(8.0));
        assert_eq!(calculator(5.0, 3.0, '-'), Some(2.0));
        assert_eq!(calculator(5.0, 3.0, '*'), Some(15.0));
        assert_eq!(calculator(6.0, 3.0, '/'), Some(2.0));
        assert_eq!(calculator(5.0, 0.0, '/'), None);
        assert_eq!(calculator(5.0, 3.0, '%'), None);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 3, 2, 5, 4]), Some(5));
        assert_eq!(find_max(&[]), None);
        assert_eq!(find_max(&[-1, -3, -2]), Some(-1));
    }

    #[test]
    fn test_person() {
        let person = Person::new("Alice".to_string(), 25);
        assert!(person.greet().contains("Alice"));
        assert!(person.greet().contains("25"));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(parse_number("-10"), Ok(-10));
        assert!(parse_number("not_a_number").is_err());
        assert!(parse_number("").is_err());
    }
}