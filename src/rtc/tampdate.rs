#[doc = "Register `TAMPDATE` reader"]
pub type R = crate::R<TampdateSpec>;
#[doc = "Field `DAY` reader - desc DAY"]
pub type DayR = crate::FieldReader;
#[doc = "Field `MONTH` reader - desc MONTH"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WeekR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - desc DAY"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - desc MONTH"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WeekR {
        WeekR::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "desc TAMPDATE\n\nYou can [`read`](crate::Reg::read) this register and get [`tampdate::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampdateSpec;
impl crate::RegisterSpec for TampdateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampdate::R`](R) reader structure"]
impl crate::Readable for TampdateSpec {}
#[doc = "`reset()` method sets TAMPDATE to value 0"]
impl crate::Resettable for TampdateSpec {}
