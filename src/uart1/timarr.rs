#[doc = "Register `TIMARR` reader"]
pub type R = crate::R<TimarrSpec>;
#[doc = "Register `TIMARR` writer"]
pub type W = crate::W<TimarrSpec>;
#[doc = "Field `TIMARR` reader - desc TIMARR"]
pub type TimarrR = crate::FieldReader<u32>;
#[doc = "Field `TIMARR` writer - desc TIMARR"]
pub type TimarrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - desc TIMARR"]
    #[inline(always)]
    pub fn timarr(&self) -> TimarrR {
        TimarrR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - desc TIMARR"]
    #[inline(always)]
    pub fn timarr(&mut self) -> TimarrW<'_, TimarrSpec> {
        TimarrW::new(self, 0)
    }
}
#[doc = "desc TIMARR\n\nYou can [`read`](crate::Reg::read) this register and get [`timarr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timarr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimarrSpec;
impl crate::RegisterSpec for TimarrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timarr::R`](R) reader structure"]
impl crate::Readable for TimarrSpec {}
#[doc = "`write(|w| ..)` method takes [`timarr::W`](W) writer structure"]
impl crate::Writable for TimarrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMARR to value 0"]
impl crate::Resettable for TimarrSpec {}
