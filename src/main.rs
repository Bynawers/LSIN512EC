mod tortue_logo;
mod algebre_lineaire;
mod jeu_de_role;

fn main() {

    tortue_logo::start();
    algebre_lineaire::position(2, 2, 3);
    println!("joueur: {:?}", jeu_de_role::new_with_check());
}