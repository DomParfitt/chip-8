use chip8::Chip8;

#[test]
fn test_call_and_return() {
    let mut cpu = Chip8::new();
    cpu.load_program(vec!(0x22, 0x22, 0x00, 0xEE));

}