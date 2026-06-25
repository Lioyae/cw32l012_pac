#[doc = "Register `DHR8R2` reader"]
pub type R = crate::R<Dhr8r2Spec>;
#[doc = "Register `DHR8R2` writer"]
pub type W = crate::W<Dhr8r2Spec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - desc DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - desc DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Dhr8r2Spec> {
        DataW::new(self, 4)
    }
}
#[doc = "desc DHR8R2\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr8r2Spec;
impl crate::RegisterSpec for Dhr8r2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr8r2::R`](R) reader structure"]
impl crate::Readable for Dhr8r2Spec {}
#[doc = "`write(|w| ..)` method takes [`dhr8r2::W`](W) writer structure"]
impl crate::Writable for Dhr8r2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR8R2 to value 0"]
impl crate::Resettable for Dhr8r2Spec {}
