#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCDS` reader - desc CCDS"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - desc CCDS"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - desc MMS"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - desc MMS"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - desc TI1S"]
pub type Ti1sR = crate::BitReader;
#[doc = "Field `TI1S` writer - desc TI1S"]
pub type Ti1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMSH` reader - desc MMSH"]
pub type MmshR = crate::FieldReader;
#[doc = "Field `MMSH` writer - desc MMSH"]
pub type MmshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc TI1S"]
    #[inline(always)]
    pub fn ti1s(&self) -> Ti1sR {
        Ti1sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc MMSH"]
    #[inline(always)]
    pub fn mmsh(&self) -> MmshR {
        MmshR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, Cr2Spec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc MMS"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<'_, Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc TI1S"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> Ti1sW<'_, Cr2Spec> {
        Ti1sW::new(self, 7)
    }
    #[doc = "Bits 25:26 - desc MMSH"]
    #[inline(always)]
    pub fn mmsh(&mut self) -> MmshW<'_, Cr2Spec> {
        MmshW::new(self, 25)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
