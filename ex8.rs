fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // a. Slice of 2nd and 3rd element (index 1 and 2)
    let slice1 = &arr[1..3];
    println!("Slice 2nd and 3rd: {:?}", slice1);

    // b. Omit start index (start from 0)
    let slice2 = &arr[..3];
    println!("Omit start index: {:?}", slice2);

    // c. Omit end index (go till end)
    let slice3 = &arr[7..];
    println!("Omit end index: {:?}", slice3);

    // d. Omit both start and end index (entire array)
    let slice4 = &arr[..];
    println!("Omit both: {:?}", slice4);
    
}
//Created by RajeshLingala