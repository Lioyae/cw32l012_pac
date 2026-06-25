#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIR` reader - desc IDIR"]
pub type IdirR = crate::FieldReader;
#[doc = "Field `IDIR` writer - desc IDIR"]
pub type IdirW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIDX` reader - desc FIDX"]
pub type FidxR = crate::BitReader;
#[doc = "Field `FIDX` writer - desc FIDX"]
pub type FidxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPOS` reader - desc IPOS"]
pub type IposR = crate::FieldReader;
#[doc = "Field `IPOS` writer - desc IPOS"]
pub type IposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc IDIR"]
    #[inline(always)]
    pub fn idir(&self) -> IdirR {
        IdirR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 5 - desc FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc IPOS"]
    #[inline(always)]
    pub fn ipos(&self) -> IposR {
        IposR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, EcrSpec> {
        IeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc IDIR"]
    #[inline(always)]
    pub fn idir(&mut self) -> IdirW<'_, EcrSpec> {
        IdirW::new(self, 1)
    }
    #[doc = "Bit 5 - desc FIDX"]
    #[inline(always)]
    pub fn fidx(&mut self) -> FidxW<'_, EcrSpec> {
        FidxW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc IPOS"]
    #[inline(always)]
    pub fn ipos(&mut self) -> IposW<'_, EcrSpec> {
        IposW::new(self, 6)
    }
}
#[doc = "Encoder control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {}
