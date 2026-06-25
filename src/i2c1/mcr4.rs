#[doc = "Register `MCR4` reader"]
pub type R = crate::R<Mcr4Spec>;
#[doc = "Register `MCR4` writer"]
pub type W = crate::W<Mcr4Spec>;
#[doc = "Field `PINLOW` reader - desc PINLOW"]
pub type PinlowR = crate::FieldReader<u16>;
#[doc = "Field `PINLOW` writer - desc PINLOW"]
pub type PinlowW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 8:19 - desc PINLOW"]
    #[inline(always)]
    pub fn pinlow(&self) -> PinlowR {
        PinlowR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - desc PINLOW"]
    #[inline(always)]
    pub fn pinlow(&mut self) -> PinlowW<'_, Mcr4Spec> {
        PinlowW::new(self, 8)
    }
}
#[doc = "Master Control Register4\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr4Spec;
impl crate::RegisterSpec for Mcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr4::R`](R) reader structure"]
impl crate::Readable for Mcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr4::W`](W) writer structure"]
impl crate::Writable for Mcr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR4 to value 0"]
impl crate::Resettable for Mcr4Spec {}
