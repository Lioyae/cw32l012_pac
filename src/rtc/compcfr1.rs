#[doc = "Register `COMPCFR1` reader"]
pub type R = crate::R<Compcfr1Spec>;
#[doc = "Register `COMPCFR1` writer"]
pub type W = crate::W<Compcfr1Spec>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type CompR = crate::FieldReader<u16>;
#[doc = "Field `COMP` writer - desc COMP"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PERIOD` reader - desc PERIOD"]
pub type PeriodR = crate::FieldReader;
#[doc = "Field `PERIOD` writer - desc PERIOD"]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIGN` reader - desc SIGN"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - desc SIGN"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - desc PERIOD"]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, Compcfr1Spec> {
        CompW::new(self, 0)
    }
    #[doc = "Bits 12:13 - desc PERIOD"]
    #[inline(always)]
    pub fn period(&mut self) -> PeriodW<'_, Compcfr1Spec> {
        PeriodW::new(self, 12)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, Compcfr1Spec> {
        SignW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Compcfr1Spec> {
        EnW::new(self, 15)
    }
}
#[doc = "desc COMPCFR1\n\nYou can [`read`](crate::Reg::read) this register and get [`compcfr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compcfr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Compcfr1Spec;
impl crate::RegisterSpec for Compcfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compcfr1::R`](R) reader structure"]
impl crate::Readable for Compcfr1Spec {}
#[doc = "`write(|w| ..)` method takes [`compcfr1::W`](W) writer structure"]
impl crate::Writable for Compcfr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPCFR1 to value 0"]
impl crate::Resettable for Compcfr1Spec {}
