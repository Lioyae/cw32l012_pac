#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVR` reader - desc OVR"]
pub type OvrR = crate::BitReader;
#[doc = "Field `ZERO` reader - desc ZERO"]
pub type ZeroR = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ZERO"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CsrSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "desc CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
