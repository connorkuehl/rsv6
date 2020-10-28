use crate::kmain;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    asm!("jmp {}", sym kmain);
}
