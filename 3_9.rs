fn main() {
    let collection = vec![10, 20, 30, 40, 50];

    let mut iterator = collection.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }
}
