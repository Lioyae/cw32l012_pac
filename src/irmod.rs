#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x00 - Input selection register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "CR (rw) register accessor: Input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Input selection register"]
pub mod cr;
