#[doc = "Register `TIMCNT` reader"]
pub type R = crate::R<TimcntSpec>;
#[doc = "Field `TIMCNT` reader - desc TIMCNT"]
pub type TimcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - desc TIMCNT"]
    #[inline(always)]
    pub fn timcnt(&self) -> TimcntR {
        TimcntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "desc TIMCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`timcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimcntSpec;
impl crate::RegisterSpec for TimcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timcnt::R`](R) reader structure"]
impl crate::Readable for TimcntSpec {}
#[doc = "`reset()` method sets TIMCNT to value 0"]
impl crate::Resettable for TimcntSpec {}
