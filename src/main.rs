// Program to convert celcius to fahrenheit both ways.

// Inputs: Fahrenheit to celcius or celcius to fahrenheit, temperature.
// Process: convert temp
// Output: The temperature in {Celsius | Fahrenheit} is {}

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
}

fn main() {
    println!("Hello, world!");
}
