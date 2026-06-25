#[doc = "Register `MFIFOSR` reader"]
pub type R = crate::R<MfifosrSpec>;
#[doc = "Field `TXCNT` reader - desc TXCNT"]
pub type TxcntR = crate::FieldReader;
#[doc = "Field `RXCNT` reader - desc RXCNT"]
pub type RxcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - desc TXCNT"]
    #[inline(always)]
    pub fn txcnt(&self) -> TxcntR {
        TxcntR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - desc RXCNT"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RxcntR {
        RxcntR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "Master FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifosr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfifosrSpec;
impl crate::RegisterSpec for MfifosrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfifosr::R`](R) reader structure"]
impl crate::Readable for MfifosrSpec {}
#[doc = "`reset()` method sets MFIFOSR to value 0"]
impl crate::Resettable for MfifosrSpec {}
