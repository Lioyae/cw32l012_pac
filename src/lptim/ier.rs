#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `CMPM` reader - desc CMPM"]
pub type CmpmR = crate::BitReader;
#[doc = "Field `CMPM` writer - desc CMPM"]
pub type CmpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRM` reader - desc ARRM"]
pub type ArrmR = crate::BitReader;
#[doc = "Field `ARRM` writer - desc ARRM"]
pub type ArrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIG` reader - desc EXTTRIG"]
pub type ExttrigR = crate::BitReader;
#[doc = "Field `EXTTRIG` writer - desc EXTTRIG"]
pub type ExttrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOK` reader - desc CMPOK"]
pub type CmpokR = crate::BitReader;
#[doc = "Field `CMPOK` writer - desc CMPOK"]
pub type CmpokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROK` reader - desc ARROK"]
pub type ArrokR = crate::BitReader;
#[doc = "Field `ARROK` writer - desc ARROK"]
pub type ArrokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - desc UP"]
pub type UpR = crate::BitReader;
#[doc = "Field `UP` writer - desc UP"]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWN` reader - desc DOWN"]
pub type DownR = crate::BitReader;
#[doc = "Field `DOWN` writer - desc DOWN"]
pub type DownW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CMPM"]
    #[inline(always)]
    pub fn cmpm(&self) -> CmpmR {
        CmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ARRM"]
    #[inline(always)]
    pub fn arrm(&self) -> ArrmR {
        ArrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&self) -> ExttrigR {
        ExttrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPOK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CmpokR {
        CmpokR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ARROK"]
    #[inline(always)]
    pub fn arrok(&self) -> ArrokR {
        ArrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc UP"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DOWN"]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMPM"]
    #[inline(always)]
    pub fn cmpm(&mut self) -> CmpmW<'_, IerSpec> {
        CmpmW::new(self, 0)
    }
    #[doc = "Bit 1 - desc ARRM"]
    #[inline(always)]
    pub fn arrm(&mut self) -> ArrmW<'_, IerSpec> {
        ArrmW::new(self, 1)
    }
    #[doc = "Bit 2 - desc EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&mut self) -> ExttrigW<'_, IerSpec> {
        ExttrigW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMPOK"]
    #[inline(always)]
    pub fn cmpok(&mut self) -> CmpokW<'_, IerSpec> {
        CmpokW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ARROK"]
    #[inline(always)]
    pub fn arrok(&mut self) -> ArrokW<'_, IerSpec> {
        ArrokW::new(self, 4)
    }
    #[doc = "Bit 5 - desc UP"]
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, IerSpec> {
        UpW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DOWN"]
    #[inline(always)]
    pub fn down(&mut self) -> DownW<'_, IerSpec> {
        DownW::new(self, 6)
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
