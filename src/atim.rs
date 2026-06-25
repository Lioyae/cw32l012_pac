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
    rcr: Rcr,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    bdtr: Bdtr,
    ccr5: Ccr5,
    ccr6: Ccr6,
    _reserved_20_ccmr: [u8; 0x04],
    dtr2: Dtr2,
    ecr: Ecr,
    tisel1: Tisel1,
    af1: Af1,
    af2: Af2,
    _reserved26: [u8; 0x04],
    tisel2: Tisel2,
    icr: Icr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Control Register2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Slave Mode Control Register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    #[doc = "0x0c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - Event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &Egr {
        &self.egr
    }
    #[doc = "0x18 - capture compare mode register 1"]
    #[inline(always)]
    pub const fn ccmr1cmp(&self) -> &Ccmr1cmp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture compare mode register 1"]
    #[inline(always)]
    pub const fn ccmr1cap(&self) -> &Ccmr1cap {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - capture compare mode register 2"]
    #[inline(always)]
    pub const fn ccmr2cmp(&self) -> &Ccmr2cmp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture compare mode register 2"]
    #[inline(always)]
    pub const fn ccmr2cap(&self) -> &Ccmr2cap {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - capture compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    #[doc = "0x24 - Counter Register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - prescaler Register"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - Auto reload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x30 - Repetition Counter Register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x34 - capture compare register1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x38 - capture compare register2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x3c - capture compare register3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x40 - capture compare register4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(&self) -> &Bdtr {
        &self.bdtr
    }
    #[doc = "0x48 - capture compare register5"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    #[doc = "0x4c - capture compare register6"]
    #[inline(always)]
    pub const fn ccr6(&self) -> &Ccr6 {
        &self.ccr6
    }
    #[doc = "0x50 - capture compare mode register 3"]
    #[inline(always)]
    pub const fn ccmr3cmp(&self) -> &Ccmr3cmp {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - capture compare mode register 3"]
    #[inline(always)]
    pub const fn ccmr3cap(&self) -> &Ccmr3cap {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - dead-time register2"]
    #[inline(always)]
    pub const fn dtr2(&self) -> &Dtr2 {
        &self.dtr2
    }
    #[doc = "0x58 - Encoder control Register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x5c - Timer Input Select Register1"]
    #[inline(always)]
    pub const fn tisel1(&self) -> &Tisel1 {
        &self.tisel1
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
    #[doc = "0x6c - Timer Input Select Register2"]
    #[inline(always)]
    pub const fn tisel2(&self) -> &Tisel2 {
        &self.tisel2
    }
    #[doc = "0x70 - Interrupt clean register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
}
#[doc = "CR1 (rw) register accessor: Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control Register1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control Register2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: Slave Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SmcrSpec>;
#[doc = "Slave Mode Control Register"]
pub mod smcr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "EGR (rw) register accessor: Event generation register\n\nYou can [`read`](crate::Reg::read) this register and get [`egr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EgrSpec>;
#[doc = "Event generation register"]
pub mod egr;
#[doc = "CCMR1CAP (rw) register accessor: capture compare mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1cap`] module"]
#[doc(alias = "CCMR1CAP")]
pub type Ccmr1cap = crate::Reg<ccmr1cap::Ccmr1capSpec>;
#[doc = "capture compare mode register 1"]
pub mod ccmr1cap;
#[doc = "CCMR1CMP (rw) register accessor: capture compare mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1cmp`] module"]
#[doc(alias = "CCMR1CMP")]
pub type Ccmr1cmp = crate::Reg<ccmr1cmp::Ccmr1cmpSpec>;
#[doc = "capture compare mode register 1"]
pub mod ccmr1cmp;
#[doc = "CCMR2CAP (rw) register accessor: capture compare mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2cap`] module"]
#[doc(alias = "CCMR2CAP")]
pub type Ccmr2cap = crate::Reg<ccmr2cap::Ccmr2capSpec>;
#[doc = "capture compare mode register 2"]
pub mod ccmr2cap;
#[doc = "CCMR2CMP (rw) register accessor: capture compare mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2cmp`] module"]
#[doc(alias = "CCMR2CMP")]
pub type Ccmr2cmp = crate::Reg<ccmr2cmp::Ccmr2cmpSpec>;
#[doc = "capture compare mode register 2"]
pub mod ccmr2cmp;
#[doc = "CCMR3CAP (rw) register accessor: capture compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3cap`] module"]
#[doc(alias = "CCMR3CAP")]
pub type Ccmr3cap = crate::Reg<ccmr3cap::Ccmr3capSpec>;
#[doc = "capture compare mode register 3"]
pub mod ccmr3cap;
#[doc = "CCMR3CMP (rw) register accessor: capture compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3cmp`] module"]
#[doc(alias = "CCMR3CMP")]
pub type Ccmr3cmp = crate::Reg<ccmr3cmp::Ccmr3cmpSpec>;
#[doc = "capture compare mode register 3"]
pub mod ccmr3cmp;
#[doc = "CCER (rw) register accessor: capture compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CcerSpec>;
#[doc = "capture compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter Register"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "prescaler Register"]
pub mod psc;
#[doc = "ARR (rw) register accessor: Auto reload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Auto reload Register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: Repetition Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "Repetition Counter Register"]
pub mod rcr;
#[doc = "CCR1 (rw) register accessor: capture compare register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "capture compare register1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: capture compare register2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "capture compare register2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: capture compare register3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "capture compare register3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: capture compare register4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "capture compare register4"]
pub mod ccr4;
#[doc = "CCR5 (rw) register accessor: capture compare register5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`] module"]
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::Ccr5Spec>;
#[doc = "capture compare register5"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: capture compare register6\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr6`] module"]
#[doc(alias = "CCR6")]
pub type Ccr6 = crate::Reg<ccr6::Ccr6Spec>;
#[doc = "capture compare register6"]
pub mod ccr6;
#[doc = "BDTR (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtr`] module"]
#[doc(alias = "BDTR")]
pub type Bdtr = crate::Reg<bdtr::BdtrSpec>;
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DTR2 (rw) register accessor: dead-time register2\n\nYou can [`read`](crate::Reg::read) this register and get [`dtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtr2`] module"]
#[doc(alias = "DTR2")]
pub type Dtr2 = crate::Reg<dtr2::Dtr2Spec>;
#[doc = "dead-time register2"]
pub mod dtr2;
#[doc = "ECR (rw) register accessor: Encoder control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Encoder control Register"]
pub mod ecr;
#[doc = "TISEL1 (rw) register accessor: Timer Input Select Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel1`] module"]
#[doc(alias = "TISEL1")]
pub type Tisel1 = crate::Reg<tisel1::Tisel1Spec>;
#[doc = "Timer Input Select Register1"]
pub mod tisel1;
#[doc = "TISEL2 (rw) register accessor: Timer Input Select Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tisel2`] module"]
#[doc(alias = "TISEL2")]
pub type Tisel2 = crate::Reg<tisel2::Tisel2Spec>;
#[doc = "Timer Input Select Register2"]
pub mod tisel2;
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
#[doc = "ICR (rw) register accessor: Interrupt clean register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt clean register"]
pub mod icr;
