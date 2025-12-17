#[cfg(not(feature = "pmr"))]
use core::arch::asm;

#[cfg(not(feature = "pmr"))]
#[inline]
pub fn local_irq_save_and_disable() -> usize {
    let flags: usize;
    // save `DAIF` flags, mask `I` bit (disable IRQs)
    unsafe { asm!("mrs {}, daif; msr daifset, #2", out(reg) flags) };
    flags
}

#[cfg(not(feature = "pmr"))]
#[inline]
pub fn local_irq_restore(flags: usize) {
    unsafe { asm!("msr daif, {}", in(reg) flags) };
}

/// Only can be used on aarch64 qemu virt now
#[cfg(feature = "pmr")]
const GICC_PMR: usize = 0xffff_0000_0800_0004;

#[cfg(feature = "pmr")]
#[inline]
pub fn local_irq_save_and_disable() -> usize {
    let pmr = unsafe {core::ptr::read_volatile((GICC_PMR) as *const u32) as u8};
    unsafe {
        core::ptr::write_volatile((GICC_PMR) as *mut u32, 0x80u32);
    }
    pmr as usize
}

#[cfg(feature = "pmr")]
#[inline]
pub fn local_irq_restore(flags: usize) {
    unsafe {
        core::ptr::write_volatile((GICC_PMR) as *mut u32, flags as u8 as u32);
    }
}