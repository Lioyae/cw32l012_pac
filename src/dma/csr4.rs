#[doc = "Register `CSR4` reader"]
pub type R = crate::R<Csr4Spec>;
#[doc = "Register `CSR4` writer"]
pub type W = crate::W<Csr4Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - desc TCIE"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - desc TCIE"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - desc TEIE"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - desc TEIE"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS` reader - desc TRANS"]
pub type TransR = crate::BitReader;
#[doc = "Field `TRANS` writer - desc TRANS"]
pub type TransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCINC` reader - desc SRCINC"]
pub type SrcincR = crate::BitReader;
#[doc = "Field `SRCINC` writer - desc SRCINC"]
pub type SrcincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTINC` reader - desc DSTINC"]
pub type DstincR = crate::BitReader;
#[doc = "Field `DSTINC` writer - desc DSTINC"]
pub type DstincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - desc SIZE"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - desc SIZE"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STATUS` reader - desc STATUS"]
pub type StatusR = crate::FieldReader;
#[doc = "Field `STATUS` writer - desc STATUS"]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESTART` reader - desc RESTART"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - desc RESTART"]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - desc TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `TE` reader - desc TE"]
pub type TeR = crate::BitReader;
#[doc = "Field `SRCLOAD` reader - desc SRCLOAD"]
pub type SrcloadR = crate::BitReader;
#[doc = "Field `SRCLOAD` writer - desc SRCLOAD"]
pub type SrcloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTLOAD` reader - desc DSTLOAD"]
pub type DstloadR = crate::BitReader;
#[doc = "Field `DSTLOAD` writer - desc DSTLOAD"]
pub type DstloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    pub fn trans(&self) -> TransR {
        TransR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc TE"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc SRCLOAD"]
    #[inline(always)]
    pub fn srcload(&self) -> SrcloadR {
        SrcloadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DSTLOAD"]
    #[inline(always)]
    pub fn dstload(&self) -> DstloadR {
        DstloadR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Csr4Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Csr4Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<'_, Csr4Spec> {
        TeieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    pub fn trans(&mut self) -> TransW<'_, Csr4Spec> {
        TransW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<'_, Csr4Spec> {
        SrcincW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<'_, Csr4Spec> {
        DstincW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, Csr4Spec> {
        SizeW::new(self, 6)
    }
    #[doc = "Bits 8:10 - desc STATUS"]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, Csr4Spec> {
        StatusW::new(self, 8)
    }
    #[doc = "Bit 11 - desc RESTART"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, Csr4Spec> {
        RestartW::new(self, 11)
    }
    #[doc = "Bit 14 - desc SRCLOAD"]
    #[inline(always)]
    pub fn srcload(&mut self) -> SrcloadW<'_, Csr4Spec> {
        SrcloadW::new(self, 14)
    }
    #[doc = "Bit 15 - desc DSTLOAD"]
    #[inline(always)]
    pub fn dstload(&mut self) -> DstloadW<'_, Csr4Spec> {
        DstloadW::new(self, 15)
    }
}
#[doc = "Channel1 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr4Spec;
impl crate::RegisterSpec for Csr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr4::R`](R) reader structure"]
impl crate::Readable for Csr4Spec {}
#[doc = "`write(|w| ..)` method takes [`csr4::W`](W) writer structure"]
impl crate::Writable for Csr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR4 to value 0"]
impl crate::Resettable for Csr4Spec {}
