#[doc = "Register `EGR` reader"]
pub type R = crate::R<EgrSpec>;
#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` reader - desc UG"]
pub type UgR = crate::BitReader;
#[doc = "Field `UG` writer - desc UG"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` reader - desc CC1G"]
pub type Cc1gR = crate::BitReader;
#[doc = "Field `CC1G` writer - desc CC1G"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` reader - desc CC2G"]
pub type Cc2gR = crate::BitReader;
#[doc = "Field `CC2G` writer - desc CC2G"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` reader - desc CC3G"]
pub type Cc3gR = crate::BitReader;
#[doc = "Field `CC3G` writer - desc CC3G"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` reader - desc CC4G"]
pub type Cc4gR = crate::BitReader;
#[doc = "Field `CC4G` writer - desc CC4G"]
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` reader - desc COMG"]
pub type ComgR = crate::BitReader;
#[doc = "Field `COMG` writer - desc COMG"]
pub type ComgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` reader - desc TG"]
pub type TgR = crate::BitReader;
#[doc = "Field `TG` writer - desc TG"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` reader - desc BG"]
pub type BgR = crate::BitReader;
#[doc = "Field `BG` writer - desc BG"]
pub type BgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2G` reader - desc B2G"]
pub type B2gR = crate::BitReader;
#[doc = "Field `B2G` writer - desc B2G"]
pub type B2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5G` reader - desc CC5G"]
pub type Cc5gR = crate::BitReader;
#[doc = "Field `CC5G` writer - desc CC5G"]
pub type Cc5gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6G` reader - desc CC6G"]
pub type Cc6gR = crate::BitReader;
#[doc = "Field `CC6G` writer - desc CC6G"]
pub type Cc6gW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UG"]
    #[inline(always)]
    pub fn ug(&self) -> UgR {
        UgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1G"]
    #[inline(always)]
    pub fn cc1g(&self) -> Cc1gR {
        Cc1gR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC2G"]
    #[inline(always)]
    pub fn cc2g(&self) -> Cc2gR {
        Cc2gR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC3G"]
    #[inline(always)]
    pub fn cc3g(&self) -> Cc3gR {
        Cc3gR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC4G"]
    #[inline(always)]
    pub fn cc4g(&self) -> Cc4gR {
        Cc4gR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COMG"]
    #[inline(always)]
    pub fn comg(&self) -> ComgR {
        ComgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TG"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BG"]
    #[inline(always)]
    pub fn bg(&self) -> BgR {
        BgR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc B2G"]
    #[inline(always)]
    pub fn b2g(&self) -> B2gR {
        B2gR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CC5G"]
    #[inline(always)]
    pub fn cc5g(&self) -> Cc5gR {
        Cc5gR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CC6G"]
    #[inline(always)]
    pub fn cc6g(&self) -> Cc6gR {
        Cc6gR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<'_, EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CC1G"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<'_, EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CC2G"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<'_, EgrSpec> {
        Cc2gW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CC3G"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<'_, EgrSpec> {
        Cc3gW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CC4G"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> Cc4gW<'_, EgrSpec> {
        Cc4gW::new(self, 4)
    }
    #[doc = "Bit 5 - desc COMG"]
    #[inline(always)]
    pub fn comg(&mut self) -> ComgW<'_, EgrSpec> {
        ComgW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<'_, EgrSpec> {
        TgW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BG"]
    #[inline(always)]
    pub fn bg(&mut self) -> BgW<'_, EgrSpec> {
        BgW::new(self, 7)
    }
    #[doc = "Bit 8 - desc B2G"]
    #[inline(always)]
    pub fn b2g(&mut self) -> B2gW<'_, EgrSpec> {
        B2gW::new(self, 8)
    }
    #[doc = "Bit 16 - desc CC5G"]
    #[inline(always)]
    pub fn cc5g(&mut self) -> Cc5gW<'_, EgrSpec> {
        Cc5gW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CC6G"]
    #[inline(always)]
    pub fn cc6g(&mut self) -> Cc6gW<'_, EgrSpec> {
        Cc6gW::new(self, 17)
    }
}
#[doc = "Event generation register\n\nYou can [`read`](crate::Reg::read) this register and get [`egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`egr::R`](R) reader structure"]
impl crate::Readable for EgrSpec {}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
