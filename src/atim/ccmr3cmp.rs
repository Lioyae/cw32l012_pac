#[doc = "Register `CCMR3CMP` reader"]
pub type R = crate::R<Ccmr3cmpSpec>;
#[doc = "Register `CCMR3CMP` writer"]
pub type W = crate::W<Ccmr3cmpSpec>;
#[doc = "Field `CC5S` reader - desc CC5S"]
pub type Cc5sR = crate::FieldReader;
#[doc = "Field `CC5S` writer - desc CC5S"]
pub type Cc5sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC5FE` reader - desc OC5FE"]
pub type Oc5feR = crate::BitReader;
#[doc = "Field `OC5FE` writer - desc OC5FE"]
pub type Oc5feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - desc OC5PE"]
pub type Oc5peR = crate::BitReader;
#[doc = "Field `OC5PE` writer - desc OC5PE"]
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - desc OC5M"]
pub type Oc5mR = crate::FieldReader;
#[doc = "Field `OC5M` writer - desc OC5M"]
pub type Oc5mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC5CE` reader - desc OC5CE"]
pub type Oc5ceR = crate::BitReader;
#[doc = "Field `OC5CE` writer - desc OC5CE"]
pub type Oc5ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6S` reader - desc CC6S"]
pub type Cc6sR = crate::FieldReader;
#[doc = "Field `CC6S` writer - desc CC6S"]
pub type Cc6sW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC6FE` reader - desc OC6FE"]
pub type Oc6feR = crate::BitReader;
#[doc = "Field `OC6FE` writer - desc OC6FE"]
pub type Oc6feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6PE` reader - desc OC6PE"]
pub type Oc6peR = crate::BitReader;
#[doc = "Field `OC6PE` writer - desc OC6PE"]
pub type Oc6peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6M` reader - desc OC6M"]
pub type Oc6mR = crate::FieldReader;
#[doc = "Field `OC6M` writer - desc OC6M"]
pub type Oc6mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC6CE` reader - desc OC6CE"]
pub type Oc6ceR = crate::BitReader;
#[doc = "Field `OC6CE` writer - desc OC6CE"]
pub type Oc6ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5MH` reader - desc OC5MH"]
pub type Oc5mhR = crate::BitReader;
#[doc = "Field `OC5MH` writer - desc OC5MH"]
pub type Oc5mhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC6MH` reader - desc OC6MH"]
pub type Oc6mhR = crate::BitReader;
#[doc = "Field `OC6MH` writer - desc OC6MH"]
pub type Oc6mhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc CC5S"]
    #[inline(always)]
    pub fn cc5s(&self) -> Cc5sR {
        Cc5sR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&self) -> Oc5feR {
        Oc5feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc OC5M"]
    #[inline(always)]
    pub fn oc5m(&self) -> Oc5mR {
        Oc5mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&self) -> Oc5ceR {
        Oc5ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc CC6S"]
    #[inline(always)]
    pub fn cc6s(&self) -> Cc6sR {
        Cc6sR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&self) -> Oc6feR {
        Oc6feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&self) -> Oc6peR {
        Oc6peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - desc OC6M"]
    #[inline(always)]
    pub fn oc6m(&self) -> Oc6mR {
        Oc6mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&self) -> Oc6ceR {
        Oc6ceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OC5MH"]
    #[inline(always)]
    pub fn oc5mh(&self) -> Oc5mhR {
        Oc5mhR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - desc OC6MH"]
    #[inline(always)]
    pub fn oc6mh(&self) -> Oc6mhR {
        Oc6mhR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc CC5S"]
    #[inline(always)]
    pub fn cc5s(&mut self) -> Cc5sW<'_, Ccmr3cmpSpec> {
        Cc5sW::new(self, 0)
    }
    #[doc = "Bit 2 - desc OC5FE"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> Oc5feW<'_, Ccmr3cmpSpec> {
        Oc5feW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OC5PE"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> Oc5peW<'_, Ccmr3cmpSpec> {
        Oc5peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - desc OC5M"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> Oc5mW<'_, Ccmr3cmpSpec> {
        Oc5mW::new(self, 4)
    }
    #[doc = "Bit 7 - desc OC5CE"]
    #[inline(always)]
    pub fn oc5ce(&mut self) -> Oc5ceW<'_, Ccmr3cmpSpec> {
        Oc5ceW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc CC6S"]
    #[inline(always)]
    pub fn cc6s(&mut self) -> Cc6sW<'_, Ccmr3cmpSpec> {
        Cc6sW::new(self, 8)
    }
    #[doc = "Bit 10 - desc OC6FE"]
    #[inline(always)]
    pub fn oc6fe(&mut self) -> Oc6feW<'_, Ccmr3cmpSpec> {
        Oc6feW::new(self, 10)
    }
    #[doc = "Bit 11 - desc OC6PE"]
    #[inline(always)]
    pub fn oc6pe(&mut self) -> Oc6peW<'_, Ccmr3cmpSpec> {
        Oc6peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - desc OC6M"]
    #[inline(always)]
    pub fn oc6m(&mut self) -> Oc6mW<'_, Ccmr3cmpSpec> {
        Oc6mW::new(self, 12)
    }
    #[doc = "Bit 15 - desc OC6CE"]
    #[inline(always)]
    pub fn oc6ce(&mut self) -> Oc6ceW<'_, Ccmr3cmpSpec> {
        Oc6ceW::new(self, 15)
    }
    #[doc = "Bit 16 - desc OC5MH"]
    #[inline(always)]
    pub fn oc5mh(&mut self) -> Oc5mhW<'_, Ccmr3cmpSpec> {
        Oc5mhW::new(self, 16)
    }
    #[doc = "Bit 24 - desc OC6MH"]
    #[inline(always)]
    pub fn oc6mh(&mut self) -> Oc6mhW<'_, Ccmr3cmpSpec> {
        Oc6mhW::new(self, 24)
    }
}
#[doc = "capture compare mode register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr3cmpSpec;
impl crate::RegisterSpec for Ccmr3cmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3cmp::R`](R) reader structure"]
impl crate::Readable for Ccmr3cmpSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr3cmp::W`](W) writer structure"]
impl crate::Writable for Ccmr3cmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR3CMP to value 0"]
impl crate::Resettable for Ccmr3cmpSpec {}
