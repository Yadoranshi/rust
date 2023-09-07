use std::io;


fn main() { 
    println!("1. F to C");
    println!("2. C to F");

    let mut conversion_type = String::new();

    io::stdin().read_line(&mut conversion_type)
        .expect("Failed to read");

    let conversion_type = conversion_type.trim();
    let conversion_type = match conversion_type {
        "1" => 1,
        "2" => 2,
        _ => panic!("ii")
    };

    println!("input temp");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read");

    let temperature: f32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("iii")
    };

    let converted_temperature = if conversion_type == 1 {
        (temperature - 32.) / 1.8
    } else {
        temperature * 1.8 + 32.
    };
    println!("The converted temperature is {}", converted_temperature);
}