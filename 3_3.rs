fn main() {
    let a = 10.0;
    let b = 5.0;
    let operator = '+';

    let result = match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b != 0.0 {
                a / b
            } else {
                println!("Cannot divide by zero");
                return;
            }
        },
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
}

// created by Rajesh lingala