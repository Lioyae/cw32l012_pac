#[doc = "Register `MFIFOCR` reader"]
pub type R = crate::R<MfifocrSpec>;
#[doc = "Register `MFIFOCR` writer"]
pub type W = crate::W<MfifocrSpec>;
#[doc = "Field `TXWATER` reader - desc TXWATER"]
pub type TxwaterR = crate::FieldReader;
#[doc = "Field `TXWATER` writer - desc TXWATER"]
pub type TxwaterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXWATER` reader - desc RXWATER"]
pub type RxwaterR = crate::FieldReader;
#[doc = "Field `RXWATER` writer - desc RXWATER"]
pub type RxwaterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - desc TXWATER"]
    #[inline(always)]
    pub fn txwater(&self) -> TxwaterR {
        TxwaterR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc RXWATER"]
    #[inline(always)]
    pub fn rxwater(&self) -> RxwaterR {
        RxwaterR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc TXWATER"]
    #[inline(always)]
    pub fn txwater(&mut self) -> TxwaterW<'_, MfifocrSpec> {
        TxwaterW::new(self, 0)
    }
    #[doc = "Bits 16:17 - desc RXWATER"]
    #[inline(always)]
    pub fn rxwater(&mut self) -> RxwaterW<'_, MfifocrSpec> {
        RxwaterW::new(self, 16)
    }
}
#[doc = "Master FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mfifocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfifocrSpec;
impl crate::RegisterSpec for MfifocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfifocr::R`](R) reader structure"]
impl crate::Readable for MfifocrSpec {}
#[doc = "`write(|w| ..)` method takes [`mfifocr::W`](W) writer structure"]
impl crate::Writable for MfifocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MFIFOCR to value 0"]
impl crate::Resettable for MfifocrSpec {}
