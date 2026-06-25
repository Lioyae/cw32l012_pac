#[doc = "Register `CSRCADDR2` reader"]
pub type R = crate::R<Csrcaddr2Spec>;
#[doc = "Field `CSRCADDR` reader - desc CSRCADDR"]
pub type CsrcaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CSRCADDR"]
    #[inline(always)]
    pub fn csrcaddr(&self) -> CsrcaddrR {
        CsrcaddrR::new(self.bits)
    }
}
#[doc = "Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csrcaddr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csrcaddr2Spec;
impl crate::RegisterSpec for Csrcaddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csrcaddr2::R`](R) reader structure"]
impl crate::Readable for Csrcaddr2Spec {}
#[doc = "`reset()` method sets CSRCADDR2 to value 0"]
impl crate::Resettable for Csrcaddr2Spec {}
