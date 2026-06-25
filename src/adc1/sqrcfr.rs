#[doc = "Register `SQRCFR` reader"]
pub type R = crate::R<SqrcfrSpec>;
#[doc = "Register `SQRCFR` writer"]
pub type W = crate::W<SqrcfrSpec>;
#[doc = "Field `SQRCH0` reader - desc SQRCH0"]
pub type Sqrch0R = crate::FieldReader;
#[doc = "Field `SQRCH0` writer - desc SQRCH0"]
pub type Sqrch0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH1` reader - desc SQRCH1"]
pub type Sqrch1R = crate::FieldReader;
#[doc = "Field `SQRCH1` writer - desc SQRCH1"]
pub type Sqrch1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH2` reader - desc SQRCH2"]
pub type Sqrch2R = crate::FieldReader;
#[doc = "Field `SQRCH2` writer - desc SQRCH2"]
pub type Sqrch2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH3` reader - desc SQRCH3"]
pub type Sqrch3R = crate::FieldReader;
#[doc = "Field `SQRCH3` writer - desc SQRCH3"]
pub type Sqrch3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH4` reader - desc SQRCH4"]
pub type Sqrch4R = crate::FieldReader;
#[doc = "Field `SQRCH4` writer - desc SQRCH4"]
pub type Sqrch4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH5` reader - desc SQRCH5"]
pub type Sqrch5R = crate::FieldReader;
#[doc = "Field `SQRCH5` writer - desc SQRCH5"]
pub type Sqrch5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH6` reader - desc SQRCH6"]
pub type Sqrch6R = crate::FieldReader;
#[doc = "Field `SQRCH6` writer - desc SQRCH6"]
pub type Sqrch6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SQRCH7` reader - desc SQRCH7"]
pub type Sqrch7R = crate::FieldReader;
#[doc = "Field `SQRCH7` writer - desc SQRCH7"]
pub type Sqrch7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc SQRCH0"]
    #[inline(always)]
    pub fn sqrch0(&self) -> Sqrch0R {
        Sqrch0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc SQRCH1"]
    #[inline(always)]
    pub fn sqrch1(&self) -> Sqrch1R {
        Sqrch1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc SQRCH2"]
    #[inline(always)]
    pub fn sqrch2(&self) -> Sqrch2R {
        Sqrch2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc SQRCH3"]
    #[inline(always)]
    pub fn sqrch3(&self) -> Sqrch3R {
        Sqrch3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc SQRCH4"]
    #[inline(always)]
    pub fn sqrch4(&self) -> Sqrch4R {
        Sqrch4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - desc SQRCH5"]
    #[inline(always)]
    pub fn sqrch5(&self) -> Sqrch5R {
        Sqrch5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc SQRCH6"]
    #[inline(always)]
    pub fn sqrch6(&self) -> Sqrch6R {
        Sqrch6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - desc SQRCH7"]
    #[inline(always)]
    pub fn sqrch7(&self) -> Sqrch7R {
        Sqrch7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SQRCH0"]
    #[inline(always)]
    pub fn sqrch0(&mut self) -> Sqrch0W<'_, SqrcfrSpec> {
        Sqrch0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc SQRCH1"]
    #[inline(always)]
    pub fn sqrch1(&mut self) -> Sqrch1W<'_, SqrcfrSpec> {
        Sqrch1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc SQRCH2"]
    #[inline(always)]
    pub fn sqrch2(&mut self) -> Sqrch2W<'_, SqrcfrSpec> {
        Sqrch2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc SQRCH3"]
    #[inline(always)]
    pub fn sqrch3(&mut self) -> Sqrch3W<'_, SqrcfrSpec> {
        Sqrch3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - desc SQRCH4"]
    #[inline(always)]
    pub fn sqrch4(&mut self) -> Sqrch4W<'_, SqrcfrSpec> {
        Sqrch4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - desc SQRCH5"]
    #[inline(always)]
    pub fn sqrch5(&mut self) -> Sqrch5W<'_, SqrcfrSpec> {
        Sqrch5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - desc SQRCH6"]
    #[inline(always)]
    pub fn sqrch6(&mut self) -> Sqrch6W<'_, SqrcfrSpec> {
        Sqrch6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - desc SQRCH7"]
    #[inline(always)]
    pub fn sqrch7(&mut self) -> Sqrch7W<'_, SqrcfrSpec> {
        Sqrch7W::new(self, 28)
    }
}
#[doc = "desc SQRCFR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqrcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqrcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SqrcfrSpec;
impl crate::RegisterSpec for SqrcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqrcfr::R`](R) reader structure"]
impl crate::Readable for SqrcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sqrcfr::W`](W) writer structure"]
impl crate::Writable for SqrcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SQRCFR to value 0"]
impl crate::Resettable for SqrcfrSpec {}
