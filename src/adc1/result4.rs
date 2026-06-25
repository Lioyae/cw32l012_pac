#[doc = "Register `RESULT4` reader"]
pub type R = crate::R<Result4Spec>;
#[doc = "Field `RESULT` reader - desc RESULT"]
pub type ResultR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - desc RESULT"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "desc RESULT4\n\nYou can [`read`](crate::Reg::read) this register and get [`result4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Result4Spec;
impl crate::RegisterSpec for Result4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result4::R`](R) reader structure"]
impl crate::Readable for Result4Spec {}
#[doc = "`reset()` method sets RESULT4 to value 0"]
impl crate::Resettable for Result4Spec {}
