#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    dividend: Dividend,
    divisor: Divisor,
    quotient: Quotient,
    remainder: Remainder,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - desc DIVIDEND"]
    #[inline(always)]
    pub const fn dividend(&self) -> &Dividend {
        &self.dividend
    }
    #[doc = "0x08 - desc DIVISOR"]
    #[inline(always)]
    pub const fn divisor(&self) -> &Divisor {
        &self.divisor
    }
    #[doc = "0x0c - desc QUOTIENT"]
    #[inline(always)]
    pub const fn quotient(&self) -> &Quotient {
        &self.quotient
    }
    #[doc = "0x10 - desc REMAINDER"]
    #[inline(always)]
    pub const fn remainder(&self) -> &Remainder {
        &self.remainder
    }
}
#[doc = "CSR (rw) register accessor: desc CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "desc CSR"]
pub mod csr;
#[doc = "DIVIDEND (rw) register accessor: desc DIVIDEND\n\nYou can [`read`](crate::Reg::read) this register and get [`dividend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dividend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dividend`] module"]
#[doc(alias = "DIVIDEND")]
pub type Dividend = crate::Reg<dividend::DividendSpec>;
#[doc = "desc DIVIDEND"]
pub mod dividend;
#[doc = "DIVISOR (rw) register accessor: desc DIVISOR\n\nYou can [`read`](crate::Reg::read) this register and get [`divisor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divisor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divisor`] module"]
#[doc(alias = "DIVISOR")]
pub type Divisor = crate::Reg<divisor::DivisorSpec>;
#[doc = "desc DIVISOR"]
pub mod divisor;
#[doc = "QUOTIENT (r) register accessor: desc QUOTIENT\n\nYou can [`read`](crate::Reg::read) this register and get [`quotient::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quotient`] module"]
#[doc(alias = "QUOTIENT")]
pub type Quotient = crate::Reg<quotient::QuotientSpec>;
#[doc = "desc QUOTIENT"]
pub mod quotient;
#[doc = "REMAINDER (r) register accessor: desc REMAINDER\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remainder`] module"]
#[doc(alias = "REMAINDER")]
pub type Remainder = crate::Reg<remainder::RemainderSpec>;
#[doc = "desc REMAINDER"]
pub mod remainder;
