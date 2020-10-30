use crate::asm::{CR0_PG, CR0_WP, CR4_PSE};
use crate::param::KSTACKSIZE;
use crate::stack::STACK;

use crate::kmain;

mod entrypgdir;

#[no_mangle]
pub unsafe extern "C" fn _start() {
    asm!(
        "mov    eax, cr4",
        "or     eax, {CR4_PSE}",
        "mov    cr4, eax",
        CR4_PSE = const CR4_PSE,
    );

    let epdr = &entrypgdir::ENTRYPGDIR as *const _ as usize;
    asm!(
        "mov eax, {0}",
        "mov cr3, eax",
        in(reg) epdr
    );

    let cr0f = (CR0_PG | CR0_WP) as u32;
    asm!(
        "mov eax, cr0",
        "or eax, {0}",
        "mov cr0, eax",
        in(reg) cr0f
    );

    let stack_bot = &STACK as *const _ as usize;
    let stack_top = stack_bot + KSTACKSIZE;
    asm!(
        "mov esp, {0}",
        in(reg) stack_top
    );

    asm!(
        "mov eax, {0}",
        "jmp eax",
        sym kmain
    );
}
