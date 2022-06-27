mod tortue_logo;
mod algebre_lineaire;
mod jeu_de_role;

fn main() {

    //tortue_logo::start();
    //println!("{}", algebre_lineaire::position(3, 3, 3));

    let joueur = jeu_de_role::Pers::new_with_check();
    println!("joueur: {:?}", joueur);
    println!("index force {:?}", joueur["force"]);
    
}