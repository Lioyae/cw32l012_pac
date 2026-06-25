#[doc = "Register `SDKCFR` reader"]
pub type R = crate::R<SdkcfrSpec>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::FieldReader;
#[doc = "Field `END` reader - desc END"]
pub type EndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - desc END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "SDK config register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdkcfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdkcfrSpec;
impl crate::RegisterSpec for SdkcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdkcfr::R`](R) reader structure"]
impl crate::Readable for SdkcfrSpec {}
#[doc = "`reset()` method sets SDKCFR to value 0"]
impl crate::Resettable for SdkcfrSpec {}
