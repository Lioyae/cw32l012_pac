#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INP1EN` reader - desc INP1EN"]
pub type Inp1enR = crate::BitReader;
#[doc = "Field `INP1EN` writer - desc INP1EN"]
pub type Inp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INP2EN` reader - desc INP2EN"]
pub type Inp2enR = crate::BitReader;
#[doc = "Field `INP2EN` writer - desc INP2EN"]
pub type Inp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INP3EN` reader - desc INP3EN"]
pub type Inp3enR = crate::BitReader;
#[doc = "Field `INP3EN` writer - desc INP3EN"]
pub type Inp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INP4EN` reader - desc INP4EN"]
pub type Inp4enR = crate::BitReader;
#[doc = "Field `INP4EN` writer - desc INP4EN"]
pub type Inp4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INN1EN` reader - desc INN1EN"]
pub type Inn1enR = crate::BitReader;
#[doc = "Field `INN1EN` writer - desc INN1EN"]
pub type Inn1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INN2EN` reader - desc INN2EN"]
pub type Inn2enR = crate::BitReader;
#[doc = "Field `INN2EN` writer - desc INN2EN"]
pub type Inn2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP` reader - desc AMP"]
pub type AmpR = crate::FieldReader;
#[doc = "Field `AMP` writer - desc AMP"]
pub type AmpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIAS` reader - desc BIAS"]
pub type BiasR = crate::FieldReader;
#[doc = "Field `BIAS` writer - desc BIAS"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - desc INP1EN"]
    #[inline(always)]
    pub fn inp1en(&self) -> Inp1enR {
        Inp1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc INP2EN"]
    #[inline(always)]
    pub fn inp2en(&self) -> Inp2enR {
        Inp2enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc INP3EN"]
    #[inline(always)]
    pub fn inp3en(&self) -> Inp3enR {
        Inp3enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc INP4EN"]
    #[inline(always)]
    pub fn inp4en(&self) -> Inp4enR {
        Inp4enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc INN1EN"]
    #[inline(always)]
    pub fn inn1en(&self) -> Inn1enR {
        Inn1enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc INN2EN"]
    #[inline(always)]
    pub fn inn2en(&self) -> Inn2enR {
        Inn2enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - desc AMP"]
    #[inline(always)]
    pub fn amp(&self) -> AmpR {
        AmpR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 4 - desc INP1EN"]
    #[inline(always)]
    pub fn inp1en(&mut self) -> Inp1enW<'_, CrSpec> {
        Inp1enW::new(self, 4)
    }
    #[doc = "Bit 5 - desc INP2EN"]
    #[inline(always)]
    pub fn inp2en(&mut self) -> Inp2enW<'_, CrSpec> {
        Inp2enW::new(self, 5)
    }
    #[doc = "Bit 6 - desc INP3EN"]
    #[inline(always)]
    pub fn inp3en(&mut self) -> Inp3enW<'_, CrSpec> {
        Inp3enW::new(self, 6)
    }
    #[doc = "Bit 7 - desc INP4EN"]
    #[inline(always)]
    pub fn inp4en(&mut self) -> Inp4enW<'_, CrSpec> {
        Inp4enW::new(self, 7)
    }
    #[doc = "Bit 8 - desc INN1EN"]
    #[inline(always)]
    pub fn inn1en(&mut self) -> Inn1enW<'_, CrSpec> {
        Inn1enW::new(self, 8)
    }
    #[doc = "Bit 9 - desc INN2EN"]
    #[inline(always)]
    pub fn inn2en(&mut self) -> Inn2enW<'_, CrSpec> {
        Inn2enW::new(self, 9)
    }
    #[doc = "Bits 10:12 - desc AMP"]
    #[inline(always)]
    pub fn amp(&mut self) -> AmpW<'_, CrSpec> {
        AmpW::new(self, 10)
    }
    #[doc = "Bits 13:15 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<'_, CrSpec> {
        BiasW::new(self, 13)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
