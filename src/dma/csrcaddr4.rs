#[doc = "Register `CSRCADDR4` reader"]
pub type R = crate::R<Csrcaddr4Spec>;
#[doc = "Field `CSRCADDR` reader - desc CSRCADDR"]
pub type CsrcaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CSRCADDR"]
    #[inline(always)]
    pub fn csrcaddr(&self) -> CsrcaddrR {
        CsrcaddrR::new(self.bits)
    }
}
#[doc = "Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csrcaddr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csrcaddr4Spec;
impl crate::RegisterSpec for Csrcaddr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csrcaddr4::R`](R) reader structure"]
impl crate::Readable for Csrcaddr4Spec {}
#[doc = "`reset()` method sets CSRCADDR4 to value 0"]
impl crate::Resettable for Csrcaddr4Spec {}
