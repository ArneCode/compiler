pub fn save_t0() -> String {
    String::from("sb $t0, 0($sp)\naddi $sp, $sp, 1\n")
}
pub fn push_value(value: &str) -> String {
    format!("addi $t0, $zero, {value}\n") + &save_t0()
}
pub fn pop() -> String {
    String::from("lb $t0, -1($sp)\naddi $sp, $sp, -1\n")
}
pub fn pop_two() -> String {
    String::from("lb $t0, -1($sp)\nlb $t1, -2($sp)\naddi $sp, $sp, -2\n")
}
pub fn syscall(code: u32) -> String {
    pop() + &format!("addi $v0, $zero, {code}\nadd $a0, $t0, $zero\nsyscall\n")
}
pub fn load_var(addr: usize) -> String {
    //addr is the offset from base pointer in $t6
    format!("#loading var\naddi $t0, $t6, {addr}\nlw $t0, 0($t0)\n") + &save_t0()
}
pub fn save_var(addr: usize) -> String {
    //addr is the offset from base pointer in $t6
    pop() + &format!("#saving var\naddi $t1, $t6, {addr}\nsb $t0, 0($t1)\n")
}