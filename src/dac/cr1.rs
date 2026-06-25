#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `C1OUT` reader - desc C1OUT"]
pub type C1outR = crate::BitReader;
#[doc = "Field `C1OUT` writer - desc C1OUT"]
pub type C1outW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2OUT` reader - desc C2OUT"]
pub type C2outR = crate::BitReader;
#[doc = "Field `C2OUT` writer - desc C2OUT"]
pub type C2outW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc C1OUT"]
    #[inline(always)]
    pub fn c1out(&self) -> C1outR {
        C1outR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc C2OUT"]
    #[inline(always)]
    pub fn c2out(&self) -> C2outR {
        C2outR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc C1OUT"]
    #[inline(always)]
    pub fn c1out(&mut self) -> C1outW<'_, Cr1Spec> {
        C1outW::new(self, 0)
    }
    #[doc = "Bit 1 - desc C2OUT"]
    #[inline(always)]
    pub fn c2out(&mut self) -> C2outW<'_, Cr1Spec> {
        C2outW::new(self, 1)
    }
}
#[doc = "Result register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
