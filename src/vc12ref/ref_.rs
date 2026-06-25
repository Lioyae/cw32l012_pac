#[doc = "Register `REF` reader"]
pub type R = crate::R<RefSpec>;
#[doc = "Register `REF` writer"]
pub type W = crate::W<RefSpec>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN` reader - desc VIN"]
pub type VinR = crate::BitReader;
#[doc = "Field `VIN` writer - desc VIN"]
pub type VinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc VIN"]
    #[inline(always)]
    pub fn vin(&self) -> VinR {
        VinR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, RefSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 4 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, RefSpec> {
        EnW::new(self, 4)
    }
    #[doc = "Bit 5 - desc VIN"]
    #[inline(always)]
    pub fn vin(&mut self) -> VinW<'_, RefSpec> {
        VinW::new(self, 5)
    }
}
#[doc = "VCREF Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefSpec;
impl crate::RegisterSpec for RefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_::R`](R) reader structure"]
impl crate::Readable for RefSpec {}
#[doc = "`write(|w| ..)` method takes [`ref_::W`](W) writer structure"]
impl crate::Writable for RefSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF to value 0"]
impl crate::Resettable for RefSpec {}
