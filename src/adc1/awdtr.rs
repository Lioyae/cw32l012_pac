#[doc = "Register `AWDTR` reader"]
pub type R = crate::R<AwdtrSpec>;
#[doc = "Register `AWDTR` writer"]
pub type W = crate::W<AwdtrSpec>;
#[doc = "Field `VTL` reader - desc VTL"]
pub type VtlR = crate::FieldReader<u16>;
#[doc = "Field `VTL` writer - desc VTL"]
pub type VtlW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `VTH` reader - desc VTH"]
pub type VthR = crate::FieldReader<u16>;
#[doc = "Field `VTH` writer - desc VTH"]
pub type VthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    pub fn vtl(&self) -> VtlR {
        VtlR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - desc VTH"]
    #[inline(always)]
    pub fn vth(&self) -> VthR {
        VthR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc VTL"]
    #[inline(always)]
    pub fn vtl(&mut self) -> VtlW<'_, AwdtrSpec> {
        VtlW::new(self, 0)
    }
    #[doc = "Bits 16:27 - desc VTH"]
    #[inline(always)]
    pub fn vth(&mut self) -> VthW<'_, AwdtrSpec> {
        VthW::new(self, 16)
    }
}
#[doc = "desc AWDTR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdtrSpec;
impl crate::RegisterSpec for AwdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awdtr::R`](R) reader structure"]
impl crate::Readable for AwdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdtr::W`](W) writer structure"]
impl crate::Writable for AwdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDTR to value 0"]
impl crate::Resettable for AwdtrSpec {}
