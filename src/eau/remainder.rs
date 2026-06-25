#[doc = "Register `REMAINDER` reader"]
pub type R = crate::R<RemainderSpec>;
#[doc = "Field `REMAINDER` reader - desc REMAINDER"]
pub type RemainderR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - desc REMAINDER"]
    #[inline(always)]
    pub fn remainder(&self) -> RemainderR {
        RemainderR::new(self.bits)
    }
}
#[doc = "desc REMAINDER\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemainderSpec;
impl crate::RegisterSpec for RemainderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remainder::R`](R) reader structure"]
impl crate::Readable for RemainderSpec {}
#[doc = "`reset()` method sets REMAINDER to value 0"]
impl crate::Resettable for RemainderSpec {}
