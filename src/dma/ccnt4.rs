#[doc = "Register `CCNT4` reader"]
pub type R = crate::R<Ccnt4Spec>;
#[doc = "Register `CCNT4` writer"]
pub type W = crate::W<Ccnt4Spec>;
#[doc = "Field `CCNT` reader - desc CCNT"]
pub type CcntR = crate::FieldReader<u16>;
#[doc = "Field `CCNT` writer - desc CCNT"]
pub type CcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CREPEAT` reader - desc CREPEAT"]
pub type CrepeatR = crate::FieldReader;
#[doc = "Field `CREPEAT` writer - desc CREPEAT"]
pub type CrepeatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - desc CCNT"]
    #[inline(always)]
    pub fn ccnt(&self) -> CcntR {
        CcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - desc CREPEAT"]
    #[inline(always)]
    pub fn crepeat(&self) -> CrepeatR {
        CrepeatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCNT"]
    #[inline(always)]
    pub fn ccnt(&mut self) -> CcntW<'_, Ccnt4Spec> {
        CcntW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CREPEAT"]
    #[inline(always)]
    pub fn crepeat(&mut self) -> CrepeatW<'_, Ccnt4Spec> {
        CrepeatW::new(self, 16)
    }
}
#[doc = "Channel1 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt4Spec;
impl crate::RegisterSpec for Ccnt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt4::R`](R) reader structure"]
impl crate::Readable for Ccnt4Spec {}
#[doc = "`write(|w| ..)` method takes [`ccnt4::W`](W) writer structure"]
impl crate::Writable for Ccnt4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCNT4 to value 0"]
impl crate::Resettable for Ccnt4Spec {}
