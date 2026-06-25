#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `PC` reader - desc PC"]
pub type PcR = crate::BitReader;
#[doc = "Field `PAGELOCK` reader - desc PAGELOCK"]
pub type PagelockR = crate::BitReader;
#[doc = "Field `SDKERR` reader - desc SDKERR"]
pub type SdkerrR = crate::BitReader;
#[doc = "Field `CACHEON` reader - desc CACHEON"]
pub type CacheonR = crate::BitReader;
#[doc = "Field `PROG` reader - desc PROG"]
pub type ProgR = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&self) -> PagelockR {
        PagelockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SDKERR"]
    #[inline(always)]
    pub fn sdkerr(&self) -> SdkerrR {
        SdkerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CACHEON"]
    #[inline(always)]
    pub fn cacheon(&self) -> CacheonR {
        CacheonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
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
