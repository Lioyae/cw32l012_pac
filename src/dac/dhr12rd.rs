#[doc = "Register `DHR12RD` reader"]
pub type R = crate::R<Dhr12rdSpec>;
#[doc = "Register `DHR12RD` writer"]
pub type W = crate::W<Dhr12rdSpec>;
#[doc = "Field `C1DATA` reader - desc C1DATA"]
pub type C1dataR = crate::FieldReader<u16>;
#[doc = "Field `C1DATA` writer - desc C1DATA"]
pub type C1dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `C2DATA` reader - desc C2DATA"]
pub type C2dataR = crate::FieldReader<u16>;
#[doc = "Field `C2DATA` writer - desc C2DATA"]
pub type C2dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&self) -> C1dataR {
        C1dataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&self) -> C2dataR {
        C2dataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&mut self) -> C1dataW<'_, Dhr12rdSpec> {
        C1dataW::new(self, 0)
    }
    #[doc = "Bits 16:27 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&mut self) -> C2dataW<'_, Dhr12rdSpec> {
        C2dataW::new(self, 16)
    }
}
#[doc = "desc DHR12RD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12rdSpec;
impl crate::RegisterSpec for Dhr12rdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12rd::R`](R) reader structure"]
impl crate::Readable for Dhr12rdSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr12rd::W`](W) writer structure"]
impl crate::Writable for Dhr12rdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12RD to value 0"]
impl crate::Resettable for Dhr12rdSpec {}
