#[doc = "Register `CCMR1CAP` reader"]
pub type R = crate::R<Ccmr1capSpec>;
#[doc = "Register `CCMR1CAP` writer"]
pub type W = crate::W<Ccmr1capSpec>;
#[doc = "Field `CC1S` reader - desc CC1S"]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - desc CC1S"]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1PSC` reader - desc IC1PSC"]
pub type Ic1pscR = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - desc IC1PSC"]
pub type Ic1pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - desc IC1F"]
pub type Ic1fR = crate::FieldReader;
#[doc = "Field `IC1F` writer - desc IC1F"]
pub type Ic1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC2S` reader - desc CC2S"]
pub type Cc2sR = crate::FieldReader;
#[doc = "Field `CC2S` writer - desc CC2S"]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2PSC` reader - desc IC2PSC"]
pub type Ic2pscR = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - desc IC2PSC"]
pub type Ic2pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - desc IC2F"]
pub type Ic2fR = crate::FieldReader;
#[doc = "Field `IC2F` writer - desc IC2F"]
pub type Ic2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - desc CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&self) -> Ic1pscR {
        Ic1pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc IC1F"]
    #[inline(always)]
    pub fn ic1f(&self) -> Ic1fR {
        Ic1fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - desc CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&self) -> Ic2pscR {
        Ic2pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - desc IC2F"]
    #[inline(always)]
    pub fn ic2f(&self) -> Ic2fR {
        Ic2fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC1S"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1capSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc IC1PSC"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> Ic1pscW<'_, Ccmr1capSpec> {
        Ic1pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - desc IC1F"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> Ic1fW<'_, Ccmr1capSpec> {
        Ic1fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - desc CC2S"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> Cc2sW<'_, Ccmr1capSpec> {
        Cc2sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc IC2PSC"]
    #[inline(always)]
    pub fn ic2psc(&mut self) -> Ic2pscW<'_, Ccmr1capSpec> {
        Ic2pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - desc IC2F"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> Ic2fW<'_, Ccmr1capSpec> {
        Ic2fW::new(self, 12)
    }
}
#[doc = "Capture control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1capSpec;
impl crate::RegisterSpec for Ccmr1capSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1cap::R`](R) reader structure"]
impl crate::Readable for Ccmr1capSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1cap::W`](W) writer structure"]
impl crate::Writable for Ccmr1capSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1CAP to value 0"]
impl crate::Resettable for Ccmr1capSpec {}
