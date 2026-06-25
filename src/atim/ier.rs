#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - desc CC1IE"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - desc CC1IE"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - desc CC2IE"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - desc CC2IE"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - desc CC3IE"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - desc CC3IE"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - desc CC4IE"]
pub type Cc4ieR = crate::BitReader;
#[doc = "Field `CC4IE` writer - desc CC4IE"]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - desc COMIE"]
pub type ComieR = crate::BitReader;
#[doc = "Field `COMIE` writer - desc COMIE"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - desc BIE"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - desc BIE"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - desc UDE"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - desc UDE"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - desc CC1DE"]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - desc CC1DE"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - desc CC2DE"]
pub type Cc2deR = crate::BitReader;
#[doc = "Field `CC2DE` writer - desc CC2DE"]
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DE` reader - desc CC3DE"]
pub type Cc3deR = crate::BitReader;
#[doc = "Field `CC3DE` writer - desc CC3DE"]
pub type Cc3deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4DE` reader - desc CC4DE"]
pub type Cc4deR = crate::BitReader;
#[doc = "Field `CC4DE` writer - desc CC4DE"]
pub type Cc4deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMDE` reader - desc COMDE"]
pub type ComdeR = crate::BitReader;
#[doc = "Field `COMDE` writer - desc COMDE"]
pub type ComdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - desc TDE"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - desc TDE"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IE` reader - desc CC5IE"]
pub type Cc5ieR = crate::BitReader;
#[doc = "Field `CC5IE` writer - desc CC5IE"]
pub type Cc5ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IE` reader - desc CC6IE"]
pub type Cc6ieR = crate::BitReader;
#[doc = "Field `CC6IE` writer - desc CC6IE"]
pub type Cc6ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5DE` reader - desc CC5DE"]
pub type Cc5deR = crate::BitReader;
#[doc = "Field `CC5DE` writer - desc CC5DE"]
pub type Cc5deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6DE` reader - desc CC6DE"]
pub type Cc6deR = crate::BitReader;
#[doc = "Field `CC6DE` writer - desc CC6DE"]
pub type Cc6deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDXIE` reader - desc IDXIE"]
pub type IdxieR = crate::BitReader;
#[doc = "Field `IDXIE` writer - desc IDXIE"]
pub type IdxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRIE` reader - desc DIRIE"]
pub type DirieR = crate::BitReader;
#[doc = "Field `DIRIE` writer - desc DIRIE"]
pub type DirieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IERRIE` reader - desc IERRIE"]
pub type IerrieR = crate::BitReader;
#[doc = "Field `IERRIE` writer - desc IERRIE"]
pub type IerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - desc TERRIE"]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - desc TERRIE"]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc COMIE"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CC1DE"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CC2DE"]
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CC3DE"]
    #[inline(always)]
    pub fn cc3de(&self) -> Cc3deR {
        Cc3deR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CC4DE"]
    #[inline(always)]
    pub fn cc4de(&self) -> Cc4deR {
        Cc4deR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc COMDE"]
    #[inline(always)]
    pub fn comde(&self) -> ComdeR {
        ComdeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc CC5IE"]
    #[inline(always)]
    pub fn cc5ie(&self) -> Cc5ieR {
        Cc5ieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc CC6IE"]
    #[inline(always)]
    pub fn cc6ie(&self) -> Cc6ieR {
        Cc6ieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc CC5DE"]
    #[inline(always)]
    pub fn cc5de(&self) -> Cc5deR {
        Cc5deR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc CC6DE"]
    #[inline(always)]
    pub fn cc6de(&self) -> Cc6deR {
        Cc6deR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IDXIE"]
    #[inline(always)]
    pub fn idxie(&self) -> IdxieR {
        IdxieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc DIRIE"]
    #[inline(always)]
    pub fn dirie(&self) -> DirieR {
        DirieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc IERRIE"]
    #[inline(always)]
    pub fn ierrie(&self) -> IerrieR {
        IerrieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc TERRIE"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, IerSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, IerSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, IerSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, IerSpec> {
        Cc3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<'_, IerSpec> {
        Cc4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - desc COMIE"]
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<'_, IerSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, IerSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc BIE"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, IerSpec> {
        BieW::new(self, 7)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, IerSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9 - desc CC1DE"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<'_, IerSpec> {
        Cc1deW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CC2DE"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> Cc2deW<'_, IerSpec> {
        Cc2deW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CC3DE"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> Cc3deW<'_, IerSpec> {
        Cc3deW::new(self, 11)
    }
    #[doc = "Bit 12 - desc CC4DE"]
    #[inline(always)]
    pub fn cc4de(&mut self) -> Cc4deW<'_, IerSpec> {
        Cc4deW::new(self, 12)
    }
    #[doc = "Bit 13 - desc COMDE"]
    #[inline(always)]
    pub fn comde(&mut self) -> ComdeW<'_, IerSpec> {
        ComdeW::new(self, 13)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, IerSpec> {
        TdeW::new(self, 14)
    }
    #[doc = "Bit 16 - desc CC5IE"]
    #[inline(always)]
    pub fn cc5ie(&mut self) -> Cc5ieW<'_, IerSpec> {
        Cc5ieW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CC6IE"]
    #[inline(always)]
    pub fn cc6ie(&mut self) -> Cc6ieW<'_, IerSpec> {
        Cc6ieW::new(self, 17)
    }
    #[doc = "Bit 18 - desc CC5DE"]
    #[inline(always)]
    pub fn cc5de(&mut self) -> Cc5deW<'_, IerSpec> {
        Cc5deW::new(self, 18)
    }
    #[doc = "Bit 19 - desc CC6DE"]
    #[inline(always)]
    pub fn cc6de(&mut self) -> Cc6deW<'_, IerSpec> {
        Cc6deW::new(self, 19)
    }
    #[doc = "Bit 20 - desc IDXIE"]
    #[inline(always)]
    pub fn idxie(&mut self) -> IdxieW<'_, IerSpec> {
        IdxieW::new(self, 20)
    }
    #[doc = "Bit 21 - desc DIRIE"]
    #[inline(always)]
    pub fn dirie(&mut self) -> DirieW<'_, IerSpec> {
        DirieW::new(self, 21)
    }
    #[doc = "Bit 22 - desc IERRIE"]
    #[inline(always)]
    pub fn ierrie(&mut self) -> IerrieW<'_, IerSpec> {
        IerrieW::new(self, 22)
    }
    #[doc = "Bit 23 - desc TERRIE"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TerrieW<'_, IerSpec> {
        TerrieW::new(self, 23)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
