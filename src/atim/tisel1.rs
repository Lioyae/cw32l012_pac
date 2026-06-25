#[doc = "Register `TISEL1` reader"]
pub type R = crate::R<Tisel1Spec>;
#[doc = "Register `TISEL1` writer"]
pub type W = crate::W<Tisel1Spec>;
#[doc = "Field `TI1SEL` reader - desc TI1SEL"]
pub type Ti1selR = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - desc TI1SEL"]
pub type Ti1selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL` reader - desc TI2SEL"]
pub type Ti2selR = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - desc TI2SEL"]
pub type Ti2selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI3SEL` reader - desc TI3SEL"]
pub type Ti3selR = crate::FieldReader;
#[doc = "Field `TI3SEL` writer - desc TI3SEL"]
pub type Ti3selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI4SEL` reader - desc TI4SEL"]
pub type Ti4selR = crate::FieldReader;
#[doc = "Field `TI4SEL` writer - desc TI4SEL"]
pub type Ti4selW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> Ti1selR {
        Ti1selR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&self) -> Ti2selR {
        Ti2selR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc TI3SEL"]
    #[inline(always)]
    pub fn ti3sel(&self) -> Ti3selR {
        Ti3selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc TI4SEL"]
    #[inline(always)]
    pub fn ti4sel(&self) -> Ti4selR {
        Ti4selR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> Ti1selW<'_, Tisel1Spec> {
        Ti1selW::new(self, 0)
    }
    #[doc = "Bits 8:11 - desc TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> Ti2selW<'_, Tisel1Spec> {
        Ti2selW::new(self, 8)
    }
    #[doc = "Bits 16:19 - desc TI3SEL"]
    #[inline(always)]
    pub fn ti3sel(&mut self) -> Ti3selW<'_, Tisel1Spec> {
        Ti3selW::new(self, 16)
    }
    #[doc = "Bits 24:27 - desc TI4SEL"]
    #[inline(always)]
    pub fn ti4sel(&mut self) -> Ti4selW<'_, Tisel1Spec> {
        Ti4selW::new(self, 24)
    }
}
#[doc = "Timer Input Select Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tisel1Spec;
impl crate::RegisterSpec for Tisel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel1::R`](R) reader structure"]
impl crate::Readable for Tisel1Spec {}
#[doc = "`write(|w| ..)` method takes [`tisel1::W`](W) writer structure"]
impl crate::Writable for Tisel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TISEL1 to value 0"]
impl crate::Resettable for Tisel1Spec {}
