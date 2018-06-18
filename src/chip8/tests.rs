use chip8::Chip8;


#[test]
fn test_call() {
    let mut cpu = init_cpu_with_program(vec!(0x22, 0x22));
    cpu.emulate_cycle();
    assert_eq!(cpu.pc, 0x222);
}

#[test]
fn test_call_and_return() {
    let mut cpu = init_cpu_with_program(vec!(0x22, 0x04, 0x00, 0x00, 0x00, 0xEE));
    cpu.emulate_cycle();
    cpu.emulate_cycle();
    println!("Stack: {:X?}", cpu.stack);
    println!("PC: {:X}", cpu.pc);
    assert_eq!(cpu.pc, 0x202);
}

fn init_cpu_with_program(program: Vec<u8>) -> Chip8 {
    let mut cpu = Chip8::new();
    cpu.load_program(program);
    cpu
}