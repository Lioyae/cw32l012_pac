#[doc = "Register `MCR3` reader"]
pub type R = crate::R<Mcr3Spec>;
#[doc = "Register `MCR3` writer"]
pub type W = crate::W<Mcr3Spec>;
#[doc = "Field `BUSIDLE` reader - desc BUSIDLE"]
pub type BusidleR = crate::FieldReader<u16>;
#[doc = "Field `BUSIDLE` writer - desc BUSIDLE"]
pub type BusidleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FLTSCL` reader - desc FLTSCL"]
pub type FltsclR = crate::FieldReader;
#[doc = "Field `FLTSCL` writer - desc FLTSCL"]
pub type FltsclW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLTSDA` reader - desc FLTSDA"]
pub type FltsdaR = crate::FieldReader;
#[doc = "Field `FLTSDA` writer - desc FLTSDA"]
pub type FltsdaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - desc BUSIDLE"]
    #[inline(always)]
    pub fn busidle(&self) -> BusidleR {
        BusidleR::new((self.bits & 0x0fff) as u16)
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
    #[doc = "Bits 0:11 - desc BUSIDLE"]
    #[inline(always)]
    pub fn busidle(&mut self) -> BusidleW<'_, Mcr3Spec> {
        BusidleW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc FLTSCL"]
    #[inline(always)]
    pub fn fltscl(&mut self) -> FltsclW<'_, Mcr3Spec> {
        FltsclW::new(self, 16)
    }
    #[doc = "Bits 24:27 - desc FLTSDA"]
    #[inline(always)]
    pub fn fltsda(&mut self) -> FltsdaW<'_, Mcr3Spec> {
        FltsdaW::new(self, 24)
    }
}
#[doc = "Master Control Register3\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr3Spec;
impl crate::RegisterSpec for Mcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr3::R`](R) reader structure"]
impl crate::Readable for Mcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr3::W`](W) writer structure"]
impl crate::Writable for Mcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR3 to value 0"]
impl crate::Resettable for Mcr3Spec {}
