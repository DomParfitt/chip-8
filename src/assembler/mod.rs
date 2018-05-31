#[cfg(test)]
mod tests;

pub fn assemble(program: String) -> Vec<u8> {
    let mut assembled_program: Vec<u8> = Vec::new();
    let tokens: Vec<&str> = program.split(" ").collect();
    let mut i = 0;
    while i < tokens.len() {
        let opcode = tokens[i];
        let mut high: u8;
        let mut low: u8;
        match opcode {
            "CLS" => {
                high = 0x00;
                low = 0xE0;
            }
            "RET" => {
                high = 0x00;
                low = 0xEE;
            }
            // "SYS" => {

            // }
            // "JP" => {}
            // "CALL" => {}
            "SE" => {
                i += 1;
                let register_token = tokens[i];
                high = (0x3 << 4) | get_register_number(register_token);
                i += 1;
                if tokens[i].starts_with("V") {
                    low = get_register_number(tokens[i]) << 4;
                } else {
                    low = u8::from_str_radix(tokens[i], 16).unwrap();
                }
            }
            // "SNE" => {}
            // "LD" => {}
            // "ADD" => {}
            // "OR" => {}
            // "AND" => {}
            // "XOR" => {}
            // "SUB" => {}
            // "SHR" => {}
            // "SUBN" => {}
            // "SHL" => {}
            // "RND" => {}
            // "DRW" => {}
            // "SKP" => {}
            // "SKNP" => {}
            _ => {high = 0x00;
                low = 0xE0;}//panic!("Unrecognised token."),
        }

        // let operand = tokens[i + 1];
        assembled_program.push(high);
        assembled_program.push(low);
        i += 1;
    }
    assembled_program
}

fn assemble_opcode(opcode: &str, operand: &str) -> (u8, u8) {
    match opcode {
        "SYS" => {}
        "JP" => {}
        "CALL" => {}
        _ => panic!("Unexpected token"),
    }
    (0, 0)
}

fn get_register_number(token: &str) -> u8 {
    let digit: &str = &token[1..2];
    u8::from_str_radix(digit, 16).unwrap()
}
