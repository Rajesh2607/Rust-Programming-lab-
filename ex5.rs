fn main() {
    let a = 5; // Compiler infers as i32
    let b = 4.5; // Inferred as f64
    println!("Implicit: a = {}, b = {}", a, b);

    // b. Explicit type
    let x: i32 = 100;
    let y: f32 = 10.5;
    println!("Explicit: x = {}, y = {}", x, y);
    
}
//Created by RajeshLingala