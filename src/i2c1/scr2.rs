#[doc = "Register `SCR2` reader"]
pub type R = crate::R<Scr2Spec>;
#[doc = "Register `SCR2` writer"]
pub type W = crate::W<Scr2Spec>;
#[doc = "Field `PINCFG` reader - desc PINCFG"]
pub type PincfgR = crate::FieldReader;
#[doc = "Field `PINCFG` writer - desc PINCFG"]
pub type PincfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 24:26 - desc PINCFG"]
    #[inline(always)]
    pub fn pincfg(&self) -> PincfgR {
        PincfgR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - desc PINCFG"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PincfgW<'_, Scr2Spec> {
        PincfgW::new(self, 24)
    }
}
#[doc = "desc SCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`scr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr2Spec;
impl crate::RegisterSpec for Scr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr2::R`](R) reader structure"]
impl crate::Readable for Scr2Spec {}
#[doc = "`write(|w| ..)` method takes [`scr2::W`](W) writer structure"]
impl crate::Writable for Scr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for Scr2Spec {}
