#[doc = "Register `MMATCH` reader"]
pub type R = crate::R<MmatchSpec>;
#[doc = "Register `MMATCH` writer"]
pub type W = crate::W<MmatchSpec>;
#[doc = "Field `MATCH0` reader - desc MATCH0"]
pub type Match0R = crate::FieldReader;
#[doc = "Field `MATCH0` writer - desc MATCH0"]
pub type Match0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MATCH1` reader - desc MATCH1"]
pub type Match1R = crate::FieldReader;
#[doc = "Field `MATCH1` writer - desc MATCH1"]
pub type Match1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc MATCH0"]
    #[inline(always)]
    pub fn match0(&self) -> Match0R {
        Match0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc MATCH1"]
    #[inline(always)]
    pub fn match1(&self) -> Match1R {
        Match1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc MATCH0"]
    #[inline(always)]
    pub fn match0(&mut self) -> Match0W<'_, MmatchSpec> {
        Match0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - desc MATCH1"]
    #[inline(always)]
    pub fn match1(&mut self) -> Match1W<'_, MmatchSpec> {
        Match1W::new(self, 16)
    }
}
#[doc = "Master Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmatch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmatch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmatchSpec;
impl crate::RegisterSpec for MmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmatch::R`](R) reader structure"]
impl crate::Readable for MmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`mmatch::W`](W) writer structure"]
impl crate::Writable for MmatchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMATCH to value 0"]
impl crate::Resettable for MmatchSpec {}
