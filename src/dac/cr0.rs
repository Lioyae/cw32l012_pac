#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `EN1` reader - desc EN1"]
pub type En1R = crate::BitReader;
#[doc = "Field `EN1` writer - desc EN1"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN1` reader - desc TEN1"]
pub type Ten1R = crate::BitReader;
#[doc = "Field `TEN1` writer - desc TEN1"]
pub type Ten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL1` reader - desc TSEL1"]
pub type Tsel1R = crate::FieldReader;
#[doc = "Field `TSEL1` writer - desc TSEL1"]
pub type Tsel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE1` reader - desc WAVE1"]
pub type Wave1R = crate::FieldReader;
#[doc = "Field `WAVE1` writer - desc WAVE1"]
pub type Wave1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP1` reader - desc MAMP1"]
pub type Mamp1R = crate::FieldReader;
#[doc = "Field `MAMP1` writer - desc MAMP1"]
pub type Mamp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN1` reader - desc DMAEN1"]
pub type Dmaen1R = crate::BitReader;
#[doc = "Field `DMAEN1` writer - desc DMAEN1"]
pub type Dmaen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE1` reader - desc DMAUDRIE1"]
pub type Dmaudrie1R = crate::BitReader;
#[doc = "Field `DMAUDRIE1` writer - desc DMAUDRIE1"]
pub type Dmaudrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN2` reader - desc EN2"]
pub type En2R = crate::BitReader;
#[doc = "Field `EN2` writer - desc EN2"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN2` reader - desc TEN2"]
pub type Ten2R = crate::BitReader;
#[doc = "Field `TEN2` writer - desc TEN2"]
pub type Ten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL2` reader - desc TSEL2"]
pub type Tsel2R = crate::FieldReader;
#[doc = "Field `TSEL2` writer - desc TSEL2"]
pub type Tsel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WAVE2` reader - desc WAVE2"]
pub type Wave2R = crate::FieldReader;
#[doc = "Field `WAVE2` writer - desc WAVE2"]
pub type Wave2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP2` reader - desc MAMP2"]
pub type Mamp2R = crate::FieldReader;
#[doc = "Field `MAMP2` writer - desc MAMP2"]
pub type Mamp2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN2` reader - desc DMAEN2"]
pub type Dmaen2R = crate::BitReader;
#[doc = "Field `DMAEN2` writer - desc DMAEN2"]
pub type Dmaen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE2` reader - desc DMAUDRIE2"]
pub type Dmaudrie2R = crate::BitReader;
#[doc = "Field `DMAUDRIE2` writer - desc DMAUDRIE2"]
pub type Dmaudrie2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN1"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TEN1"]
    #[inline(always)]
    pub fn ten1(&self) -> Ten1R {
        Ten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - desc TSEL1"]
    #[inline(always)]
    pub fn tsel1(&self) -> Tsel1R {
        Tsel1R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - desc WAVE1"]
    #[inline(always)]
    pub fn wave1(&self) -> Wave1R {
        Wave1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - desc MAMP1"]
    #[inline(always)]
    pub fn mamp1(&self) -> Mamp1R {
        Mamp1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&self) -> Dmaen1R {
        Dmaen1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> Dmaudrie1R {
        Dmaudrie1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EN2"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc TEN2"]
    #[inline(always)]
    pub fn ten2(&self) -> Ten2R {
        Ten2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - desc TSEL2"]
    #[inline(always)]
    pub fn tsel2(&self) -> Tsel2R {
        Tsel2R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - desc WAVE2"]
    #[inline(always)]
    pub fn wave2(&self) -> Wave2R {
        Wave2R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - desc MAMP2"]
    #[inline(always)]
    pub fn mamp2(&self) -> Mamp2R {
        Mamp2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc DMAEN2"]
    #[inline(always)]
    pub fn dmaen2(&self) -> Dmaen2R {
        Dmaen2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DMAUDRIE2"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> Dmaudrie2R {
        Dmaudrie2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN1"]
    #[inline(always)]
    pub fn en1(&mut self) -> En1W<'_, Cr0Spec> {
        En1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc TEN1"]
    #[inline(always)]
    pub fn ten1(&mut self) -> Ten1W<'_, Cr0Spec> {
        Ten1W::new(self, 1)
    }
    #[doc = "Bits 2:5 - desc TSEL1"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> Tsel1W<'_, Cr0Spec> {
        Tsel1W::new(self, 2)
    }
    #[doc = "Bits 6:7 - desc WAVE1"]
    #[inline(always)]
    pub fn wave1(&mut self) -> Wave1W<'_, Cr0Spec> {
        Wave1W::new(self, 6)
    }
    #[doc = "Bits 8:11 - desc MAMP1"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> Mamp1W<'_, Cr0Spec> {
        Mamp1W::new(self, 8)
    }
    #[doc = "Bit 12 - desc DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> Dmaen1W<'_, Cr0Spec> {
        Dmaen1W::new(self, 12)
    }
    #[doc = "Bit 13 - desc DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> Dmaudrie1W<'_, Cr0Spec> {
        Dmaudrie1W::new(self, 13)
    }
    #[doc = "Bit 16 - desc EN2"]
    #[inline(always)]
    pub fn en2(&mut self) -> En2W<'_, Cr0Spec> {
        En2W::new(self, 16)
    }
    #[doc = "Bit 17 - desc TEN2"]
    #[inline(always)]
    pub fn ten2(&mut self) -> Ten2W<'_, Cr0Spec> {
        Ten2W::new(self, 17)
    }
    #[doc = "Bits 18:21 - desc TSEL2"]
    #[inline(always)]
    pub fn tsel2(&mut self) -> Tsel2W<'_, Cr0Spec> {
        Tsel2W::new(self, 18)
    }
    #[doc = "Bits 22:23 - desc WAVE2"]
    #[inline(always)]
    pub fn wave2(&mut self) -> Wave2W<'_, Cr0Spec> {
        Wave2W::new(self, 22)
    }
    #[doc = "Bits 24:27 - desc MAMP2"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> Mamp2W<'_, Cr0Spec> {
        Mamp2W::new(self, 24)
    }
    #[doc = "Bit 28 - desc DMAEN2"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> Dmaen2W<'_, Cr0Spec> {
        Dmaen2W::new(self, 28)
    }
    #[doc = "Bit 29 - desc DMAUDRIE2"]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> Dmaudrie2W<'_, Cr0Spec> {
        Dmaudrie2W::new(self, 29)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
