#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `BGREN` reader - desc BGREN"]
pub type BgrenR = crate::BitReader;
#[doc = "Field `BGREN` writer - desc BGREN"]
pub type BgrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - desc TSEN"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - desc TSEN"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BgrenR {
        BgrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&mut self) -> BgrenW<'_, CrSpec> {
        BgrenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, CrSpec> {
        TsenW::new(self, 1)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
