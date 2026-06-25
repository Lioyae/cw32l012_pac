#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    swtrgr: Swtrgr,
    dhr12r1: Dhr12r1,
    dhr12l1: Dhr12l1,
    dhr8r1: Dhr8r1,
    dhr12r2: Dhr12r2,
    dhr12l2: Dhr12l2,
    dhr8r2: Dhr8r2,
    dhr12rd: Dhr12rd,
    dhr12ld: Dhr12ld,
    dhr8rd: Dhr8rd,
    dor1: Dor1,
    dor2: Dor2,
    cr1: Cr1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x04 - Data register"]
    #[inline(always)]
    pub const fn swtrgr(&self) -> &Swtrgr {
        &self.swtrgr
    }
    #[doc = "0x08 - desc DHR12R1"]
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &Dhr12r1 {
        &self.dhr12r1
    }
    #[doc = "0x0c - desc DHR12L1"]
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &Dhr12l1 {
        &self.dhr12l1
    }
    #[doc = "0x10 - desc DHR8R1"]
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &Dhr8r1 {
        &self.dhr8r1
    }
    #[doc = "0x14 - desc DHR12R2"]
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &Dhr12r2 {
        &self.dhr12r2
    }
    #[doc = "0x18 - desc DHR12L2"]
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &Dhr12l2 {
        &self.dhr12l2
    }
    #[doc = "0x1c - desc DHR8R2"]
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &Dhr8r2 {
        &self.dhr8r2
    }
    #[doc = "0x20 - desc DHR12RD"]
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &Dhr12rd {
        &self.dhr12rd
    }
    #[doc = "0x24 - desc DHR12LD"]
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &Dhr12ld {
        &self.dhr12ld
    }
    #[doc = "0x28 - desc DHR8RD"]
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &Dhr8rd {
        &self.dhr8rd
    }
    #[doc = "0x2c - desc DOR1"]
    #[inline(always)]
    pub const fn dor1(&self) -> &Dor1 {
        &self.dor1
    }
    #[doc = "0x30 - desc DOR2"]
    #[inline(always)]
    pub const fn dor2(&self) -> &Dor2 {
        &self.dor2
    }
    #[doc = "0x34 - Result register"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
}
#[doc = "CR0 (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`] module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "Control register"]
pub mod cr0;
#[doc = "SWTRGR (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`] module"]
#[doc(alias = "SWTRGR")]
pub type Swtrgr = crate::Reg<swtrgr::SwtrgrSpec>;
#[doc = "Data register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: desc DHR12R1\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`] module"]
#[doc(alias = "DHR12R1")]
pub type Dhr12r1 = crate::Reg<dhr12r1::Dhr12r1Spec>;
#[doc = "desc DHR12R1"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: desc DHR12L1\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`] module"]
#[doc(alias = "DHR12L1")]
pub type Dhr12l1 = crate::Reg<dhr12l1::Dhr12l1Spec>;
#[doc = "desc DHR12L1"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: desc DHR8R1\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`] module"]
#[doc(alias = "DHR8R1")]
pub type Dhr8r1 = crate::Reg<dhr8r1::Dhr8r1Spec>;
#[doc = "desc DHR8R1"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: desc DHR12R2\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r2`] module"]
#[doc(alias = "DHR12R2")]
pub type Dhr12r2 = crate::Reg<dhr12r2::Dhr12r2Spec>;
#[doc = "desc DHR12R2"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: desc DHR12L2\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l2`] module"]
#[doc(alias = "DHR12L2")]
pub type Dhr12l2 = crate::Reg<dhr12l2::Dhr12l2Spec>;
#[doc = "desc DHR12L2"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: desc DHR8R2\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r2`] module"]
#[doc(alias = "DHR8R2")]
pub type Dhr8r2 = crate::Reg<dhr8r2::Dhr8r2Spec>;
#[doc = "desc DHR8R2"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: desc DHR12RD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`] module"]
#[doc(alias = "DHR12RD")]
pub type Dhr12rd = crate::Reg<dhr12rd::Dhr12rdSpec>;
#[doc = "desc DHR12RD"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: desc DHR12LD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`] module"]
#[doc(alias = "DHR12LD")]
pub type Dhr12ld = crate::Reg<dhr12ld::Dhr12ldSpec>;
#[doc = "desc DHR12LD"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: desc DHR8RD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`] module"]
#[doc(alias = "DHR8RD")]
pub type Dhr8rd = crate::Reg<dhr8rd::Dhr8rdSpec>;
#[doc = "desc DHR8RD"]
pub mod dhr8rd;
#[doc = "DOR1 (rw) register accessor: desc DOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`] module"]
#[doc(alias = "DOR1")]
pub type Dor1 = crate::Reg<dor1::Dor1Spec>;
#[doc = "desc DOR1"]
pub mod dor1;
#[doc = "DOR2 (rw) register accessor: desc DOR2\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`] module"]
#[doc(alias = "DOR2")]
pub type Dor2 = crate::Reg<dor2::Dor2Spec>;
#[doc = "desc DOR2"]
pub mod dor2;
#[doc = "CR1 (rw) register accessor: Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Result register"]
pub mod cr1;
