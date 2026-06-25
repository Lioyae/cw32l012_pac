#[doc = "Register `SASR` reader"]
pub type R = crate::R<SasrSpec>;
#[doc = "Field `RADDR` reader - desc RADDR"]
pub type RaddrR = crate::FieldReader<u16>;
#[doc = "Field `ANV` reader - desc ANV"]
pub type AnvR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - desc RADDR"]
    #[inline(always)]
    pub fn raddr(&self) -> RaddrR {
        RaddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - desc ANV"]
    #[inline(always)]
    pub fn anv(&self) -> AnvR {
        AnvR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "desc SASR\n\nYou can [`read`](crate::Reg::read) this register and get [`sasr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SasrSpec;
impl crate::RegisterSpec for SasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sasr::R`](R) reader structure"]
impl crate::Readable for SasrSpec {}
#[doc = "`reset()` method sets SASR to value 0"]
impl crate::Resettable for SasrSpec {}
