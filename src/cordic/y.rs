#[doc = "Register `Y` reader"]
pub type R = crate::R<YSpec>;
#[doc = "Register `Y` writer"]
pub type W = crate::W<YSpec>;
#[doc = "Field `Y` reader - desc Y"]
pub type YR = crate::FieldReader<u32>;
#[doc = "Field `Y` writer - desc Y"]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc Y"]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc Y"]
    #[inline(always)]
    pub fn y(&mut self) -> YW<'_, YSpec> {
        YW::new(self, 0)
    }
}
#[doc = "Data register Y\n\nYou can [`read`](crate::Reg::read) this register and get [`y::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`y::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YSpec;
impl crate::RegisterSpec for YSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`y::R`](R) reader structure"]
impl crate::Readable for YSpec {}
#[doc = "`write(|w| ..)` method takes [`y::W`](W) writer structure"]
impl crate::Writable for YSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Y to value 0"]
impl crate::Resettable for YSpec {}
