#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `FLTTIME` reader - desc FLTTIME"]
pub type FlttimeR = crate::FieldReader;
#[doc = "Field `FLTTIME` writer - desc FLTTIME"]
pub type FlttimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FltclkR = crate::BitReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FltclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLIE` reader - desc FALLIE"]
pub type FallieR = crate::BitReader;
#[doc = "Field `FALLIE` writer - desc FALLIE"]
pub type FallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEIE` reader - desc RISEIE"]
pub type RiseieR = crate::BitReader;
#[doc = "Field `RISEIE` writer - desc RISEIE"]
pub type RiseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHIE` reader - desc HIGHIE"]
pub type HighieR = crate::BitReader;
#[doc = "Field `HIGHIE` writer - desc HIGHIE"]
pub type HighieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKTIME` reader - desc BLANKTIME"]
pub type BlanktimeR = crate::FieldReader;
#[doc = "Field `BLANKTIME` writer - desc BLANKTIME"]
pub type BlanktimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLANKLVL` reader - desc BLANKLVL"]
pub type BlanklvlR = crate::BitReader;
#[doc = "Field `BLANKLVL` writer - desc BLANKLVL"]
pub type BlanklvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&self) -> FlttimeR {
        FlttimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&self) -> FallieR {
        FallieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&self) -> RiseieR {
        RiseieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&self) -> HighieR {
        HighieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc BLANKTIME"]
    #[inline(always)]
    pub fn blanktime(&self) -> BlanktimeR {
        BlanktimeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc BLANKLVL"]
    #[inline(always)]
    pub fn blanklvl(&self) -> BlanklvlR {
        BlanklvlR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&mut self) -> FlttimeW<'_, Cr1Spec> {
        FlttimeW::new(self, 0)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, Cr1Spec> {
        FltclkW::new(self, 4)
    }
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&mut self) -> FallieW<'_, Cr1Spec> {
        FallieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&mut self) -> RiseieW<'_, Cr1Spec> {
        RiseieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&mut self) -> HighieW<'_, Cr1Spec> {
        HighieW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc BLANKTIME"]
    #[inline(always)]
    pub fn blanktime(&mut self) -> BlanktimeW<'_, Cr1Spec> {
        BlanktimeW::new(self, 8)
    }
    #[doc = "Bit 11 - desc BLANKLVL"]
    #[inline(always)]
    pub fn blanklvl(&mut self) -> BlanklvlW<'_, Cr1Spec> {
        BlanklvlW::new(self, 11)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
