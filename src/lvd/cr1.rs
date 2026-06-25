#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL` reader - desc LEVEL"]
pub type LevelR = crate::BitReader;
#[doc = "Field `LEVEL` writer - desc LEVEL"]
pub type LevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISE` reader - desc RISE"]
pub type RiseR = crate::BitReader;
#[doc = "Field `RISE` writer - desc RISE"]
pub type RiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALL` reader - desc FALL"]
pub type FallR = crate::BitReader;
#[doc = "Field `FALL` writer - desc FALL"]
pub type FallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTTIME` reader - desc FLTTIME"]
pub type FlttimeR = crate::FieldReader;
#[doc = "Field `FLTTIME` writer - desc FLTTIME"]
pub type FlttimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RISE"]
    #[inline(always)]
    pub fn rise(&self) -> RiseR {
        RiseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FALL"]
    #[inline(always)]
    pub fn fall(&self) -> FallR {
        FallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&self) -> FlttimeR {
        FlttimeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, Cr1Spec> {
        IeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, Cr1Spec> {
        LevelW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RISE"]
    #[inline(always)]
    pub fn rise(&mut self) -> RiseW<'_, Cr1Spec> {
        RiseW::new(self, 2)
    }
    #[doc = "Bit 3 - desc FALL"]
    #[inline(always)]
    pub fn fall(&mut self) -> FallW<'_, Cr1Spec> {
        FallW::new(self, 3)
    }
    #[doc = "Bits 4:7 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&mut self) -> FlttimeW<'_, Cr1Spec> {
        FlttimeW::new(self, 4)
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
