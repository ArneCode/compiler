pub fn save_t0() -> String {
    String::from("sb $t0, ($sp)\naddi $sp, $sp, 1\n")
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
