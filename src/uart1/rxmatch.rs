#[doc = "Register `RXMATCH` reader"]
pub type R = crate::R<RxmatchSpec>;
#[doc = "Register `RXMATCH` writer"]
pub type W = crate::W<RxmatchSpec>;
#[doc = "Field `RXMATCH` reader - desc RXMATCH"]
pub type RxmatchR = crate::FieldReader<u16>;
#[doc = "Field `RXMATCH` writer - desc RXMATCH"]
pub type RxmatchW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc RXMATCH"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RxmatchR {
        RxmatchR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc RXMATCH"]
    #[inline(always)]
    pub fn rxmatch(&mut self) -> RxmatchW<'_, RxmatchSpec> {
        RxmatchW::new(self, 0)
    }
}
#[doc = "desc RXMATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmatchSpec;
impl crate::RegisterSpec for RxmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmatch::R`](R) reader structure"]
impl crate::Readable for RxmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmatch::W`](W) writer structure"]
impl crate::Writable for RxmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXMATCH to value 0"]
impl crate::Resettable for RxmatchSpec {}
