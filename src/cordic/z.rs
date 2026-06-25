#[doc = "Register `Z` reader"]
pub type R = crate::R<ZSpec>;
#[doc = "Register `Z` writer"]
pub type W = crate::W<ZSpec>;
#[doc = "Field `Z` reader - desc Z"]
pub type ZR = crate::FieldReader<u32>;
#[doc = "Field `Z` writer - desc Z"]
pub type ZW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc Z"]
    #[inline(always)]
    pub fn z(&self) -> ZR {
        ZR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc Z"]
    #[inline(always)]
    pub fn z(&mut self) -> ZW<'_, ZSpec> {
        ZW::new(self, 0)
    }
}
#[doc = "desc Z\n\nYou can [`read`](crate::Reg::read) this register and get [`z::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`z::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZSpec;
impl crate::RegisterSpec for ZSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`z::R`](R) reader structure"]
impl crate::Readable for ZSpec {}
#[doc = "`write(|w| ..)` method takes [`z::W`](W) writer structure"]
impl crate::Writable for ZSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Z to value 0"]
impl crate::Resettable for ZSpec {}
