use core::mem::MaybeUninit;
use core::ops::{BitAnd, BitOr, Not};

use super::Io;

#[repr(transparent)]
pub struct Mmio<T> {
    value: MaybeUninit<T>,
}

impl<T> Mmio<T> {
    /// # Safety
    ///
    /// This function is unsafe because `base_addr` may be an arbitrary address.
    pub unsafe fn from_base<'a, R>(base_addr: usize) -> &'a mut R {
        &mut *(base_addr as *mut R)
    }
}

impl<T> Io for Mmio<T>
where
    T: Copy + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
{
    type Value = T;

    fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(self.value.as_ptr()) }
    }

    fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.value.as_mut_ptr(), value) };
    }
}
