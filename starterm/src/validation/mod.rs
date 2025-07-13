//! Defines validation logic for user inputs, particularly for interactive
//! workflows like MCQs.

/// Represents a validation rule for an input.
pub enum ValidationRule {
    NotEmpty,
    IsNumber,
    Regex(String),
    // Add more complex rules as needed
}

/// Validates a given input string against a set of rules.
pub fn validate(input: &str, rules: &[ValidationRule]) -> Result<(), String> {
    for rule in rules {
        match rule {
            ValidationRule::NotEmpty if input.is_empty() => {
                return Err("Input cannot be empty.".to_string());
            }
            ValidationRule::IsNumber if input.parse::<f64>().is_err() => {
                return Err("Input must be a number.".to_string());
            }
            // TODO: Implement regex validation using the `regex` crate.
            _ => {}
        }
    }
    Ok(())
} 