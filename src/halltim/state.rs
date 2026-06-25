#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `CH1F` reader - desc CH1F"]
pub type Ch1fR = crate::BitReader;
#[doc = "Field `CH2F` reader - desc CH2F"]
pub type Ch2fR = crate::BitReader;
#[doc = "Field `CH3F` reader - desc CH3F"]
pub type Ch3fR = crate::BitReader;
#[doc = "Field `CH1S` reader - desc CH1S"]
pub type Ch1sR = crate::BitReader;
#[doc = "Field `CH2S` reader - desc CH2S"]
pub type Ch2sR = crate::BitReader;
#[doc = "Field `CH3S` reader - desc CH3S"]
pub type Ch3sR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> Ch1fR {
        Ch1fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> Ch2fR {
        Ch2fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> Ch3fR {
        Ch3fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CH1S"]
    #[inline(always)]
    pub fn ch1s(&self) -> Ch1sR {
        Ch1sR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CH2S"]
    #[inline(always)]
    pub fn ch2s(&self) -> Ch2sR {
        Ch2sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CH3S"]
    #[inline(always)]
    pub fn ch3s(&self) -> Ch3sR {
        Ch3sR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "State register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
