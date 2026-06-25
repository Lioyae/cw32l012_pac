#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `AWDL` reader - desc AWDL"]
pub type AwdlR = crate::BitReader;
#[doc = "Field `AWDH` reader - desc AWDH"]
pub type AwdhR = crate::BitReader;
#[doc = "Field `AWDH` writer - desc AWDH"]
pub type AwdhW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 4 - desc AWDH"]
    #[inline(always)]
    pub fn awdh(&mut self) -> AwdhW<'_, IsrSpec> {
        AwdhW::new(self, 4)
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
