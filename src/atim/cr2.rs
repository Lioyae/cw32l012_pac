#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCPC` reader - desc CCPC"]
pub type CcpcR = crate::BitReader;
#[doc = "Field `CCPC` writer - desc CCPC"]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - desc CCUS"]
pub type CcusR = crate::BitReader;
#[doc = "Field `CCUS` writer - desc CCUS"]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - desc CCDS"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - desc CCDS"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - desc MMS"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - desc MMS"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - desc TI1S"]
pub type Ti1sR = crate::BitReader;
#[doc = "Field `TI1S` writer - desc TI1S"]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - desc OIS1"]
pub type Ois1R = crate::BitReader;
#[doc = "Field `OIS1` writer - desc OIS1"]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - desc OIS1N"]
pub type Ois1nR = crate::BitReader;
#[doc = "Field `OIS1N` writer - desc OIS1N"]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - desc OIS2"]
pub type Ois2R = crate::BitReader;
#[doc = "Field `OIS2` writer - desc OIS2"]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - desc OIS2N"]
pub type Ois2nR = crate::BitReader;
#[doc = "Field `OIS2N` writer - desc OIS2N"]
pub type Ois2nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - desc OIS3"]
pub type Ois3R = crate::BitReader;
#[doc = "Field `OIS3` writer - desc OIS3"]
pub type Ois3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - desc OIS3N"]
pub type Ois3nR = crate::BitReader;
#[doc = "Field `OIS3N` writer - desc OIS3N"]
pub type Ois3nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - desc OIS4"]
pub type Ois4R = crate::BitReader;
#[doc = "Field `OIS4` writer - desc OIS4"]
pub type Ois4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4N` reader - desc OIS4N"]
pub type Ois4nR = crate::BitReader;
#[doc = "Field `OIS4N` writer - desc OIS4N"]
pub type Ois4nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - desc OIS5"]
pub type Ois5R = crate::BitReader;
#[doc = "Field `OIS5` writer - desc OIS5"]
pub type Ois5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5N` reader - desc OIS5N"]
pub type Ois5nR = crate::BitReader;
#[doc = "Field `OIS5N` writer - desc OIS5N"]
pub type Ois5nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - desc OIS6"]
pub type Ois6R = crate::BitReader;
#[doc = "Field `OIS6` writer - desc OIS6"]
pub type Ois6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6N` reader - desc OIS6N"]
pub type Ois6nR = crate::BitReader;
#[doc = "Field `OIS6N` writer - desc OIS6N"]
pub type Ois6nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - desc MMS2"]
pub type Mms2R = crate::FieldReader;
#[doc = "Field `MMS2` writer - desc MMS2"]
pub type Mms2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MMSH` reader - desc MMSH"]
pub type MmshR = crate::FieldReader;
#[doc = "Field `MMSH` writer - desc MMSH"]
pub type MmshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc CCPC"]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc CCUS"]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc OIS1"]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc OIS1N"]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc OIS2"]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OIS2N"]
    #[inline(always)]
    pub fn ois2n(&self) -> Ois2nR {
        Ois2nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc OIS3"]
    #[inline(always)]
    pub fn ois3(&self) -> Ois3R {
        Ois3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc OIS3N"]
    #[inline(always)]
    pub fn ois3n(&self) -> Ois3nR {
        Ois3nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc OIS4"]
    #[inline(always)]
    pub fn ois4(&self) -> Ois4R {
        Ois4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc OIS4N"]
    #[inline(always)]
    pub fn ois4n(&self) -> Ois4nR {
        Ois4nR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OIS5"]
    #[inline(always)]
    pub fn ois5(&self) -> Ois5R {
        Ois5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc OIS5N"]
    #[inline(always)]
    pub fn ois5n(&self) -> Ois5nR {
        Ois5nR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc OIS6"]
    #[inline(always)]
    pub fn ois6(&self) -> Ois6R {
        Ois6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc OIS6N"]
    #[inline(always)]
    pub fn ois6n(&self) -> Ois6nR {
        Ois6nR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - desc MMS2"]
    #[inline(always)]
    pub fn mms2(&self) -> Mms2R {
        Mms2R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - desc MMSH"]
    #[inline(always)]
    pub fn mmsh(&self) -> MmshR {
        MmshR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc CCPC"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CcpcW<'_, Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - desc CCUS"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CcusW<'_, Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<'_, Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc TI1S"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> Ti1sW<'_, Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bit 8 - desc OIS1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> Ois1W<'_, Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc OIS1N"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> Ois1nW<'_, Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10 - desc OIS2"]
    #[inline(always)]
    pub fn ois2(&mut self) -> Ois2W<'_, Cr2Spec> {
        Ois2W::new(self, 10)
    }
    #[doc = "Bit 11 - desc OIS2N"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> Ois2nW<'_, Cr2Spec> {
        Ois2nW::new(self, 11)
    }
    #[doc = "Bit 12 - desc OIS3"]
    #[inline(always)]
    pub fn ois3(&mut self) -> Ois3W<'_, Cr2Spec> {
        Ois3W::new(self, 12)
    }
    #[doc = "Bit 13 - desc OIS3N"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> Ois3nW<'_, Cr2Spec> {
        Ois3nW::new(self, 13)
    }
    #[doc = "Bit 14 - desc OIS4"]
    #[inline(always)]
    pub fn ois4(&mut self) -> Ois4W<'_, Cr2Spec> {
        Ois4W::new(self, 14)
    }
    #[doc = "Bit 15 - desc OIS4N"]
    #[inline(always)]
    pub fn ois4n(&mut self) -> Ois4nW<'_, Cr2Spec> {
        Ois4nW::new(self, 15)
    }
    #[doc = "Bit 16 - desc OIS5"]
    #[inline(always)]
    pub fn ois5(&mut self) -> Ois5W<'_, Cr2Spec> {
        Ois5W::new(self, 16)
    }
    #[doc = "Bit 17 - desc OIS5N"]
    #[inline(always)]
    pub fn ois5n(&mut self) -> Ois5nW<'_, Cr2Spec> {
        Ois5nW::new(self, 17)
    }
    #[doc = "Bit 18 - desc OIS6"]
    #[inline(always)]
    pub fn ois6(&mut self) -> Ois6W<'_, Cr2Spec> {
        Ois6W::new(self, 18)
    }
    #[doc = "Bit 19 - desc OIS6N"]
    #[inline(always)]
    pub fn ois6n(&mut self) -> Ois6nW<'_, Cr2Spec> {
        Ois6nW::new(self, 19)
    }
    #[doc = "Bits 20:24 - desc MMS2"]
    #[inline(always)]
    pub fn mms2(&mut self) -> Mms2W<'_, Cr2Spec> {
        Mms2W::new(self, 20)
    }
    #[doc = "Bits 25:26 - desc MMSH"]
    #[inline(always)]
    pub fn mmsh(&mut self) -> MmshW<'_, Cr2Spec> {
        MmshW::new(self, 25)
    }
}
#[doc = "Control Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
