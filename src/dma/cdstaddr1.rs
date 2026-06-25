#[doc = "Register `CDSTADDR1` reader"]
pub type R = crate::R<Cdstaddr1Spec>;
#[doc = "Field `CDSTADDR` reader - desc CDSTADDR"]
pub type CdstaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CDSTADDR"]
    #[inline(always)]
    pub fn cdstaddr(&self) -> CdstaddrR {
        CdstaddrR::new(self.bits)
    }
}
#[doc = "Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdstaddr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdstaddr1Spec;
impl crate::RegisterSpec for Cdstaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdstaddr1::R`](R) reader structure"]
impl crate::Readable for Cdstaddr1Spec {}
#[doc = "`reset()` method sets CDSTADDR1 to value 0"]
impl crate::Resettable for Cdstaddr1Spec {}
