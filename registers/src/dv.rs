// Licensed under the Apache-2.0 license.
//
// generated by caliptra_registers_generator with caliptra-rtl repo at 67563371b8208a8e6720fa6c33456c37914d858d
//
#![allow(clippy::erasing_op)]
#![allow(clippy::identity_op)]
/// A zero-sized type that represents ownership of this
/// peripheral, used to get access to a Register lock. Most
/// programs create one of these in unsafe code near the top of
/// main(), and pass it to the driver responsible for managing
/// all access to the hardware.
pub struct DvReg {
    _priv: (),
}
impl DvReg {
    pub const PTR: *mut u32 = 0x1001c000 as *mut u32;
    /// # Safety
    ///
    /// Caller must ensure that all concurrent use of this
    /// peripheral in the firmware is done so in a compatible
    /// way. The simplest way to enforce this is to only call
    /// this function once.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self { _priv: () }
    }
    /// Returns a register block that can be used to read
    /// registers from this peripheral, but cannot write.
    #[inline(always)]
    pub fn regs(&self) -> RegisterBlock<ureg::RealMmio> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
    /// Return a register block that can be used to read and
    /// write this peripheral's registers.
    #[inline(always)]
    pub fn regs_mut(&mut self) -> RegisterBlock<ureg::RealMmioMut> {
        RegisterBlock {
            ptr: Self::PTR,
            mmio: core::default::Default::default(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct RegisterBlock<TMmio: ureg::Mmio + core::borrow::Borrow<TMmio>> {
    ptr: *mut u32,
    mmio: TMmio,
}
impl<TMmio: ureg::Mmio + core::default::Default> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new(ptr: *mut u32) -> Self {
        Self {
            ptr,
            mmio: core::default::Default::default(),
        }
    }
}
impl<TMmio: ureg::Mmio> RegisterBlock<TMmio> {
    /// # Safety
    ///
    /// The caller is responsible for ensuring that ptr is valid for
    /// volatile reads and writes at any of the offsets in this register
    /// block.
    #[inline(always)]
    pub unsafe fn new_with_mmio(ptr: *mut u32, mmio: TMmio) -> Self {
        Self { ptr, mmio }
    }
    /// Controls for the Sticky Data Vault Entries
    ///
    /// Read value: [`dv::regs::StickydatavaultctrlReadVal`]; Write value: [`dv::regs::StickydatavaultctrlWriteVal`]
    #[inline(always)]
    pub fn sticky_data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Stickydatavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn sticky_data_vault_entry(
        &self,
    ) -> ureg::Array<10, ureg::Array<12, ureg::RegRef<crate::dv::meta::StickyDataVaultEntry, &TMmio>>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x28 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Controls for the Non-Sticky Data Vault Entries
    ///
    /// Read value: [`dv::regs::StickydatavaultctrlReadVal`]; Write value: [`dv::regs::StickydatavaultctrlWriteVal`]
    #[inline(always)]
    pub fn non_sticky_data_vault_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickydatavaultctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x208 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn nonsticky_data_vault_entry(
        &self,
    ) -> ureg::Array<
        10,
        ureg::Array<12, ureg::RegRef<crate::dv::meta::NonstickyDataVaultEntry, &TMmio>>,
    > {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x230 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Non-Sticky Scratch Register Controls
    ///
    /// Read value: [`dv::regs::StickylockablescratchregctrlReadVal`]; Write value: [`dv::regs::StickylockablescratchregctrlWriteVal`]
    #[inline(always)]
    pub fn non_sticky_lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickylockablescratchregctrl, &TMmio>>
    {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x410 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn non_sticky_lockable_scratch_reg(
        &self,
    ) -> ureg::Array<10, ureg::RegRef<crate::dv::meta::Nonstickylockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x438 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn non_sticky_generic_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Nonstickygenericscratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x460 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Sticky Scratch Register Controls
    ///
    /// Read value: [`dv::regs::StickylockablescratchregctrlReadVal`]; Write value: [`dv::regs::StickylockablescratchregctrlWriteVal`]
    #[inline(always)]
    pub fn sticky_lockable_scratch_reg_ctrl(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchregctrl, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x480 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
    /// Read value: [`u32`]; Write value: [`u32`]
    #[inline(always)]
    pub fn sticky_lockable_scratch_reg(
        &self,
    ) -> ureg::Array<8, ureg::RegRef<crate::dv::meta::Stickylockablescratchreg, &TMmio>> {
        unsafe {
            ureg::Array::new_with_mmio(
                self.ptr.wrapping_add(0x4a0 / core::mem::size_of::<u32>()),
                core::borrow::Borrow::borrow(&self.mmio),
            )
        }
    }
}
pub mod regs {
    //! Types that represent the values held by registers.
    #[derive(Clone, Copy)]
    pub struct StickydatavaultctrlReadVal(u32);
    impl StickydatavaultctrlReadVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> StickydatavaultctrlWriteVal {
            StickydatavaultctrlWriteVal(self.0)
        }
    }
    impl From<u32> for StickydatavaultctrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickydatavaultctrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: StickydatavaultctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickydatavaultctrlWriteVal(u32);
    impl StickydatavaultctrlWriteVal {
        /// Lock writes to this entry. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for StickydatavaultctrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickydatavaultctrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: StickydatavaultctrlWriteVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickylockablescratchregctrlReadVal(u32);
    impl StickylockablescratchregctrlReadVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(&self) -> bool {
            ((self.0 >> 0) & 1) != 0
        }
        /// Construct a WriteVal that can be used to modify the contents of this register value.
        #[inline(always)]
        pub fn modify(self) -> StickylockablescratchregctrlWriteVal {
            StickylockablescratchregctrlWriteVal(self.0)
        }
    }
    impl From<u32> for StickylockablescratchregctrlReadVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickylockablescratchregctrlReadVal> for u32 {
        #[inline(always)]
        fn from(val: StickylockablescratchregctrlReadVal) -> u32 {
            val.0
        }
    }
    #[derive(Clone, Copy)]
    pub struct StickylockablescratchregctrlWriteVal(u32);
    impl StickylockablescratchregctrlWriteVal {
        /// Lock writes to the Scratch registers. Writes will be suppressed when locked.
        #[inline(always)]
        pub fn lock_entry(self, val: bool) -> Self {
            Self((self.0 & !(1 << 0)) | (u32::from(val) << 0))
        }
    }
    impl From<u32> for StickylockablescratchregctrlWriteVal {
        #[inline(always)]
        fn from(val: u32) -> Self {
            Self(val)
        }
    }
    impl From<StickylockablescratchregctrlWriteVal> for u32 {
        #[inline(always)]
        fn from(val: StickylockablescratchregctrlWriteVal) -> u32 {
            val.0
        }
    }
}
pub mod enums {
    //! Enumerations used by some register fields.
    pub mod selector {}
}
pub mod meta {
    //! Additional metadata needed by ureg.
    pub type Stickydatavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickydatavaultctrlReadVal,
        crate::dv::regs::StickydatavaultctrlWriteVal,
    >;
    pub type StickyDataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickydatavaultctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickydatavaultctrlReadVal,
        crate::dv::regs::StickydatavaultctrlWriteVal,
    >;
    pub type NonstickyDataVaultEntry = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickylockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickylockablescratchregctrlReadVal,
        crate::dv::regs::StickylockablescratchregctrlWriteVal,
    >;
    pub type Nonstickylockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Nonstickygenericscratchreg = ureg::ReadWriteReg32<0, u32, u32>;
    pub type Stickylockablescratchregctrl = ureg::ReadWriteReg32<
        0,
        crate::dv::regs::StickylockablescratchregctrlReadVal,
        crate::dv::regs::StickylockablescratchregctrlWriteVal,
    >;
    pub type Stickylockablescratchreg = ureg::ReadWriteReg32<0, u32, u32>;
}
