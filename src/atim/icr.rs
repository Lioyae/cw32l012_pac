#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `UIF` reader - desc UIF"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - desc UIF"]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - desc CC1IF"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - desc CC1IF"]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - desc CC2IF"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - desc CC2IF"]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - desc CC3IF"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - desc CC3IF"]
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - desc CC4IF"]
pub type Cc4ifR = crate::BitReader;
#[doc = "Field `CC4IF` writer - desc CC4IF"]
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - desc COMIF"]
pub type ComifR = crate::BitReader;
#[doc = "Field `COMIF` writer - desc COMIF"]
pub type ComifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - desc TIF"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - desc TIF"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - desc BIF"]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - desc BIF"]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2IF` reader - desc B2IF"]
pub type B2ifR = crate::BitReader;
#[doc = "Field `B2IF` writer - desc B2IF"]
pub type B2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1OF` reader - desc CC1OF"]
pub type Cc1ofR = crate::BitReader;
#[doc = "Field `CC1OF` writer - desc CC1OF"]
pub type Cc1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2OF` reader - desc CC2OF"]
pub type Cc2ofR = crate::BitReader;
#[doc = "Field `CC2OF` writer - desc CC2OF"]
pub type Cc2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - desc CC3OF"]
pub type Cc3ofR = crate::BitReader;
#[doc = "Field `CC3OF` writer - desc CC3OF"]
pub type Cc3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - desc CC4OF"]
pub type Cc4ofR = crate::BitReader;
#[doc = "Field `CC4OF` writer - desc CC4OF"]
pub type Cc4ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIF` reader - desc SBIF"]
pub type SbifR = crate::BitReader;
#[doc = "Field `SBIF` writer - desc SBIF"]
pub type SbifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - desc CC5IF"]
pub type Cc5ifR = crate::BitReader;
#[doc = "Field `CC5IF` writer - desc CC5IF"]
pub type Cc5ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - desc CC6IF"]
pub type Cc6ifR = crate::BitReader;
#[doc = "Field `CC6IF` writer - desc CC6IF"]
pub type Cc6ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5OF` reader - desc CC5OF"]
pub type Cc5ofR = crate::BitReader;
#[doc = "Field `CC5OF` writer - desc CC5OF"]
pub type Cc5ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6OF` reader - desc CC6OF"]
pub type Cc6ofR = crate::BitReader;
#[doc = "Field `CC6OF` writer - desc CC6OF"]
pub type Cc6ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXF` reader - desc IDXF"]
pub type IdxfR = crate::BitReader;
#[doc = "Field `IDXF` writer - desc IDXF"]
pub type IdxfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRF` reader - desc DIRF"]
pub type DirfR = crate::BitReader;
#[doc = "Field `DIRF` writer - desc DIRF"]
pub type DirfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRF` reader - desc IERRF"]
pub type IerrfR = crate::BitReader;
#[doc = "Field `IERRF` writer - desc IERRF"]
pub type IerrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRF` reader - desc TERRF"]
pub type TerrfR = crate::BitReader;
#[doc = "Field `TERRF` writer - desc TERRF"]
pub type TerrfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1IF"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC2IF"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC3IF"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC4IF"]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COMIF"]
    #[inline(always)]
    pub fn comif(&self) -> ComifR {
        ComifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc B2IF"]
    #[inline(always)]
    pub fn b2if(&self) -> B2ifR {
        B2ifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CC1OF"]
    #[inline(always)]
    pub fn cc1of(&self) -> Cc1ofR {
        Cc1ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CC2OF"]
    #[inline(always)]
    pub fn cc2of(&self) -> Cc2ofR {
        Cc2ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CC3OF"]
    #[inline(always)]
    pub fn cc3of(&self) -> Cc3ofR {
        Cc3ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CC4OF"]
    #[inline(always)]
    pub fn cc4of(&self) -> Cc4ofR {
        Cc4ofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SBIF"]
    #[inline(always)]
    pub fn sbif(&self) -> SbifR {
        SbifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CC5IF"]
    #[inline(always)]
    pub fn cc5if(&self) -> Cc5ifR {
        Cc5ifR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CC6IF"]
    #[inline(always)]
    pub fn cc6if(&self) -> Cc6ifR {
        Cc6ifR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc CC5OF"]
    #[inline(always)]
    pub fn cc5of(&self) -> Cc5ofR {
        Cc5ofR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CC6OF"]
    #[inline(always)]
    pub fn cc6of(&self) -> Cc6ofR {
        Cc6ofR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IDXF"]
    #[inline(always)]
    pub fn idxf(&self) -> IdxfR {
        IdxfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc DIRF"]
    #[inline(always)]
    pub fn dirf(&self) -> DirfR {
        DirfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc IERRF"]
    #[inline(always)]
    pub fn ierrf(&self) -> IerrfR {
        IerrfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TERRF"]
    #[inline(always)]
    pub fn terrf(&self) -> TerrfR {
        TerrfR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, IcrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CC1IF"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<'_, IcrSpec> {
        Cc1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CC2IF"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<'_, IcrSpec> {
        Cc2ifW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CC3IF"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<'_, IcrSpec> {
        Cc3ifW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CC4IF"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<'_, IcrSpec> {
        Cc4ifW::new(self, 4)
    }
    #[doc = "Bit 5 - desc COMIF"]
    #[inline(always)]
    pub fn comif(&mut self) -> ComifW<'_, IcrSpec> {
        ComifW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TIF"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, IcrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BIF"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<'_, IcrSpec> {
        BifW::new(self, 7)
    }
    #[doc = "Bit 8 - desc B2IF"]
    #[inline(always)]
    pub fn b2if(&mut self) -> B2ifW<'_, IcrSpec> {
        B2ifW::new(self, 8)
    }
    #[doc = "Bit 9 - desc CC1OF"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> Cc1ofW<'_, IcrSpec> {
        Cc1ofW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CC2OF"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> Cc2ofW<'_, IcrSpec> {
        Cc2ofW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CC3OF"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> Cc3ofW<'_, IcrSpec> {
        Cc3ofW::new(self, 11)
    }
    #[doc = "Bit 12 - desc CC4OF"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> Cc4ofW<'_, IcrSpec> {
        Cc4ofW::new(self, 12)
    }
    #[doc = "Bit 13 - desc SBIF"]
    #[inline(always)]
    pub fn sbif(&mut self) -> SbifW<'_, IcrSpec> {
        SbifW::new(self, 13)
    }
    #[doc = "Bit 16 - desc CC5IF"]
    #[inline(always)]
    pub fn cc5if(&mut self) -> Cc5ifW<'_, IcrSpec> {
        Cc5ifW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CC6IF"]
    #[inline(always)]
    pub fn cc6if(&mut self) -> Cc6ifW<'_, IcrSpec> {
        Cc6ifW::new(self, 17)
    }
    #[doc = "Bit 18 - desc CC5OF"]
    #[inline(always)]
    pub fn cc5of(&mut self) -> Cc5ofW<'_, IcrSpec> {
        Cc5ofW::new(self, 18)
    }
    #[doc = "Bit 19 - desc CC6OF"]
    #[inline(always)]
    pub fn cc6of(&mut self) -> Cc6ofW<'_, IcrSpec> {
        Cc6ofW::new(self, 19)
    }
    #[doc = "Bit 20 - desc IDXF"]
    #[inline(always)]
    pub fn idxf(&mut self) -> IdxfW<'_, IcrSpec> {
        IdxfW::new(self, 20)
    }
    #[doc = "Bit 21 - desc DIRF"]
    #[inline(always)]
    pub fn dirf(&mut self) -> DirfW<'_, IcrSpec> {
        DirfW::new(self, 21)
    }
    #[doc = "Bit 22 - desc IERRF"]
    #[inline(always)]
    pub fn ierrf(&mut self) -> IerrfW<'_, IcrSpec> {
        IerrfW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TERRF"]
    #[inline(always)]
    pub fn terrf(&mut self) -> TerrfW<'_, IcrSpec> {
        TerrfW::new(self, 23)
    }
}
#[doc = "Interrupt clean register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
