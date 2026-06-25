#[doc = "Register `INSEL` reader"]
pub type R = crate::R<InselSpec>;
#[doc = "Register `INSEL` writer"]
pub type W = crate::W<InselSpec>;
#[doc = "Field `SCLINSEL` reader - desc SCLINSEL"]
pub type SclinselR = crate::FieldReader;
#[doc = "Field `SCLINSEL` writer - desc SCLINSEL"]
pub type SclinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDAINSEL` reader - desc SDAINSEL"]
pub type SdainselR = crate::FieldReader;
#[doc = "Field `SDAINSEL` writer - desc SDAINSEL"]
pub type SdainselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc SCLINSEL"]
    #[inline(always)]
    pub fn sclinsel(&self) -> SclinselR {
        SclinselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc SDAINSEL"]
    #[inline(always)]
    pub fn sdainsel(&self) -> SdainselR {
        SdainselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SCLINSEL"]
    #[inline(always)]
    pub fn sclinsel(&mut self) -> SclinselW<'_, InselSpec> {
        SclinselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - desc SDAINSEL"]
    #[inline(always)]
    pub fn sdainsel(&mut self) -> SdainselW<'_, InselSpec> {
        SdainselW::new(self, 3)
    }
}
#[doc = "desc INSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`insel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`insel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InselSpec;
impl crate::RegisterSpec for InselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`insel::R`](R) reader structure"]
impl crate::Readable for InselSpec {}
#[doc = "`write(|w| ..)` method takes [`insel::W`](W) writer structure"]
impl crate::Writable for InselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INSEL to value 0"]
impl crate::Resettable for InselSpec {}
