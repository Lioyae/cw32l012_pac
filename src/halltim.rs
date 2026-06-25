#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    dier: Dier,
    cnt: Cnt,
    arr: Arr,
    width: Width,
    ccr: Ccr,
    isr: Isr,
    icr: Icr,
    state: State,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register1"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DMA and Interrupt register2"]
    #[inline(always)]
    pub const fn dier(&self) -> &Dier {
        &self.dier
    }
    #[doc = "0x08 - counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x0c - Auto Reload Register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x10 - Width Register"]
    #[inline(always)]
    pub const fn width(&self) -> &Width {
        &self.width
    }
    #[doc = "0x14 - Compare control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x18 - Interrupt states register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x1c - Interrupt clean register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x20 - State register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
}
#[doc = "CR (rw) register accessor: Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register1"]
pub mod cr;
#[doc = "DIER (rw) register accessor: DMA and Interrupt register2\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
#[doc(alias = "DIER")]
pub type Dier = crate::Reg<dier::DierSpec>;
#[doc = "DMA and Interrupt register2"]
pub mod dier;
#[doc = "CNT (rw) register accessor: counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "counter register"]
pub mod cnt;
#[doc = "ARR (rw) register accessor: Auto Reload Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Auto Reload Register"]
pub mod arr;
#[doc = "WIDTH (r) register accessor: Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`width::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@width`] module"]
#[doc(alias = "WIDTH")]
pub type Width = crate::Reg<width::WidthSpec>;
#[doc = "Width Register"]
pub mod width;
#[doc = "CCR (rw) register accessor: Compare control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Compare control Register"]
pub mod ccr;
#[doc = "ISR (r) register accessor: Interrupt states register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt states register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt clean register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt clean register"]
pub mod icr;
#[doc = "STATE (r) register accessor: State register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "State register"]
pub mod state;
