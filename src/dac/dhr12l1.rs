#[doc = "Register `DHR12L1` reader"]
pub type R = crate::R<Dhr12l1Spec>;
#[doc = "Register `DHR12L1` writer"]
pub type W = crate::W<Dhr12l1Spec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Dhr12l1Spec> {
        DataW::new(self, 4)
    }
}
#[doc = "desc DHR12L1\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12l1Spec;
impl crate::RegisterSpec for Dhr12l1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12l1::R`](R) reader structure"]
impl crate::Readable for Dhr12l1Spec {}
#[doc = "`write(|w| ..)` method takes [`dhr12l1::W`](W) writer structure"]
impl crate::Writable for Dhr12l1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12L1 to value 0"]
impl crate::Resettable for Dhr12l1Spec {}
