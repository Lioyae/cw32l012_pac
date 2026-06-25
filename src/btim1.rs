#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    dier: Dier,
    isr: Isr,
    egr: Egr,
    icr: Icr,
    _reserved7: [u8; 0x08],
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
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
    #[doc = "0x08 - Slave Mode Control register"]
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    #[doc = "0x0c - Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &Dier {
        &self.dier
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
    #[doc = "0x18 - Interrupt flag clear register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
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
#[doc = "SMCR (rw) register accessor: Slave Mode Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SmcrSpec>;
#[doc = "Slave Mode Control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
#[doc(alias = "DIER")]
pub type Dier = crate::Reg<dier::DierSpec>;
#[doc = "Interrupt enable register"]
pub mod dier;
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
#[doc = "EGR (w) register accessor: Event Generate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EgrSpec>;
#[doc = "Event Generate register"]
pub mod egr;
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
