fn main() {
    println!("Input the measurement to convert from, e.g. 32F, 22.3C: ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let (measurement, unit) = parse_measurement_input(&input);

    match unit {
        'C' => {
            println!(
                "{}{} is {}{}",
                measurement,
                unit,
                celsius_to_fahrenheit(&measurement),
                'F'
            );
        }
        'F' => {
            println!(
                "{}{} is {}{}",
                measurement,
                unit,
                fahrenheit_to_celsius(&measurement),
                'C'
            );
        }
        _ => panic!("Invalid unit: {unit}"),
    };
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    let before_trunc = temp * 1.8 + 32.0;

    (before_trunc * 10.0).floor() / 10.0
}

fn fahrenheit_to_celsius(temp: &f64) -> f64 {
    let before_trunc = (temp - 32.0) / 1.8;

    (before_trunc * 10.0).floor() / 10.0
}

fn parse_measurement_input(input: &String) -> (f64, char) {
    let (measurement, unit) = input.split_at(input.len() - 2);

    let measurement = match measurement.parse::<f64>() {
        Ok(measurement) => measurement,
        Err(_) => match measurement.parse::<i32>() {
            Ok(measurement) => measurement as f64,
            Err(e) => {
                panic!("Failed to read numerical value from input: {e}");
            }
        },
    };

    let unit: char = unit
        .trim_end()
        .to_ascii_uppercase()
        .parse()
        .expect("Failed to read unit from input");

    (measurement, unit)
}
