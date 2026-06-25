#[doc = "Register `SCR0` reader"]
pub type R = crate::R<Scr0Spec>;
#[doc = "Register `SCR0` writer"]
pub type W = crate::W<Scr0Spec>;
#[doc = "Field `SEN` reader - desc SEN"]
pub type SenR = crate::BitReader;
#[doc = "Field `SEN` writer - desc SEN"]
pub type SenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - desc RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTEN` reader - desc FLTEN"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - desc FLTEN"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSRC` reader - desc CLKSRC"]
pub type ClksrcR = crate::FieldReader;
#[doc = "Field `CLKSRC` writer - desc CLKSRC"]
pub type ClksrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXFIFORST` reader - desc TXFIFORST"]
pub type TxfiforstR = crate::BitReader;
#[doc = "Field `TXFIFORST` writer - desc TXFIFORST"]
pub type TxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFORST` reader - desc RXFIFORST"]
pub type RxfiforstR = crate::BitReader;
#[doc = "Field `RXFIFORST` writer - desc RXFIFORST"]
pub type RxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SEN"]
    #[inline(always)]
    pub fn sen(&self) -> SenR {
        SenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - desc TXFIFORST"]
    #[inline(always)]
    pub fn txfiforst(&self) -> TxfiforstR {
        TxfiforstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RXFIFORST"]
    #[inline(always)]
    pub fn rxfiforst(&self) -> RxfiforstR {
        RxfiforstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc SEN"]
    #[inline(always)]
    pub fn sen(&mut self) -> SenW<'_, Scr0Spec> {
        SenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, Scr0Spec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 4 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, Scr0Spec> {
        FltenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, Scr0Spec> {
        ClksrcW::new(self, 6)
    }
    #[doc = "Bit 8 - desc TXFIFORST"]
    #[inline(always)]
    pub fn txfiforst(&mut self) -> TxfiforstW<'_, Scr0Spec> {
        TxfiforstW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RXFIFORST"]
    #[inline(always)]
    pub fn rxfiforst(&mut self) -> RxfiforstW<'_, Scr0Spec> {
        RxfiforstW::new(self, 9)
    }
}
#[doc = "Slave Control Register0\n\nYou can [`read`](crate::Reg::read) this register and get [`scr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr0Spec;
impl crate::RegisterSpec for Scr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr0::R`](R) reader structure"]
impl crate::Readable for Scr0Spec {}
#[doc = "`write(|w| ..)` method takes [`scr0::W`](W) writer structure"]
impl crate::Writable for Scr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR0 to value 0"]
impl crate::Resettable for Scr0Spec {}
