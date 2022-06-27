use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug)]
pub struct Pers {
    nom: String,
    characteristique: HashMap<u32, BuildingType>
}

#[derive(Debug)]
pub enum Competence {
    Acrobaties,
    Arcanes
}

impl Pers {

    fn new(nom: String) -> Self {

        return Pers {
            nom: nom,
            force: Pers::alea_number(),
            dexterite: Pers::alea_number(),
            constitution: Pers::alea_number(),
            intelligence: Pers::alea_number(),
            sagesse: Pers::alea_number(),
            charisme: Pers::alea_number(),
            competence: vec![]
        };
    }
    
    pub fn new_with_check() -> Self {
    
        let mut joueur = Pers::new(String::from("théo"));
    
        while !joueur.is_valid_char() {
            joueur = Pers::new(String::from("théo"));
        }
        println!("{:?}", joueur.get_force());
        return joueur;
    }

    fn alea_number() -> u8 {

        let mut rng = thread_rng();
    
        let mut alea: [u8; 3] = [rng.gen_range(0, 6), rng.gen_range(0, 6), rng.gen_range(0, 6)];
        // (value, index)
        let mut index_min = (alea[0], 0);
        let number = rng.gen_range(0, 6);
    
        for j in 1..3 {
            if alea[j] < index_min.0 { index_min.0 = alea[j]; index_min.1 = j }
        }

        if index_min.0 < number { alea[index_min.1] = number }
    
        return alea[0] + alea[1] + alea[2];
    }
    
    fn is_valid_char(&self) -> bool {
        
        let mut sum = 0;
    
        sum = self.force + self.dexterite + self.constitution + self.intelligence + self.sagesse + self.charisme;
    
        if sum < 60 || sum > 80 { return false }
        
        return true;
    }

    pub fn get_force(&self) -> (u8, i8) {
        let mut test = HashMap::from([]);
        return (self.force, Pers::get_modificateur(self.force));
    }
    pub fn get_dexterite(&self) -> (u8, i8) {
        return (self.dexterite, Pers::get_modificateur(self.dexterite));
    }
    pub fn get_constitution(&self) -> (u8, i8) {
        return (self.constitution, Pers::get_modificateur(self.constitution));
    }
    pub fn get_intelligence(&self) -> (u8, i8) {
        return (self.intelligence, Pers::get_modificateur(self.intelligence));
    }
    pub fn get_sagesse(&self) -> (u8, i8) {
        return (self.sagesse, Pers::get_modificateur(self.sagesse));
    }
    pub fn get_charisme(&self) -> (u8, i8) {
        return (self.charisme, Pers::get_modificateur(self.charisme));
    }

    fn get_modificateur(value: u8) -> i8 {

        let mut modificateur = 0;

        match value {
            2 | 3 => modificateur = -4,
            4..=9 => modificateur = -3,
            10..=19 => modificateur = 0,
            20 => modificateur = 5,
            _ => println!("error modificateur"),
        }
        return modificateur;
    }
}

impl Index<&str> for Pers {
    type Output = u8;

    fn index(&self, string: &str) -> &u8 {
        match string {
            "force" => { &self.force },
            _ => return &44,
        }
    }
}
/*
"dexterite" => &self.get_dexterite().1,
            "constitution" => &self.get_constitution().1,
            "intelligence" => &self.get_intelligence().1,
            "sagesse" => &self.get_sagesse().1,
            "charisme" => &self.get_charisme().1,*/