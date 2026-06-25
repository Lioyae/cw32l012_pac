#[doc = "Register `AWDCR` reader"]
pub type R = crate::R<AwdcrSpec>;
#[doc = "Register `AWDCR` writer"]
pub type W = crate::W<AwdcrSpec>;
#[doc = "Field `IN0` reader - desc IN0"]
pub type In0R = crate::BitReader;
#[doc = "Field `IN0` writer - desc IN0"]
pub type In0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN1` reader - desc IN1"]
pub type In1R = crate::BitReader;
#[doc = "Field `IN1` writer - desc IN1"]
pub type In1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN2` reader - desc IN2"]
pub type In2R = crate::BitReader;
#[doc = "Field `IN2` writer - desc IN2"]
pub type In2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN3` reader - desc IN3"]
pub type In3R = crate::BitReader;
#[doc = "Field `IN3` writer - desc IN3"]
pub type In3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN4` reader - desc IN4"]
pub type In4R = crate::BitReader;
#[doc = "Field `IN4` writer - desc IN4"]
pub type In4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN5` reader - desc IN5"]
pub type In5R = crate::BitReader;
#[doc = "Field `IN5` writer - desc IN5"]
pub type In5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN6` reader - desc IN6"]
pub type In6R = crate::BitReader;
#[doc = "Field `IN6` writer - desc IN6"]
pub type In6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN7` reader - desc IN7"]
pub type In7R = crate::BitReader;
#[doc = "Field `IN7` writer - desc IN7"]
pub type In7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN8` reader - desc IN8"]
pub type In8R = crate::BitReader;
#[doc = "Field `IN8` writer - desc IN8"]
pub type In8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN9` reader - desc IN9"]
pub type In9R = crate::BitReader;
#[doc = "Field `IN9` writer - desc IN9"]
pub type In9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN10` reader - desc IN10"]
pub type In10R = crate::BitReader;
#[doc = "Field `IN10` writer - desc IN10"]
pub type In10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN11` reader - desc IN11"]
pub type In11R = crate::BitReader;
#[doc = "Field `IN11` writer - desc IN11"]
pub type In11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN12` reader - desc IN12"]
pub type In12R = crate::BitReader;
#[doc = "Field `IN12` writer - desc IN12"]
pub type In12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN13` reader - desc IN13"]
pub type In13R = crate::BitReader;
#[doc = "Field `IN13` writer - desc IN13"]
pub type In13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN14` reader - desc IN14"]
pub type In14R = crate::BitReader;
#[doc = "Field `IN14` writer - desc IN14"]
pub type In14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN15` reader - desc IN15"]
pub type In15R = crate::BitReader;
#[doc = "Field `IN15` writer - desc IN15"]
pub type In15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc IN0"]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc IN1"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc IN2"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc IN3"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IN4"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc IN5"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc IN6"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc IN7"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc IN8"]
    #[inline(always)]
    pub fn in8(&self) -> In8R {
        In8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IN9"]
    #[inline(always)]
    pub fn in9(&self) -> In9R {
        In9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc IN10"]
    #[inline(always)]
    pub fn in10(&self) -> In10R {
        In10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc IN11"]
    #[inline(always)]
    pub fn in11(&self) -> In11R {
        In11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IN12"]
    #[inline(always)]
    pub fn in12(&self) -> In12R {
        In12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc IN13"]
    #[inline(always)]
    pub fn in13(&self) -> In13R {
        In13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc IN14"]
    #[inline(always)]
    pub fn in14(&self) -> In14R {
        In14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc IN15"]
    #[inline(always)]
    pub fn in15(&self) -> In15R {
        In15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc IN0"]
    #[inline(always)]
    pub fn in0(&mut self) -> In0W<'_, AwdcrSpec> {
        In0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc IN1"]
    #[inline(always)]
    pub fn in1(&mut self) -> In1W<'_, AwdcrSpec> {
        In1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc IN2"]
    #[inline(always)]
    pub fn in2(&mut self) -> In2W<'_, AwdcrSpec> {
        In2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc IN3"]
    #[inline(always)]
    pub fn in3(&mut self) -> In3W<'_, AwdcrSpec> {
        In3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc IN4"]
    #[inline(always)]
    pub fn in4(&mut self) -> In4W<'_, AwdcrSpec> {
        In4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc IN5"]
    #[inline(always)]
    pub fn in5(&mut self) -> In5W<'_, AwdcrSpec> {
        In5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc IN6"]
    #[inline(always)]
    pub fn in6(&mut self) -> In6W<'_, AwdcrSpec> {
        In6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc IN7"]
    #[inline(always)]
    pub fn in7(&mut self) -> In7W<'_, AwdcrSpec> {
        In7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc IN8"]
    #[inline(always)]
    pub fn in8(&mut self) -> In8W<'_, AwdcrSpec> {
        In8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc IN9"]
    #[inline(always)]
    pub fn in9(&mut self) -> In9W<'_, AwdcrSpec> {
        In9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc IN10"]
    #[inline(always)]
    pub fn in10(&mut self) -> In10W<'_, AwdcrSpec> {
        In10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc IN11"]
    #[inline(always)]
    pub fn in11(&mut self) -> In11W<'_, AwdcrSpec> {
        In11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc IN12"]
    #[inline(always)]
    pub fn in12(&mut self) -> In12W<'_, AwdcrSpec> {
        In12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc IN13"]
    #[inline(always)]
    pub fn in13(&mut self) -> In13W<'_, AwdcrSpec> {
        In13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc IN14"]
    #[inline(always)]
    pub fn in14(&mut self) -> In14W<'_, AwdcrSpec> {
        In14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc IN15"]
    #[inline(always)]
    pub fn in15(&mut self) -> In15W<'_, AwdcrSpec> {
        In15W::new(self, 15)
    }
}
#[doc = "desc AWDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdcrSpec;
impl crate::RegisterSpec for AwdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awdcr::R`](R) reader structure"]
impl crate::Readable for AwdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdcr::W`](W) writer structure"]
impl crate::Writable for AwdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDCR to value 0"]
impl crate::Resettable for AwdcrSpec {}
