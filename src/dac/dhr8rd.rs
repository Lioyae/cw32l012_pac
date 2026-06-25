#[doc = "Register `DHR8RD` reader"]
pub type R = crate::R<Dhr8rdSpec>;
#[doc = "Register `DHR8RD` writer"]
pub type W = crate::W<Dhr8rdSpec>;
#[doc = "Field `C1DATA` reader - desc C1DATA"]
pub type C1dataR = crate::FieldReader;
#[doc = "Field `C1DATA` writer - desc C1DATA"]
pub type C1dataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `C2DATA` reader - desc C2DATA"]
pub type C2dataR = crate::FieldReader;
#[doc = "Field `C2DATA` writer - desc C2DATA"]
pub type C2dataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&self) -> C1dataR {
        C1dataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&self) -> C2dataR {
        C2dataR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&mut self) -> C1dataW<'_, Dhr8rdSpec> {
        C1dataW::new(self, 0)
    }
    #[doc = "Bits 8:15 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&mut self) -> C2dataW<'_, Dhr8rdSpec> {
        C2dataW::new(self, 8)
    }
}
#[doc = "desc DHR8RD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr8rdSpec;
impl crate::RegisterSpec for Dhr8rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr8rd::R`](R) reader structure"]
impl crate::Readable for Dhr8rdSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr8rd::W`](W) writer structure"]
impl crate::Writable for Dhr8rdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR8RD to value 0"]
impl crate::Resettable for Dhr8rdSpec {}
