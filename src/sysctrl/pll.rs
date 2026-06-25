#[doc = "Register `PLL` reader"]
pub type R = crate::R<PllSpec>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PllSpec>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::BitReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WaitcycleR = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MUL` reader - desc MUL"]
pub type MulR = crate::FieldReader;
#[doc = "Field `MUL` writer - desc MUL"]
pub type MulW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WaitcycleR {
        WaitcycleR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - desc MUL"]
    #[inline(always)]
    pub fn mul(&self) -> MulR {
        MulR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, PllSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bits 1:4 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, PllSpec> {
        DivW::new(self, 1)
    }
    #[doc = "Bits 5:7 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&mut self) -> WaitcycleW<'_, PllSpec> {
        WaitcycleW::new(self, 5)
    }
    #[doc = "Bits 8:15 - desc MUL"]
    #[inline(always)]
    pub fn mul(&mut self) -> MulW<'_, PllSpec> {
        MulW::new(self, 8)
    }
}
#[doc = "PLL Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSpec;
impl crate::RegisterSpec for PllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PllSpec {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PllSpec {}
