#[doc = "Register `CCR6` reader"]
pub type R = crate::R<Ccr6Spec>;
#[doc = "Register `CCR6` writer"]
pub type W = crate::W<Ccr6Spec>;
#[doc = "Field `CCR6` reader - desc CCR6"]
pub type Ccr6R = crate::FieldReader<u16>;
#[doc = "Field `CCR6` writer - desc CCR6"]
pub type Ccr6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GC6C1` reader - desc GC6C1"]
pub type Gc6c1R = crate::BitReader;
#[doc = "Field `GC6C1` writer - desc GC6C1"]
pub type Gc6c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC6C2` reader - desc GC6C2"]
pub type Gc6c2R = crate::BitReader;
#[doc = "Field `GC6C2` writer - desc GC6C2"]
pub type Gc6c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC6C3` reader - desc GC6C3"]
pub type Gc6c3R = crate::BitReader;
#[doc = "Field `GC6C3` writer - desc GC6C3"]
pub type Gc6c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC6C4` reader - desc GC6C4"]
pub type Gc6c4R = crate::BitReader;
#[doc = "Field `GC6C4` writer - desc GC6C4"]
pub type Gc6c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC6C5` reader - desc GC6C5"]
pub type Gc6c5R = crate::BitReader;
#[doc = "Field `GC6C5` writer - desc GC6C5"]
pub type Gc6c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC6C6` reader - desc GC6C6"]
pub type Gc6c6R = crate::BitReader;
#[doc = "Field `GC6C6` writer - desc GC6C6"]
pub type Gc6c6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - desc CCR6"]
    #[inline(always)]
    pub fn ccr6(&self) -> Ccr6R {
        Ccr6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 26 - desc GC6C1"]
    #[inline(always)]
    pub fn gc6c1(&self) -> Gc6c1R {
        Gc6c1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc GC6C2"]
    #[inline(always)]
    pub fn gc6c2(&self) -> Gc6c2R {
        Gc6c2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc GC6C3"]
    #[inline(always)]
    pub fn gc6c3(&self) -> Gc6c3R {
        Gc6c3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc GC6C4"]
    #[inline(always)]
    pub fn gc6c4(&self) -> Gc6c4R {
        Gc6c4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc GC6C5"]
    #[inline(always)]
    pub fn gc6c5(&self) -> Gc6c5R {
        Gc6c5R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc GC6C6"]
    #[inline(always)]
    pub fn gc6c6(&self) -> Gc6c6R {
        Gc6c6R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR6"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> Ccr6W<'_, Ccr6Spec> {
        Ccr6W::new(self, 0)
    }
    #[doc = "Bit 26 - desc GC6C1"]
    #[inline(always)]
    pub fn gc6c1(&mut self) -> Gc6c1W<'_, Ccr6Spec> {
        Gc6c1W::new(self, 26)
    }
    #[doc = "Bit 27 - desc GC6C2"]
    #[inline(always)]
    pub fn gc6c2(&mut self) -> Gc6c2W<'_, Ccr6Spec> {
        Gc6c2W::new(self, 27)
    }
    #[doc = "Bit 28 - desc GC6C3"]
    #[inline(always)]
    pub fn gc6c3(&mut self) -> Gc6c3W<'_, Ccr6Spec> {
        Gc6c3W::new(self, 28)
    }
    #[doc = "Bit 29 - desc GC6C4"]
    #[inline(always)]
    pub fn gc6c4(&mut self) -> Gc6c4W<'_, Ccr6Spec> {
        Gc6c4W::new(self, 29)
    }
    #[doc = "Bit 30 - desc GC6C5"]
    #[inline(always)]
    pub fn gc6c5(&mut self) -> Gc6c5W<'_, Ccr6Spec> {
        Gc6c5W::new(self, 30)
    }
    #[doc = "Bit 31 - desc GC6C6"]
    #[inline(always)]
    pub fn gc6c6(&mut self) -> Gc6c6W<'_, Ccr6Spec> {
        Gc6c6W::new(self, 31)
    }
}
#[doc = "capture compare register6\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr6Spec;
impl crate::RegisterSpec for Ccr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr6::R`](R) reader structure"]
impl crate::Readable for Ccr6Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr6::W`](W) writer structure"]
impl crate::Writable for Ccr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR6 to value 0"]
impl crate::Resettable for Ccr6Spec {}
