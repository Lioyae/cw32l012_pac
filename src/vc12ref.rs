#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ref_: Ref,
}
impl RegisterBlock {
    #[doc = "0x00 - VCREF Control register"]
    #[inline(always)]
    pub const fn ref_(&self) -> &Ref {
        &self.ref_
    }
}
#[doc = "REF (rw) register accessor: VCREF Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_`] module"]
#[doc(alias = "REF")]
pub type Ref = crate::Reg<ref_::RefSpec>;
#[doc = "VCREF Control register"]
pub mod ref_;
