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
}

fn bigger_while(a: i32, b: i32) -> i32 {
    println!("the value is {}", if a > b { a } else { b });
}

fn main() {
    //1 fonction max entre 2 chiffres
    println!("{}",max(2,5));

    //2 focntion max entre 2 chiffres
    println!("{}", bigger(21, 32));

    //fonction sum entre une liste de chiffres
    let a = [1, 2, 3, 4, 5];
    let sum: u8 = a.iter().sum();
    println!("the total sum is: {}", sum);

    println!("boucle while bigger one{}", bigger_while(15, 35));
}