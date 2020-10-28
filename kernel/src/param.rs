/// The kernel's stack size.
pub const KSTACKSIZE: usize = 4096;

/// First kernel virtual address.
pub const KERNBASE: usize = 0x80000000;

// Offset of PDX in a linear address.
pub const PDXSHIFT: usize = 22;
