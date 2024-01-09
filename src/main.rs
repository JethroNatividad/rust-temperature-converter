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

enum TemperatureConvertion {
    Celcius_Fahrenheit = 0,
    Fahrenheit_Celcius = 1
}

fn convert_temperature(temperature: f64, convertion: TemperatureConvertion ) -> f64 {
    match convertion {
        TemperatureConvertion::Celcius_Fahrenheit => {
            round_decimal(((temperature - 32.0) * 5.0) / 9.0, 2)
        },
        TemperatureConvertion::Fahrenheit_Celcius => {
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
        assert_eq!(fahrenheit_to_celcius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celcius(18.0), -7.78);
        assert_eq!(fahrenheit_to_celcius(100.0), 37.78);
        assert_eq!(fahrenheit_to_celcius(0.0), -17.78);
    }

    #[test]
    fn test_celcius_to_fahrenheit() {
        assert_eq!(celcius_to_fahrenheit(32.0), 89.6);
        assert_eq!(celcius_to_fahrenheit(18.0), 64.4);
        assert_eq!(celcius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celcius_to_fahrenheit(0.0), 32.0);
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

fn main() {
    // Prompt for conversion_type, "Press C to convert from Fahrenheit to Celsius.\nPress F to convert from Celsius to Fahrenheit.\nYour choice: "
    let input_conversion_type: String = get_input("Press C to convert from Fahrenheit to Celsius.\nPress F to convert from Celsius to Fahrenheit.\nYour choice: ");
    // if C
        // prompt for fahrenheit_temperature, "Please enter the temperature in Fahrenheit: "
        // convert to celcius
        // display, "The temperature in Celsius is {}"
    // if F
        // prompt for celcius_temperature, "Please enter the temperature in Celcius: "
        // convert to fahrenheit
        // display, "The temperature in Fahrenheit is {}"
    match input_conversion_type.to_lowercase().as_str() {
        "c" => {
            let temperature: f64 = get_input("Please enter the temperature in Fahrenheit: ");
            let converted: f64 = fahrenheit_to_celcius(temperature);
            println!("The temperature in Celsius is {}", converted);
        },
        "f" => {
            let temperature: f64 = get_input("Please enter the temperature in Celcius: ");
            let converted: f64 = celcius_to_fahrenheit(temperature);
            println!("The temperature in Fahrenheit is {}", converted);
        },
        _ => {
            println!("Invalid type.");
        }
    }
    
} 
