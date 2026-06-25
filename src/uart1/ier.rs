#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - desc TC"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - desc RC"]
pub type RcR = crate::BitReader;
#[doc = "Field `RC` writer - desc RC"]
pub type RcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIDLE` reader - desc RXIDLE"]
pub type RxidleR = crate::BitReader;
#[doc = "Field `RXIDLE` writer - desc RXIDLE"]
pub type RxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` reader - desc RXBRK"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `RXBRK` writer - desc RXBRK"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD` reader - desc BAUD"]
pub type BaudR = crate::BitReader;
#[doc = "Field `BAUD` writer - desc BAUD"]
pub type BaudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMOV` reader - desc TIMOV"]
pub type TimovR = crate::BitReader;
#[doc = "Field `TIMOV` writer - desc TIMOV"]
pub type TimovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - desc CTS"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - desc CTS"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - desc FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - desc FE"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - desc PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - desc PE"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NE` reader - desc NE"]
pub type NeR = crate::BitReader;
#[doc = "Field `NE` writer - desc NE"]
pub type NeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORE` reader - desc ORE"]
pub type OreR = crate::BitReader;
#[doc = "Field `ORE` writer - desc ORE"]
pub type OreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCH` reader - desc RXMATCH"]
pub type RxmatchR = crate::BitReader;
#[doc = "Field `RXMATCH` writer - desc RXMATCH"]
pub type RxmatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RC"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RXIDLE"]
    #[inline(always)]
    pub fn rxidle(&self) -> RxidleR {
        RxidleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXBRK"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BAUD"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIMOV"]
    #[inline(always)]
    pub fn timov(&self) -> TimovR {
        TimovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc NE"]
    #[inline(always)]
    pub fn ne(&self) -> NeR {
        NeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc ORE"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc RXMATCH"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RxmatchR {
        RxmatchR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, IerSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, IerSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RC"]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<'_, IerSpec> {
        RcW::new(self, 2)
    }
    #[doc = "Bit 3 - desc RXIDLE"]
    #[inline(always)]
    pub fn rxidle(&mut self) -> RxidleW<'_, IerSpec> {
        RxidleW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RXBRK"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RxbrkW<'_, IerSpec> {
        RxbrkW::new(self, 4)
    }
    #[doc = "Bit 5 - desc BAUD"]
    #[inline(always)]
    pub fn baud(&mut self) -> BaudW<'_, IerSpec> {
        BaudW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TIMOV"]
    #[inline(always)]
    pub fn timov(&mut self) -> TimovW<'_, IerSpec> {
        TimovW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IerSpec> {
        CtsW::new(self, 7)
    }
    #[doc = "Bit 8 - desc FE"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, IerSpec> {
        FeW::new(self, 8)
    }
    #[doc = "Bit 9 - desc PE"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, IerSpec> {
        PeW::new(self, 9)
    }
    #[doc = "Bit 10 - desc NE"]
    #[inline(always)]
    pub fn ne(&mut self) -> NeW<'_, IerSpec> {
        NeW::new(self, 10)
    }
    #[doc = "Bit 11 - desc ORE"]
    #[inline(always)]
    pub fn ore(&mut self) -> OreW<'_, IerSpec> {
        OreW::new(self, 11)
    }
    #[doc = "Bit 12 - desc RXMATCH"]
    #[inline(always)]
    pub fn rxmatch(&mut self) -> RxmatchW<'_, IerSpec> {
        RxmatchW::new(self, 12)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
