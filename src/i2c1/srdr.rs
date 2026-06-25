#[doc = "Register `SRDR` reader"]
pub type R = crate::R<SrdrSpec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader;
#[doc = "Field `RXEMPTY` reader - desc RXEMPTY"]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `SOF` reader - desc SOF"]
pub type SofR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - desc RXEMPTY"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "desc SRDR\n\nYou can [`read`](crate::Reg::read) this register and get [`srdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrdrSpec;
impl crate::RegisterSpec for SrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdr::R`](R) reader structure"]
impl crate::Readable for SrdrSpec {}
#[doc = "`reset()` method sets SRDR to value 0"]
impl crate::Resettable for SrdrSpec {}
