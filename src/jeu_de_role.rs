use rand::{thread_rng, Rng};
use std::ops::Index;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Pers {
    nom: String,
    force: (u8, Option<i8>),
    dexterite: (u8, Option<i8>),
    constitution: (u8, Option<i8>),
    intelligence: (u8, Option<i8>),
    sagesse: (u8, Option<i8>),
    charisme: (u8, Option<i8>),
    competence: Vec<Competence>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Competence {
    Acrobaties,
    Arcanes
}

impl Pers {

    fn new(nom: String) -> Self {

        return Pers {
            nom: nom,
            force: (Pers::alea_number(), None),
            dexterite: (Pers::alea_number(), None),
            constitution: (Pers::alea_number(), None),
            intelligence: (Pers::alea_number(), None),
            sagesse: (Pers::alea_number(), None),
            charisme: (Pers::alea_number(), None),
            competence: vec![]
        };  
    }
    
    pub fn new_with_check() -> Self {
    
        let mut joueur = Pers::new(String::from("théo"));
    
        while !joueur.is_valid_char() {
            joueur = Pers::new(String::from("théo"));
        }
        joueur.set_modificateur();
        return joueur;
    }

    fn set_modificateur(&mut self) {
        self.force.1 = Some(Pers::get_modificateur(self.force.0));
        self.dexterite.1 = Some(Pers::get_modificateur(self.dexterite.0));
        self.constitution.1 = Some(Pers::get_modificateur(self.constitution.0));
        self.intelligence.1 = Some(Pers::get_modificateur(self.intelligence.0));
        self.sagesse.1 = Some(Pers::get_modificateur(self.sagesse.0));
        self.charisme.1 = Some(Pers::get_modificateur(self.charisme.0));
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

        let sum = self.force.0 + self.dexterite.0 + self.constitution.0 + self.intelligence.0 + self.sagesse.0 + self.charisme.0;
    
        if sum < 60 || sum > 80 { return false }
        return true;
    }

    pub fn get_force(&self) -> (u8, Option<i8>) {
        return self.force;
    }
    pub fn get_dexterite(&self) -> (u8, Option<i8>) {
        return self.dexterite;
    }
    pub fn get_constitution(&self) -> (u8, Option<i8>) {
        return self.constitution;
    }
    pub fn get_intelligence(&self) -> (u8, Option<i8>) {
        return self.intelligence;
    }
    pub fn get_sagesse(&self) -> (u8, Option<i8>) {
        return self.sagesse;
    }
    pub fn get_charisme(&self) -> (u8, Option<i8>) {
        return self.charisme;
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
    type Output = i8;

    fn index(&self, string: &str) -> &i8 {
        match string {
            "force" => { match self.force.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            "dexterite" => { match self.dexterite.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            "constitution" => { match self.constitution.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            "intelligence" => { match self.intelligence.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            "sagesse" => { match self.sagesse.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            "charisme" => { match self.charisme.1.as_ref() {
                Some(value) => return &value,
                None => { println!("error no value"); return &0 },
            } },
            _ => { println!("error no value"); return &0; },
        }
    }
}