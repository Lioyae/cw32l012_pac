#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADDREN` reader - desc ADDREN"]
pub type AddrenR = crate::BitReader;
#[doc = "Field `ADDREN` writer - desc ADDREN"]
pub type AddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMATCHEN` reader - desc RXMATCHEN"]
pub type RxmatchenR = crate::BitReader;
#[doc = "Field `RXMATCHEN` writer - desc RXMATCHEN"]
pub type RxmatchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - desc CTSEN"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - desc CTSEN"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - desc RTSEN"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - desc RTSEN"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - desc RXINV"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - desc RXINV"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - desc TXINV"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - desc TXINV"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARX` reader - desc DMARX"]
pub type DmarxR = crate::BitReader;
#[doc = "Field `DMARX` writer - desc DMARX"]
pub type DmarxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATX` reader - desc DMATX"]
pub type DmatxR = crate::BitReader;
#[doc = "Field `DMATX` writer - desc DMATX"]
pub type DmatxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMCR` reader - desc TIMCR"]
pub type TimcrR = crate::FieldReader;
#[doc = "Field `TIMCR` writer - desc TIMCR"]
pub type TimcrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWAP` reader - desc SWAP"]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - desc SWAP"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRX` reader - desc ADCRX"]
pub type AdcrxR = crate::BitReader;
#[doc = "Field `ADCRX` writer - desc ADCRX"]
pub type AdcrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTX` reader - desc ADCTX"]
pub type AdctxR = crate::BitReader;
#[doc = "Field `ADCTX` writer - desc ADCTX"]
pub type AdctxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP` reader - desc LOOP"]
pub type LoopR = crate::BitReader;
#[doc = "Field `LOOP` writer - desc LOOP"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSRC` reader - desc RXSRC"]
pub type RxsrcR = crate::FieldReader;
#[doc = "Field `RXSRC` writer - desc RXSRC"]
pub type RxsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    pub fn addren(&self) -> AddrenR {
        AddrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXMATCHEN"]
    #[inline(always)]
    pub fn rxmatchen(&self) -> RxmatchenR {
        RxmatchenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&self) -> DmarxR {
        DmarxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&self) -> DmatxR {
        DmatxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc TIMCR"]
    #[inline(always)]
    pub fn timcr(&self) -> TimcrR {
        TimcrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc SWAP"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc ADCRX"]
    #[inline(always)]
    pub fn adcrx(&self) -> AdcrxR {
        AdcrxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ADCTX"]
    #[inline(always)]
    pub fn adctx(&self) -> AdctxR {
        AdctxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LOOP"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - desc RXSRC"]
    #[inline(always)]
    pub fn rxsrc(&self) -> RxsrcR {
        RxsrcR::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    pub fn addren(&mut self) -> AddrenW<'_, Cr2Spec> {
        AddrenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXMATCHEN"]
    #[inline(always)]
    pub fn rxmatchen(&mut self) -> RxmatchenW<'_, Cr2Spec> {
        RxmatchenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Cr2Spec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, Cr2Spec> {
        RtsenW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, Cr2Spec> {
        RxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, Cr2Spec> {
        TxinvW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&mut self) -> DmarxW<'_, Cr2Spec> {
        DmarxW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&mut self) -> DmatxW<'_, Cr2Spec> {
        DmatxW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc TIMCR"]
    #[inline(always)]
    pub fn timcr(&mut self) -> TimcrW<'_, Cr2Spec> {
        TimcrW::new(self, 8)
    }
    #[doc = "Bit 11 - desc SWAP"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr2Spec> {
        SwapW::new(self, 11)
    }
    #[doc = "Bit 12 - desc ADCRX"]
    #[inline(always)]
    pub fn adcrx(&mut self) -> AdcrxW<'_, Cr2Spec> {
        AdcrxW::new(self, 12)
    }
    #[doc = "Bit 13 - desc ADCTX"]
    #[inline(always)]
    pub fn adctx(&mut self) -> AdctxW<'_, Cr2Spec> {
        AdctxW::new(self, 13)
    }
    #[doc = "Bit 14 - desc LOOP"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<'_, Cr2Spec> {
        LoopW::new(self, 14)
    }
    #[doc = "Bits 15:17 - desc RXSRC"]
    #[inline(always)]
    pub fn rxsrc(&mut self) -> RxsrcW<'_, Cr2Spec> {
        RxsrcW::new(self, 15)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
