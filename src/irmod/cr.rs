#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `MOD` reader - desc MOD"]
pub type ModR = crate::FieldReader;
#[doc = "Field `MOD` writer - desc MOD"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IRSW` reader - desc IRSW"]
pub type IrswR = crate::BitReader;
#[doc = "Field `IRSW` writer - desc IRSW"]
pub type IrswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - desc INV"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - desc INV"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - desc IRSW"]
    #[inline(always)]
    pub fn irsw(&self) -> IrswR {
        IrswR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc INV"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MOD"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, CrSpec> {
        ModW::new(self, 0)
    }
    #[doc = "Bit 4 - desc IRSW"]
    #[inline(always)]
    pub fn irsw(&mut self) -> IrswW<'_, CrSpec> {
        IrswW::new(self, 4)
    }
    #[doc = "Bit 5 - desc INV"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, CrSpec> {
        InvW::new(self, 5)
    }
}
#[doc = "Input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
