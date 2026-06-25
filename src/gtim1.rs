#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    ier: Ier,
    isr: Isr,
    egr: Egr,
    _reserved_6_ccmr: [u8; 0x04],
    _reserved_7_ccmr: [u8; 0x04],
    ccer: Ccer,
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
    _reserved12: [u8; 0x04],
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    _reserved16: [u8; 0x14],
    ecr: Ecr,
    tisel: Tisel,
    af1: Af1,
    af2: Af2,
    _reserved20: [u8; 0x08],
    icr: Icr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Control register2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Slave Mode control register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    #[doc = "0x0c - Interrupt control register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - Event Generate register"]
    #[inline(always)]
    pub const fn egr(&self) -> &Egr {
        &self.egr
    }
    #[doc = "0x18 - Compare control Register1"]
    #[inline(always)]
    pub const fn ccmr1cmp(&self) -> &Ccmr1cmp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Capture control Register1"]
    #[inline(always)]
    pub const fn ccmr1cap(&self) -> &Ccmr1cap {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - Compare control Register2"]
    #[inline(always)]
    pub const fn ccmr2cmp(&self) -> &Ccmr2cmp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Capture control Register2"]
    #[inline(always)]
    pub const fn ccmr2cap(&self) -> &Ccmr2cap {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Capture compare Enable Register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    #[doc = "0x24 - Counter Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - Prescaler Control Register"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - Auto Reload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x34 - Capture control Register1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x38 - Capture control Register2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x3c - Capture control Register3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x40 - Capture control Register4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x58 - Encoder control Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x5c - Timer Input Select Register"]
    #[inline(always)]
    pub const fn tisel(&self) -> &Tisel {
        &self.tisel
    }
    #[doc = "0x60 - Alternate function Register1"]
    #[inline(always)]
    pub const fn af1(&self) -> &Af1 {
        &self.af1
    }
    #[doc = "0x64 - Alternate function Register2"]
    #[inline(always)]
    pub const fn af2(&self) -> &Af2 {
        &self.af2
    }
    #[doc = "0x70 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
}
#[doc = "CR1 (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control register1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control register2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: Slave Mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SmcrSpec>;
#[doc = "Slave Mode control register"]
pub mod smcr;
#[doc = "IER (rw) register accessor: Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt control register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "EGR (rw) register accessor: Event Generate register\n\nYou can [`read`](crate::Reg::read) this register and get [`egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EgrSpec>;
#[doc = "Event Generate register"]
pub mod egr;
#[doc = "CCMR1CAP (rw) register accessor: Capture control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1cap`] module"]
#[doc(alias = "CCMR1CAP")]
pub type Ccmr1cap = crate::Reg<ccmr1cap::Ccmr1capSpec>;
#[doc = "Capture control Register1"]
pub mod ccmr1cap;
#[doc = "CCMR1CMP (rw) register accessor: Compare control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1cmp`] module"]
#[doc(alias = "CCMR1CMP")]
pub type Ccmr1cmp = crate::Reg<ccmr1cmp::Ccmr1cmpSpec>;
#[doc = "Compare control Register1"]
pub mod ccmr1cmp;
#[doc = "CCMR2CAP (rw) register accessor: Capture control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2cap`] module"]
#[doc(alias = "CCMR2CAP")]
pub type Ccmr2cap = crate::Reg<ccmr2cap::Ccmr2capSpec>;
#[doc = "Capture control Register2"]
pub mod ccmr2cap;
#[doc = "CCMR2CMP (rw) register accessor: Compare control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2cmp`] module"]
#[doc(alias = "CCMR2CMP")]
pub type Ccmr2cmp = crate::Reg<ccmr2cmp::Ccmr2cmpSpec>;
#[doc = "Compare control Register2"]
pub mod ccmr2cmp;
#[doc = "CCER (rw) register accessor: Capture compare Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CcerSpec>;
#[doc = "Capture compare Enable Register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: Prescaler Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "Prescaler Control Register"]
pub mod psc;
#[doc = "ARR (rw) register accessor: Auto Reload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Auto Reload Register"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: Capture control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "Capture control Register1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: Capture control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "Capture control Register2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: Capture control Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "Capture control Register3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: Capture control Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "Capture control Register4"]
pub mod ccr4;
#[doc = "ECR (rw) register accessor: Encoder control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Encoder control Register"]
pub mod ecr;
#[doc = "TISEL (rw) register accessor: Timer Input Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel`] module"]
#[doc(alias = "TISEL")]
pub type Tisel = crate::Reg<tisel::TiselSpec>;
#[doc = "Timer Input Select Register"]
pub mod tisel;
#[doc = "AF1 (rw) register accessor: Alternate function Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af1`] module"]
#[doc(alias = "AF1")]
pub type Af1 = crate::Reg<af1::Af1Spec>;
#[doc = "Alternate function Register1"]
pub mod af1;
#[doc = "AF2 (rw) register accessor: Alternate function Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@af2`] module"]
#[doc(alias = "AF2")]
pub type Af2 = crate::Reg<af2::Af2Spec>;
#[doc = "Alternate function Register2"]
pub mod af2;
