#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CAPF` reader - desc CAPF"]
pub type CapfR = crate::BitReader;
#[doc = "Field `CAPF` writer - desc CAPF"]
pub type CapfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - desc OVF"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHF` reader - desc MATCHF"]
pub type MatchfR = crate::BitReader;
#[doc = "Field `MATCHF` writer - desc MATCHF"]
pub type MatchfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CAPF"]
    #[inline(always)]
    pub fn capf(&self) -> CapfR {
        CapfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MATCHF"]
    #[inline(always)]
    pub fn matchf(&self) -> MatchfR {
        MatchfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CAPF"]
    #[inline(always)]
    pub fn capf(&mut self) -> CapfW<'_, IcrSpec> {
        CapfW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<'_, IcrSpec> {
        OvfW::new(self, 1)
    }
    #[doc = "Bit 2 - desc MATCHF"]
    #[inline(always)]
    pub fn matchf(&mut self) -> MatchfW<'_, IcrSpec> {
        MatchfW::new(self, 2)
    }
}
#[doc = "Interrupt clean register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
