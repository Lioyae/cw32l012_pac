#[doc = "Register `SWTRGR` reader"]
pub type R = crate::R<SwtrgrSpec>;
#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SwtrgrSpec>;
#[doc = "Field `SWTRIG1` reader - desc SWTRIG1"]
pub type Swtrig1R = crate::BitReader;
#[doc = "Field `SWTRIG1` writer - desc SWTRIG1"]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` reader - desc SWTRIG2"]
pub type Swtrig2R = crate::BitReader;
#[doc = "Field `SWTRIG2` writer - desc SWTRIG2"]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SWTRIG1"]
    #[inline(always)]
    pub fn swtrig1(&self) -> Swtrig1R {
        Swtrig1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SWTRIG2"]
    #[inline(always)]
    pub fn swtrig2(&self) -> Swtrig2R {
        Swtrig2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SWTRIG1"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<'_, SwtrgrSpec> {
        Swtrig1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc SWTRIG2"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> Swtrig2W<'_, SwtrgrSpec> {
        Swtrig2W::new(self, 1)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrgrSpec;
impl crate::RegisterSpec for SwtrgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrgr::R`](R) reader structure"]
impl crate::Readable for SwtrgrSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SwtrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SwtrgrSpec {}
