#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    ier: Ier,
    isr: Isr,
    icr: Icr,
    hsi: Hsi,
    hse: Hse,
    lsi: Lsi,
    lse: Lse,
    pll: Pll,
    debug: Debug,
    ahben: Ahben,
    apben2: Apben2,
    apben1: Apben1,
    _reserved15: [u8; 0x04],
    ahbrst: Ahbrst,
    apbrst2: Apbrst2,
    apbrst1: Apbrst1,
    resetflag: Resetflag,
    _reserved19: [u8; 0x20],
    mco: Mco,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Reg0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - Control Reg1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x08 - Control Reg2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - Interupt Enable Reg"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x10 - Interupt Status Reg"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x14 - Interupt Clear Reg"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x18 - HSI Control Reg"]
    #[inline(always)]
    pub const fn hsi(&self) -> &Hsi {
        &self.hsi
    }
    #[doc = "0x1c - desc HSE"]
    #[inline(always)]
    pub const fn hse(&self) -> &Hse {
        &self.hse
    }
    #[doc = "0x20 - LSI Control Reg"]
    #[inline(always)]
    pub const fn lsi(&self) -> &Lsi {
        &self.lsi
    }
    #[doc = "0x24 - LSE Control Reg"]
    #[inline(always)]
    pub const fn lse(&self) -> &Lse {
        &self.lse
    }
    #[doc = "0x28 - PLL Control Reg"]
    #[inline(always)]
    pub const fn pll(&self) -> &Pll {
        &self.pll
    }
    #[doc = "0x2c - Debug Control Reg"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x30 - AHB Clock Control Reg"]
    #[inline(always)]
    pub const fn ahben(&self) -> &Ahben {
        &self.ahben
    }
    #[doc = "0x34 - APB Clock Control Reg2"]
    #[inline(always)]
    pub const fn apben2(&self) -> &Apben2 {
        &self.apben2
    }
    #[doc = "0x38 - APB Clock Control Reg1"]
    #[inline(always)]
    pub const fn apben1(&self) -> &Apben1 {
        &self.apben1
    }
    #[doc = "0x40 - AHB Reset Control Reg"]
    #[inline(always)]
    pub const fn ahbrst(&self) -> &Ahbrst {
        &self.ahbrst
    }
    #[doc = "0x44 - APB Reset Control Reg2"]
    #[inline(always)]
    pub const fn apbrst2(&self) -> &Apbrst2 {
        &self.apbrst2
    }
    #[doc = "0x48 - APB Reset Control Reg1"]
    #[inline(always)]
    pub const fn apbrst1(&self) -> &Apbrst1 {
        &self.apbrst1
    }
    #[doc = "0x4c - Reset Status Reg"]
    #[inline(always)]
    pub const fn resetflag(&self) -> &Resetflag {
        &self.resetflag
    }
    #[doc = "0x70 - Master Clock Output Control Reg"]
    #[inline(always)]
    pub const fn mco(&self) -> &Mco {
        &self.mco
    }
}
#[doc = "CR0 (rw) register accessor: Control Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control Reg0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control Reg1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control Reg2"]
pub mod cr2;
#[doc = "HSI (rw) register accessor: HSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`hsi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsi`] module"]
#[doc(alias = "HSI")]
pub type Hsi = crate::Reg<hsi::HsiSpec>;
#[doc = "HSI Control Reg"]
pub mod hsi;
#[doc = "LSI (rw) register accessor: LSI Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lsi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsi`] module"]
#[doc(alias = "LSI")]
pub type Lsi = crate::Reg<lsi::LsiSpec>;
#[doc = "LSI Control Reg"]
pub mod lsi;
#[doc = "HSE (rw) register accessor: desc HSE\n\nYou can [`read`](crate::Reg::read) this register and get [`hse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hse`] module"]
#[doc(alias = "HSE")]
pub type Hse = crate::Reg<hse::HseSpec>;
#[doc = "desc HSE"]
pub mod hse;
#[doc = "LSE (rw) register accessor: LSE Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`lse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lse`] module"]
#[doc(alias = "LSE")]
pub type Lse = crate::Reg<lse::LseSpec>;
#[doc = "LSE Control Reg"]
pub mod lse;
#[doc = "IER (rw) register accessor: Interupt Enable Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interupt Enable Reg"]
pub mod ier;
#[doc = "ISR (r) register accessor: Interupt Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interupt Status Reg"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interupt Clear Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interupt Clear Reg"]
pub mod icr;
#[doc = "AHBEN (rw) register accessor: AHB Clock Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahben`] module"]
#[doc(alias = "AHBEN")]
pub type Ahben = crate::Reg<ahben::AhbenSpec>;
#[doc = "AHB Clock Control Reg"]
pub mod ahben;
#[doc = "APBEN1 (rw) register accessor: APB Clock Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apben1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben1`] module"]
#[doc(alias = "APBEN1")]
pub type Apben1 = crate::Reg<apben1::Apben1Spec>;
#[doc = "APB Clock Control Reg1"]
pub mod apben1;
#[doc = "APBEN2 (rw) register accessor: APB Clock Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apben2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apben2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apben2`] module"]
#[doc(alias = "APBEN2")]
pub type Apben2 = crate::Reg<apben2::Apben2Spec>;
#[doc = "APB Clock Control Reg2"]
pub mod apben2;
#[doc = "AHBRST (rw) register accessor: AHB Reset Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrst`] module"]
#[doc(alias = "AHBRST")]
pub type Ahbrst = crate::Reg<ahbrst::AhbrstSpec>;
#[doc = "AHB Reset Control Reg"]
pub mod ahbrst;
#[doc = "APBRST1 (rw) register accessor: APB Reset Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst1`] module"]
#[doc(alias = "APBRST1")]
pub type Apbrst1 = crate::Reg<apbrst1::Apbrst1Spec>;
#[doc = "APB Reset Control Reg1"]
pub mod apbrst1;
#[doc = "APBRST2 (rw) register accessor: APB Reset Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbrst2`] module"]
#[doc(alias = "APBRST2")]
pub type Apbrst2 = crate::Reg<apbrst2::Apbrst2Spec>;
#[doc = "APB Reset Control Reg2"]
pub mod apbrst2;
#[doc = "RESETFLAG (rw) register accessor: Reset Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`resetflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetflag`] module"]
#[doc(alias = "RESETFLAG")]
pub type Resetflag = crate::Reg<resetflag::ResetflagSpec>;
#[doc = "Reset Status Reg"]
pub mod resetflag;
#[doc = "DEBUG (rw) register accessor: Debug Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`] module"]
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "Debug Control Reg"]
pub mod debug;
#[doc = "MCO (rw) register accessor: Master Clock Output Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`mco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mco`] module"]
#[doc(alias = "MCO")]
pub type Mco = crate::Reg<mco::McoSpec>;
#[doc = "Master Clock Output Control Reg"]
pub mod mco;
#[doc = "PLL (rw) register accessor: PLL Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll`] module"]
#[doc(alias = "PLL")]
pub type Pll = crate::Reg<pll::PllSpec>;
#[doc = "PLL Control Reg"]
pub mod pll;
