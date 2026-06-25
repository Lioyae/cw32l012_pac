#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS` reader - desc SMS"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - desc SMS"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSTISRC` reader - desc RSTISRC"]
pub type RstisrcR = crate::FieldReader;
#[doc = "Field `RSTISRC` writer - desc RSTISRC"]
pub type RstisrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRGISRC` reader - desc TRGISRC"]
pub type TrgisrcR = crate::FieldReader;
#[doc = "Field `TRGISRC` writer - desc TRGISRC"]
pub type TrgisrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSM` reader - desc MSM"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - desc MSM"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIFLT` reader - desc TRGIFLT"]
pub type TrgifltR = crate::FieldReader;
#[doc = "Field `TRGIFLT` writer - desc TRGIFLT"]
pub type TrgifltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSTIPOL` reader - desc RSTIPOL"]
pub type RstipolR = crate::BitReader;
#[doc = "Field `RSTIPOL` writer - desc RSTIPOL"]
pub type RstipolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIPOL` reader - desc TRGIPOL"]
pub type TrgipolR = crate::BitReader;
#[doc = "Field `TRGIPOL` writer - desc TRGIPOL"]
pub type TrgipolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - desc RSTISRC"]
    #[inline(always)]
    pub fn rstisrc(&self) -> RstisrcR {
        RstisrcR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - desc TRGISRC"]
    #[inline(always)]
    pub fn trgisrc(&self) -> TrgisrcR {
        TrgisrcR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - desc MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc TRGIFLT"]
    #[inline(always)]
    pub fn trgiflt(&self) -> TrgifltR {
        TrgifltR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - desc RSTIPOL"]
    #[inline(always)]
    pub fn rstipol(&self) -> RstipolR {
        RstipolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TRGIPOL"]
    #[inline(always)]
    pub fn trgipol(&self) -> TrgipolR {
        TrgipolR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SMS"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<'_, SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bits 3:6 - desc RSTISRC"]
    #[inline(always)]
    pub fn rstisrc(&mut self) -> RstisrcW<'_, SmcrSpec> {
        RstisrcW::new(self, 3)
    }
    #[doc = "Bits 7:10 - desc TRGISRC"]
    #[inline(always)]
    pub fn trgisrc(&mut self) -> TrgisrcW<'_, SmcrSpec> {
        TrgisrcW::new(self, 7)
    }
    #[doc = "Bit 11 - desc MSM"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc TRGIFLT"]
    #[inline(always)]
    pub fn trgiflt(&mut self) -> TrgifltW<'_, SmcrSpec> {
        TrgifltW::new(self, 12)
    }
    #[doc = "Bit 16 - desc RSTIPOL"]
    #[inline(always)]
    pub fn rstipol(&mut self) -> RstipolW<'_, SmcrSpec> {
        RstipolW::new(self, 16)
    }
    #[doc = "Bit 17 - desc TRGIPOL"]
    #[inline(always)]
    pub fn trgipol(&mut self) -> TrgipolW<'_, SmcrSpec> {
        TrgipolW::new(self, 17)
    }
}
#[doc = "Slave Mode Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {}
