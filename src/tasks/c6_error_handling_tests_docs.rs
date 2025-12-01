// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    text.chars().next().ok_or_else(|| "Empty string".into())
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|_| "Invalid number".into()))
        .collect()
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        self.email
            .as_ref()
            .and_then(|e| e.split_once("@"))
            .map(|(_, domain)| domain.into())
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.

fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::factorial;

    #[test]
    fn test_factorial_logic() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(120, factorial(5));
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.

fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::is_prime;

    #[test]
    fn test_valid_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(101));
    }

    #[test]
    fn test_non_primes() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));

        assert!(!is_prime(4));
        assert!(!is_prime(6));

        assert!(!is_prime(9));
        assert!(!is_prime(15));
        assert!(!is_prime(25));
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// Stores a city name and a list of daily temperature readings.
///
/// This struct allows you to collect multiple temperature data points for a specific location
/// and perform calculations on them, such as finding the average temperature.
///
/// # Examples
///
/// ```
/// // Assuming the struct is in scope
/// let mut log = TemperatureLog::new("London");
///
/// log.add_reading(19.5);
/// log.add_reading(21.0);
/// log.add_reading(22.5);
///
/// if let Some(avg) = log.average() {
///     println!("The average temperature in {} is {:.1}", log.city, avg);
/// }
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// The name of the city associated with these readings.
    pub city: String,
    /// A vector storing individual temperature recordings as floating-point numbers.
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new `TemperatureLog` instance for the specified city.
    ///
    /// # Arguments
    ///
    /// * `city` - A string slice that holds the name of the city.
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Adds a new temperature reading to the log.
    ///
    /// # Arguments
    ///
    /// * `value` - The temperature value to be recorded.
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Calculates the average value of the recorded readings.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` containing the mean temperature if readings exist.
    /// * `None` if the readings list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut log = TemperatureLog::new("Paris");
    ///
    /// // Case 1: No readings
    /// assert_eq!(log.average(), None);
    ///
    /// // Case 2: With readings
    /// log.add_reading(10.0);
    /// log.add_reading(20.0);
    /// assert_eq!(log.average(), Some(15.0));
    /// ```
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}
