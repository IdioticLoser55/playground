use std::io;

fn main() {
    println!("Please enter the unit you would like to convert from (C/F): ");

    let stdin_handle = io::stdin();
    let mut user_input = String::new();
    let _ = stdin_handle.read_line(&mut user_input).expect("Please enter a string.");

    let initial_unit = loop {
        user_input = user_input.trim().to_lowercase();
        if user_input == "c" || user_input == "f" {
            break user_input;
        }

        println!("Please enter the unit you would like to convert from (C/F): ");
        user_input.clear();
        let _ = stdin_handle.read_line(&mut user_input).expect("Please enter a string.");
    };

    println!("Initial_Unit: {initial_unit}");

    let conversion_unit = if initial_unit == "c" { "f" } else { "c" };
    println!("Conversion unit: {conversion_unit}");


    println!("Please enter the temperature to convert:");
    user_input = String::new();
    let _ = stdin_handle.read_line(&mut user_input).expect("Please enter a string.");

    let temperature: f64 = loop {
        match user_input.trim().parse() {
            Ok(v) => break v,
            _ => {},
        }

        println!("Please enter a number.");
        user_input.clear();
        let _ = stdin_handle.read_line(&mut user_input).expect("Please enter a string.");
    };

    println!("Temperature: {temperature}");

    let converted_temperature = convert_temp(&temperature, &initial_unit);
    println!("Converted Temperature: {converted_temperature}")
}

fn convert_temp(temperature: &f64, unit: &str) -> f64 {
    match unit {
        "c" => temperature * 9.0 / 5.0 + 32.0,
        "f" => (temperature - 32.0) * 5.0 / 9.0,
        _ => panic!("\"{unit}\" is not a unit, please use \"f\" or \"c\""),
    }

}
