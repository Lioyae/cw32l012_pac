#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - desc UDIS"]
pub type UdisR = crate::BitReader;
#[doc = "Field `UDIS` writer - desc UDIS"]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - desc URS"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - desc URS"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - desc ONESHOT"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - desc ONESHOT"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UIFREMAP` reader - desc UIFREMAP"]
pub type UifremapR = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - desc UIFREMAP"]
pub type UifremapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOGEN` reader - desc TOGEN"]
pub type TogenR = crate::BitReader;
#[doc = "Field `TOGEN` writer - desc TOGEN"]
pub type TogenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UdisR {
        UdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc URS"]
    #[inline(always)]
    pub fn urs(&self) -> UrsR {
        UrsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - desc UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UifremapR {
        UifremapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&self) -> TogenR {
        TogenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc UDIS"]
    #[inline(always)]
    pub fn udis(&mut self) -> UdisW<'_, Cr1Spec> {
        UdisW::new(self, 1)
    }
    #[doc = "Bit 2 - desc URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> UrsW<'_, Cr1Spec> {
        UrsW::new(self, 2)
    }
    #[doc = "Bit 3 - desc ONESHOT"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, Cr1Spec> {
        OneshotW::new(self, 3)
    }
    #[doc = "Bit 11 - desc UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UifremapW<'_, Cr1Spec> {
        UifremapW::new(self, 11)
    }
    #[doc = "Bit 15 - desc TOGEN"]
    #[inline(always)]
    pub fn togen(&mut self) -> TogenW<'_, Cr1Spec> {
        TogenW::new(self, 15)
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
