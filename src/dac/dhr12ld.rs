#[doc = "Register `DHR12LD` reader"]
pub type R = crate::R<Dhr12ldSpec>;
#[doc = "Register `DHR12LD` writer"]
pub type W = crate::W<Dhr12ldSpec>;
#[doc = "Field `C1DATA` reader - desc C1DATA"]
pub type C1dataR = crate::FieldReader<u16>;
#[doc = "Field `C1DATA` writer - desc C1DATA"]
pub type C1dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `C2DATA` reader - desc C2DATA"]
pub type C2dataR = crate::FieldReader<u16>;
#[doc = "Field `C2DATA` writer - desc C2DATA"]
pub type C2dataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&self) -> C1dataR {
        C1dataR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&self) -> C2dataR {
        C2dataR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - desc C1DATA"]
    #[inline(always)]
    pub fn c1data(&mut self) -> C1dataW<'_, Dhr12ldSpec> {
        C1dataW::new(self, 4)
    }
    #[doc = "Bits 20:31 - desc C2DATA"]
    #[inline(always)]
    pub fn c2data(&mut self) -> C2dataW<'_, Dhr12ldSpec> {
        C2dataW::new(self, 20)
    }
}
#[doc = "desc DHR12LD\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dhr12ldSpec;
impl crate::RegisterSpec for Dhr12ldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr12ld::R`](R) reader structure"]
impl crate::Readable for Dhr12ldSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr12ld::W`](W) writer structure"]
impl crate::Writable for Dhr12ldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR12LD to value 0"]
impl crate::Resettable for Dhr12ldSpec {}
