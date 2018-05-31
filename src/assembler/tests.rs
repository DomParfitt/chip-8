use assembler::*;

#[test]
pub fn test() {
    let program = assemble("SE V0, V1".to_string());
    println!("{:X?}", program);
}