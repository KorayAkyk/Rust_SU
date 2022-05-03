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

fn somme(values: Vec<u32>) -> u32 {
    // values.iter().sum()
    let mut res = 0;
    for v in values {
        res += v;
    }
    res
}

fn somme_with_iter_sum(values: &Vec<u32>) -> u32 {
    values.iter().sum()
}

fn maximum(values: Vec<u32>) -> Option<u32> {
    // values.iter().max()
    let mut res = None;
    for v in values {
        if res.is_none() || v > res.unwrap() {
            res = Some(v);
        }
    }
    res
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

    //Vec
    let vecteur = vec![1,2,3,4];
    println!("{:?}", vecteur);

    for i in vecteur{
        println!("{:?}",i);
    }

    let mut v2 = vec![3, 7, 4, 7, 8, 9];
    println!("{:?}",v2);

    v2.push(5);
    v2.push(6);
    v2.push(7);

    println!("{:?}", v2);

    println!("Somme : {}", somme(v2.clone()));
    println!("Somme iter sum : {}", somme_with_iter_sum(&v2));

    println!("Maximum : {:?}", maximum(v2.clone()));
    
}
