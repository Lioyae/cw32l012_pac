#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `CALEN` reader - desc CALEN"]
pub type CalenR = crate::BitReader;
#[doc = "Field `CALEN` writer - desc CALEN"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALMODE` reader - desc CALMODE"]
pub type CalmodeR = crate::BitReader;
#[doc = "Field `CALMODE` writer - desc CALMODE"]
pub type CalmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTTRIG` reader - desc SOFTTRIG"]
pub type SofttrigR = crate::BitReader;
#[doc = "Field `SOFTTRIG` writer - desc SOFTTRIG"]
pub type SofttrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALPERIOD` reader - desc CALPERIOD"]
pub type CalperiodR = crate::FieldReader;
#[doc = "Field `CALPERIOD` writer - desc CALPERIOD"]
pub type CalperiodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADCSTART` reader - desc ADCSTART"]
pub type AdcstartR = crate::BitReader;
#[doc = "Field `ADCSTART` writer - desc ADCSTART"]
pub type AdcstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZRUN` reader - desc AZRUN"]
pub type AzrunR = crate::BitReader;
#[doc = "Field `CLKDIV` reader - desc CLKDIV"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - desc CLKDIV"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADC1START1` reader - desc ADC1START1"]
pub type Adc1start1R = crate::BitReader;
#[doc = "Field `ADC1START1` writer - desc ADC1START1"]
pub type Adc1start1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2START1` reader - desc ADC2START1"]
pub type Adc2start1R = crate::BitReader;
#[doc = "Field `ADC2START1` writer - desc ADC2START1"]
pub type Adc2start1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM1TRGO` reader - desc ATIM1TRGO"]
pub type Atim1trgoR = crate::BitReader;
#[doc = "Field `ATIM1TRGO` writer - desc ATIM1TRGO"]
pub type Atim1trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM1OC4` reader - desc ATIM1OC4"]
pub type Atim1oc4R = crate::BitReader;
#[doc = "Field `ATIM1OC4` writer - desc ATIM1OC4"]
pub type Atim1oc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM1OC5` reader - desc ATIM1OC5"]
pub type Atim1oc5R = crate::BitReader;
#[doc = "Field `ATIM1OC5` writer - desc ATIM1OC5"]
pub type Atim1oc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM1OC6` reader - desc ATIM1OC6"]
pub type Atim1oc6R = crate::BitReader;
#[doc = "Field `ATIM1OC6` writer - desc ATIM1OC6"]
pub type Atim1oc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1TRGO` reader - desc GTIM1TRGO"]
pub type Gtim1trgoR = crate::BitReader;
#[doc = "Field `GTIM1TRGO` writer - desc GTIM1TRGO"]
pub type Gtim1trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1OC1` reader - desc GTIM1OC1"]
pub type Gtim1oc1R = crate::BitReader;
#[doc = "Field `GTIM1OC1` writer - desc GTIM1OC1"]
pub type Gtim1oc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1OC2` reader - desc GTIM1OC2"]
pub type Gtim1oc2R = crate::BitReader;
#[doc = "Field `GTIM1OC2` writer - desc GTIM1OC2"]
pub type Gtim1oc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2TRGO` reader - desc GTIM2TRGO"]
pub type Gtim2trgoR = crate::BitReader;
#[doc = "Field `GTIM2TRGO` writer - desc GTIM2TRGO"]
pub type Gtim2trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2OC1` reader - desc GTIM2OC1"]
pub type Gtim2oc1R = crate::BitReader;
#[doc = "Field `GTIM2OC1` writer - desc GTIM2OC1"]
pub type Gtim2oc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2OC2` reader - desc GTIM2OC2"]
pub type Gtim2oc2R = crate::BitReader;
#[doc = "Field `GTIM2OC2` writer - desc GTIM2OC2"]
pub type Gtim2oc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3TRGO` reader - desc GTIM3TRGO"]
pub type Gtim3trgoR = crate::BitReader;
#[doc = "Field `GTIM3TRGO` writer - desc GTIM3TRGO"]
pub type Gtim3trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3OC1` reader - desc GTIM3OC1"]
pub type Gtim3oc1R = crate::BitReader;
#[doc = "Field `GTIM3OC1` writer - desc GTIM3OC1"]
pub type Gtim3oc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3OC2` reader - desc GTIM3OC2"]
pub type Gtim3oc2R = crate::BitReader;
#[doc = "Field `GTIM3OC2` writer - desc GTIM3OC2"]
pub type Gtim3oc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4TRGO` reader - desc GTIM4TRGO"]
pub type Gtim4trgoR = crate::BitReader;
#[doc = "Field `GTIM4TRGO` writer - desc GTIM4TRGO"]
pub type Gtim4trgoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4OC1` reader - desc GTIM4OC1"]
pub type Gtim4oc1R = crate::BitReader;
#[doc = "Field `GTIM4OC1` writer - desc GTIM4OC1"]
pub type Gtim4oc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4OC2` reader - desc GTIM4OC2"]
pub type Gtim4oc2R = crate::BitReader;
#[doc = "Field `GTIM4OC2` writer - desc GTIM4OC2"]
pub type Gtim4oc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc CALEN"]
    #[inline(always)]
    pub fn calen(&self) -> CalenR {
        CalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CALMODE"]
    #[inline(always)]
    pub fn calmode(&self) -> CalmodeR {
        CalmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SOFTTRIG"]
    #[inline(always)]
    pub fn softtrig(&self) -> SofttrigR {
        SofttrigR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CALPERIOD"]
    #[inline(always)]
    pub fn calperiod(&self) -> CalperiodR {
        CalperiodR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - desc ADCSTART"]
    #[inline(always)]
    pub fn adcstart(&self) -> AdcstartR {
        AdcstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - desc AZRUN"]
    #[inline(always)]
    pub fn azrun(&self) -> AzrunR {
        AzrunR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - desc ADC1START1"]
    #[inline(always)]
    pub fn adc1start1(&self) -> Adc1start1R {
        Adc1start1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADC2START1"]
    #[inline(always)]
    pub fn adc2start1(&self) -> Adc2start1R {
        Adc2start1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc ATIM1TRGO"]
    #[inline(always)]
    pub fn atim1trgo(&self) -> Atim1trgoR {
        Atim1trgoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc ATIM1OC4"]
    #[inline(always)]
    pub fn atim1oc4(&self) -> Atim1oc4R {
        Atim1oc4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc ATIM1OC5"]
    #[inline(always)]
    pub fn atim1oc5(&self) -> Atim1oc5R {
        Atim1oc5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc ATIM1OC6"]
    #[inline(always)]
    pub fn atim1oc6(&self) -> Atim1oc6R {
        Atim1oc6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc GTIM1TRGO"]
    #[inline(always)]
    pub fn gtim1trgo(&self) -> Gtim1trgoR {
        Gtim1trgoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc GTIM1OC1"]
    #[inline(always)]
    pub fn gtim1oc1(&self) -> Gtim1oc1R {
        Gtim1oc1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc GTIM1OC2"]
    #[inline(always)]
    pub fn gtim1oc2(&self) -> Gtim1oc2R {
        Gtim1oc2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc GTIM2TRGO"]
    #[inline(always)]
    pub fn gtim2trgo(&self) -> Gtim2trgoR {
        Gtim2trgoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc GTIM2OC1"]
    #[inline(always)]
    pub fn gtim2oc1(&self) -> Gtim2oc1R {
        Gtim2oc1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc GTIM2OC2"]
    #[inline(always)]
    pub fn gtim2oc2(&self) -> Gtim2oc2R {
        Gtim2oc2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - desc GTIM3TRGO"]
    #[inline(always)]
    pub fn gtim3trgo(&self) -> Gtim3trgoR {
        Gtim3trgoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - desc GTIM3OC1"]
    #[inline(always)]
    pub fn gtim3oc1(&self) -> Gtim3oc1R {
        Gtim3oc1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - desc GTIM3OC2"]
    #[inline(always)]
    pub fn gtim3oc2(&self) -> Gtim3oc2R {
        Gtim3oc2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc GTIM4TRGO"]
    #[inline(always)]
    pub fn gtim4trgo(&self) -> Gtim4trgoR {
        Gtim4trgoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc GTIM4OC1"]
    #[inline(always)]
    pub fn gtim4oc1(&self) -> Gtim4oc1R {
        Gtim4oc1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc GTIM4OC2"]
    #[inline(always)]
    pub fn gtim4oc2(&self) -> Gtim4oc2R {
        Gtim4oc2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CALEN"]
    #[inline(always)]
    pub fn calen(&mut self) -> CalenW<'_, CalSpec> {
        CalenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CALMODE"]
    #[inline(always)]
    pub fn calmode(&mut self) -> CalmodeW<'_, CalSpec> {
        CalmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CalSpec> {
        StartW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SOFTTRIG"]
    #[inline(always)]
    pub fn softtrig(&mut self) -> SofttrigW<'_, CalSpec> {
        SofttrigW::new(self, 3)
    }
    #[doc = "Bits 4:7 - desc CALPERIOD"]
    #[inline(always)]
    pub fn calperiod(&mut self) -> CalperiodW<'_, CalSpec> {
        CalperiodW::new(self, 4)
    }
    #[doc = "Bit 8 - desc ADCSTART"]
    #[inline(always)]
    pub fn adcstart(&mut self) -> AdcstartW<'_, CalSpec> {
        AdcstartW::new(self, 8)
    }
    #[doc = "Bits 11:13 - desc CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, CalSpec> {
        ClkdivW::new(self, 11)
    }
    #[doc = "Bit 14 - desc ADC1START1"]
    #[inline(always)]
    pub fn adc1start1(&mut self) -> Adc1start1W<'_, CalSpec> {
        Adc1start1W::new(self, 14)
    }
    #[doc = "Bit 15 - desc ADC2START1"]
    #[inline(always)]
    pub fn adc2start1(&mut self) -> Adc2start1W<'_, CalSpec> {
        Adc2start1W::new(self, 15)
    }
    #[doc = "Bit 16 - desc ATIM1TRGO"]
    #[inline(always)]
    pub fn atim1trgo(&mut self) -> Atim1trgoW<'_, CalSpec> {
        Atim1trgoW::new(self, 16)
    }
    #[doc = "Bit 17 - desc ATIM1OC4"]
    #[inline(always)]
    pub fn atim1oc4(&mut self) -> Atim1oc4W<'_, CalSpec> {
        Atim1oc4W::new(self, 17)
    }
    #[doc = "Bit 18 - desc ATIM1OC5"]
    #[inline(always)]
    pub fn atim1oc5(&mut self) -> Atim1oc5W<'_, CalSpec> {
        Atim1oc5W::new(self, 18)
    }
    #[doc = "Bit 19 - desc ATIM1OC6"]
    #[inline(always)]
    pub fn atim1oc6(&mut self) -> Atim1oc6W<'_, CalSpec> {
        Atim1oc6W::new(self, 19)
    }
    #[doc = "Bit 20 - desc GTIM1TRGO"]
    #[inline(always)]
    pub fn gtim1trgo(&mut self) -> Gtim1trgoW<'_, CalSpec> {
        Gtim1trgoW::new(self, 20)
    }
    #[doc = "Bit 21 - desc GTIM1OC1"]
    #[inline(always)]
    pub fn gtim1oc1(&mut self) -> Gtim1oc1W<'_, CalSpec> {
        Gtim1oc1W::new(self, 21)
    }
    #[doc = "Bit 22 - desc GTIM1OC2"]
    #[inline(always)]
    pub fn gtim1oc2(&mut self) -> Gtim1oc2W<'_, CalSpec> {
        Gtim1oc2W::new(self, 22)
    }
    #[doc = "Bit 23 - desc GTIM2TRGO"]
    #[inline(always)]
    pub fn gtim2trgo(&mut self) -> Gtim2trgoW<'_, CalSpec> {
        Gtim2trgoW::new(self, 23)
    }
    #[doc = "Bit 24 - desc GTIM2OC1"]
    #[inline(always)]
    pub fn gtim2oc1(&mut self) -> Gtim2oc1W<'_, CalSpec> {
        Gtim2oc1W::new(self, 24)
    }
    #[doc = "Bit 25 - desc GTIM2OC2"]
    #[inline(always)]
    pub fn gtim2oc2(&mut self) -> Gtim2oc2W<'_, CalSpec> {
        Gtim2oc2W::new(self, 25)
    }
    #[doc = "Bit 26 - desc GTIM3TRGO"]
    #[inline(always)]
    pub fn gtim3trgo(&mut self) -> Gtim3trgoW<'_, CalSpec> {
        Gtim3trgoW::new(self, 26)
    }
    #[doc = "Bit 27 - desc GTIM3OC1"]
    #[inline(always)]
    pub fn gtim3oc1(&mut self) -> Gtim3oc1W<'_, CalSpec> {
        Gtim3oc1W::new(self, 27)
    }
    #[doc = "Bit 28 - desc GTIM3OC2"]
    #[inline(always)]
    pub fn gtim3oc2(&mut self) -> Gtim3oc2W<'_, CalSpec> {
        Gtim3oc2W::new(self, 28)
    }
    #[doc = "Bit 29 - desc GTIM4TRGO"]
    #[inline(always)]
    pub fn gtim4trgo(&mut self) -> Gtim4trgoW<'_, CalSpec> {
        Gtim4trgoW::new(self, 29)
    }
    #[doc = "Bit 30 - desc GTIM4OC1"]
    #[inline(always)]
    pub fn gtim4oc1(&mut self) -> Gtim4oc1W<'_, CalSpec> {
        Gtim4oc1W::new(self, 30)
    }
    #[doc = "Bit 31 - desc GTIM4OC2"]
    #[inline(always)]
    pub fn gtim4oc2(&mut self) -> Gtim4oc2W<'_, CalSpec> {
        Gtim4oc2W::new(self, 31)
    }
}
#[doc = "desc CAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0"]
impl crate::Resettable for CalSpec {}
