#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    cnt: Cnt,
    srcaddr: Srcaddr,
    dstaddr: Dstaddr,
    trig: Trig,
    ccnt: Ccnt,
    csrcaddr: Csrcaddr,
    cdstaddr: Cdstaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel1 control and status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Channel1 counter register"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x08 - Channel1 source address register"]
    #[inline(always)]
    pub const fn srcaddr(&self) -> &Srcaddr {
        &self.srcaddr
    }
    #[doc = "0x0c - Channel1 destination address register"]
    #[inline(always)]
    pub const fn dstaddr(&self) -> &Dstaddr {
        &self.dstaddr
    }
    #[doc = "0x10 - Channel1 trigger register"]
    #[inline(always)]
    pub const fn trig(&self) -> &Trig {
        &self.trig
    }
    #[doc = "0x14 - Channel1 trigger register"]
    #[inline(always)]
    pub const fn ccnt(&self) -> &Ccnt {
        &self.ccnt
    }
    #[doc = "0x18 - Channel1 source address register"]
    #[inline(always)]
    pub const fn csrcaddr(&self) -> &Csrcaddr {
        &self.csrcaddr
    }
    #[doc = "0x1c - Channel1 destination address register"]
    #[inline(always)]
    pub const fn cdstaddr(&self) -> &Cdstaddr {
        &self.cdstaddr
    }
}
#[doc = "CSR (rw) register accessor: Channel1 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Channel1 control and status register"]
pub mod csr;
#[doc = "CNT (rw) register accessor: Channel1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Channel1 counter register"]
pub mod cnt;
#[doc = "SRCADDR (rw) register accessor: Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`] module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "Channel1 source address register"]
pub mod srcaddr;
#[doc = "DSTADDR (rw) register accessor: Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstaddr`] module"]
#[doc(alias = "DSTADDR")]
pub type Dstaddr = crate::Reg<dstaddr::DstaddrSpec>;
#[doc = "Channel1 destination address register"]
pub mod dstaddr;
#[doc = "TRIG (rw) register accessor: Channel1 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trig`] module"]
#[doc(alias = "TRIG")]
pub type Trig = crate::Reg<trig::TrigSpec>;
#[doc = "Channel1 trigger register"]
pub mod trig;
#[doc = "CCNT (rw) register accessor: Channel1 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccnt`] module"]
#[doc(alias = "CCNT")]
pub type Ccnt = crate::Reg<ccnt::CcntSpec>;
#[doc = "Channel1 trigger register"]
pub mod ccnt;
#[doc = "CSRCADDR (r) register accessor: Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csrcaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csrcaddr`] module"]
#[doc(alias = "CSRCADDR")]
pub type Csrcaddr = crate::Reg<csrcaddr::CsrcaddrSpec>;
#[doc = "Channel1 source address register"]
pub mod csrcaddr;
#[doc = "CDSTADDR (r) register accessor: Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdstaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdstaddr`] module"]
#[doc(alias = "CDSTADDR")]
pub type Cdstaddr = crate::Reg<cdstaddr::CdstaddrSpec>;
#[doc = "Channel1 destination address register"]
pub mod cdstaddr;
