#[doc = "Register `DOR1` reader"]
pub type R = crate::R<Dor1Spec>;
#[doc = "Register `DOR1` writer"]
pub type W = crate::W<Dor1Spec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMAUDR` reader - desc DMAUDR"]
pub type DmaudrR = crate::BitReader;
#[doc = "Field `DMAUDR` writer - desc DMAUDR"]
pub type DmaudrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - desc DMAUDR"]
    #[inline(always)]
    pub fn dmaudr(&self) -> DmaudrR {
        DmaudrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Dor1Spec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 15 - desc DMAUDR"]
    #[inline(always)]
    pub fn dmaudr(&mut self) -> DmaudrW<'_, Dor1Spec> {
        DmaudrW::new(self, 15)
    }
}
#[doc = "desc DOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dor1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dor1Spec;
impl crate::RegisterSpec for Dor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor1::R`](R) reader structure"]
impl crate::Readable for Dor1Spec {}
#[doc = "`write(|w| ..)` method takes [`dor1::W`](W) writer structure"]
impl crate::Writable for Dor1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOR1 to value 0"]
impl crate::Resettable for Dor1Spec {}
