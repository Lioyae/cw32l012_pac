#[doc = "Register `STAR` reader"]
pub type R = crate::R<StarSpec>;
#[doc = "Register `STAR` writer"]
pub type W = crate::W<StarSpec>;
#[doc = "Field `TXNACK` reader - desc TXNACK"]
pub type TxnackR = crate::BitReader;
#[doc = "Field `TXNACK` writer - desc TXNACK"]
pub type TxnackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXNACK"]
    #[inline(always)]
    pub fn txnack(&self) -> TxnackR {
        TxnackR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXNACK"]
    #[inline(always)]
    pub fn txnack(&mut self) -> TxnackW<'_, StarSpec> {
        TxnackW::new(self, 0)
    }
}
#[doc = "desc STAR\n\nYou can [`read`](crate::Reg::read) this register and get [`star::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`star::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StarSpec;
impl crate::RegisterSpec for StarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`star::R`](R) reader structure"]
impl crate::Readable for StarSpec {}
#[doc = "`write(|w| ..)` method takes [`star::W`](W) writer structure"]
impl crate::Writable for StarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAR to value 0"]
impl crate::Resettable for StarSpec {}
