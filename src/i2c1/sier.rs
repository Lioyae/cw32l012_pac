#[doc = "Register `SIER` reader"]
pub type R = crate::R<SierSpec>;
#[doc = "Register `SIER` writer"]
pub type W = crate::W<SierSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type AddrR = crate::BitReader;
#[doc = "Field `ADDR` writer - desc ADDR"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACK` reader - desc TACK"]
pub type TackR = crate::BitReader;
#[doc = "Field `TACK` writer - desc TACK"]
pub type TackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - desc RESTART"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - desc RESTART"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT` reader - desc BIT"]
pub type BitR = crate::BitReader;
#[doc = "Field `BIT` writer - desc BIT"]
pub type BitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO` reader - desc FIFO"]
pub type FifoR = crate::BitReader;
#[doc = "Field `FIFO` writer - desc FIFO"]
pub type FifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AM0` reader - desc AM0"]
pub type Am0R = crate::BitReader;
#[doc = "Field `AM0` writer - desc AM0"]
pub type Am0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AM1` reader - desc AM1"]
pub type Am1R = crate::BitReader;
#[doc = "Field `AM1` writer - desc AM1"]
pub type Am1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC` reader - desc GC"]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - desc GC"]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT` reader - desc ALERT"]
pub type AlertR = crate::BitReader;
#[doc = "Field `ALERT` writer - desc ALERT"]
pub type AlertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TACK"]
    #[inline(always)]
    pub fn tack(&self) -> TackR {
        TackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BIT"]
    #[inline(always)]
    pub fn bit(&self) -> BitR {
        BitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc FIFO"]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc AM0"]
    #[inline(always)]
    pub fn am0(&self) -> Am0R {
        Am0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc AM1"]
    #[inline(always)]
    pub fn am1(&self) -> Am1R {
        Am1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc GC"]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&self) -> AlertR {
        AlertR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, SierSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, SierSpec> {
        RxneW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SierSpec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TACK"]
    #[inline(always)]
    pub fn tack(&mut self) -> TackW<'_, SierSpec> {
        TackW::new(self, 3)
    }
    #[doc = "Bit 8 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, SierSpec> {
        RestartW::new(self, 8)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, SierSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BIT"]
    #[inline(always)]
    pub fn bit(&mut self) -> BitW<'_, SierSpec> {
        BitW::new(self, 10)
    }
    #[doc = "Bit 11 - desc FIFO"]
    #[inline(always)]
    pub fn fifo(&mut self) -> FifoW<'_, SierSpec> {
        FifoW::new(self, 11)
    }
    #[doc = "Bit 12 - desc AM0"]
    #[inline(always)]
    pub fn am0(&mut self) -> Am0W<'_, SierSpec> {
        Am0W::new(self, 12)
    }
    #[doc = "Bit 13 - desc AM1"]
    #[inline(always)]
    pub fn am1(&mut self) -> Am1W<'_, SierSpec> {
        Am1W::new(self, 13)
    }
    #[doc = "Bit 14 - desc GC"]
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<'_, SierSpec> {
        GcW::new(self, 14)
    }
    #[doc = "Bit 15 - desc ALERT"]
    #[inline(always)]
    pub fn alert(&mut self) -> AlertW<'_, SierSpec> {
        AlertW::new(self, 15)
    }
}
#[doc = "desc SIER\n\nYou can [`read`](crate::Reg::read) this register and get [`sier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SierSpec;
impl crate::RegisterSpec for SierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sier::R`](R) reader structure"]
impl crate::Readable for SierSpec {}
#[doc = "`write(|w| ..)` method takes [`sier::W`](W) writer structure"]
impl crate::Writable for SierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIER to value 0"]
impl crate::Resettable for SierSpec {}
