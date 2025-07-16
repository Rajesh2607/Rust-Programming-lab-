fn main() {
    let mut count = 0;
    let mut i = 1;

    while i <= 100 {
        count += 1;
        if i % 13 == 0 {
            break;
        }
        i += 1;
    }

    println!("Total iterations until condition met: {}", count);
}
