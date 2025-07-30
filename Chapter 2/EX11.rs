fn main() {
    let integer: i32 = 42;
    let float: f64 = integer as f64; // type casting i32 to f64

    println!("Integer: {}", integer);
    println!("After casting to float: {}", float);

    let character: char = 65 as u8 as char; // casting 65 to u8 and then to char ('A')
    println!("Character from 65: {}", character);
}
