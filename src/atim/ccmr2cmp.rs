#[doc = "Register `CCMR2CMP` reader"]
pub type R = crate::R<Ccmr2cmpSpec>;
#[doc = "Register `CCMR2CMP` writer"]
pub type W = crate::W<Ccmr2cmpSpec>;
#[doc = "Field `CC3S` reader - desc CC3S"]
pub type Cc3sR = crate::FieldReader;
#[doc = "Field `CC3S` writer - desc CC3S"]
pub type Cc3sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - desc OC3FE"]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - desc OC3FE"]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - desc OC3PE"]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - desc OC3PE"]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M` reader - desc OC3M"]
pub type Oc3mR = crate::FieldReader;
#[doc = "Field `OC3M` writer - desc OC3M"]
pub type Oc3mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - desc OC3CE"]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - desc OC3CE"]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - desc CC4S"]
pub type Cc4sR = crate::FieldReader;
#[doc = "Field `CC4S` writer - desc CC4S"]
pub type Cc4sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - desc OC4FE"]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - desc OC4FE"]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - desc OC4PE"]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - desc OC4PE"]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - desc OC4M"]
pub type Oc4mR = crate::FieldReader;
#[doc = "Field `OC4M` writer - desc OC4M"]
pub type Oc4mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - desc OC4CE"]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - desc OC4CE"]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3MH` reader - desc OC3MH"]
pub type Oc3mhR = crate::BitReader;
#[doc = "Field `OC3MH` writer - desc OC3MH"]
pub type Oc3mhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4MH` reader - desc OC4MH"]
pub type Oc4mhR = crate::BitReader;
#[doc = "Field `OC4MH` writer - desc OC4MH"]
pub type Oc4mhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> Cc3sR {
        Cc3sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OC3M"]
    #[inline(always)]
    pub fn oc3m(&self) -> Oc3mR {
        Oc3mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> Cc4sR {
        Cc4sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc OC4M"]
    #[inline(always)]
    pub fn oc4m(&self) -> Oc4mR {
        Oc4mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc OC4CE"]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OC3MH"]
    #[inline(always)]
    pub fn oc3mh(&self) -> Oc3mhR {
        Oc3mhR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - desc OC4MH"]
    #[inline(always)]
    pub fn oc4mh(&self) -> Oc4mhR {
        Oc4mhR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC3S"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> Cc3sW<'_, Ccmr2cmpSpec> {
        Cc3sW::new(self, 0)
    }
    #[doc = "Bit 2 - desc OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<'_, Ccmr2cmpSpec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<'_, Ccmr2cmpSpec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc OC3M"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> Oc3mW<'_, Ccmr2cmpSpec> {
        Oc3mW::new(self, 4)
    }
    #[doc = "Bit 7 - desc OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<'_, Ccmr2cmpSpec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc CC4S"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> Cc4sW<'_, Ccmr2cmpSpec> {
        Cc4sW::new(self, 8)
    }
    #[doc = "Bit 10 - desc OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> Oc4feW<'_, Ccmr2cmpSpec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11 - desc OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<'_, Ccmr2cmpSpec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc OC4M"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> Oc4mW<'_, Ccmr2cmpSpec> {
        Oc4mW::new(self, 12)
    }
    #[doc = "Bit 15 - desc OC4CE"]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<'_, Ccmr2cmpSpec> {
        Oc4ceW::new(self, 15)
    }
    #[doc = "Bit 16 - desc OC3MH"]
    #[inline(always)]
    pub fn oc3mh(&mut self) -> Oc3mhW<'_, Ccmr2cmpSpec> {
        Oc3mhW::new(self, 16)
    }
    #[doc = "Bit 24 - desc OC4MH"]
    #[inline(always)]
    pub fn oc4mh(&mut self) -> Oc4mhW<'_, Ccmr2cmpSpec> {
        Oc4mhW::new(self, 24)
    }
}
#[doc = "capture compare mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2cmpSpec;
impl crate::RegisterSpec for Ccmr2cmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2cmp::R`](R) reader structure"]
impl crate::Readable for Ccmr2cmpSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2cmp::W`](W) writer structure"]
impl crate::Writable for Ccmr2cmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2CMP to value 0"]
impl crate::Resettable for Ccmr2cmpSpec {}
