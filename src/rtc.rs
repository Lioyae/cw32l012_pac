#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: Key,
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    compcfr1: Compcfr1,
    date: Date,
    time: Time,
    alarma: Alarma,
    alarmb: Alarmb,
    tampdate: Tampdate,
    tamptime: Tamptime,
    awtarr: Awtarr,
    ier: Ier,
    isr: Isr,
    icr: Icr,
    awtcnt: Awtcnt,
    psc: Psc,
    sscnt: Sscnt,
}
impl RegisterBlock {
    #[doc = "0x00 - desc KEY"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x04 - desc CR0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x08 - desc CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x0c - desc CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x10 - desc COMPCFR1"]
    #[inline(always)]
    pub const fn compcfr1(&self) -> &Compcfr1 {
        &self.compcfr1
    }
    #[doc = "0x14 - desc DATE"]
    #[inline(always)]
    pub const fn date(&self) -> &Date {
        &self.date
    }
    #[doc = "0x18 - desc TIME"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x1c - desc ALARMA"]
    #[inline(always)]
    pub const fn alarma(&self) -> &Alarma {
        &self.alarma
    }
    #[doc = "0x20 - desc ALARMB"]
    #[inline(always)]
    pub const fn alarmb(&self) -> &Alarmb {
        &self.alarmb
    }
    #[doc = "0x24 - desc TAMPDATE"]
    #[inline(always)]
    pub const fn tampdate(&self) -> &Tampdate {
        &self.tampdate
    }
    #[doc = "0x28 - desc TAMPTIME"]
    #[inline(always)]
    pub const fn tamptime(&self) -> &Tamptime {
        &self.tamptime
    }
    #[doc = "0x2c - desc AWTARR"]
    #[inline(always)]
    pub const fn awtarr(&self) -> &Awtarr {
        &self.awtarr
    }
    #[doc = "0x30 - desc IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x34 - desc ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x38 - desc ICR"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x3c - desc AWTCNT"]
    #[inline(always)]
    pub const fn awtcnt(&self) -> &Awtcnt {
        &self.awtcnt
    }
    #[doc = "0x40 - desc PSC"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x44 - desc SSCNT"]
    #[inline(always)]
    pub const fn sscnt(&self) -> &Sscnt {
        &self.sscnt
    }
}
#[doc = "KEY (w) register accessor: desc KEY\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "desc KEY"]
pub mod key;
#[doc = "CR0 (rw) register accessor: desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "desc CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "desc CR2"]
pub mod cr2;
#[doc = "PSC (rw) register accessor: desc PSC\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "desc PSC"]
pub mod psc;
#[doc = "DATE (rw) register accessor: desc DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
#[doc(alias = "DATE")]
pub type Date = crate::Reg<date::DateSpec>;
#[doc = "desc DATE"]
pub mod date;
#[doc = "TIME (rw) register accessor: desc TIME\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`] module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "desc TIME"]
pub mod time;
#[doc = "SSCNT (r) register accessor: desc SSCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`sscnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sscnt`] module"]
#[doc(alias = "SSCNT")]
pub type Sscnt = crate::Reg<sscnt::SscntSpec>;
#[doc = "desc SSCNT"]
pub mod sscnt;
#[doc = "ALARMA (rw) register accessor: desc ALARMA\n\nYou can [`read`](crate::Reg::read) this register and get [`alarma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarma`] module"]
#[doc(alias = "ALARMA")]
pub type Alarma = crate::Reg<alarma::AlarmaSpec>;
#[doc = "desc ALARMA"]
pub mod alarma;
#[doc = "ALARMB (rw) register accessor: desc ALARMB\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarmb`] module"]
#[doc(alias = "ALARMB")]
pub type Alarmb = crate::Reg<alarmb::AlarmbSpec>;
#[doc = "desc ALARMB"]
pub mod alarmb;
#[doc = "TAMPDATE (r) register accessor: desc TAMPDATE\n\nYou can [`read`](crate::Reg::read) this register and get [`tampdate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampdate`] module"]
#[doc(alias = "TAMPDATE")]
pub type Tampdate = crate::Reg<tampdate::TampdateSpec>;
#[doc = "desc TAMPDATE"]
pub mod tampdate;
#[doc = "TAMPTIME (r) register accessor: desc TAMPTIME\n\nYou can [`read`](crate::Reg::read) this register and get [`tamptime::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamptime`] module"]
#[doc(alias = "TAMPTIME")]
pub type Tamptime = crate::Reg<tamptime::TamptimeSpec>;
#[doc = "desc TAMPTIME"]
pub mod tamptime;
#[doc = "IER (rw) register accessor: desc IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "desc IER"]
pub mod ier;
#[doc = "ISR (r) register accessor: desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR (rw) register accessor: desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "AWTARR (rw) register accessor: desc AWTARR\n\nYou can [`read`](crate::Reg::read) this register and get [`awtarr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awtarr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awtarr`] module"]
#[doc(alias = "AWTARR")]
pub type Awtarr = crate::Reg<awtarr::AwtarrSpec>;
#[doc = "desc AWTARR"]
pub mod awtarr;
#[doc = "AWTCNT (r) register accessor: desc AWTCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`awtcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awtcnt`] module"]
#[doc(alias = "AWTCNT")]
pub type Awtcnt = crate::Reg<awtcnt::AwtcntSpec>;
#[doc = "desc AWTCNT"]
pub mod awtcnt;
#[doc = "COMPCFR1 (rw) register accessor: desc COMPCFR1\n\nYou can [`read`](crate::Reg::read) this register and get [`compcfr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compcfr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compcfr1`] module"]
#[doc(alias = "COMPCFR1")]
pub type Compcfr1 = crate::Reg<compcfr1::Compcfr1Spec>;
#[doc = "desc COMPCFR1"]
pub mod compcfr1;
