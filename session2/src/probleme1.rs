use std::ops::Index;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Unite {
    M,
    Dm,
    Cm
}

#[derive(Debug)]
pub struct Longueur {
    value: f32,
    unit: Unite
}

impl Longueur {

    pub fn new(value: f32, unit: Unite) -> Self {

        return Longueur { 
            value: value,
            unit: unit
        };
    }

    pub fn get_value(&self) -> f32 {
        return self.value;
    }
    pub fn get_unit(&self) -> Unite {
        match self.unit {
            Unite::M => return Unite::M,
            Unite::Dm => return Unite::Dm,
            Unite::Cm => return Unite::Cm
        }
    }

    pub fn convert(&mut self, to: &Unite) {

        match to {
            Unite::M => {
                if self.unit == Unite::Cm { self.value = self.value / 100.0; self.unit = Unite::M; }
                else if self.unit == Unite::Dm { self.value = self.value / 10.0; self.unit = Unite::M; }
            }
            Unite::Dm => {
                if self.unit == Unite::Cm { self.value = self.value / 10.0; self.unit = Unite::Dm; }
                else if self.unit == Unite::M { self.value = self.value * 10.0; self.unit = Unite::Dm; }
            }
            Unite::Cm => {
                if self.unit == Unite::M { self.value = self.value * 100.0; self.unit = Unite::Cm; }
                else if self.unit == Unite::Dm { self.value = self.value * 10.0; self.unit = Unite::Cm; }
            }
        }
    }
    pub fn addition(&mut self, l1: &mut Longueur, unit_res: Unite) -> f32 {
        self.convert(&unit_res);
        l1.convert(&unit_res);
        self.value = self.value + l1.value;
        return self.value;
    }
}  

impl Index<Unite> for Longueur {
    type Output = f32;

    fn index(&self, to: Unite) -> &f32 {

        if self.unit == to { return &self.value; }

        match to {
            Unite::M => {
                if self.unit == Unite::Cm { return &self.value; }
                else if self.unit == Unite::Dm { return &self.value; }
            }
            Unite::Dm => {
                if self.unit == Unite::Cm { return &self.value; }
                else if self.unit == Unite::M { return &self.value; }
            }
            Unite::Cm => {
                if self.unit == Unite::M { return &self.value; }
                else if self.unit == Unite::Dm { return &self.value; }
            }
        }
        return &0.0;
    }
}