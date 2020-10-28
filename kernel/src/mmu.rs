/// The number of bytes mapped by a page.
pub const PGSIZE: usize = 4096;

/// The number of entries in a page directory.
pub const NPDENTRIES: usize = 1024;

/// Present.
pub const PTE_P: usize = 0x001;

/// Writable.
pub const PTE_W: usize = 0x002;

/// User.
pub const PTE_U: usize = 0x004;

/// Page size.
pub const PTE_PS: usize = 0x080;
