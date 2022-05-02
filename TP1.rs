fn max(a: i32, b: i32) -> i32 {
    if a > b {
        println!("{}",a);
        return a;
    }
    println!("{}",b);
    return b;
}
fn bigger(a: i32, b: i32) -> i32 {
    println!("the value is {}", if a > b { a } else { b });
    0
}
/*
fn mad(a:i32, b:i32, c:i32) -> i32{
    println!("the result : ", a * b + c)
}
//println!("{}",mad(12,92,52))
*/
fn main() {

    println!("{}", bigger(21, 32));
    let a = [1, 2, 3, 4, 5];
    let sum: u8 = a.iter().sum();
    println!("the total sum is: {}", sum);
}