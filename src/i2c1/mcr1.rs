#[doc = "Register `MCR1` reader"]
pub type R = crate::R<Mcr1Spec>;
#[doc = "Register `MCR1` writer"]
pub type W = crate::W<Mcr1Spec>;
#[doc = "Field `CIRFIFO` reader - desc CIRFIFO"]
pub type CirfifoR = crate::BitReader;
#[doc = "Field `CIRFIFO` writer - desc CIRFIFO"]
pub type CirfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMO` reader - desc RDMO"]
pub type RdmoR = crate::BitReader;
#[doc = "Field `RDMO` writer - desc RDMO"]
pub type RdmoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - desc CIRFIFO"]
    #[inline(always)]
    pub fn cirfifo(&self) -> CirfifoR {
        CirfifoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc RDMO"]
    #[inline(always)]
    pub fn rdmo(&self) -> RdmoR {
        RdmoR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - desc CIRFIFO"]
    #[inline(always)]
    pub fn cirfifo(&mut self) -> CirfifoW<'_, Mcr1Spec> {
        CirfifoW::new(self, 8)
    }
    #[doc = "Bit 9 - desc RDMO"]
    #[inline(always)]
    pub fn rdmo(&mut self) -> RdmoW<'_, Mcr1Spec> {
        RdmoW::new(self, 9)
    }
}
#[doc = "Master Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr1Spec;
impl crate::RegisterSpec for Mcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr1::R`](R) reader structure"]
impl crate::Readable for Mcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr1::W`](W) writer structure"]
impl crate::Writable for Mcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR1 to value 0"]
impl crate::Resettable for Mcr1Spec {}
