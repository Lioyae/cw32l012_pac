#[doc = "Register `SCR3` reader"]
pub type R = crate::R<Scr3Spec>;
#[doc = "Register `SCR3` writer"]
pub type W = crate::W<Scr3Spec>;
#[doc = "Field `CLKHOLD` reader - desc CLKHOLD"]
pub type ClkholdR = crate::FieldReader;
#[doc = "Field `CLKHOLD` writer - desc CLKHOLD"]
pub type ClkholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAVD` reader - desc DATAVD"]
pub type DatavdR = crate::FieldReader;
#[doc = "Field `DATAVD` writer - desc DATAVD"]
pub type DatavdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FLTSCL` reader - desc FLTSCL"]
pub type FltsclR = crate::FieldReader;
#[doc = "Field `FLTSCL` writer - desc FLTSCL"]
pub type FltsclW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLTSDA` reader - desc FLTSDA"]
pub type FltsdaR = crate::FieldReader;
#[doc = "Field `FLTSDA` writer - desc FLTSDA"]
pub type FltsdaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CLKHOLD"]
    #[inline(always)]
    pub fn clkhold(&self) -> ClkholdR {
        ClkholdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - desc DATAVD"]
    #[inline(always)]
    pub fn datavd(&self) -> DatavdR {
        DatavdR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - desc FLTSCL"]
    #[inline(always)]
    pub fn fltscl(&self) -> FltsclR {
        FltsclR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc FLTSDA"]
    #[inline(always)]
    pub fn fltsda(&self) -> FltsdaR {
        FltsdaR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CLKHOLD"]
    #[inline(always)]
    pub fn clkhold(&mut self) -> ClkholdW<'_, Scr3Spec> {
        ClkholdW::new(self, 0)
    }
    #[doc = "Bits 8:13 - desc DATAVD"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DatavdW<'_, Scr3Spec> {
        DatavdW::new(self, 8)
    }
    #[doc = "Bits 16:19 - desc FLTSCL"]
    #[inline(always)]
    pub fn fltscl(&mut self) -> FltsclW<'_, Scr3Spec> {
        FltsclW::new(self, 16)
    }
    #[doc = "Bits 24:27 - desc FLTSDA"]
    #[inline(always)]
    pub fn fltsda(&mut self) -> FltsdaW<'_, Scr3Spec> {
        FltsdaW::new(self, 24)
    }
}
#[doc = "desc SCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`scr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr3Spec;
impl crate::RegisterSpec for Scr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr3::R`](R) reader structure"]
impl crate::Readable for Scr3Spec {}
#[doc = "`write(|w| ..)` method takes [`scr3::W`](W) writer structure"]
impl crate::Writable for Scr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR3 to value 0"]
impl crate::Resettable for Scr3Spec {}
