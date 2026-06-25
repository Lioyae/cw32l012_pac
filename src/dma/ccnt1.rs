#[doc = "Register `CCNT1` reader"]
pub type R = crate::R<Ccnt1Spec>;
#[doc = "Register `CCNT1` writer"]
pub type W = crate::W<Ccnt1Spec>;
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
    pub fn ccnt(&mut self) -> CcntW<'_, Ccnt1Spec> {
        CcntW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc CREPEAT"]
    #[inline(always)]
    pub fn crepeat(&mut self) -> CrepeatW<'_, Ccnt1Spec> {
        CrepeatW::new(self, 16)
    }
}
#[doc = "Channel1 trigger register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccnt1Spec;
impl crate::RegisterSpec for Ccnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccnt1::R`](R) reader structure"]
impl crate::Readable for Ccnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccnt1::W`](W) writer structure"]
impl crate::Writable for Ccnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCNT1 to value 0"]
impl crate::Resettable for Ccnt1Spec {}
