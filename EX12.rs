
fn main() {
    let x = 10;
    let y = &x; // Borrowing x (reference to x)

    println!("Value of x: {}", x);
    println!("Value of y (reference to x): {}", y);

    let z = *y; // Dereferencing y to get the value it points to
    println!("Value of z (dereferenced y): {}", z);
}
//Created by RajeshLingala