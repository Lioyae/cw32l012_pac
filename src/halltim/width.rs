#[doc = "Register `WIDTH` reader"]
pub type R = crate::R<WidthSpec>;
#[doc = "Field `DATA` reader - desc DATA"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - desc DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`width::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WidthSpec;
impl crate::RegisterSpec for WidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`width::R`](R) reader structure"]
impl crate::Readable for WidthSpec {}
#[doc = "`reset()` method sets WIDTH to value 0"]
impl crate::Resettable for WidthSpec {}
