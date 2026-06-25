#[doc = "Register `MCR2` reader"]
pub type R = crate::R<Mcr2Spec>;
#[doc = "Register `MCR2` writer"]
pub type W = crate::W<Mcr2Spec>;
#[doc = "Field `PRESCALE` reader - desc PRESCALE"]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - desc PRESCALE"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AUTOSTOP` reader - desc AUTOSTOP"]
pub type AutostopR = crate::BitReader;
#[doc = "Field `AUTOSTOP` writer - desc AUTOSTOP"]
pub type AutostopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNACK` reader - desc IGNACK"]
pub type IgnackR = crate::BitReader;
#[doc = "Field `IGNACK` writer - desc IGNACK"]
pub type IgnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECFG` reader - desc TIMECFG"]
pub type TimecfgR = crate::BitReader;
#[doc = "Field `TIMECFG` writer - desc TIMECFG"]
pub type TimecfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::BitReader;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCFG` reader - desc MATCFG"]
pub type MatcfgR = crate::FieldReader;
#[doc = "Field `MATCFG` writer - desc MATCFG"]
pub type MatcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PINCFG` reader - desc PINCFG"]
pub type PincfgR = crate::FieldReader;
#[doc = "Field `PINCFG` writer - desc PINCFG"]
pub type PincfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc PRESCALE"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&self) -> AutostopR {
        AutostopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IGNACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IgnackR {
        IgnackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TIMECFG"]
    #[inline(always)]
    pub fn timecfg(&self) -> TimecfgR {
        TimecfgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc MATCFG"]
    #[inline(always)]
    pub fn matcfg(&self) -> MatcfgR {
        MatcfgR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - desc PINCFG"]
    #[inline(always)]
    pub fn pincfg(&self) -> PincfgR {
        PincfgR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PRESCALE"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<'_, Mcr2Spec> {
        PrescaleW::new(self, 0)
    }
    #[doc = "Bit 8 - desc AUTOSTOP"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AutostopW<'_, Mcr2Spec> {
        AutostopW::new(self, 8)
    }
    #[doc = "Bit 9 - desc IGNACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IgnackW<'_, Mcr2Spec> {
        IgnackW::new(self, 9)
    }
    #[doc = "Bit 10 - desc TIMECFG"]
    #[inline(always)]
    pub fn timecfg(&mut self) -> TimecfgW<'_, Mcr2Spec> {
        TimecfgW::new(self, 10)
    }
    #[doc = "Bit 15 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Mcr2Spec> {
        TrgselW::new(self, 15)
    }
    #[doc = "Bits 16:18 - desc MATCFG"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MatcfgW<'_, Mcr2Spec> {
        MatcfgW::new(self, 16)
    }
    #[doc = "Bits 24:26 - desc PINCFG"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PincfgW<'_, Mcr2Spec> {
        PincfgW::new(self, 24)
    }
}
#[doc = "Master Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr2Spec;
impl crate::RegisterSpec for Mcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr2::R`](R) reader structure"]
impl crate::Readable for Mcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr2::W`](W) writer structure"]
impl crate::Writable for Mcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR2 to value 0"]
impl crate::Resettable for Mcr2Spec {}
