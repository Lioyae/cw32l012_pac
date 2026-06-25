#[doc = "Register `AWTCNT` reader"]
pub type R = crate::R<AwtcntSpec>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "desc AWTCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`awtcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwtcntSpec;
impl crate::RegisterSpec for AwtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awtcnt::R`](R) reader structure"]
impl crate::Readable for AwtcntSpec {}
#[doc = "`reset()` method sets AWTCNT to value 0"]
impl crate::Resettable for AwtcntSpec {}
