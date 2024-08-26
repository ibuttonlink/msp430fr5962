#[doc = "Register `RTCCNT12` reader"]
pub type R = crate::R<Rtccnt12Spec>;
#[doc = "Register `RTCCNT12` writer"]
pub type W = crate::W<Rtccnt12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Real-Time Clock Counter 1 and 2 Register Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt12Spec;
impl crate::RegisterSpec for Rtccnt12Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtccnt12::R`](R) reader structure"]
impl crate::Readable for Rtccnt12Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt12::W`](W) writer structure"]
impl crate::Writable for Rtccnt12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCCNT12 to value 0"]
impl crate::Resettable for Rtccnt12Spec {
    const RESET_VALUE: u16 = 0;
}
