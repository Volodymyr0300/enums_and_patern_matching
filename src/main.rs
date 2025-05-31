use std::char;

fn main() {
    let some_u8_value = Some(0u8);
    
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let tup (u8, i32, f64, string, char, array, tup) = (0u8, 0i32, 0f64, String::new(), 'a', [0u8; 10], (0u8, 0i32));
    println!("tup: {:?}", tup);
}