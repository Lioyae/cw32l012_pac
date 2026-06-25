#[doc = "Register `SICR` reader"]
pub type R = crate::R<SicrSpec>;
#[doc = "Register `SICR` writer"]
pub type W = crate::W<SicrSpec>;
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
impl R {
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
}
impl W {
    #[doc = "Bit 8 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, SicrSpec> {
        RestartW::new(self, 8)
    }
    #[doc = "Bit 9 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, SicrSpec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BIT"]
    #[inline(always)]
    pub fn bit(&mut self) -> BitW<'_, SicrSpec> {
        BitW::new(self, 10)
    }
    #[doc = "Bit 11 - desc FIFO"]
    #[inline(always)]
    pub fn fifo(&mut self) -> FifoW<'_, SicrSpec> {
        FifoW::new(self, 11)
    }
}
#[doc = "desc SICR\n\nYou can [`read`](crate::Reg::read) this register and get [`sicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SicrSpec;
impl crate::RegisterSpec for SicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sicr::R`](R) reader structure"]
impl crate::Readable for SicrSpec {}
#[doc = "`write(|w| ..)` method takes [`sicr::W`](W) writer structure"]
impl crate::Writable for SicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SicrSpec {}
