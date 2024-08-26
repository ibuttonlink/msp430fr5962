#[doc = "Register `RTCTIM1` reader"]
pub type R = crate::R<Rtctim1Spec>;
#[doc = "Register `RTCTIM1` writer"]
pub type W = crate::W<Rtctim1Spec>;
#[doc = "Field `HOURS` reader - 4:0\\]
Hours (0 to 23)"]
pub type HoursR = crate::FieldReader;
#[doc = "Field `HOURS` writer - 4:0\\]
Hours (0 to 23)"]
pub type HoursW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAYOFWEEK` reader - 10:8\\]
Day of week (0 to 6)"]
pub type DayofweekR = crate::FieldReader;
#[doc = "Field `DAYOFWEEK` writer - 10:8\\]
Day of week (0 to 6)"]
pub type DayofweekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HoursR {
        HoursR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&self) -> DayofweekR {
        DayofweekR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Hours (0 to 23)"]
    #[inline(always)]
    #[must_use]
    pub fn hours(&mut self) -> HoursW<Rtctim1Spec> {
        HoursW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    #[must_use]
    pub fn dayofweek(&mut self) -> DayofweekW<Rtctim1Spec> {
        DayofweekW::new(self, 8)
    }
}
#[doc = "Real-Time Clock Hour, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtctim1Spec;
impl crate::RegisterSpec for Rtctim1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtctim1::R`](R) reader structure"]
impl crate::Readable for Rtctim1Spec {}
#[doc = "`write(|w| ..)` method takes [`rtctim1::W`](W) writer structure"]
impl crate::Writable for Rtctim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCTIM1 to value 0"]
impl crate::Resettable for Rtctim1Spec {
    const RESET_VALUE: u16 = 0;
}
