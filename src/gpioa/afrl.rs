#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `PIN0` reader - desc AFR0"]
pub type Pin0R = crate::FieldReader;
#[doc = "Field `PIN0` writer - desc AFR0"]
pub type Pin0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN1` reader - desc AFR1"]
pub type Pin1R = crate::FieldReader;
#[doc = "Field `PIN1` writer - desc AFR1"]
pub type Pin1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN2` reader - desc AFR2"]
pub type Pin2R = crate::FieldReader;
#[doc = "Field `PIN2` writer - desc AFR2"]
pub type Pin2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN3` reader - desc AFR3"]
pub type Pin3R = crate::FieldReader;
#[doc = "Field `PIN3` writer - desc AFR3"]
pub type Pin3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN4` reader - desc AFR4"]
pub type Pin4R = crate::FieldReader;
#[doc = "Field `PIN4` writer - desc AFR4"]
pub type Pin4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN5` reader - desc AFR5"]
pub type Pin5R = crate::FieldReader;
#[doc = "Field `PIN5` writer - desc AFR5"]
pub type Pin5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN6` reader - desc AFR6"]
pub type Pin6R = crate::FieldReader;
#[doc = "Field `PIN6` writer - desc AFR6"]
pub type Pin6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN7` reader - desc AFR7"]
pub type Pin7R = crate::FieldReader;
#[doc = "Field `PIN7` writer - desc AFR7"]
pub type Pin7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc AFR2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc AFR4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - desc AFR5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AFR0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, AfrlSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc AFR1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, AfrlSpec> {
        Pin1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc AFR2"]
    #[inline(always)]
    pub fn pin2(&mut self) -> Pin2W<'_, AfrlSpec> {
        Pin2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc AFR3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, AfrlSpec> {
        Pin3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - desc AFR4"]
    #[inline(always)]
    pub fn pin4(&mut self) -> Pin4W<'_, AfrlSpec> {
        Pin4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - desc AFR5"]
    #[inline(always)]
    pub fn pin5(&mut self) -> Pin5W<'_, AfrlSpec> {
        Pin5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc AFR6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, AfrlSpec> {
        Pin6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc AFR7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, AfrlSpec> {
        Pin7W::new(self, 28)
    }
}
#[doc = "desc AFRL\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {}
