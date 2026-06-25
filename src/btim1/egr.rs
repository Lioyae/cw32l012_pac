#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - desc UG"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - desc TG"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 6 - desc TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
    }
}
#[doc = "Event Generate register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
