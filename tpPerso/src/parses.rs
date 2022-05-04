
fn _main1() {
    let a = [1, 2, 3, 4, 5];

    let premier = a[0];
    let second = a[1];
    println!("{}", premier);
    println!("{}", second);
}



fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let slice = &vector[3..6];
    println!("length of slice: {}", slice.len()); // 3
    println!("slice: {:?}", slice); // [4, 5, 6]
    println!("All: {:?}", vector); // [4, 5, 6]
    for i in vector{
        println!("{:?}",i);
    }
}
