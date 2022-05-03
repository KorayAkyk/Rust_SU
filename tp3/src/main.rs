
fn isbig(_s: &String, _n: usize) -> bool {
    let lasoeurdeclement = true;
    lasoeurdeclement
} 

fn main() {
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
    
}
