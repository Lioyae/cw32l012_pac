#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - desc UIE"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - desc UIE"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - desc TIE"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - desc TIE"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - desc UDE"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - desc UDE"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - desc TDE"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - desc TDE"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 6 - desc TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 8 - desc UDE"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 14 - desc TDE"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, DierSpec> {
        TdeW::new(self, 14)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}
