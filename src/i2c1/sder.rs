#[doc = "Register `SDER` reader"]
pub type R = crate::R<SderSpec>;
#[doc = "Register `SDER` writer"]
pub type W = crate::W<SderSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - desc ADDR"]
pub type AddrR = crate::BitReader;
#[doc = "Field `ADDR` writer - desc ADDR"]
pub type AddrW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, SderSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, SderSpec> {
        RxneW::new(self, 1)
    }
    #[doc = "Bit 2 - desc ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SderSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "desc SDER\n\nYou can [`read`](crate::Reg::read) this register and get [`sder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SderSpec;
impl crate::RegisterSpec for SderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sder::R`](R) reader structure"]
impl crate::Readable for SderSpec {}
#[doc = "`write(|w| ..)` method takes [`sder::W`](W) writer structure"]
impl crate::Writable for SderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDER to value 0"]
impl crate::Resettable for SderSpec {}
