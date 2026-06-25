#[doc = "Register `CCMR3CAP` reader"]
pub type R = crate::R<Ccmr3capSpec>;
#[doc = "Register `CCMR3CAP` writer"]
pub type W = crate::W<Ccmr3capSpec>;
#[doc = "Field `CC5S` reader - desc CC5S"]
pub type Cc5sR = crate::FieldReader;
#[doc = "Field `CC5S` writer - desc CC5S"]
pub type Cc5sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC5PSC` reader - desc IC5PSC"]
pub type Ic5pscR = crate::FieldReader;
#[doc = "Field `IC5PSC` writer - desc IC5PSC"]
pub type Ic5pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC5F` reader - desc IC5F"]
pub type Ic5fR = crate::FieldReader;
#[doc = "Field `IC5F` writer - desc IC5F"]
pub type Ic5fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC6S` reader - desc CC6S"]
pub type Cc6sR = crate::FieldReader;
#[doc = "Field `CC6S` writer - desc CC6S"]
pub type Cc6sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC6PSC` reader - desc IC6PSC"]
pub type Ic6pscR = crate::FieldReader;
#[doc = "Field `IC6PSC` writer - desc IC6PSC"]
pub type Ic6pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC6F` reader - desc IC6F"]
pub type Ic6fR = crate::FieldReader;
#[doc = "Field `IC6F` writer - desc IC6F"]
pub type Ic6fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - desc CC5S"]
    #[inline(always)]
    pub fn cc5s(&self) -> Cc5sR {
        Cc5sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc IC5PSC"]
    #[inline(always)]
    pub fn ic5psc(&self) -> Ic5pscR {
        Ic5pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc IC5F"]
    #[inline(always)]
    pub fn ic5f(&self) -> Ic5fR {
        Ic5fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - desc CC6S"]
    #[inline(always)]
    pub fn cc6s(&self) -> Cc6sR {
        Cc6sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc IC6PSC"]
    #[inline(always)]
    pub fn ic6psc(&self) -> Ic6pscR {
        Ic6pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - desc IC6F"]
    #[inline(always)]
    pub fn ic6f(&self) -> Ic6fR {
        Ic6fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC5S"]
    #[inline(always)]
    pub fn cc5s(&mut self) -> Cc5sW<'_, Ccmr3capSpec> {
        Cc5sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc IC5PSC"]
    #[inline(always)]
    pub fn ic5psc(&mut self) -> Ic5pscW<'_, Ccmr3capSpec> {
        Ic5pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - desc IC5F"]
    #[inline(always)]
    pub fn ic5f(&mut self) -> Ic5fW<'_, Ccmr3capSpec> {
        Ic5fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - desc CC6S"]
    #[inline(always)]
    pub fn cc6s(&mut self) -> Cc6sW<'_, Ccmr3capSpec> {
        Cc6sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc IC6PSC"]
    #[inline(always)]
    pub fn ic6psc(&mut self) -> Ic6pscW<'_, Ccmr3capSpec> {
        Ic6pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - desc IC6F"]
    #[inline(always)]
    pub fn ic6f(&mut self) -> Ic6fW<'_, Ccmr3capSpec> {
        Ic6fW::new(self, 12)
    }
}
#[doc = "capture compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr3capSpec;
impl crate::RegisterSpec for Ccmr3capSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3cap::R`](R) reader structure"]
impl crate::Readable for Ccmr3capSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr3cap::W`](W) writer structure"]
impl crate::Writable for Ccmr3capSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR3CAP to value 0"]
impl crate::Resettable for Ccmr3capSpec {}
