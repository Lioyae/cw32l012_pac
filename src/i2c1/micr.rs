#[doc = "Register `MICR` reader"]
pub type R = crate::R<MicrSpec>;
#[doc = "Register `MICR` writer"]
pub type W = crate::W<MicrSpec>;
#[doc = "Field `PACKET` reader - desc PACKET"]
pub type PacketR = crate::BitReader;
#[doc = "Field `PACKET` writer - desc PACKET"]
pub type PacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - desc NACK"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - desc NACK"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBI` reader - desc ARBI"]
pub type ArbiR = crate::BitReader;
#[doc = "Field `ARBI` writer - desc ARBI"]
pub type ArbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO` reader - desc FIFO"]
pub type FifoR = crate::BitReader;
#[doc = "Field `FIFO` writer - desc FIFO"]
pub type FifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINLOW` reader - desc PINLOW"]
pub type PinlowR = crate::BitReader;
#[doc = "Field `PINLOW` writer - desc PINLOW"]
pub type PinlowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH` reader - desc MATCH"]
pub type MatchR = crate::BitReader;
#[doc = "Field `MATCH` writer - desc MATCH"]
pub type MatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - desc PACKET"]
    #[inline(always)]
    pub fn packet(&self) -> PacketR {
        PacketR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc NACK"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ARBI"]
    #[inline(always)]
    pub fn arbi(&self) -> ArbiR {
        ArbiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc FIFO"]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PINLOW"]
    #[inline(always)]
    pub fn pinlow(&self) -> PinlowR {
        PinlowR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc MATCH"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - desc PACKET"]
    #[inline(always)]
    pub fn packet(&mut self) -> PacketW<'_, MicrSpec> {
        PacketW::new(self, 8)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, MicrSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - desc NACK"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, MicrSpec> {
        NackW::new(self, 10)
    }
    #[doc = "Bit 11 - desc ARBI"]
    #[inline(always)]
    pub fn arbi(&mut self) -> ArbiW<'_, MicrSpec> {
        ArbiW::new(self, 11)
    }
    #[doc = "Bit 12 - desc FIFO"]
    #[inline(always)]
    pub fn fifo(&mut self) -> FifoW<'_, MicrSpec> {
        FifoW::new(self, 12)
    }
    #[doc = "Bit 13 - desc PINLOW"]
    #[inline(always)]
    pub fn pinlow(&mut self) -> PinlowW<'_, MicrSpec> {
        PinlowW::new(self, 13)
    }
    #[doc = "Bit 14 - desc MATCH"]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<'_, MicrSpec> {
        MatchW::new(self, 14)
    }
}
#[doc = "Master Interrupt Flag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`micr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`micr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MicrSpec;
impl crate::RegisterSpec for MicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`micr::R`](R) reader structure"]
impl crate::Readable for MicrSpec {}
#[doc = "`write(|w| ..)` method takes [`micr::W`](W) writer structure"]
impl crate::Writable for MicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MicrSpec {}
