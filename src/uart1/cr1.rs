#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `TXEN` reader - desc TXEN"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - desc TXEN"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - desc RXEN"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - desc RXEN"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - desc PARITY"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - desc PARITY"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYEN` reader - desc PARITYEN"]
pub type ParityenR = crate::BitReader;
#[doc = "Field `PARITYEN` writer - desc PARITYEN"]
pub type ParityenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHLEN` reader - desc CHLEN"]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - desc CHLEN"]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBF` reader - desc MSBF"]
pub type MsbfR = crate::BitReader;
#[doc = "Field `MSBF` writer - desc MSBF"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER` reader - desc OVER"]
pub type OverR = crate::FieldReader;
#[doc = "Field `OVER` writer - desc OVER"]
pub type OverW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIGNAL` reader - desc SIGNAL"]
pub type SignalR = crate::BitReader;
#[doc = "Field `SIGNAL` writer - desc SIGNAL"]
pub type SignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC` reader - desc SYNC"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - desc SYNC"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PARITYEN"]
    #[inline(always)]
    pub fn parityen(&self) -> ParityenR {
        ParityenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc CHLEN"]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MSBF"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    pub fn over(&self) -> OverR {
        OverR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - desc SIGNAL"]
    #[inline(always)]
    pub fn signal(&self) -> SignalR {
        SignalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, Cr1Spec> {
        TxenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, Cr1Spec> {
        RxenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, Cr1Spec> {
        ParityW::new(self, 2)
    }
    #[doc = "Bit 3 - desc PARITYEN"]
    #[inline(always)]
    pub fn parityen(&mut self) -> ParityenW<'_, Cr1Spec> {
        ParityenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 6 - desc CHLEN"]
    #[inline(always)]
    pub fn chlen(&mut self) -> ChlenW<'_, Cr1Spec> {
        ChlenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MSBF"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<'_, Cr1Spec> {
        MsbfW::new(self, 7)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    pub fn over(&mut self) -> OverW<'_, Cr1Spec> {
        OverW::new(self, 9)
    }
    #[doc = "Bit 11 - desc SIGNAL"]
    #[inline(always)]
    pub fn signal(&mut self) -> SignalW<'_, Cr1Spec> {
        SignalW::new(self, 11)
    }
    #[doc = "Bits 12:13 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, Cr1Spec> {
        SourceW::new(self, 12)
    }
    #[doc = "Bit 14 - desc SYNC"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, Cr1Spec> {
        SyncW::new(self, 14)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
