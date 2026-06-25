#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cal: Cal,
}
impl RegisterBlock {
    #[doc = "0x00 - desc CR"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - desc CAL"]
    #[inline(always)]
    pub const fn cal(&self) -> &Cal {
        &self.cal
    }
}
#[doc = "CR (rw) register accessor: desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "CAL (rw) register accessor: desc CAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal`] module"]
#[doc(alias = "CAL")]
pub type Cal = crate::Reg<cal::CalSpec>;
#[doc = "desc CAL"]
pub mod cal;
