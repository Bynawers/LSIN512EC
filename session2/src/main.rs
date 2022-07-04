mod probleme1;
mod probleme2;
mod probleme3;

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_with_accessor() {
        let l1 = probleme1::Longueur::new(1.0, probleme1::Unite::M);
        assert_eq!(l1.get_unit(), probleme1::Unite::M);
        assert_eq!(l1.get_value(), 1.0);
    }
    #[test]
    fn conversion() {
        let mut l1 = probleme1::Longueur::new(12.0, probleme1::Unite::M);
        l1.convert(&probleme1::Unite::Cm);
        assert_eq!(l1.get_value(), 1200.0);
        assert_eq!(l1.get_unit(), probleme1::Unite::Cm);
        l1.convert(&probleme1::Unite::Dm);
        assert_eq!(l1.get_value(), 120.0);
        assert_eq!(l1.get_unit(), probleme1::Unite::Dm);
    }
    #[test]
    fn addition() {
        let mut l1 = probleme1::Longueur::new(12.0, probleme1::Unite::M);
        let mut l2 = probleme1::Longueur::new(10.0, probleme1::Unite::Dm);
        let result = l1.addition(&mut l2, probleme1::Unite::Cm);
        assert_eq!(result, 1300.0);
    }

    #[test]
    fn execute() {
        let ruban1 = vec![
            probleme2::Commande::Echange(1, 2),
            probleme2::Commande::Incremente(3),
            probleme2::Commande::Echange(3, 1),
            probleme2::Commande::Ecrete(3)
        ];
        let mut ruban2 = vec![0, 1, 2, 3, 4];

        probleme2::execute(&ruban1, &mut ruban2);
        assert_eq!(ruban2[0], 0);
        assert_eq!(ruban2[1], 3);
        assert_eq!(ruban2[2], 1);
        assert_eq!(ruban2[3], 2);
        assert_eq!(ruban2[4], 3);
        probleme2::execute(&ruban1, &mut ruban2);
        assert_eq!(ruban2[0], 0);
        assert_eq!(ruban2[1], 3);
        assert_eq!(ruban2[2], 3);
        assert_eq!(ruban2[3], 1);
        assert_eq!(ruban2[4], 3);
    }
}
