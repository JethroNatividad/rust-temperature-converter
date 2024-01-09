// Program to convert celcius to fahrenheit both ways.

// Inputs: Fahrenheit to celcius or celcius to fahrenheit, temperature.
// Process: convert temp
// Output: The temperature in {Celsius | Fahrenheit} is {}

fn fahrenheit_to_celcius(temperature: f64) -> f64 {
    // (F − 32) × 5 / 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celcius() {
        assert_eq!(fahrenheit_to_celcius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celcius(18.0), -7.78);
        assert_eq!(fahrenheit_to_celcius(100.0), 37.8);
        assert_eq!(fahrenheit_to_celcius(0.0), -17.8);
    }

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert_eq!(celcius_to_fahrenheit(32.0), 89.6);
        assert_eq!(celcius_to_fahrenheit(18.0), 64.4);
        assert_eq!(celcius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celcius_to_fahrenheit(0.0), 32.0);
    }
}

fn main() {
    println!("Hello, world!");
}
