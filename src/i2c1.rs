#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    insel: Insel,
    micr: Micr,
    mcr0: Mcr0,
    misr: Misr,
    mier: Mier,
    mder: Mder,
    mcr1: Mcr1,
    _reserved_7_mcr2: [u8; 0x04],
    mcr3: Mcr3,
    mcr4: Mcr4,
    _reserved10: [u8; 0x10],
    mmatch: Mmatch,
    _reserved11: [u8; 0x04],
    mccr: Mccr,
    _reserved12: [u8; 0x0c],
    mfifocr: Mfifocr,
    mfifosr: Mfifosr,
    mtdr: Mtdr,
    _reserved15: [u8; 0x0c],
    mrdr: Mrdr,
    _reserved16: [u8; 0x98],
    sicr: Sicr,
    scr0: Scr0,
    sisr: Sisr,
    sier: Sier,
    sder: Sder,
    _reserved21: [u8; 0x04],
    scr1: Scr1,
    scr3: Scr3,
    _reserved23: [u8; 0x14],
    samr: Samr,
    _reserved24: [u8; 0x0c],
    sasr: Sasr,
    star: Star,
    _reserved26: [u8; 0x08],
    stdr: Stdr,
    _reserved27: [u8; 0x0c],
    srdr: Srdr,
}
impl RegisterBlock {
    #[doc = "0x08 - desc INSEL"]
    #[inline(always)]
    pub const fn insel(&self) -> &Insel {
        &self.insel
    }
    #[doc = "0x0c - Master Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn micr(&self) -> &Micr {
        &self.micr
    }
    #[doc = "0x10 - desc MCR0"]
    #[inline(always)]
    pub const fn mcr0(&self) -> &Mcr0 {
        &self.mcr0
    }
    #[doc = "0x14 - Master Interrupt Status Register"]
    #[inline(always)]
    pub const fn misr(&self) -> &Misr {
        &self.misr
    }
    #[doc = "0x18 - Master Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mier(&self) -> &Mier {
        &self.mier
    }
    #[doc = "0x1c - Master DMA Enable Register"]
    #[inline(always)]
    pub const fn mder(&self) -> &Mder {
        &self.mder
    }
    #[doc = "0x20 - Master Control Register1"]
    #[inline(always)]
    pub const fn mcr1(&self) -> &Mcr1 {
        &self.mcr1
    }
    #[doc = "0x24 - desc SCR2"]
    #[inline(always)]
    pub const fn scr2(&self) -> &Scr2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - Master Control Register2"]
    #[inline(always)]
    pub const fn mcr2(&self) -> &Mcr2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x28 - Master Control Register3"]
    #[inline(always)]
    pub const fn mcr3(&self) -> &Mcr3 {
        &self.mcr3
    }
    #[doc = "0x2c - Master Control Register4"]
    #[inline(always)]
    pub const fn mcr4(&self) -> &Mcr4 {
        &self.mcr4
    }
    #[doc = "0x40 - Master Match Register"]
    #[inline(always)]
    pub const fn mmatch(&self) -> &Mmatch {
        &self.mmatch
    }
    #[doc = "0x48 - Master Clock Control Register"]
    #[inline(always)]
    pub const fn mccr(&self) -> &Mccr {
        &self.mccr
    }
    #[doc = "0x58 - Master FIFO Control Register"]
    #[inline(always)]
    pub const fn mfifocr(&self) -> &Mfifocr {
        &self.mfifocr
    }
    #[doc = "0x5c - Master FIFO Status Register"]
    #[inline(always)]
    pub const fn mfifosr(&self) -> &Mfifosr {
        &self.mfifosr
    }
    #[doc = "0x60 - Master Tx Data Register"]
    #[inline(always)]
    pub const fn mtdr(&self) -> &Mtdr {
        &self.mtdr
    }
    #[doc = "0x70 - Master Rx Data Register"]
    #[inline(always)]
    pub const fn mrdr(&self) -> &Mrdr {
        &self.mrdr
    }
    #[doc = "0x10c - desc SICR"]
    #[inline(always)]
    pub const fn sicr(&self) -> &Sicr {
        &self.sicr
    }
    #[doc = "0x110 - Slave Control Register0"]
    #[inline(always)]
    pub const fn scr0(&self) -> &Scr0 {
        &self.scr0
    }
    #[doc = "0x114 - desc SISR"]
    #[inline(always)]
    pub const fn sisr(&self) -> &Sisr {
        &self.sisr
    }
    #[doc = "0x118 - desc SIER"]
    #[inline(always)]
    pub const fn sier(&self) -> &Sier {
        &self.sier
    }
    #[doc = "0x11c - desc SDER"]
    #[inline(always)]
    pub const fn sder(&self) -> &Sder {
        &self.sder
    }
    #[doc = "0x124 - desc SCR1"]
    #[inline(always)]
    pub const fn scr1(&self) -> &Scr1 {
        &self.scr1
    }
    #[doc = "0x128 - desc SCR3"]
    #[inline(always)]
    pub const fn scr3(&self) -> &Scr3 {
        &self.scr3
    }
    #[doc = "0x140 - desc SAMR"]
    #[inline(always)]
    pub const fn samr(&self) -> &Samr {
        &self.samr
    }
    #[doc = "0x150 - desc SASR"]
    #[inline(always)]
    pub const fn sasr(&self) -> &Sasr {
        &self.sasr
    }
    #[doc = "0x154 - desc STAR"]
    #[inline(always)]
    pub const fn star(&self) -> &Star {
        &self.star
    }
    #[doc = "0x160 - desc STDR"]
    #[inline(always)]
    pub const fn stdr(&self) -> &Stdr {
        &self.stdr
    }
    #[doc = "0x170 - desc SRDR"]
    #[inline(always)]
    pub const fn srdr(&self) -> &Srdr {
        &self.srdr
    }
}
#[doc = "INSEL (rw) register accessor: desc INSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`insel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@insel`] module"]
#[doc(alias = "INSEL")]
pub type Insel = crate::Reg<insel::InselSpec>;
#[doc = "desc INSEL"]
pub mod insel;
#[doc = "MCR0 (rw) register accessor: desc MCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr0`] module"]
#[doc(alias = "MCR0")]
pub type Mcr0 = crate::Reg<mcr0::Mcr0Spec>;
#[doc = "desc MCR0"]
pub mod mcr0;
#[doc = "MCR1 (rw) register accessor: Master Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr1`] module"]
#[doc(alias = "MCR1")]
pub type Mcr1 = crate::Reg<mcr1::Mcr1Spec>;
#[doc = "Master Control Register1"]
pub mod mcr1;
#[doc = "MCR2 (rw) register accessor: Master Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr2`] module"]
#[doc(alias = "MCR2")]
pub type Mcr2 = crate::Reg<mcr2::Mcr2Spec>;
#[doc = "Master Control Register2"]
pub mod mcr2;
#[doc = "MCR3 (rw) register accessor: Master Control Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr3`] module"]
#[doc(alias = "MCR3")]
pub type Mcr3 = crate::Reg<mcr3::Mcr3Spec>;
#[doc = "Master Control Register3"]
pub mod mcr3;
#[doc = "MCR4 (rw) register accessor: Master Control Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr4`] module"]
#[doc(alias = "MCR4")]
pub type Mcr4 = crate::Reg<mcr4::Mcr4Spec>;
#[doc = "Master Control Register4"]
pub mod mcr4;
#[doc = "MCCR (rw) register accessor: Master Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mccr`] module"]
#[doc(alias = "MCCR")]
pub type Mccr = crate::Reg<mccr::MccrSpec>;
#[doc = "Master Clock Control Register"]
pub mod mccr;
#[doc = "MMATCH (rw) register accessor: Master Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmatch`] module"]
#[doc(alias = "MMATCH")]
pub type Mmatch = crate::Reg<mmatch::MmatchSpec>;
#[doc = "Master Match Register"]
pub mod mmatch;
#[doc = "MFIFOCR (rw) register accessor: Master FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfifocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfifocr`] module"]
#[doc(alias = "MFIFOCR")]
pub type Mfifocr = crate::Reg<mfifocr::MfifocrSpec>;
#[doc = "Master FIFO Control Register"]
pub mod mfifocr;
#[doc = "MFIFOSR (r) register accessor: Master FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifosr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfifosr`] module"]
#[doc(alias = "MFIFOSR")]
pub type Mfifosr = crate::Reg<mfifosr::MfifosrSpec>;
#[doc = "Master FIFO Status Register"]
pub mod mfifosr;
#[doc = "MTDR (w) register accessor: Master Tx Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtdr`] module"]
#[doc(alias = "MTDR")]
pub type Mtdr = crate::Reg<mtdr::MtdrSpec>;
#[doc = "Master Tx Data Register"]
pub mod mtdr;
#[doc = "MRDR (r) register accessor: Master Rx Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdr`] module"]
#[doc(alias = "MRDR")]
pub type Mrdr = crate::Reg<mrdr::MrdrSpec>;
#[doc = "Master Rx Data Register"]
pub mod mrdr;
#[doc = "MDER (rw) register accessor: Master DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mder`] module"]
#[doc(alias = "MDER")]
pub type Mder = crate::Reg<mder::MderSpec>;
#[doc = "Master DMA Enable Register"]
pub mod mder;
#[doc = "MIER (rw) register accessor: Master Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mier`] module"]
#[doc(alias = "MIER")]
pub type Mier = crate::Reg<mier::MierSpec>;
#[doc = "Master Interrupt Enable Register"]
pub mod mier;
#[doc = "MISR (r) register accessor: Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`] module"]
#[doc(alias = "MISR")]
pub type Misr = crate::Reg<misr::MisrSpec>;
#[doc = "Master Interrupt Status Register"]
pub mod misr;
#[doc = "MICR (rw) register accessor: Master Interrupt Flag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`micr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`micr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@micr`] module"]
#[doc(alias = "MICR")]
pub type Micr = crate::Reg<micr::MicrSpec>;
#[doc = "Master Interrupt Flag Clear Register"]
pub mod micr;
#[doc = "SCR0 (rw) register accessor: Slave Control Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`scr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr0`] module"]
#[doc(alias = "SCR0")]
pub type Scr0 = crate::Reg<scr0::Scr0Spec>;
#[doc = "Slave Control Register0"]
pub mod scr0;
#[doc = "SCR1 (rw) register accessor: desc SCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr1`] module"]
#[doc(alias = "SCR1")]
pub type Scr1 = crate::Reg<scr1::Scr1Spec>;
#[doc = "desc SCR1"]
pub mod scr1;
#[doc = "SCR2 (rw) register accessor: desc SCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr2`] module"]
#[doc(alias = "SCR2")]
pub type Scr2 = crate::Reg<scr2::Scr2Spec>;
#[doc = "desc SCR2"]
pub mod scr2;
#[doc = "SCR3 (rw) register accessor: desc SCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`scr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr3`] module"]
#[doc(alias = "SCR3")]
pub type Scr3 = crate::Reg<scr3::Scr3Spec>;
#[doc = "desc SCR3"]
pub mod scr3;
#[doc = "SAMR (rw) register accessor: desc SAMR\n\nYou can [`read`](crate::Reg::read) this register and get [`samr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samr`] module"]
#[doc(alias = "SAMR")]
pub type Samr = crate::Reg<samr::SamrSpec>;
#[doc = "desc SAMR"]
pub mod samr;
#[doc = "STAR (rw) register accessor: desc STAR\n\nYou can [`read`](crate::Reg::read) this register and get [`star::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`star::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@star`] module"]
#[doc(alias = "STAR")]
pub type Star = crate::Reg<star::StarSpec>;
#[doc = "desc STAR"]
pub mod star;
#[doc = "STDR (w) register accessor: desc STDR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stdr`] module"]
#[doc(alias = "STDR")]
pub type Stdr = crate::Reg<stdr::StdrSpec>;
#[doc = "desc STDR"]
pub mod stdr;
#[doc = "SRDR (r) register accessor: desc SRDR\n\nYou can [`read`](crate::Reg::read) this register and get [`srdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srdr`] module"]
#[doc(alias = "SRDR")]
pub type Srdr = crate::Reg<srdr::SrdrSpec>;
#[doc = "desc SRDR"]
pub mod srdr;
#[doc = "SASR (r) register accessor: desc SASR\n\nYou can [`read`](crate::Reg::read) this register and get [`sasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sasr`] module"]
#[doc(alias = "SASR")]
pub type Sasr = crate::Reg<sasr::SasrSpec>;
#[doc = "desc SASR"]
pub mod sasr;
#[doc = "SDER (rw) register accessor: desc SDER\n\nYou can [`read`](crate::Reg::read) this register and get [`sder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sder`] module"]
#[doc(alias = "SDER")]
pub type Sder = crate::Reg<sder::SderSpec>;
#[doc = "desc SDER"]
pub mod sder;
#[doc = "SIER (rw) register accessor: desc SIER\n\nYou can [`read`](crate::Reg::read) this register and get [`sier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sier`] module"]
#[doc(alias = "SIER")]
pub type Sier = crate::Reg<sier::SierSpec>;
#[doc = "desc SIER"]
pub mod sier;
#[doc = "SISR (r) register accessor: desc SISR\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sisr`] module"]
#[doc(alias = "SISR")]
pub type Sisr = crate::Reg<sisr::SisrSpec>;
#[doc = "desc SISR"]
pub mod sisr;
#[doc = "SICR (rw) register accessor: desc SICR\n\nYou can [`read`](crate::Reg::read) this register and get [`sicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sicr`] module"]
#[doc(alias = "SICR")]
pub type Sicr = crate::Reg<sicr::SicrSpec>;
#[doc = "desc SICR"]
pub mod sicr;
