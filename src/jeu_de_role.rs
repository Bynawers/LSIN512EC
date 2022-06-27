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
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Competence {
    Acrobaties,
    Arcanes,
    Combat,
    Commandement,
    Volonte
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
    pub fn get_competence(&self) -> Vec<Competence> {
        let mut competence = Vec::new();

        for comp in &self.competence {
            competence.push(comp.clone());
        }
        return competence;
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

    pub fn jet_characteristique(characteristique: (u8, Option<i8>), difficulte: i8) -> bool {

        if characteristique.1 == None {
            println!("Error");
        }

        let mut rng = thread_rng();
        let value = rng.gen_range(0, 20) + characteristique.1.unwrap();

        if value >= difficulte { return true; }
        else { return false; }
    }

    pub fn jet_competence(&self, competence: Competence, difficulte: u8) -> bool {
        
        let mut rng = thread_rng();
        let mut value = rng.gen_range(0, 20);

        if self.competence.contains(&competence) {
            value += Pers::get_bonus_competence(competence);
        }
        if value >= difficulte { return true; }
        else { return false; }
    }

    pub fn add_competence(&mut self, competence: Competence){

        for competence_joueur in &self.competence {

            if competence == *competence_joueur { 

                println!("Competence add! {:?}", competence);

                match competence {
                    Competence::Acrobaties => { 
                        self.dexterite.0 += Pers::get_bonus_competence(Competence::Acrobaties);
                        if self.dexterite.0 > 20 { self.dexterite.0 = 20; }
                    },
                    Competence::Arcanes => { 
                        self.intelligence.0 += Pers::get_bonus_competence(Competence::Arcanes);; 
                        if self.intelligence.0 > 20 { self.intelligence.0 = 20; }
                    },
                    Competence::Combat => {
                        self.force.0 += Pers::get_bonus_competence(Competence::Combat);;
                        if self.force.0 > 20 { self.force.0 = 20; }
                    },
                    Competence::Commandement => {
                        self.charisme.0 += Pers::get_bonus_competence(Competence::Commandement);;
                        if self.charisme.0 > 20 { self.charisme.0 = 20; }
                    }, 
                    Competence::Volonte => {
                        self.sagesse.0 += Pers::get_bonus_competence(Competence::Volonte);;
                        if self.sagesse.0 > 20 { self.sagesse.0 = 20; }
                    },
                }
                return;
            }
        }
        self.competence.push(competence);
    }

    fn get_bonus_competence(competence: Competence) -> u8 {
        let mut bonus = 0;

        match competence {
            Competence::Acrobaties => bonus = 4,
            Competence::Arcanes => bonus = 3,
            Competence::Combat => bonus = 3,
            Competence::Commandement => bonus = 2,
            Competence::Volonte => bonus = 3,
        }
        return bonus;
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