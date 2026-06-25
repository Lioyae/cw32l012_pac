#[doc = "Register `CSRCADDR` reader"]
pub type R = crate::R<CsrcaddrSpec>;
#[doc = "Field `CSRCADDR` reader - desc CSRCADDR"]
pub type CsrcaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CSRCADDR"]
    #[inline(always)]
    pub fn csrcaddr(&self) -> CsrcaddrR {
        CsrcaddrR::new(self.bits)
    }
}
#[doc = "Channel1 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csrcaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrcaddrSpec;
impl crate::RegisterSpec for CsrcaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csrcaddr::R`](R) reader structure"]
impl crate::Readable for CsrcaddrSpec {}
#[doc = "`reset()` method sets CSRCADDR to value 0"]
impl crate::Resettable for CsrcaddrSpec {}
