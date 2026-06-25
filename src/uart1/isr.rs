#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `RC` reader - desc RC"]
pub type RcR = crate::BitReader;
#[doc = "Field `RXIDLE` reader - desc RXIDLE"]
pub type RxidleR = crate::BitReader;
#[doc = "Field `RXBRK` reader - desc RXBRK"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `BAUD` reader - desc BAUD"]
pub type BaudR = crate::BitReader;
#[doc = "Field `TIMOV` reader - desc TIMOV"]
pub type TimovR = crate::BitReader;
#[doc = "Field `CTS` reader - desc CTS"]
pub type CtsR = crate::BitReader;
#[doc = "Field `FE` reader - desc FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - desc PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `NE` reader - desc NE"]
pub type NeR = crate::BitReader;
#[doc = "Field `ORE` reader - desc ORE"]
pub type OreR = crate::BitReader;
#[doc = "Field `RXMATCH` reader - desc RXMATCH"]
pub type RxmatchR = crate::BitReader;
#[doc = "Field `SLVMATCH` reader - desc SLVMATCH"]
pub type SlvmatchR = crate::BitReader;
#[doc = "Field `TXBUSY` reader - desc TXBUSY"]
pub type TxbusyR = crate::BitReader;
#[doc = "Field `CTSLV` reader - desc CTSLV"]
pub type CtslvR = crate::BitReader;
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
    #[doc = "Bit 13 - desc SLVMATCH"]
    #[inline(always)]
    pub fn slvmatch(&self) -> SlvmatchR {
        SlvmatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc CTSLV"]
    #[inline(always)]
    pub fn ctslv(&self) -> CtslvR {
        CtslvR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
