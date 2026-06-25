#[doc = "Register `MDER` reader"]
pub type R = crate::R<MderSpec>;
#[doc = "Register `MDER` writer"]
pub type W = crate::W<MderSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, MderSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, MderSpec> {
        RxneW::new(self, 1)
    }
}
#[doc = "Master DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MderSpec;
impl crate::RegisterSpec for MderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mder::R`](R) reader structure"]
impl crate::Readable for MderSpec {}
#[doc = "`write(|w| ..)` method takes [`mder::W`](W) writer structure"]
impl crate::Writable for MderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDER to value 0"]
impl crate::Resettable for MderSpec {}
