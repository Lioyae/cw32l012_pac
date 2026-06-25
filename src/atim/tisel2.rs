#[doc = "Register `TISEL2` reader"]
pub type R = crate::R<Tisel2Spec>;
#[doc = "Register `TISEL2` writer"]
pub type W = crate::W<Tisel2Spec>;
#[doc = "Field `TI5SEL` reader - desc TI5SEL"]
pub type Ti5selR = crate::FieldReader;
#[doc = "Field `TI5SEL` writer - desc TI5SEL"]
pub type Ti5selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI6SEL` reader - desc TI6SEL"]
pub type Ti6selR = crate::FieldReader;
#[doc = "Field `TI6SEL` writer - desc TI6SEL"]
pub type Ti6selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc TI5SEL"]
    #[inline(always)]
    pub fn ti5sel(&self) -> Ti5selR {
        Ti5selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc TI6SEL"]
    #[inline(always)]
    pub fn ti6sel(&self) -> Ti6selR {
        Ti6selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc TI5SEL"]
    #[inline(always)]
    pub fn ti5sel(&mut self) -> Ti5selW<'_, Tisel2Spec> {
        Ti5selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - desc TI6SEL"]
    #[inline(always)]
    pub fn ti6sel(&mut self) -> Ti6selW<'_, Tisel2Spec> {
        Ti6selW::new(self, 8)
    }
}
#[doc = "Timer Input Select Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tisel2Spec;
impl crate::RegisterSpec for Tisel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel2::R`](R) reader structure"]
impl crate::Readable for Tisel2Spec {}
#[doc = "`write(|w| ..)` method takes [`tisel2::W`](W) writer structure"]
impl crate::Writable for Tisel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TISEL2 to value 0"]
impl crate::Resettable for Tisel2Spec {}
