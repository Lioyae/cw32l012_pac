#[doc = "Register `SISR` reader"]
pub type R = crate::R<SisrSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type AddrR = crate::BitReader;
#[doc = "Field `TACK` reader - desc TACK"]
pub type TackR = crate::BitReader;
#[doc = "Field `RESTART` reader - desc RESTART"]
pub type RestartR = crate::BitReader;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `BIT` reader - desc BIT"]
pub type BitR = crate::BitReader;
#[doc = "Field `FIFO` reader - desc FIFO"]
pub type FifoR = crate::BitReader;
#[doc = "Field `AM0` reader - desc AM0"]
pub type Am0R = crate::BitReader;
#[doc = "Field `AM1` reader - desc AM1"]
pub type Am1R = crate::BitReader;
#[doc = "Field `GC` reader - desc GC"]
pub type GcR = crate::BitReader;
#[doc = "Field `ALERT` reader - desc ALERT"]
pub type AlertR = crate::BitReader;
#[doc = "Field `SLVBUSY` reader - desc SLVBUSY"]
pub type SlvbusyR = crate::BitReader;
#[doc = "Field `BUSBUSY` reader - desc BUSBUSY"]
pub type BusbusyR = crate::BitReader;
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
    #[doc = "Bit 24 - desc SLVBUSY"]
    #[inline(always)]
    pub fn slvbusy(&self) -> SlvbusyR {
        SlvbusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&self) -> BusbusyR {
        BusbusyR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "desc SISR\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SisrSpec;
impl crate::RegisterSpec for SisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sisr::R`](R) reader structure"]
impl crate::Readable for SisrSpec {}
#[doc = "`reset()` method sets SISR to value 0"]
impl crate::Resettable for SisrSpec {}
