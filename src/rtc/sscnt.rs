#[doc = "Register `SSCNT` reader"]
pub type R = crate::R<SscntSpec>;
#[doc = "Field `SSCNT0` reader - desc SSCNT0"]
pub type Sscnt0R = crate::FieldReader<u32>;
#[doc = "Field `SSCNT1` reader - desc SSCNT1"]
pub type Sscnt1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - desc SSCNT0"]
    #[inline(always)]
    pub fn sscnt0(&self) -> Sscnt0R {
        Sscnt0R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - desc SSCNT1"]
    #[inline(always)]
    pub fn sscnt1(&self) -> Sscnt1R {
        Sscnt1R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "desc SSCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`sscnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SscntSpec;
impl crate::RegisterSpec for SscntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sscnt::R`](R) reader structure"]
impl crate::Readable for SscntSpec {}
#[doc = "`reset()` method sets SSCNT to value 0"]
impl crate::Resettable for SscntSpec {}
