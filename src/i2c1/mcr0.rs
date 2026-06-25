#[doc = "Register `MCR0` reader"]
pub type R = crate::R<Mcr0Spec>;
#[doc = "Register `MCR0` writer"]
pub type W = crate::W<Mcr0Spec>;
#[doc = "Field `MEN` reader - desc MEN"]
pub type MenR = crate::BitReader;
#[doc = "Field `MEN` writer - desc MEN"]
pub type MenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - desc RESET"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEPEN` reader - desc SLEEPEN"]
pub type SleepenR = crate::BitReader;
#[doc = "Field `SLEEPEN` writer - desc SLEEPEN"]
pub type SleepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - desc DBGEN"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - desc DBGEN"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - desc MEN"]
    #[inline(always)]
    pub fn men(&self) -> MenR {
        MenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SLEEPEN"]
    #[inline(always)]
    pub fn sleepen(&self) -> SleepenR {
        SleepenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DBGEN"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 0 - desc MEN"]
    #[inline(always)]
    pub fn men(&mut self) -> MenW<'_, Mcr0Spec> {
        MenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, Mcr0Spec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SLEEPEN"]
    #[inline(always)]
    pub fn sleepen(&mut self) -> SleepenW<'_, Mcr0Spec> {
        SleepenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc DBGEN"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, Mcr0Spec> {
        DbgenW::new(self, 3)
    }
    #[doc = "Bits 6:7 - desc CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, Mcr0Spec> {
        ClksrcW::new(self, 6)
    }
    #[doc = "Bit 8 - desc TXFIFORST"]
    #[inline(always)]
    pub fn txfiforst(&mut self) -> TxfiforstW<'_, Mcr0Spec> {
        TxfiforstW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RXFIFORST"]
    #[inline(always)]
    pub fn rxfiforst(&mut self) -> RxfiforstW<'_, Mcr0Spec> {
        RxfiforstW::new(self, 9)
    }
}
#[doc = "desc MCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr0Spec;
impl crate::RegisterSpec for Mcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr0::R`](R) reader structure"]
impl crate::Readable for Mcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr0::W`](W) writer structure"]
impl crate::Writable for Mcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR0 to value 0"]
impl crate::Resettable for Mcr0Spec {}
