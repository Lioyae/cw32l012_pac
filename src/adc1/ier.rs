#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - desc EOC"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - desc EOS"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDL` reader - desc AWDL"]
pub type AwdlR = crate::BitReader;
#[doc = "Field `AWDL` writer - desc AWDL"]
pub type AwdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDH` reader - desc AWDH"]
pub type AwdhR = crate::BitReader;
#[doc = "Field `AWDH` writer - desc AWDH"]
pub type AwdhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEOC` reader - desc DMAEOC"]
pub type DmaeocR = crate::BitReader;
#[doc = "Field `DMAEOC` writer - desc DMAEOC"]
pub type DmaeocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEOS` reader - desc DMAEOS"]
pub type DmaeosR = crate::BitReader;
#[doc = "Field `DMAEOS` writer - desc DMAEOS"]
pub type DmaeosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc AWDL"]
    #[inline(always)]
    pub fn awdl(&self) -> AwdlR {
        AwdlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AWDH"]
    #[inline(always)]
    pub fn awdh(&self) -> AwdhR {
        AwdhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DMAEOC"]
    #[inline(always)]
    pub fn dmaeoc(&self) -> DmaeocR {
        DmaeocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DMAEOS"]
    #[inline(always)]
    pub fn dmaeos(&self) -> DmaeosR {
        DmaeosR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IerSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IerSpec> {
        EosW::new(self, 1)
    }
    #[doc = "Bit 3 - desc AWDL"]
    #[inline(always)]
    pub fn awdl(&mut self) -> AwdlW<'_, IerSpec> {
        AwdlW::new(self, 3)
    }
    #[doc = "Bit 4 - desc AWDH"]
    #[inline(always)]
    pub fn awdh(&mut self) -> AwdhW<'_, IerSpec> {
        AwdhW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DMAEOC"]
    #[inline(always)]
    pub fn dmaeoc(&mut self) -> DmaeocW<'_, IerSpec> {
        DmaeocW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DMAEOS"]
    #[inline(always)]
    pub fn dmaeos(&mut self) -> DmaeosW<'_, IerSpec> {
        DmaeosW::new(self, 6)
    }
}
#[doc = "desc IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
