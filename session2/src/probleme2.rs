pub enum Commande {
    Echange(usize, usize),
    Incremente(usize),
    Ecrete(i32)
}

pub fn execute(ruban1: &Vec<Commande>, ruban2: &mut Vec<i32>) {

    for instruction in ruban1 {

        match instruction {
            
            Commande::Echange(x, y) => {
                let index = (*x as usize, *y as usize);
                let left = ruban2[index.0].clone();
                let right = ruban2[index.1].clone();

                ruban2[index.0] = right;
                ruban2[index.1] = left;
            },

            Commande::Incremente(incr_index) => {
                let index = *incr_index as usize;
                ruban2[index] += 1;
            },

            Commande::Ecrete(max) => {
                for index in 0..=4 {
                    if ruban2[index] > *max { ruban2[index] = *max; }
                }
                
            }
        }
    }
}

/* Question 1 */
// Il n'est pas necessaire de rajouter le qualificatif
// mut devant ruban1 car on ne modifie pas ses valeurs contraiement
// au ruban 2

/* Question 2 */
// etape 0 : (0, 3, 1, 2, 3)
// etape 1 : Echange(1, 2) => (0, 1, 3, 2, 3)
// etape 2 : Incremente(3) => (0, 1, 3, 3, 3)
// etape 3 : Echange(3, 1) => (0, 3, 3, 1, 3)
// etape 4 : Ecrete(3) => (0, 3, 3, 1, 3)