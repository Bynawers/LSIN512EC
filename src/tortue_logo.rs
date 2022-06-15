/* Question 1 */
enum Instruction {
    Avance(i32),
    Tourne,
}

pub fn start(){

    let programme = vec![
        Instruction::Avance(10),
        Instruction::Tourne,
        Instruction::Avance(5),
        Instruction::Tourne,
        Instruction::Avance(15)];

        let position = execute_logo(&programme);

        println!("Coordonnée : {:?}", position);
}

fn execute_logo(programme: &Vec<Instruction>) -> (i32, i32) {

    /* (x, y) */
    let mut position = (0, 0);

    /* 0: droite, 1: haut, 2: gauche, 3: bas */
    let mut rotation = 0;

    for instruction in programme.iter() {
        
        match instruction {

            Instruction::Tourne => {
                rotation += 1;
                if rotation == 4 { rotation = 0; }
            },
            Instruction::Avance(value) => {
                match rotation {
                    0 => position.0 += value,
                    1 => position.1 += value,
                    2 => position.0 -= value,
                    3 => position.1 -= value,
                    _ => println!("error"),
                }
            },
        }
    }
    return position;
}