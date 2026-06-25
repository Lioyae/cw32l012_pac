#[doc = "Register `AF2` reader"]
pub type R = crate::R<Af2Spec>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<Af2Spec>;
#[doc = "Field `BK2INE` reader - desc BK2INE"]
pub type Bk2ineR = crate::BitReader;
#[doc = "Field `BK2INE` writer - desc BK2INE"]
pub type Bk2ineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC1E` reader - desc BK2VC1E"]
pub type Bk2vc1eR = crate::BitReader;
#[doc = "Field `BK2VC1E` writer - desc BK2VC1E"]
pub type Bk2vc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC2E` reader - desc BK2VC2E"]
pub type Bk2vc2eR = crate::BitReader;
#[doc = "Field `BK2VC2E` writer - desc BK2VC2E"]
pub type Bk2vc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC3E` reader - desc BK2VC3E"]
pub type Bk2vc3eR = crate::BitReader;
#[doc = "Field `BK2VC3E` writer - desc BK2VC3E"]
pub type Bk2vc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC4E` reader - desc BK2VC4E"]
pub type Bk2vc4eR = crate::BitReader;
#[doc = "Field `BK2VC4E` writer - desc BK2VC4E"]
pub type Bk2vc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - desc BK2INP"]
pub type Bk2inpR = crate::BitReader;
#[doc = "Field `BK2INP` writer - desc BK2INP"]
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC1P` reader - desc BK2VC1P"]
pub type Bk2vc1pR = crate::BitReader;
#[doc = "Field `BK2VC1P` writer - desc BK2VC1P"]
pub type Bk2vc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC2P` reader - desc BK2VC2P"]
pub type Bk2vc2pR = crate::BitReader;
#[doc = "Field `BK2VC2P` writer - desc BK2VC2P"]
pub type Bk2vc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC3P` reader - desc BK2VC3P"]
pub type Bk2vc3pR = crate::BitReader;
#[doc = "Field `BK2VC3P` writer - desc BK2VC3P"]
pub type Bk2vc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2VC4P` reader - desc BK2VC4P"]
pub type Bk2vc4pR = crate::BitReader;
#[doc = "Field `BK2VC4P` writer - desc BK2VC4P"]
pub type Bk2vc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCRSEL` reader - desc OCRSEL"]
pub type OcrselR = crate::FieldReader;
#[doc = "Field `OCRSEL` writer - desc OCRSEL"]
pub type OcrselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - desc BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&self) -> Bk2ineR {
        Bk2ineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BK2VC1E"]
    #[inline(always)]
    pub fn bk2vc1e(&self) -> Bk2vc1eR {
        Bk2vc1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BK2VC2E"]
    #[inline(always)]
    pub fn bk2vc2e(&self) -> Bk2vc2eR {
        Bk2vc2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BK2VC3E"]
    #[inline(always)]
    pub fn bk2vc3e(&self) -> Bk2vc3eR {
        Bk2vc3eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BK2VC4E"]
    #[inline(always)]
    pub fn bk2vc4e(&self) -> Bk2vc4eR {
        Bk2vc4eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BK2VC1P"]
    #[inline(always)]
    pub fn bk2vc1p(&self) -> Bk2vc1pR {
        Bk2vc1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BK2VC2P"]
    #[inline(always)]
    pub fn bk2vc2p(&self) -> Bk2vc2pR {
        Bk2vc2pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BK2VC3P"]
    #[inline(always)]
    pub fn bk2vc3p(&self) -> Bk2vc3pR {
        Bk2vc3pR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc BK2VC4P"]
    #[inline(always)]
    pub fn bk2vc4p(&self) -> Bk2vc4pR {
        Bk2vc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc OCRSEL"]
    #[inline(always)]
    pub fn ocrsel(&self) -> OcrselR {
        OcrselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc BK2INE"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> Bk2ineW<'_, Af2Spec> {
        Bk2ineW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BK2VC1E"]
    #[inline(always)]
    pub fn bk2vc1e(&mut self) -> Bk2vc1eW<'_, Af2Spec> {
        Bk2vc1eW::new(self, 1)
    }
    #[doc = "Bit 2 - desc BK2VC2E"]
    #[inline(always)]
    pub fn bk2vc2e(&mut self) -> Bk2vc2eW<'_, Af2Spec> {
        Bk2vc2eW::new(self, 2)
    }
    #[doc = "Bit 3 - desc BK2VC3E"]
    #[inline(always)]
    pub fn bk2vc3e(&mut self) -> Bk2vc3eW<'_, Af2Spec> {
        Bk2vc3eW::new(self, 3)
    }
    #[doc = "Bit 4 - desc BK2VC4E"]
    #[inline(always)]
    pub fn bk2vc4e(&mut self) -> Bk2vc4eW<'_, Af2Spec> {
        Bk2vc4eW::new(self, 4)
    }
    #[doc = "Bit 9 - desc BK2INP"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> Bk2inpW<'_, Af2Spec> {
        Bk2inpW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BK2VC1P"]
    #[inline(always)]
    pub fn bk2vc1p(&mut self) -> Bk2vc1pW<'_, Af2Spec> {
        Bk2vc1pW::new(self, 10)
    }
    #[doc = "Bit 11 - desc BK2VC2P"]
    #[inline(always)]
    pub fn bk2vc2p(&mut self) -> Bk2vc2pW<'_, Af2Spec> {
        Bk2vc2pW::new(self, 11)
    }
    #[doc = "Bit 12 - desc BK2VC3P"]
    #[inline(always)]
    pub fn bk2vc3p(&mut self) -> Bk2vc3pW<'_, Af2Spec> {
        Bk2vc3pW::new(self, 12)
    }
    #[doc = "Bit 13 - desc BK2VC4P"]
    #[inline(always)]
    pub fn bk2vc4p(&mut self) -> Bk2vc4pW<'_, Af2Spec> {
        Bk2vc4pW::new(self, 13)
    }
    #[doc = "Bits 16:19 - desc OCRSEL"]
    #[inline(always)]
    pub fn ocrsel(&mut self) -> OcrselW<'_, Af2Spec> {
        OcrselW::new(self, 16)
    }
}
#[doc = "Alternate function Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af2Spec;
impl crate::RegisterSpec for Af2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for Af2Spec {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for Af2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for Af2Spec {}
