//rust string
fn isbig(_s: &String, _n: usize) -> bool {
    let lasoeurdeclement = true;
    lasoeurdeclement
} 
fn inverse(v: f32) -> f32{
    1.0 / v
}

fn safe_inverse(v: f32) -> Option<f32>{
    if v == 0.0 {
        None
    } else {
        Some(1.0 / v)
    }
}

fn main() {
    //string
    let ricard = String::from("ricard!");
    let vodka = String::from("vodka!");
    let biere = String::from("biere!");
    let rhum = String::from("rhum!");


    println!("{}",ricard);
    println!("{}",vodka);
    println!("{}",biere);
    println!("{}",rhum);

    let name = "guillaume".to_string();
    println!("{}", isbig(&name, 10));
    println!("{}", isbig(&name, 5));
    
    println!("{}",isbig(&"guillaume".to_string(), 10) == false);

    //option
    let v = Some(10);

    match v {
        Some(n) => println!("Pas vide : {}", n),
        None => println!("Vide"),
    }

    println!("Inverse de 2 : {}", inverse(2.0));
    println!("Inverse de 1 : {}", inverse(1.0));
    println!("Inverse de -1 : {}", inverse(-1.0));
    println!("Inverse de 0 : {}", inverse(0.0));

    println!("Inverse de 2 : {:?}", safe_inverse(2.0));
    println!("Inverse de 1 : {:?}", safe_inverse(1.0));
    println!("Inverse de -1 : {:?}", safe_inverse(-1.0));
    println!("Inverse de 0 : {:?}", safe_inverse(0.0));
}
