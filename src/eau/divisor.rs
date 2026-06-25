#[doc = "Register `DIVISOR` reader"]
pub type R = crate::R<DivisorSpec>;
#[doc = "Register `DIVISOR` writer"]
pub type W = crate::W<DivisorSpec>;
#[doc = "Field `DIVISOR` reader - desc DIVISOR"]
pub type DivisorR = crate::FieldReader<u32>;
#[doc = "Field `DIVISOR` writer - desc DIVISOR"]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DIVISOR"]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DIVISOR"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DivisorW<'_, DivisorSpec> {
        DivisorW::new(self, 0)
    }
}
#[doc = "desc DIVISOR\n\nYou can [`read`](crate::Reg::read) this register and get [`divisor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divisor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivisorSpec;
impl crate::RegisterSpec for DivisorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divisor::R`](R) reader structure"]
impl crate::Readable for DivisorSpec {}
#[doc = "`write(|w| ..)` method takes [`divisor::W`](W) writer structure"]
impl crate::Writable for DivisorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVISOR to value 0"]
impl crate::Resettable for DivisorSpec {}
