mod tortue_logo;
mod algebre_lineaire;
mod jeu_de_role;

fn main() {

    jeu_de_role::Pers::new_with_check();
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tortue_logo() {
        let programme = vec![
            tortue_logo::Instruction::Avance(10),
            tortue_logo::Instruction::Tourne,
            tortue_logo::Instruction::Avance(5),
            tortue_logo::Instruction::Tourne,
            tortue_logo::Instruction::Avance(15)];

        let position = tortue_logo::execute_logo(&programme);

        assert_eq!(position, (-5, 5));
    }

    #[test]
    fn algebre_lineaire() {
        let a = algebre_lineaire::position(3, 3, 3);
        let b = algebre_lineaire::position(2, 2, 3);
        let c = algebre_lineaire::position(1, 1, 3);

        assert_eq!(a, 8);
        assert_eq!(b, 4);
        assert_eq!(c, 0);
    }

    #[test]
    fn jeu_de_role_trait_index() {
        let joueur = jeu_de_role::Pers::new_with_check();
        assert_eq!(joueur["force"], joueur.get_force().1.unwrap());
        assert_eq!(joueur["dexterite"], joueur.get_dexterite().1.unwrap());
        assert_eq!(joueur["constitution"], joueur.get_constitution().1.unwrap());
        assert_eq!(joueur["intelligence"], joueur.get_intelligence().1.unwrap());
        assert_eq!(joueur["sagesse"], joueur.get_sagesse().1.unwrap());
        assert_eq!(joueur["charisme"], joueur.get_charisme().1.unwrap());
    }
}