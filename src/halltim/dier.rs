#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `CAPIE` reader - desc CAPIE"]
pub type CapieR = crate::BitReader;
#[doc = "Field `CAPIE` writer - desc CAPIE"]
pub type CapieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIE` reader - desc OVIE"]
pub type OvieR = crate::BitReader;
#[doc = "Field `OVIE` writer - desc OVIE"]
pub type OvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHIE` reader - desc MATCHIE"]
pub type MatchieR = crate::BitReader;
#[doc = "Field `MATCHIE` writer - desc MATCHIE"]
pub type MatchieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPDMA` reader - desc CAPDMA"]
pub type CapdmaR = crate::BitReader;
#[doc = "Field `CAPDMA` writer - desc CAPDMA"]
pub type CapdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVDMA` reader - desc OVDMA"]
pub type OvdmaR = crate::BitReader;
#[doc = "Field `OVDMA` writer - desc OVDMA"]
pub type OvdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHDMA` reader - desc MATCHDMA"]
pub type MatchdmaR = crate::BitReader;
#[doc = "Field `MATCHDMA` writer - desc MATCHDMA"]
pub type MatchdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTDMA` reader - desc SOFTDMA"]
pub type SoftdmaR = crate::BitReader;
#[doc = "Field `SOFTDMA` writer - desc SOFTDMA"]
pub type SoftdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CAPIE"]
    #[inline(always)]
    pub fn capie(&self) -> CapieR {
        CapieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OVIE"]
    #[inline(always)]
    pub fn ovie(&self) -> OvieR {
        OvieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MATCHIE"]
    #[inline(always)]
    pub fn matchie(&self) -> MatchieR {
        MatchieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CAPDMA"]
    #[inline(always)]
    pub fn capdma(&self) -> CapdmaR {
        CapdmaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OVDMA"]
    #[inline(always)]
    pub fn ovdma(&self) -> OvdmaR {
        OvdmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc MATCHDMA"]
    #[inline(always)]
    pub fn matchdma(&self) -> MatchdmaR {
        MatchdmaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SOFTDMA"]
    #[inline(always)]
    pub fn softdma(&self) -> SoftdmaR {
        SoftdmaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAPIE"]
    #[inline(always)]
    pub fn capie(&mut self) -> CapieW<'_, DierSpec> {
        CapieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OVIE"]
    #[inline(always)]
    pub fn ovie(&mut self) -> OvieW<'_, DierSpec> {
        OvieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc MATCHIE"]
    #[inline(always)]
    pub fn matchie(&mut self) -> MatchieW<'_, DierSpec> {
        MatchieW::new(self, 2)
    }
    #[doc = "Bit 4 - desc CAPDMA"]
    #[inline(always)]
    pub fn capdma(&mut self) -> CapdmaW<'_, DierSpec> {
        CapdmaW::new(self, 4)
    }
    #[doc = "Bit 5 - desc OVDMA"]
    #[inline(always)]
    pub fn ovdma(&mut self) -> OvdmaW<'_, DierSpec> {
        OvdmaW::new(self, 5)
    }
    #[doc = "Bit 6 - desc MATCHDMA"]
    #[inline(always)]
    pub fn matchdma(&mut self) -> MatchdmaW<'_, DierSpec> {
        MatchdmaW::new(self, 6)
    }
    #[doc = "Bit 7 - desc SOFTDMA"]
    #[inline(always)]
    pub fn softdma(&mut self) -> SoftdmaW<'_, DierSpec> {
        SoftdmaW::new(self, 7)
    }
}
#[doc = "DMA and Interrupt register2\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}
