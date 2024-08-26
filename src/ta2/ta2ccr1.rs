#[doc = "Register `TA2CCR1` reader"]
pub type R = crate::R<Ta2ccr1Spec>;
#[doc = "Register `TA2CCR1` writer"]
pub type W = crate::W<Ta2ccr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer_A Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta2ccr1Spec;
impl crate::RegisterSpec for Ta2ccr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta2ccr1::R`](R) reader structure"]
impl crate::Readable for Ta2ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ta2ccr1::W`](W) writer structure"]
impl crate::Writable for Ta2ccr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TA2CCR1 to value 0"]
impl crate::Resettable for Ta2ccr1Spec {
    const RESET_VALUE: u16 = 0;
}
