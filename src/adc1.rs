#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    start: Start,
    awdtr: Awdtr,
    awdcr: Awdcr,
    trigger: Trigger,
    _reserved5: [u8; 0x04],
    sample: Sample,
    _reserved6: [u8; 0x04],
    sqrcfr: Sqrcfr,
    _reserved7: [u8; 0x0c],
    result0: Result0,
    result1: Result1,
    result2: Result2,
    result3: Result3,
    result4: Result4,
    result5: Result5,
    result6: Result6,
    result7: Result7,
    _reserved15: [u8; 0x20],
    ier: Ier,
    icr: Icr,
    isr: Isr,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - desc START"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x08 - desc AWDTR"]
    #[inline(always)]
    pub const fn awdtr(&self) -> &Awdtr {
        &self.awdtr
    }
    #[doc = "0x0c - desc AWDCR"]
    #[inline(always)]
    pub const fn awdcr(&self) -> &Awdcr {
        &self.awdcr
    }
    #[doc = "0x10 - desc TRIGGER"]
    #[inline(always)]
    pub const fn trigger(&self) -> &Trigger {
        &self.trigger
    }
    #[doc = "0x18 - desc SAMPLE"]
    #[inline(always)]
    pub const fn sample(&self) -> &Sample {
        &self.sample
    }
    #[doc = "0x20 - desc SQRCFR"]
    #[inline(always)]
    pub const fn sqrcfr(&self) -> &Sqrcfr {
        &self.sqrcfr
    }
    #[doc = "0x30 - desc RESULT0"]
    #[inline(always)]
    pub const fn result0(&self) -> &Result0 {
        &self.result0
    }
    #[doc = "0x34 - desc RESULT1"]
    #[inline(always)]
    pub const fn result1(&self) -> &Result1 {
        &self.result1
    }
    #[doc = "0x38 - desc RESULT2"]
    #[inline(always)]
    pub const fn result2(&self) -> &Result2 {
        &self.result2
    }
    #[doc = "0x3c - desc RESULT3"]
    #[inline(always)]
    pub const fn result3(&self) -> &Result3 {
        &self.result3
    }
    #[doc = "0x40 - desc RESULT4"]
    #[inline(always)]
    pub const fn result4(&self) -> &Result4 {
        &self.result4
    }
    #[doc = "0x44 - desc RESULT5"]
    #[inline(always)]
    pub const fn result5(&self) -> &Result5 {
        &self.result5
    }
    #[doc = "0x48 - desc RESULT6"]
    #[inline(always)]
    pub const fn result6(&self) -> &Result6 {
        &self.result6
    }
    #[doc = "0x4c - desc RESULT7"]
    #[inline(always)]
    pub const fn result7(&self) -> &Result7 {
        &self.result7
    }
    #[doc = "0x70 - desc IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x74 - desc ICR"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x78 - desc ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "START (rw) register accessor: desc START\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "desc START"]
pub mod start;
#[doc = "AWDTR (rw) register accessor: desc AWDTR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awdtr`] module"]
#[doc(alias = "AWDTR")]
pub type Awdtr = crate::Reg<awdtr::AwdtrSpec>;
#[doc = "desc AWDTR"]
pub mod awdtr;
#[doc = "AWDCR (rw) register accessor: desc AWDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awdcr`] module"]
#[doc(alias = "AWDCR")]
pub type Awdcr = crate::Reg<awdcr::AwdcrSpec>;
#[doc = "desc AWDCR"]
pub mod awdcr;
#[doc = "TRIGGER (rw) register accessor: desc TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
#[doc = "desc TRIGGER"]
pub mod trigger;
#[doc = "SAMPLE (rw) register accessor: desc SAMPLE\n\nYou can [`read`](crate::Reg::read) this register and get [`sample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample`] module"]
#[doc(alias = "SAMPLE")]
pub type Sample = crate::Reg<sample::SampleSpec>;
#[doc = "desc SAMPLE"]
pub mod sample;
#[doc = "SQRCFR (rw) register accessor: desc SQRCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqrcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqrcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqrcfr`] module"]
#[doc(alias = "SQRCFR")]
pub type Sqrcfr = crate::Reg<sqrcfr::SqrcfrSpec>;
#[doc = "desc SQRCFR"]
pub mod sqrcfr;
#[doc = "IER (rw) register accessor: desc IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "desc IER"]
pub mod ier;
#[doc = "ISR (rw) register accessor: desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "desc ISR"]
pub mod isr;
#[doc = "ICR (rw) register accessor: desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "RESULT0 (r) register accessor: desc RESULT0\n\nYou can [`read`](crate::Reg::read) this register and get [`result0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result0`] module"]
#[doc(alias = "RESULT0")]
pub type Result0 = crate::Reg<result0::Result0Spec>;
#[doc = "desc RESULT0"]
pub mod result0;
#[doc = "RESULT1 (r) register accessor: desc RESULT1\n\nYou can [`read`](crate::Reg::read) this register and get [`result1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result1`] module"]
#[doc(alias = "RESULT1")]
pub type Result1 = crate::Reg<result1::Result1Spec>;
#[doc = "desc RESULT1"]
pub mod result1;
#[doc = "RESULT2 (r) register accessor: desc RESULT2\n\nYou can [`read`](crate::Reg::read) this register and get [`result2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result2`] module"]
#[doc(alias = "RESULT2")]
pub type Result2 = crate::Reg<result2::Result2Spec>;
#[doc = "desc RESULT2"]
pub mod result2;
#[doc = "RESULT3 (r) register accessor: desc RESULT3\n\nYou can [`read`](crate::Reg::read) this register and get [`result3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result3`] module"]
#[doc(alias = "RESULT3")]
pub type Result3 = crate::Reg<result3::Result3Spec>;
#[doc = "desc RESULT3"]
pub mod result3;
#[doc = "RESULT4 (r) register accessor: desc RESULT4\n\nYou can [`read`](crate::Reg::read) this register and get [`result4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result4`] module"]
#[doc(alias = "RESULT4")]
pub type Result4 = crate::Reg<result4::Result4Spec>;
#[doc = "desc RESULT4"]
pub mod result4;
#[doc = "RESULT5 (r) register accessor: desc RESULT5\n\nYou can [`read`](crate::Reg::read) this register and get [`result5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result5`] module"]
#[doc(alias = "RESULT5")]
pub type Result5 = crate::Reg<result5::Result5Spec>;
#[doc = "desc RESULT5"]
pub mod result5;
#[doc = "RESULT6 (r) register accessor: desc RESULT6\n\nYou can [`read`](crate::Reg::read) this register and get [`result6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result6`] module"]
#[doc(alias = "RESULT6")]
pub type Result6 = crate::Reg<result6::Result6Spec>;
#[doc = "desc RESULT6"]
pub mod result6;
#[doc = "RESULT7 (r) register accessor: desc RESULT7\n\nYou can [`read`](crate::Reg::read) this register and get [`result7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result7`] module"]
#[doc(alias = "RESULT7")]
pub type Result7 = crate::Reg<result7::Result7Spec>;
#[doc = "desc RESULT7"]
pub mod result7;
