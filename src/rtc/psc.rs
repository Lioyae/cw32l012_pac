#[doc = "Register `PSC` reader"]
pub type R = crate::R<PscSpec>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PscSpec>;
#[doc = "Field `PSC2` reader - desc PSC2"]
pub type Psc2R = crate::FieldReader<u32>;
#[doc = "Field `PSC2` writer - desc PSC2"]
pub type Psc2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PSC1` reader - desc PSC1"]
pub type Psc1R = crate::FieldReader;
#[doc = "Field `PSC1` writer - desc PSC1"]
pub type Psc1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:19 - desc PSC2"]
    #[inline(always)]
    pub fn psc2(&self) -> Psc2R {
        Psc2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:27 - desc PSC1"]
    #[inline(always)]
    pub fn psc1(&self) -> Psc1R {
        Psc1R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - desc PSC2"]
    #[inline(always)]
    pub fn psc2(&mut self) -> Psc2W<'_, PscSpec> {
        Psc2W::new(self, 0)
    }
    #[doc = "Bits 20:27 - desc PSC1"]
    #[inline(always)]
    pub fn psc1(&mut self) -> Psc1W<'_, PscSpec> {
        Psc1W::new(self, 20)
    }
}
#[doc = "desc PSC\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscSpec;
impl crate::RegisterSpec for PscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PscSpec {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PscSpec {}
