#[doc = "Register `APBRST1` reader"]
pub type R = crate::R<Apbrst1Spec>;
#[doc = "Register `APBRST1` writer"]
pub type W = crate::W<Apbrst1Spec>;
#[doc = "Field `ADC` reader - desc ADC"]
pub type AdcR = crate::BitReader;
#[doc = "Field `ADC` writer - desc ADC"]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC` reader - desc VC"]
pub type VcR = crate::BitReader;
#[doc = "Field `VC` writer - desc VC"]
pub type VcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type AtimR = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type AtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type Gtim1R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type Gtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type Gtim2R = crate::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type Gtim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type Uart3R = crate::BitReader;
#[doc = "Field `UART3` writer - desc UART3"]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type Gtim3R = crate::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type Gtim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type Gtim4R = crate::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type Gtim4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3` reader - desc SPI3"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI3` writer - desc SPI3"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ADC"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc VC"]
    #[inline(always)]
    pub fn vc(&self) -> VcR {
        VcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> AtimR {
        AtimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&self) -> Gtim1R {
        Gtim1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&self) -> Gtim2R {
        Gtim2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&self) -> Gtim3R {
        Gtim3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&self) -> Gtim4R {
        Gtim4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<'_, Apbrst1Spec> {
        AdcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc VC"]
    #[inline(always)]
    pub fn vc(&mut self) -> VcW<'_, Apbrst1Spec> {
        VcW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Apbrst1Spec> {
        Spi1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Apbrst1Spec> {
        Uart1W::new(self, 3)
    }
    #[doc = "Bit 4 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Apbrst1Spec> {
        Uart2W::new(self, 4)
    }
    #[doc = "Bit 5 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, Apbrst1Spec> {
        AtimW::new(self, 5)
    }
    #[doc = "Bit 6 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, Apbrst1Spec> {
        Gtim1W::new(self, 6)
    }
    #[doc = "Bit 7 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&mut self) -> Gtim2W<'_, Apbrst1Spec> {
        Gtim2W::new(self, 7)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<'_, Apbrst1Spec> {
        Uart3W::new(self, 8)
    }
    #[doc = "Bit 11 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&mut self) -> Gtim3W<'_, Apbrst1Spec> {
        Gtim3W::new(self, 11)
    }
    #[doc = "Bit 12 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&mut self) -> Gtim4W<'_, Apbrst1Spec> {
        Gtim4W::new(self, 12)
    }
    #[doc = "Bit 13 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Apbrst1Spec> {
        Spi2W::new(self, 13)
    }
    #[doc = "Bit 14 - desc SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> Spi3W<'_, Apbrst1Spec> {
        Spi3W::new(self, 14)
    }
}
#[doc = "APB Reset Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrst1Spec;
impl crate::RegisterSpec for Apbrst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst1::R`](R) reader structure"]
impl crate::Readable for Apbrst1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrst1::W`](W) writer structure"]
impl crate::Writable for Apbrst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST1 to value 0"]
impl crate::Resettable for Apbrst1Spec {}
