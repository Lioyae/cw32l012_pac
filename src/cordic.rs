#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    x: X,
    y: Y,
    z: Z,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and State register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Data register X"]
    #[inline(always)]
    pub const fn x(&self) -> &X {
        &self.x
    }
    #[doc = "0x08 - Data register Y"]
    #[inline(always)]
    pub const fn y(&self) -> &Y {
        &self.y
    }
    #[doc = "0x0c - desc Z"]
    #[inline(always)]
    pub const fn z(&self) -> &Z {
        &self.z
    }
}
#[doc = "CSR (rw) register accessor: Control and State register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control and State register"]
pub mod csr;
#[doc = "X (rw) register accessor: Data register X\n\nYou can [`read`](crate::Reg::read) this register and get [`x::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@x`] module"]
pub type X = crate::Reg<x::XSpec>;
#[doc = "Data register X"]
pub mod x;
#[doc = "Y (rw) register accessor: Data register Y\n\nYou can [`read`](crate::Reg::read) this register and get [`y::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@y`] module"]
pub type Y = crate::Reg<y::YSpec>;
#[doc = "Data register Y"]
pub mod y;
#[doc = "Z (rw) register accessor: desc Z\n\nYou can [`read`](crate::Reg::read) this register and get [`z::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`z::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@z`] module"]
pub type Z = crate::Reg<z::ZSpec>;
#[doc = "desc Z"]
pub mod z;
