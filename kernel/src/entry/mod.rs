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
        "mov {0}, eax",
        "mov eax, cr3",
        in(reg) epdr
    );

    let cr0f = CR0_PG | CR0_WP;
    asm!(
        "mov cr0, eax",
        "or eax, {0}",
        "mov eax, cr0",
        in(reg) cr0f
    );

    let stack_bot = &STACK as *const _ as usize;
    let stack_top = stack_bot + KSTACKSIZE;
    asm!(
        "mov {0}, esp",
        in(reg) stack_top
    );

    asm!(
        "mov {}, eax",
        "jmp eax",
        sym kmain
    );
}
