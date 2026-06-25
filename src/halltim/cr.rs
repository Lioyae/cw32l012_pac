#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1EN` reader - desc FLT1EN"]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - desc FLT1EN"]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMS` reader - desc MMS"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - desc MMS"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLT2LEN` reader - desc FLT2LEN"]
pub type Flt2lenR = crate::FieldReader<u16>;
#[doc = "Field `FLT2LEN` writer - desc FLT2LEN"]
pub type Flt2lenW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SOFTCAP` reader - desc SOFTCAP"]
pub type SoftcapR = crate::BitReader;
#[doc = "Field `SOFTCAP` writer - desc SOFTCAP"]
pub type SoftcapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLT1EN"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:22 - desc FLT2LEN"]
    #[inline(always)]
    pub fn flt2len(&self) -> Flt2lenR {
        Flt2lenR::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bit 23 - desc SOFTCAP"]
    #[inline(always)]
    pub fn softcap(&self) -> SoftcapR {
        SoftcapR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc FLT1EN"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> Flt1enW<'_, CrSpec> {
        Flt1enW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, CrSpec> {
        DivW::new(self, 2)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<'_, CrSpec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bits 8:22 - desc FLT2LEN"]
    #[inline(always)]
    pub fn flt2len(&mut self) -> Flt2lenW<'_, CrSpec> {
        Flt2lenW::new(self, 8)
    }
    #[doc = "Bit 23 - desc SOFTCAP"]
    #[inline(always)]
    pub fn softcap(&mut self) -> SoftcapW<'_, CrSpec> {
        SoftcapW::new(self, 23)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
