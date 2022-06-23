use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Pers {
    nom: String,
    force: u8,
    dexterite: u8,
    constitution: u8,
    intelligence: u8,
    sagesse: u8,
    charisme: u8,
    competence: Vec<Competence>
}

#[derive(Debug)]
pub enum Competence{
    Acrobaties,
    Arcanes
}

fn alea_number() -> u8 {

    let mut rng = thread_rng();

    let mut alea: [u8; 3] = [0, 0, 0];
    let mut number = 0;

    for _ in 0..4 {
        number = rng.gen_range(0, 6);

        for j in 0..3 {
            if alea[j] < number { alea[j] = number; }
        }
        return 0;
    }

    return alea[0] + alea[1] + alea[2];
}

pub fn new(nom: String) -> Pers {

    return Pers {
        nom: nom,
        force: alea_number(),
        dexterite: alea_number(),
        constitution: alea_number(),
        intelligence: alea_number(),
        sagesse: alea_number(),
        charisme: alea_number(),
        competence: vec![]
    };
}

pub fn is_valid_char(joueur: &Pers) -> bool {

    let mut sum = 0;

    sum = joueur.force + joueur.dexterite + joueur.constitution + joueur.intelligence + joueur.sagesse + joueur.charisme;

    if sum < 60 || sum > 80 { return false }
    
    return true;
}

pub fn new_with_check() -> Pers {

    let mut joueur = new(String::from("théo"));

    while !is_valid_char(&joueur) {
        joueur = new(String::from("théo"));
    }

    return joueur;

}