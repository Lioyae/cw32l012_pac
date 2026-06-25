#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CH1SRC` reader - desc CH1SRC"]
pub type Ch1srcR = crate::FieldReader;
#[doc = "Field `CH1SRC` writer - desc CH1SRC"]
pub type Ch1srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2SRC` reader - desc CH2SRC"]
pub type Ch2srcR = crate::FieldReader;
#[doc = "Field `CH2SRC` writer - desc CH2SRC"]
pub type Ch2srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc CH1SRC"]
    #[inline(always)]
    pub fn ch1src(&self) -> Ch1srcR {
        Ch1srcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc CH2SRC"]
    #[inline(always)]
    pub fn ch2src(&self) -> Ch2srcR {
        Ch2srcR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CH1SRC"]
    #[inline(always)]
    pub fn ch1src(&mut self) -> Ch1srcW<'_, Cr1Spec> {
        Ch1srcW::new(self, 0)
    }
    #[doc = "Bits 3:5 - desc CH2SRC"]
    #[inline(always)]
    pub fn ch2src(&mut self) -> Ch2srcW<'_, Cr1Spec> {
        Ch2srcW::new(self, 3)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
