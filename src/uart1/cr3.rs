#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `DEM` reader - desc DEM"]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - desc DEM"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - desc DEP"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - desc DEP"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DETIME` reader - desc DETIME"]
pub type DetimeR = crate::FieldReader;
#[doc = "Field `DETIME` writer - desc DETIME"]
pub type DetimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LIN` reader - desc LIN"]
pub type LinR = crate::BitReader;
#[doc = "Field `LIN` writer - desc LIN"]
pub type LinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKL` reader - desc BRKL"]
pub type BrklR = crate::BitReader;
#[doc = "Field `BRKL` writer - desc BRKL"]
pub type BrklW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DEM"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DEP"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - desc DETIME"]
    #[inline(always)]
    pub fn detime(&self) -> DetimeR {
        DetimeR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - desc LIN"]
    #[inline(always)]
    pub fn lin(&self) -> LinR {
        LinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BRKL"]
    #[inline(always)]
    pub fn brkl(&self) -> BrklR {
        BrklR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DEM"]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Cr3Spec> {
        DemW::new(self, 0)
    }
    #[doc = "Bit 1 - desc DEP"]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<'_, Cr3Spec> {
        DepW::new(self, 1)
    }
    #[doc = "Bits 2:6 - desc DETIME"]
    #[inline(always)]
    pub fn detime(&mut self) -> DetimeW<'_, Cr3Spec> {
        DetimeW::new(self, 2)
    }
    #[doc = "Bit 7 - desc LIN"]
    #[inline(always)]
    pub fn lin(&mut self) -> LinW<'_, Cr3Spec> {
        LinW::new(self, 7)
    }
    #[doc = "Bit 8 - desc BRKL"]
    #[inline(always)]
    pub fn brkl(&mut self) -> BrklW<'_, Cr3Spec> {
        BrklW::new(self, 8)
    }
}
#[doc = "Control register3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
