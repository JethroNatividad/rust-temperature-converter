use std::io;
use std::io::Write;

// Program to convert celcius to fahrenheit both ways.

// Inputs: Fahrenheit to celcius or celcius to fahrenheit, temperature.
// Process: convert temp
// Output: The temperature in {Celsius | Fahrenheit} is {}

fn round_decimal(number: f64, place: i32) -> f64 {
    let multiplier: f64 = 10_f64.powi(place);
    (number * multiplier).round() / multiplier
}

enum ConversionType {
    CelciusToFahrenheit = 0,
    Fahrenheit_Celcius = 1
}

fn convert_temperature(temperature: f64, conversion_type: ConversionType ) -> f64 {
    match conversion_type {
        ConversionType::CelciusToFahrenheit => {
            round_decimal(((temperature - 32.0) * 5.0) / 9.0, 2)
        },
        ConversionType::Fahrenheit_Celcius => {
            round_decimal(((temperature * 9.0) / 5.0) + 32.0, 2)
        },
        _ => 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fahrenheit_to_celcius() {
        assert_eq!(convert_temperature(32.0, ConversionType::CelciusToFahrenheit), 0.0);
        assert_eq!(convert_temperature(18.0, ConversionType::CelciusToFahrenheit), -7.78);
        assert_eq!(convert_temperature(100.0, ConversionType::CelciusToFahrenheit), 37.78);
        assert_eq!(convert_temperature(0.0, ConversionType::CelciusToFahrenheit), -17.78);
    }

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert_eq!(convert_temperature(32.0, ConversionType::Fahrenheit_Celcius), 89.6);
        assert_eq!(convert_temperature(18.0, ConversionType::Fahrenheit_Celcius), 64.4);
        assert_eq!(convert_temperature(100.0, ConversionType::Fahrenheit_Celcius), 212.0);
        assert_eq!(convert_temperature(0.0, ConversionType::Fahrenheit_Celcius), 32.0);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn get_conversion_type() -> ConversionType {
    loop {
        let input_conversion_type: String = get_input("Press C to convert from Fahrenheit to Celsius.\nPress F to convert from Celsius to Fahrenheit.\nYour choice: ");
        match input_conversion_type.to_lowercase().as_str() {
            "c" => break ConversionType::Fahrenheit_Celcius,
            "f" => break ConversionType::CelciusToFahrenheit,
            _ => println!("Invalid input. Please try again.")
        }
    }
}

fn main() {
    // Prompt for conversion_type, "Press C to convert from Fahrenheit to Celsius.\nPress F to convert from Celsius to Fahrenheit.\nYour choice: "
    let conversion_type: ConversionType = get_conversion_type();
    // if C
        // prompt for fahrenheit_temperature, "Please enter the temperature in Fahrenheit: "
        // convert to celcius
        // display, "The temperature in Celsius is {}"
    // if F
        // prompt for celcius_temperature, "Please enter the temperature in Celcius: "
        // convert to fahrenheit
        // display, "The temperature in Fahrenheit is {}"
    let from_conversion: &str = match conversion_type {
        ConversionType::Fahrenheit_Celcius => "Fahrenheit",
        ConversionType::CelciusToFahrenheit => "Celcius",
    }

    let to_conversion: &str = match conversion_type {
        ConversionType::Fahrenheit_Celcius => "Celcius",
        ConversionType::CelciusToFahrenheit => "Fahrenheit",
    }

    let temperature: f64 = get_input("Please enter the temperature in {}: ", from_conversion);
    let converted: f64 = fahrenheit_to_celcius(temperature);
    println!("The temperature in {} is {}", to_conversion, converted);
    
} 
