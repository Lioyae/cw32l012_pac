#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `CMPM` reader - desc CMPM"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `ARRM` reader - desc ARRM"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - desc EXTTRIG"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `CMPOK` reader - desc CMPOK"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `ARROK` reader - desc ARROK"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `UP` reader - desc UP"]
pub type UpR = crate::BitReader;
#[doc = "Field `DOWN` reader - desc DOWN"]
pub type DownR = crate::BitReader;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc CMPM"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ARRM"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPOK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ARROK"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc UP"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DOWN"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 7) & 3) as u8)
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
