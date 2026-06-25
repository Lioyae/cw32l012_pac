#[doc = "Register `MRDR` reader"]
pub type R = crate::R<MrdrSpec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader;
#[doc = "Field `EMPTY` reader - desc EMPTY"]
pub type EmptyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - desc EMPTY"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Master Rx Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrdrSpec;
impl crate::RegisterSpec for MrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdr::R`](R) reader structure"]
impl crate::Readable for MrdrSpec {}
#[doc = "`reset()` method sets MRDR to value 0"]
impl crate::Resettable for MrdrSpec {}
