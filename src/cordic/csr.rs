#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `FUNC` reader - desc FUNC"]
pub type FuncR = crate::FieldReader;
#[doc = "Field `FUNC` writer - desc FUNC"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE` reader - desc SCALE"]
pub type ScaleR = crate::FieldReader;
#[doc = "Field `SCALE` writer - desc SCALE"]
pub type ScaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FORMAT` reader - desc FORMAT"]
pub type FormatR = crate::BitReader;
#[doc = "Field `FORMAT` writer - desc FORMAT"]
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` reader - desc ITER"]
pub type IterR = crate::FieldReader;
#[doc = "Field `ITER` writer - desc ITER"]
pub type IterW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type CompR = crate::BitReader;
#[doc = "Field `COMP` writer - desc COMP"]
pub type CompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEOC` reader - desc DMAEOC"]
pub type DmaeocR = crate::BitReader;
#[doc = "Field `DMAEOC` writer - desc DMAEOC"]
pub type DmaeocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIDLE` reader - desc DMAIDLE"]
pub type DmaidleR = crate::BitReader;
#[doc = "Field `DMAIDLE` writer - desc DMAIDLE"]
pub type DmaidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - desc FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc SCALE"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - desc FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc ITER"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc DMAEOC"]
    #[inline(always)]
    pub fn dmaeoc(&self) -> DmaeocR {
        DmaeocR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DMAIDLE"]
    #[inline(always)]
    pub fn dmaidle(&self) -> DmaidleR {
        DmaidleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc FUNC"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, CsrSpec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc SCALE"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<'_, CsrSpec> {
        ScaleW::new(self, 4)
    }
    #[doc = "Bit 7 - desc FORMAT"]
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<'_, CsrSpec> {
        FormatW::new(self, 7)
    }
    #[doc = "Bits 8:11 - desc ITER"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<'_, CsrSpec> {
        IterW::new(self, 8)
    }
    #[doc = "Bit 12 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, CsrSpec> {
        CompW::new(self, 12)
    }
    #[doc = "Bit 13 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CsrSpec> {
        IeW::new(self, 13)
    }
    #[doc = "Bit 14 - desc DMAEOC"]
    #[inline(always)]
    pub fn dmaeoc(&mut self) -> DmaeocW<'_, CsrSpec> {
        DmaeocW::new(self, 14)
    }
    #[doc = "Bit 15 - desc DMAIDLE"]
    #[inline(always)]
    pub fn dmaidle(&mut self) -> DmaidleW<'_, CsrSpec> {
        DmaidleW::new(self, 15)
    }
}
#[doc = "Control and State register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
