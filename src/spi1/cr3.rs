#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `HDOE` reader - desc HDOE"]
pub type HdoeR = crate::BitReader;
#[doc = "Field `HDOE` writer - desc HDOE"]
pub type HdoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    pub fn hdoe(&self) -> HdoeR {
        HdoeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    pub fn hdoe(&mut self) -> HdoeW<'_, Cr3Spec> {
        HdoeW::new(self, 0)
    }
}
#[doc = "desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
