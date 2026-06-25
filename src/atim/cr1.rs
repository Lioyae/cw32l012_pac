#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CEN` reader - desc CEN"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - desc CEN"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - desc UDIS"]
pub type UdisR = crate::BitReader;
#[doc = "Field `UDIS` writer - desc UDIS"]
pub type UdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - desc URS"]
pub type UrsR = crate::BitReader;
#[doc = "Field `URS` writer - desc URS"]
pub type UrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - desc OPM"]
pub type OpmR = crate::BitReader;
#[doc = "Field `OPM` writer - desc OPM"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMS` reader - desc CMS"]
pub type CmsR = crate::FieldReader;
#[doc = "Field `CMS` writer - desc CMS"]
pub type CmsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPE` reader - desc ARPE"]
pub type ArpeR = crate::BitReader;
#[doc = "Field `ARPE` writer - desc ARPE"]
pub type ArpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKD` reader - desc CKD"]
pub type CkdR = crate::FieldReader;
#[doc = "Field `CKD` writer - desc CKD"]
pub type CkdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UIFREMAP` reader - desc UIFREMAP"]
pub type UifremapR = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - desc UIFREMAP"]
pub type UifremapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - desc OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc CMS"]
    #[inline(always)]
    pub fn cms(&self) -> CmsR {
        CmsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - desc ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ArpeR {
        ArpeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc CKD"]
    #[inline(always)]
    pub fn ckd(&self) -> CkdR {
        CkdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - desc UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UifremapR {
        UifremapR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, Cr1Spec> {
        CenW::new(self, 0)
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
    #[doc = "Bit 3 - desc OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<'_, Cr1Spec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bit 4 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Cr1Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - desc CMS"]
    #[inline(always)]
    pub fn cms(&mut self) -> CmsW<'_, Cr1Spec> {
        CmsW::new(self, 5)
    }
    #[doc = "Bit 7 - desc ARPE"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ArpeW<'_, Cr1Spec> {
        ArpeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc CKD"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CkdW<'_, Cr1Spec> {
        CkdW::new(self, 8)
    }
    #[doc = "Bit 11 - desc UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UifremapW<'_, Cr1Spec> {
        UifremapW::new(self, 11)
    }
}
#[doc = "Control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
