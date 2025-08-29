fn test_divisibility_by_3_4(n: i32) -> i32 {
    let by3 = n % 3 == 0;
    let by4 = n % 4 == 0;
    match (by3, by4) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => -1,
    }
}

fn main() {
    for n in [12, 9, 8, 7] {
        println!("{n} -> {}", test_divisibility_by_3_4(n));
    }
}
