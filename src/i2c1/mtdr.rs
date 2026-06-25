#[doc = "Register `MTDR` writer"]
pub type W = crate::W<MtdrSpec>;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD` writer - desc CMD"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl W {
    #[doc = "Bits 0:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, MtdrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:10 - desc CMD"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, MtdrSpec> {
        CmdW::new(self, 8)
    }
}
#[doc = "Master Tx Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtdrSpec;
impl crate::RegisterSpec for MtdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mtdr::W`](W) writer structure"]
impl crate::Writable for MtdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTDR to value 0"]
impl crate::Resettable for MtdrSpec {}
