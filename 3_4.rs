fn main() {
    let some_value = Some(10);

    if let Some(x) = some_value {
        println!("Matched! The value is {}", x);
    } else {
        println!("No match found");
    }
}


// created by Rajesh lingala