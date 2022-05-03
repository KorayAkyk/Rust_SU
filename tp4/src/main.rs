enum Instruction{
    let code = vec![Plus, Plus, Moins, Moins, Droite, Gauche, Affiche];
}

fn interpreteur(memoire: &mut vec<i32>, instructions: &vec<Instruction>){

}

fn main() {
    
    let mem = vec![0, 0, 0];
    let instructions = vec![Plus, Plus, Plus, Droite, Plus, Droite, Moins];
    interpreteur(&mem, &instructions);
 
    println!("{:?}", mem);

}
