#[doc = "Register `X` reader"]
pub type R = crate::R<XSpec>;
#[doc = "Register `X` writer"]
pub type W = crate::W<XSpec>;
#[doc = "Field `X` reader - desc X"]
pub type XR = crate::FieldReader<u32>;
#[doc = "Field `X` writer - desc X"]
pub type XW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc X"]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc X"]
    #[inline(always)]
    pub fn x(&mut self) -> XW<'_, XSpec> {
        XW::new(self, 0)
    }
}
#[doc = "Data register X\n\nYou can [`read`](crate::Reg::read) this register and get [`x::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`x::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XSpec;
impl crate::RegisterSpec for XSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`x::R`](R) reader structure"]
impl crate::Readable for XSpec {}
#[doc = "`write(|w| ..)` method takes [`x::W`](W) writer structure"]
impl crate::Writable for XSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets X to value 0"]
impl crate::Resettable for XSpec {}
