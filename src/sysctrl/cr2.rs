#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SWDIO` reader - desc SWDIO"]
pub type SwdioR = crate::BitReader;
#[doc = "Field `SWDIO` writer - desc SWDIO"]
pub type SwdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LockupR = crate::BitReader;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPCLK` reader - desc WAKEUPCLK"]
pub type WakeupclkR = crate::BitReader;
#[doc = "Field `WAKEUPCLK` writer - desc WAKEUPCLK"]
pub type WakeupclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHWAIT` reader - desc FLASHWAIT"]
pub type FlashwaitR = crate::FieldReader;
#[doc = "Field `FLASHWAIT` writer - desc FLASHWAIT"]
pub type FlashwaitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSEBRKEN` reader - desc HSEBRKEN"]
pub type HsebrkenR = crate::BitReader;
#[doc = "Field `HSEBRKEN` writer - desc HSEBRKEN"]
pub type HsebrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBRKEN` reader - desc LSEBRKEN"]
pub type LsebrkenR = crate::BitReader;
#[doc = "Field `LSEBRKEN` writer - desc LSEBRKEN"]
pub type LsebrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLLBRKEN` reader - desc CLLBRKEN"]
pub type CllbrkenR = crate::BitReader;
#[doc = "Field `CLLBRKEN` writer - desc CLLBRKEN"]
pub type CllbrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSBRKEN` reader - desc DSBRKEN"]
pub type DsbrkenR = crate::BitReader;
#[doc = "Field `DSBRKEN` writer - desc DSBRKEN"]
pub type DsbrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDBRKEN` reader - desc LVDBRKEN"]
pub type LvdbrkenR = crate::BitReader;
#[doc = "Field `LVDBRKEN` writer - desc LVDBRKEN"]
pub type LvdbrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMBRKEN` reader - desc RAMBRKEN"]
pub type RambrkenR = crate::BitReader;
#[doc = "Field `RAMBRKEN` writer - desc RAMBRKEN"]
pub type RambrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    pub fn swdio(&self) -> SwdioR {
        SwdioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    pub fn wakeupclk(&self) -> WakeupclkR {
        WakeupclkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc FLASHWAIT"]
    #[inline(always)]
    pub fn flashwait(&self) -> FlashwaitR {
        FlashwaitR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 10 - desc HSEBRKEN"]
    #[inline(always)]
    pub fn hsebrken(&self) -> HsebrkenR {
        HsebrkenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc LSEBRKEN"]
    #[inline(always)]
    pub fn lsebrken(&self) -> LsebrkenR {
        LsebrkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CLLBRKEN"]
    #[inline(always)]
    pub fn cllbrken(&self) -> CllbrkenR {
        CllbrkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DSBRKEN"]
    #[inline(always)]
    pub fn dsbrken(&self) -> DsbrkenR {
        DsbrkenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LVDBRKEN"]
    #[inline(always)]
    pub fn lvdbrken(&self) -> LvdbrkenR {
        LvdbrkenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc RAMBRKEN"]
    #[inline(always)]
    pub fn rambrken(&self) -> RambrkenR {
        RambrkenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc SWDIO"]
    #[inline(always)]
    pub fn swdio(&mut self) -> SwdioW<'_, Cr2Spec> {
        SwdioW::new(self, 1)
    }
    #[doc = "Bit 2 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, Cr2Spec> {
        LockupW::new(self, 2)
    }
    #[doc = "Bit 3 - desc WAKEUPCLK"]
    #[inline(always)]
    pub fn wakeupclk(&mut self) -> WakeupclkW<'_, Cr2Spec> {
        WakeupclkW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc FLASHWAIT"]
    #[inline(always)]
    pub fn flashwait(&mut self) -> FlashwaitW<'_, Cr2Spec> {
        FlashwaitW::new(self, 4)
    }
    #[doc = "Bit 10 - desc HSEBRKEN"]
    #[inline(always)]
    pub fn hsebrken(&mut self) -> HsebrkenW<'_, Cr2Spec> {
        HsebrkenW::new(self, 10)
    }
    #[doc = "Bit 11 - desc LSEBRKEN"]
    #[inline(always)]
    pub fn lsebrken(&mut self) -> LsebrkenW<'_, Cr2Spec> {
        LsebrkenW::new(self, 11)
    }
    #[doc = "Bit 12 - desc CLLBRKEN"]
    #[inline(always)]
    pub fn cllbrken(&mut self) -> CllbrkenW<'_, Cr2Spec> {
        CllbrkenW::new(self, 12)
    }
    #[doc = "Bit 13 - desc DSBRKEN"]
    #[inline(always)]
    pub fn dsbrken(&mut self) -> DsbrkenW<'_, Cr2Spec> {
        DsbrkenW::new(self, 13)
    }
    #[doc = "Bit 14 - desc LVDBRKEN"]
    #[inline(always)]
    pub fn lvdbrken(&mut self) -> LvdbrkenW<'_, Cr2Spec> {
        LvdbrkenW::new(self, 14)
    }
    #[doc = "Bit 15 - desc RAMBRKEN"]
    #[inline(always)]
    pub fn rambrken(&mut self) -> RambrkenW<'_, Cr2Spec> {
        RambrkenW::new(self, 15)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Cr2Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
