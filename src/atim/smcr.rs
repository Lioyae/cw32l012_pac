#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS` reader - desc SMS"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - desc SMS"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - desc OCCS"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - desc OCCS"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - desc TS"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - desc TS"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - desc MSM"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - desc MSM"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - desc ETF"]
pub type EtfR = crate::FieldReader;
#[doc = "Field `ETF` writer - desc ETF"]
pub type EtfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - desc ETPS"]
pub type EtpsR = crate::FieldReader;
#[doc = "Field `ETPS` writer - desc ETPS"]
pub type EtpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - desc ECE"]
pub type EceR = crate::BitReader;
#[doc = "Field `ECE` writer - desc ECE"]
pub type EceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - desc ETP"]
pub type EtpR = crate::BitReader;
#[doc = "Field `ETP` writer - desc ETP"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSH` reader - desc SMSH"]
pub type SmshR = crate::BitReader;
#[doc = "Field `SMSH` writer - desc SMSH"]
pub type SmshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSH` reader - desc TSH"]
pub type TshR = crate::FieldReader;
#[doc = "Field `TSH` writer - desc TSH"]
pub type TshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMSPE` reader - desc SMSPE"]
pub type SmspeR = crate::BitReader;
#[doc = "Field `SMSPE` writer - desc SMSPE"]
pub type SmspeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMSPS` reader - desc SMSPS"]
pub type SmspsR = crate::BitReader;
#[doc = "Field `SMSPS` writer - desc SMSPS"]
pub type SmspsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc TS"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc ETF"]
    #[inline(always)]
    pub fn etf(&self) -> EtfR {
        EtfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - desc ETPS"]
    #[inline(always)]
    pub fn etps(&self) -> EtpsR {
        EtpsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc ECE"]
    #[inline(always)]
    pub fn ece(&self) -> EceR {
        EceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ETP"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc SMSH"]
    #[inline(always)]
    pub fn smsh(&self) -> SmshR {
        SmshR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - desc TSH"]
    #[inline(always)]
    pub fn tsh(&self) -> TshR {
        TshR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - desc SMSPE"]
    #[inline(always)]
    pub fn smspe(&self) -> SmspeR {
        SmspeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc SMSPS"]
    #[inline(always)]
    pub fn smsps(&self) -> SmspsR {
        SmspsR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SMS"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<'_, SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bit 3 - desc OCCS"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<'_, SmcrSpec> {
        OccsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, SmcrSpec> {
        TsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc MSM"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bits 8:11 - desc ETF"]
    #[inline(always)]
    pub fn etf(&mut self) -> EtfW<'_, SmcrSpec> {
        EtfW::new(self, 8)
    }
    #[doc = "Bits 12:13 - desc ETPS"]
    #[inline(always)]
    pub fn etps(&mut self) -> EtpsW<'_, SmcrSpec> {
        EtpsW::new(self, 12)
    }
    #[doc = "Bit 14 - desc ECE"]
    #[inline(always)]
    pub fn ece(&mut self) -> EceW<'_, SmcrSpec> {
        EceW::new(self, 14)
    }
    #[doc = "Bit 15 - desc ETP"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, SmcrSpec> {
        EtpW::new(self, 15)
    }
    #[doc = "Bit 16 - desc SMSH"]
    #[inline(always)]
    pub fn smsh(&mut self) -> SmshW<'_, SmcrSpec> {
        SmshW::new(self, 16)
    }
    #[doc = "Bits 20:21 - desc TSH"]
    #[inline(always)]
    pub fn tsh(&mut self) -> TshW<'_, SmcrSpec> {
        TshW::new(self, 20)
    }
    #[doc = "Bit 24 - desc SMSPE"]
    #[inline(always)]
    pub fn smspe(&mut self) -> SmspeW<'_, SmcrSpec> {
        SmspeW::new(self, 24)
    }
    #[doc = "Bit 25 - desc SMSPS"]
    #[inline(always)]
    pub fn smsps(&mut self) -> SmspsW<'_, SmcrSpec> {
        SmspsW::new(self, 25)
    }
}
#[doc = "Slave Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
