#[doc = "Register `RTCPS` reader"]
pub type R = crate::R<RtcpsSpec>;
#[doc = "Register `RTCPS` writer"]
pub type W = crate::W<RtcpsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Real-Time Clock Prescale Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcpsSpec;
impl crate::RegisterSpec for RtcpsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcps::R`](R) reader structure"]
impl crate::Readable for RtcpsSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcps::W`](W) writer structure"]
impl crate::Writable for RtcpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCPS to value 0"]
impl crate::Resettable for RtcpsSpec {
    const RESET_VALUE: u16 = 0;
}
