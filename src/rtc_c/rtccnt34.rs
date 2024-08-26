#[doc = "Register `RTCCNT34` reader"]
pub type R = crate::R<Rtccnt34Spec>;
#[doc = "Register `RTCCNT34` writer"]
pub type W = crate::W<Rtccnt34Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Real-Time Clock Counter 3 and 4 Register Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt34Spec;
impl crate::RegisterSpec for Rtccnt34Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtccnt34::R`](R) reader structure"]
impl crate::Readable for Rtccnt34Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt34::W`](W) writer structure"]
impl crate::Writable for Rtccnt34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCCNT34 to value 0"]
impl crate::Resettable for Rtccnt34Spec {
    const RESET_VALUE: u16 = 0;
}
