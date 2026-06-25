#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - desc CONT"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - desc CONT"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK` reader - desc CLK"]
pub type ClkR = crate::FieldReader;
#[doc = "Field `CLK` writer - desc CLK"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type EnsR = crate::FieldReader;
#[doc = "Field `ENS` writer - desc ENS"]
pub type EnsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLAVE` reader - desc SLAVE"]
pub type SlaveR = crate::BitReader;
#[doc = "Field `SLAVE` writer - desc SLAVE"]
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAM` reader - desc SAM"]
pub type SamR = crate::FieldReader;
#[doc = "Field `SAM` writer - desc SAM"]
pub type SamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CONT"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc CLK"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc SLAVE"]
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc SAM"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, CrSpec> {
        ContW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc CLK"]
    #[inline(always)]
    pub fn clk(&mut self) -> ClkW<'_, CrSpec> {
        ClkW::new(self, 2)
    }
    #[doc = "Bits 4:6 - desc ENS"]
    #[inline(always)]
    pub fn ens(&mut self) -> EnsW<'_, CrSpec> {
        EnsW::new(self, 4)
    }
    #[doc = "Bit 7 - desc SLAVE"]
    #[inline(always)]
    pub fn slave(&mut self) -> SlaveW<'_, CrSpec> {
        SlaveW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc SAM"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<'_, CrSpec> {
        SamW::new(self, 8)
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
