#[doc = "Register `CDSTADDR` reader"]
pub type R = crate::R<CdstaddrSpec>;
#[doc = "Field `CDSTADDR` reader - desc CDSTADDR"]
pub type CdstaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc CDSTADDR"]
    #[inline(always)]
    pub fn cdstaddr(&self) -> CdstaddrR {
        CdstaddrR::new(self.bits)
    }
}
#[doc = "Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdstaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdstaddrSpec;
impl crate::RegisterSpec for CdstaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdstaddr::R`](R) reader structure"]
impl crate::Readable for CdstaddrSpec {}
#[doc = "`reset()` method sets CDSTADDR to value 0"]
impl crate::Resettable for CdstaddrSpec {}
