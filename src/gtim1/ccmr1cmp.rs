#[doc = "Register `CCMR1CMP` reader"]
pub type R = crate::R<Ccmr1cmpSpec>;
#[doc = "Register `CCMR1CMP` writer"]
pub type W = crate::W<Ccmr1cmpSpec>;
#[doc = "Field `CC1S` reader - desc CC1S"]
pub type Cc1sR = crate::FieldReader;
#[doc = "Field `CC1S` writer - desc CC1S"]
pub type Cc1sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - desc OC1FE"]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - desc OC1FE"]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - desc OC1PE"]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - desc OC1PE"]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - desc OC1M"]
pub type Oc1mR = crate::FieldReader;
#[doc = "Field `OC1M` writer - desc OC1M"]
pub type Oc1mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - desc OC1CE"]
pub type Oc1ceR = crate::BitReader;
#[doc = "Field `OC1CE` writer - desc OC1CE"]
pub type Oc1ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2S` reader - desc CC2S"]
pub type Cc2sR = crate::FieldReader;
#[doc = "Field `CC2S` writer - desc CC2S"]
pub type Cc2sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC2FE` reader - desc OC2FE"]
pub type Oc2feR = crate::BitReader;
#[doc = "Field `OC2FE` writer - desc OC2FE"]
pub type Oc2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - desc OC2PE"]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - desc OC2PE"]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M` reader - desc OC2M"]
pub type Oc2mR = crate::FieldReader;
#[doc = "Field `OC2M` writer - desc OC2M"]
pub type Oc2mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - desc OC2CE"]
pub type Oc2ceR = crate::BitReader;
#[doc = "Field `OC2CE` writer - desc OC2CE"]
pub type Oc2ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1MH` reader - desc OC1MH"]
pub type Oc1mhR = crate::BitReader;
#[doc = "Field `OC1MH` writer - desc OC1MH"]
pub type Oc1mhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2MH` reader - desc OC2MH"]
pub type Oc2mhR = crate::BitReader;
#[doc = "Field `OC2MH` writer - desc OC2MH"]
pub type Oc2mhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc CC1S"]
    #[inline(always)]
    pub fn cc1s(&self) -> Cc1sR {
        Cc1sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc OC1FE"]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OC1PE"]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OC1M"]
    #[inline(always)]
    pub fn oc1m(&self) -> Oc1mR {
        Oc1mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc OC1CE"]
    #[inline(always)]
    pub fn oc1ce(&self) -> Oc1ceR {
        Oc1ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc CC2S"]
    #[inline(always)]
    pub fn cc2s(&self) -> Cc2sR {
        Cc2sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc OC2FE"]
    #[inline(always)]
    pub fn oc2fe(&self) -> Oc2feR {
        Oc2feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OC2PE"]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc OC2M"]
    #[inline(always)]
    pub fn oc2m(&self) -> Oc2mR {
        Oc2mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc OC2CE"]
    #[inline(always)]
    pub fn oc2ce(&self) -> Oc2ceR {
        Oc2ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OC1MH"]
    #[inline(always)]
    pub fn oc1mh(&self) -> Oc1mhR {
        Oc1mhR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - desc OC2MH"]
    #[inline(always)]
    pub fn oc2mh(&self) -> Oc2mhR {
        Oc2mhR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC1S"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> Cc1sW<'_, Ccmr1cmpSpec> {
        Cc1sW::new(self, 0)
    }
    #[doc = "Bit 2 - desc OC1FE"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<'_, Ccmr1cmpSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OC1PE"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<'_, Ccmr1cmpSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc OC1M"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> Oc1mW<'_, Ccmr1cmpSpec> {
        Oc1mW::new(self, 4)
    }
    #[doc = "Bit 7 - desc OC1CE"]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> Oc1ceW<'_, Ccmr1cmpSpec> {
        Oc1ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc CC2S"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> Cc2sW<'_, Ccmr1cmpSpec> {
        Cc2sW::new(self, 8)
    }
    #[doc = "Bit 10 - desc OC2FE"]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> Oc2feW<'_, Ccmr1cmpSpec> {
        Oc2feW::new(self, 10)
    }
    #[doc = "Bit 11 - desc OC2PE"]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> Oc2peW<'_, Ccmr1cmpSpec> {
        Oc2peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc OC2M"]
    #[inline(always)]
    pub fn oc2m(&mut self) -> Oc2mW<'_, Ccmr1cmpSpec> {
        Oc2mW::new(self, 12)
    }
    #[doc = "Bit 15 - desc OC2CE"]
    #[inline(always)]
    pub fn oc2ce(&mut self) -> Oc2ceW<'_, Ccmr1cmpSpec> {
        Oc2ceW::new(self, 15)
    }
    #[doc = "Bit 16 - desc OC1MH"]
    #[inline(always)]
    pub fn oc1mh(&mut self) -> Oc1mhW<'_, Ccmr1cmpSpec> {
        Oc1mhW::new(self, 16)
    }
    #[doc = "Bit 24 - desc OC2MH"]
    #[inline(always)]
    pub fn oc2mh(&mut self) -> Oc2mhW<'_, Ccmr1cmpSpec> {
        Oc2mhW::new(self, 24)
    }
}
#[doc = "Compare control Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1cmpSpec;
impl crate::RegisterSpec for Ccmr1cmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1cmp::R`](R) reader structure"]
impl crate::Readable for Ccmr1cmpSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1cmp::W`](W) writer structure"]
impl crate::Writable for Ccmr1cmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1CMP to value 0"]
impl crate::Resettable for Ccmr1cmpSpec {}
