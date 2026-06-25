#[doc = "Register `QUOTIENT` reader"]
pub type R = crate::R<QuotientSpec>;
#[doc = "Field `QUOTIENT` reader - desc QUOTIENT"]
pub type QuotientR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc QUOTIENT"]
    #[inline(always)]
    pub fn quotient(&self) -> QuotientR {
        QuotientR::new(self.bits)
    }
}
#[doc = "desc QUOTIENT\n\nYou can [`read`](crate::Reg::read) this register and get [`quotient::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QuotientSpec;
impl crate::RegisterSpec for QuotientSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quotient::R`](R) reader structure"]
impl crate::Readable for QuotientSpec {}
#[doc = "`reset()` method sets QUOTIENT to value 0"]
impl crate::Resettable for QuotientSpec {}
