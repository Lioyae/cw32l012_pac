#[doc = "Register `MISR` reader"]
pub type R = crate::R<MisrSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `PACKET` reader - desc PACKET"]
pub type PacketR = crate::BitReader;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `NACK` reader - desc NACK"]
pub type NackR = crate::BitReader;
#[doc = "Field `ARBI` reader - desc ARBI"]
pub type ArbiR = crate::BitReader;
#[doc = "Field `FIFO` reader - desc FIFO"]
pub type FifoR = crate::BitReader;
#[doc = "Field `PINLOW` reader - desc PINLOW"]
pub type PinlowR = crate::BitReader;
#[doc = "Field `MATCH` reader - desc MATCH"]
pub type MatchR = crate::BitReader;
#[doc = "Field `MSTBUSY` reader - desc MSTBUSY"]
pub type MstbusyR = crate::BitReader;
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
    #[doc = "Bit 24 - desc MSTBUSY"]
    #[inline(always)]
    pub fn mstbusy(&self) -> MstbusyR {
        MstbusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc BUSBUSY"]
    #[inline(always)]
    pub fn busbusy(&self) -> BusbusyR {
        BusbusyR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrSpec;
impl crate::RegisterSpec for MisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MisrSpec {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MisrSpec {}
