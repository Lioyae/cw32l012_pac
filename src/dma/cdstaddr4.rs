#[doc = "Register `CDSTADDR4` reader"]
pub type R = crate::R<Cdstaddr4Spec>;
#[doc = "Field `CDSTADDR` reader - desc CDSTADDR"]
pub type CdstaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CDSTADDR"]
    #[inline(always)]
    pub fn cdstaddr(&self) -> CdstaddrR {
        CdstaddrR::new(self.bits)
    }
}
#[doc = "Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdstaddr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdstaddr4Spec;
impl crate::RegisterSpec for Cdstaddr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdstaddr4::R`](R) reader structure"]
impl crate::Readable for Cdstaddr4Spec {}
#[doc = "`reset()` method sets CDSTADDR4 to value 0"]
impl crate::Resettable for Cdstaddr4Spec {}
