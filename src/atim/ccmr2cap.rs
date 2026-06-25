#[doc = "Register `CCMR2CAP` reader"]
pub type R = crate::R<Ccmr2capSpec>;
#[doc = "Register `CCMR2CAP` writer"]
pub type W = crate::W<Ccmr2capSpec>;
#[doc = "Field `CC3S` reader - desc CC3S"]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - desc CC3S"]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3PSC` reader - desc IC3PSC"]
pub type Ic3pscR = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - desc IC3PSC"]
pub type Ic3pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - desc IC3F"]
pub type Ic3fR = crate::FieldReader;
#[doc = "Field `IC3F` writer - desc IC3F"]
pub type Ic3fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CC4S` reader - desc CC4S"]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - desc CC4S"]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4PSC` reader - desc IC4PSC"]
pub type Ic4pscR = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - desc IC4PSC"]
pub type Ic4pscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - desc IC4F"]
pub type Ic4fR = crate::FieldReader;
#[doc = "Field `IC4F` writer - desc IC4F"]
pub type Ic4fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - desc CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&self) -> Ic3pscR {
        Ic3pscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc IC3F"]
    #[inline(always)]
    pub fn ic3f(&self) -> Ic3fR {
        Ic3fR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - desc CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - desc IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&self) -> Ic4pscR {
        Ic4pscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - desc IC4F"]
    #[inline(always)]
    pub fn ic4f(&self) -> Ic4fR {
        Ic4fR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC3S"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2capSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc IC3PSC"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> Ic3pscW<'_, Ccmr2capSpec> {
        Ic3pscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - desc IC3F"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> Ic3fW<'_, Ccmr2capSpec> {
        Ic3fW::new(self, 4)
    }
    #[doc = "Bits 8:9 - desc CC4S"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2capSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bits 10:11 - desc IC4PSC"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> Ic4pscW<'_, Ccmr2capSpec> {
        Ic4pscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - desc IC4F"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> Ic4fW<'_, Ccmr2capSpec> {
        Ic4fW::new(self, 12)
    }
}
#[doc = "capture compare mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2capSpec;
impl crate::RegisterSpec for Ccmr2capSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2cap::R`](R) reader structure"]
impl crate::Readable for Ccmr2capSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2cap::W`](W) writer structure"]
impl crate::Writable for Ccmr2capSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2CAP to value 0"]
impl crate::Resettable for Ccmr2capSpec {}
