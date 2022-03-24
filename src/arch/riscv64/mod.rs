use core::arch::{global_asm, asm};

mod context;

pub use context::*;

global_asm!(include_str!("switch.S"));
global_asm!(include_str!("executor_entry.S"));

extern "C" {
    pub fn switch(old: *const ContextData, new: *const ContextData);
    pub fn executor_entry();
}

pub(crate) fn cpu_id() -> u8 {
    let mut cpu_id;
    unsafe {
        asm!("mv {0}, tp", out(reg) cpu_id, options(nomem, nostack));
    }
    cpu_id
}

pub(crate) fn pg_base_addr() -> usize {
    riscv::register::satp::read().ppn() << 12
}

pub(crate) fn pg_base_register() -> usize {
    riscv::register::satp::read().bits()
}

use riscv::{asm, register::sstatus};

pub(crate) fn wait_for_interrupt() {
    // interrupt disable?
    unsafe { asm::wfi() };
}

pub(crate) fn intr_on() {
    unsafe { sstatus::set_sie() };
}

pub(crate) fn intr_off() {
    unsafe { sstatus::clear_sie() };
}

pub(crate) fn intr_get() -> bool {
    sstatus::read().sie()
}