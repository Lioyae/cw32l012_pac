#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `PIN8` reader - desc AFR8"]
pub type Pin8R = crate::FieldReader;
#[doc = "Field `PIN8` writer - desc AFR8"]
pub type Pin8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN9` reader - desc AFR9"]
pub type Pin9R = crate::FieldReader;
#[doc = "Field `PIN9` writer - desc AFR9"]
pub type Pin9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN10` reader - desc AFR10"]
pub type Pin10R = crate::FieldReader;
#[doc = "Field `PIN10` writer - desc AFR10"]
pub type Pin10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN11` reader - desc AFR11"]
pub type Pin11R = crate::FieldReader;
#[doc = "Field `PIN11` writer - desc AFR11"]
pub type Pin11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN12` reader - desc AFR12"]
pub type Pin12R = crate::FieldReader;
#[doc = "Field `PIN12` writer - desc AFR12"]
pub type Pin12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN13` reader - desc AFR13"]
pub type Pin13R = crate::FieldReader;
#[doc = "Field `PIN13` writer - desc AFR13"]
pub type Pin13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN14` reader - desc AFR14"]
pub type Pin14R = crate::FieldReader;
#[doc = "Field `PIN14` writer - desc AFR14"]
pub type Pin14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN15` reader - desc AFR15"]
pub type Pin15R = crate::FieldReader;
#[doc = "Field `PIN15` writer - desc AFR15"]
pub type Pin15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AFR8"]
    #[inline(always)]
    pub fn pin8(&mut self) -> Pin8W<'_, AfrhSpec> {
        Pin8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc AFR9"]
    #[inline(always)]
    pub fn pin9(&mut self) -> Pin9W<'_, AfrhSpec> {
        Pin9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc AFR10"]
    #[inline(always)]
    pub fn pin10(&mut self) -> Pin10W<'_, AfrhSpec> {
        Pin10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc AFR11"]
    #[inline(always)]
    pub fn pin11(&mut self) -> Pin11W<'_, AfrhSpec> {
        Pin11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - desc AFR12"]
    #[inline(always)]
    pub fn pin12(&mut self) -> Pin12W<'_, AfrhSpec> {
        Pin12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - desc AFR13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, AfrhSpec> {
        Pin13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc AFR14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, AfrhSpec> {
        Pin14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc AFR15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, AfrhSpec> {
        Pin15W::new(self, 28)
    }
}
#[doc = "desc AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {}
