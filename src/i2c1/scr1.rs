#[doc = "Register `SCR1` reader"]
pub type R = crate::R<Scr1Spec>;
#[doc = "Register `SCR1` writer"]
pub type W = crate::W<Scr1Spec>;
#[doc = "Field `ADRSTALL` reader - desc ADRSTALL"]
pub type AdrstallR = crate::BitReader;
#[doc = "Field `ADRSTALL` writer - desc ADRSTALL"]
pub type AdrstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTALL` reader - desc RXSTALL"]
pub type RxstallR = crate::BitReader;
#[doc = "Field `RXSTALL` writer - desc RXSTALL"]
pub type RxstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTALL` reader - desc TXSTALL"]
pub type TxstallR = crate::BitReader;
#[doc = "Field `TXSTALL` writer - desc TXSTALL"]
pub type TxstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKSTALL` reader - desc ACKSTALL"]
pub type AckstallR = crate::BitReader;
#[doc = "Field `ACKSTALL` writer - desc ACKSTALL"]
pub type AckstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN` reader - desc GCEN"]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - desc GCEN"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTEN` reader - desc ALERTEN"]
pub type AlertenR = crate::BitReader;
#[doc = "Field `ALERTEN` writer - desc ALERTEN"]
pub type AlertenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCFG` reader - desc TXCFG"]
pub type TxcfgR = crate::BitReader;
#[doc = "Field `TXCFG` writer - desc TXCFG"]
pub type TxcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXCFG` reader - desc RXCFG"]
pub type RxcfgR = crate::BitReader;
#[doc = "Field `RXCFG` writer - desc RXCFG"]
pub type RxcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNACK` reader - desc IGNACK"]
pub type IgnackR = crate::BitReader;
#[doc = "Field `IGNACK` writer - desc IGNACK"]
pub type IgnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::BitReader;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRCFG` reader - desc ADDRCFG"]
pub type AddrcfgR = crate::FieldReader;
#[doc = "Field `ADDRCFG` writer - desc ADDRCFG"]
pub type AddrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc ADRSTALL"]
    #[inline(always)]
    pub fn adrstall(&self) -> AdrstallR {
        AdrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXSTALL"]
    #[inline(always)]
    pub fn rxstall(&self) -> RxstallR {
        RxstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TXSTALL"]
    #[inline(always)]
    pub fn txstall(&self) -> TxstallR {
        TxstallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ACKSTALL"]
    #[inline(always)]
    pub fn ackstall(&self) -> AckstallR {
        AckstallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - desc GCEN"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ALERTEN"]
    #[inline(always)]
    pub fn alerten(&self) -> AlertenR {
        AlertenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc TXCFG"]
    #[inline(always)]
    pub fn txcfg(&self) -> TxcfgR {
        TxcfgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc RXCFG"]
    #[inline(always)]
    pub fn rxcfg(&self) -> RxcfgR {
        RxcfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IGNACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IgnackR {
        IgnackR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - desc ADDRCFG"]
    #[inline(always)]
    pub fn addrcfg(&self) -> AddrcfgR {
        AddrcfgR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADRSTALL"]
    #[inline(always)]
    pub fn adrstall(&mut self) -> AdrstallW<'_, Scr1Spec> {
        AdrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXSTALL"]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RxstallW<'_, Scr1Spec> {
        RxstallW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TXSTALL"]
    #[inline(always)]
    pub fn txstall(&mut self) -> TxstallW<'_, Scr1Spec> {
        TxstallW::new(self, 2)
    }
    #[doc = "Bit 3 - desc ACKSTALL"]
    #[inline(always)]
    pub fn ackstall(&mut self) -> AckstallW<'_, Scr1Spec> {
        AckstallW::new(self, 3)
    }
    #[doc = "Bit 8 - desc GCEN"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GcenW<'_, Scr1Spec> {
        GcenW::new(self, 8)
    }
    #[doc = "Bit 9 - desc ALERTEN"]
    #[inline(always)]
    pub fn alerten(&mut self) -> AlertenW<'_, Scr1Spec> {
        AlertenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc TXCFG"]
    #[inline(always)]
    pub fn txcfg(&mut self) -> TxcfgW<'_, Scr1Spec> {
        TxcfgW::new(self, 10)
    }
    #[doc = "Bit 11 - desc RXCFG"]
    #[inline(always)]
    pub fn rxcfg(&mut self) -> RxcfgW<'_, Scr1Spec> {
        RxcfgW::new(self, 11)
    }
    #[doc = "Bit 12 - desc IGNACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IgnackW<'_, Scr1Spec> {
        IgnackW::new(self, 12)
    }
    #[doc = "Bit 15 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Scr1Spec> {
        TrgselW::new(self, 15)
    }
    #[doc = "Bits 16:18 - desc ADDRCFG"]
    #[inline(always)]
    pub fn addrcfg(&mut self) -> AddrcfgW<'_, Scr1Spec> {
        AddrcfgW::new(self, 16)
    }
}
#[doc = "desc SCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`scr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scr1Spec;
impl crate::RegisterSpec for Scr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr1::R`](R) reader structure"]
impl crate::Readable for Scr1Spec {}
#[doc = "`write(|w| ..)` method takes [`scr1::W`](W) writer structure"]
impl crate::Writable for Scr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR1 to value 0"]
impl crate::Resettable for Scr1Spec {}
