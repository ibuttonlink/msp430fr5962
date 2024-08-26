#[doc = "Register `RTCYEAR` reader"]
pub type R = crate::R<RtcyearSpec>;
#[doc = "Register `RTCYEAR` writer"]
pub type W = crate::W<RtcyearSpec>;
#[doc = "Field `YEARLOWBYTE` reader - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
pub type YearlowbyteR = crate::FieldReader;
#[doc = "Field `YEARLOWBYTE` writer - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
pub type YearlowbyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEARHIGHBYTE` reader - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
pub type YearhighbyteR = crate::FieldReader;
#[doc = "Field `YEARHIGHBYTE` writer - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
pub type YearhighbyteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearlowbyte(&self) -> YearlowbyteR {
        YearlowbyteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    pub fn yearhighbyte(&self) -> YearhighbyteR {
        YearhighbyteR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Year low byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    #[must_use]
    pub fn yearlowbyte(&mut self) -> YearlowbyteW<RtcyearSpec> {
        YearlowbyteW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Year high byte. Valid values for Year are 0 to 4095."]
    #[inline(always)]
    #[must_use]
    pub fn yearhighbyte(&mut self) -> YearhighbyteW<RtcyearSpec> {
        YearhighbyteW::new(self, 8)
    }
}
#[doc = "RTCYEAR Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcyear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcyear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcyearSpec;
impl crate::RegisterSpec for RtcyearSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcyear::R`](R) reader structure"]
impl crate::Readable for RtcyearSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcyear::W`](W) writer structure"]
impl crate::Writable for RtcyearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCYEAR to value 0"]
impl crate::Resettable for RtcyearSpec {
    const RESET_VALUE: u16 = 0;
}
