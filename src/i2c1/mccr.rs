#[doc = "Register `MCCR` reader"]
pub type R = crate::R<MccrSpec>;
#[doc = "Register `MCCR` writer"]
pub type W = crate::W<MccrSpec>;
#[doc = "Field `CLKLO` reader - desc CLKLO"]
pub type ClkloR = crate::FieldReader;
#[doc = "Field `CLKLO` writer - desc CLKLO"]
pub type ClkloW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLKHI` reader - desc CLKHI"]
pub type ClkhiR = crate::FieldReader;
#[doc = "Field `CLKHI` writer - desc CLKHI"]
pub type ClkhiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SETHOLD` reader - desc SETHOLD"]
pub type SetholdR = crate::FieldReader;
#[doc = "Field `SETHOLD` writer - desc SETHOLD"]
pub type SetholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DATAVD` reader - desc DATAVD"]
pub type DatavdR = crate::FieldReader;
#[doc = "Field `DATAVD` writer - desc DATAVD"]
pub type DatavdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - desc CLKLO"]
    #[inline(always)]
    pub fn clklo(&self) -> ClkloR {
        ClkloR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - desc CLKHI"]
    #[inline(always)]
    pub fn clkhi(&self) -> ClkhiR {
        ClkhiR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc SETHOLD"]
    #[inline(always)]
    pub fn sethold(&self) -> SetholdR {
        SetholdR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - desc DATAVD"]
    #[inline(always)]
    pub fn datavd(&self) -> DatavdR {
        DatavdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc CLKLO"]
    #[inline(always)]
    pub fn clklo(&mut self) -> ClkloW<'_, MccrSpec> {
        ClkloW::new(self, 0)
    }
    #[doc = "Bits 8:13 - desc CLKHI"]
    #[inline(always)]
    pub fn clkhi(&mut self) -> ClkhiW<'_, MccrSpec> {
        ClkhiW::new(self, 8)
    }
    #[doc = "Bits 16:21 - desc SETHOLD"]
    #[inline(always)]
    pub fn sethold(&mut self) -> SetholdW<'_, MccrSpec> {
        SetholdW::new(self, 16)
    }
    #[doc = "Bits 24:29 - desc DATAVD"]
    #[inline(always)]
    pub fn datavd(&mut self) -> DatavdW<'_, MccrSpec> {
        DatavdW::new(self, 24)
    }
}
#[doc = "Master Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MccrSpec;
impl crate::RegisterSpec for MccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mccr::R`](R) reader structure"]
impl crate::Readable for MccrSpec {}
#[doc = "`write(|w| ..)` method takes [`mccr::W`](W) writer structure"]
impl crate::Writable for MccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCCR to value 0"]
impl crate::Resettable for MccrSpec {}
