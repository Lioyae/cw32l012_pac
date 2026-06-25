#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNGSTART` reader - desc SNGSTART"]
pub type SngstartR = crate::BitReader;
#[doc = "Field `SNGSTART` writer - desc SNGSTART"]
pub type SngstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTART` reader - desc CNTSTART"]
pub type CntstartR = crate::BitReader;
#[doc = "Field `CNTSTART` writer - desc CNTSTART"]
pub type CntstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRST` reader - desc SRST"]
pub type SrstR = crate::BitReader;
#[doc = "Field `SRST` writer - desc SRST"]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARST` reader - desc ARST"]
pub type ArstR = crate::BitReader;
#[doc = "Field `ARST` writer - desc ARST"]
pub type ArstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SNGSTART"]
    #[inline(always)]
    pub fn sngstart(&self) -> SngstartR {
        SngstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CNTSTART"]
    #[inline(always)]
    pub fn cntstart(&self) -> CntstartR {
        CntstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SRST"]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ARST"]
    #[inline(always)]
    pub fn arst(&self) -> ArstR {
        ArstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SNGSTART"]
    #[inline(always)]
    pub fn sngstart(&mut self) -> SngstartW<'_, Cr0Spec> {
        SngstartW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CNTSTART"]
    #[inline(always)]
    pub fn cntstart(&mut self) -> CntstartW<'_, Cr0Spec> {
        CntstartW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SRST"]
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<'_, Cr0Spec> {
        SrstW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ARST"]
    #[inline(always)]
    pub fn arst(&mut self) -> ArstW<'_, Cr0Spec> {
        ArstW::new(self, 4)
    }
}
#[doc = "desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
