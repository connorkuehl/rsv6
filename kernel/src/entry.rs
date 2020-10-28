use crate::asm::CR4_PSE;

use crate::kmain;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    asm!(
        "mov    eax, cr4",
        "or     eax, {CR4_PSE}",
        "mov    cr4, eax",
        CR4_PSE = const CR4_PSE,
    );

    asm!("jmp {}", sym kmain);
}
