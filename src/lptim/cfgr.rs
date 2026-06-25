#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CkselR = crate::BitReader;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCMD_CKPOL` reader - desc ENCMD_CKPOL"]
pub type EncmdCkpolR = crate::FieldReader;
#[doc = "Field `ENCMD_CKPOL` writer - desc ENCMD_CKPOL"]
pub type EncmdCkpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHFLT` reader - desc CHFLT"]
pub type ChfltR = crate::FieldReader;
#[doc = "Field `CHFLT` writer - desc CHFLT"]
pub type ChfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIGFLT` reader - desc TRIGFLT"]
pub type TrigfltR = crate::FieldReader;
#[doc = "Field `TRIGFLT` writer - desc TRIGFLT"]
pub type TrigfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIGSEL` reader - desc TRIGSEL"]
pub type TrigselR = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - desc TRIGSEL"]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGEN` reader - desc TRIGEN"]
pub type TrigenR = crate::FieldReader;
#[doc = "Field `TRIGEN` writer - desc TRIGEN"]
pub type TrigenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMOUT` reader - desc TIMOUT"]
pub type TimoutR = crate::BitReader;
#[doc = "Field `TIMOUT` writer - desc TIMOUT"]
pub type TimoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVE` reader - desc WAVE"]
pub type WaveR = crate::BitReader;
#[doc = "Field `WAVE` writer - desc WAVE"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVPOL` reader - desc WAVPOL"]
pub type WavpolR = crate::BitReader;
#[doc = "Field `WAVPOL` writer - desc WAVPOL"]
pub type WavpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELOAD` reader - desc PRELOAD"]
pub type PreloadR = crate::BitReader;
#[doc = "Field `PRELOAD` writer - desc PRELOAD"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COUNTMD` reader - desc COUNTMD"]
pub type CountmdR = crate::BitReader;
#[doc = "Field `COUNTMD` writer - desc COUNTMD"]
pub type CountmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - desc ENC"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - desc ENC"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICLKSRC` reader - desc ICLKSRC"]
pub type IclksrcR = crate::FieldReader;
#[doc = "Field `ICLKSRC` writer - desc ICLKSRC"]
pub type IclksrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc ENCMD_CKPOL"]
    #[inline(always)]
    pub fn encmd_ckpol(&self) -> EncmdCkpolR {
        EncmdCkpolR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - desc CHFLT"]
    #[inline(always)]
    pub fn chflt(&self) -> ChfltR {
        ChfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc TRIGFLT"]
    #[inline(always)]
    pub fn trigflt(&self) -> TrigfltR {
        TrigfltR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - desc TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - desc TRIGEN"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - desc TIMOUT"]
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&self) -> WavpolR {
        WavpolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc PRELOAD"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc COUNTMD"]
    #[inline(always)]
    pub fn countmd(&self) -> CountmdR {
        CountmdR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc ENC"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc ICLKSRC"]
    #[inline(always)]
    pub fn iclksrc(&self) -> IclksrcR {
        IclksrcR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<'_, CfgrSpec> {
        CkselW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc ENCMD_CKPOL"]
    #[inline(always)]
    pub fn encmd_ckpol(&mut self) -> EncmdCkpolW<'_, CfgrSpec> {
        EncmdCkpolW::new(self, 1)
    }
    #[doc = "Bits 3:4 - desc CHFLT"]
    #[inline(always)]
    pub fn chflt(&mut self) -> ChfltW<'_, CfgrSpec> {
        ChfltW::new(self, 3)
    }
    #[doc = "Bits 6:7 - desc TRIGFLT"]
    #[inline(always)]
    pub fn trigflt(&mut self) -> TrigfltW<'_, CfgrSpec> {
        TrigfltW::new(self, 6)
    }
    #[doc = "Bits 9:11 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, CfgrSpec> {
        PrsW::new(self, 9)
    }
    #[doc = "Bits 12:15 - desc TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<'_, CfgrSpec> {
        TrigselW::new(self, 12)
    }
    #[doc = "Bits 17:18 - desc TRIGEN"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<'_, CfgrSpec> {
        TrigenW::new(self, 17)
    }
    #[doc = "Bit 19 - desc TIMOUT"]
    #[inline(always)]
    pub fn timout(&mut self) -> TimoutW<'_, CfgrSpec> {
        TimoutW::new(self, 19)
    }
    #[doc = "Bit 20 - desc WAVE"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, CfgrSpec> {
        WaveW::new(self, 20)
    }
    #[doc = "Bit 21 - desc WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WavpolW<'_, CfgrSpec> {
        WavpolW::new(self, 21)
    }
    #[doc = "Bit 22 - desc PRELOAD"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<'_, CfgrSpec> {
        PreloadW::new(self, 22)
    }
    #[doc = "Bit 23 - desc COUNTMD"]
    #[inline(always)]
    pub fn countmd(&mut self) -> CountmdW<'_, CfgrSpec> {
        CountmdW::new(self, 23)
    }
    #[doc = "Bit 24 - desc ENC"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, CfgrSpec> {
        EncW::new(self, 24)
    }
    #[doc = "Bits 25:26 - desc ICLKSRC"]
    #[inline(always)]
    pub fn iclksrc(&mut self) -> IclksrcW<'_, CfgrSpec> {
        IclksrcW::new(self, 25)
    }
}
#[doc = "desc CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
