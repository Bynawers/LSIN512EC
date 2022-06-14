enum Instruction {
    Avance(i32),
    Tourne,
}

let programme = vec![
    Instruction::Avance(10),
    Instruction::Tourne(),
    Instruction::Avance(5),
    Instruction::Tourne(),
    Instruction::Avance(15)];

pub fn test(){
    println!("test function");
}