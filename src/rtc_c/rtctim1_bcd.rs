#[doc = "Register `RTCTIM1_BCD` reader"]
pub type R = crate::R<Rtctim1BcdSpec>;
#[doc = "Register `RTCTIM1_BCD` writer"]
pub type W = crate::W<Rtctim1BcdSpec>;
#[doc = "Field `HOURSLOWDIGIT` reader - 3:0\\]
Hours low digit (0 to 9)"]
pub type HourslowdigitR = crate::FieldReader;
#[doc = "Field `HOURSLOWDIGIT` writer - 3:0\\]
Hours low digit (0 to 9)"]
pub type HourslowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURSHIGHDIGIT` reader - 5:4\\]
Hours high digit (0 to 2)"]
pub type HourshighdigitR = crate::FieldReader;
#[doc = "Field `HOURSHIGHDIGIT` writer - 5:4\\]
Hours high digit (0 to 2)"]
pub type HourshighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAYOFWEEK` reader - 10:8\\]
Day of week (0 to 6)"]
pub type DayofweekR = crate::FieldReader;
#[doc = "Field `DAYOFWEEK` writer - 10:8\\]
Day of week (0 to 6)"]
pub type DayofweekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    pub fn hourslowdigit(&self) -> HourslowdigitR {
        HourslowdigitR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    pub fn hourshighdigit(&self) -> HourshighdigitR {
        HourshighdigitR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayofweek(&self) -> DayofweekR {
        DayofweekR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Hours low digit (0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn hourslowdigit(&mut self) -> HourslowdigitW<Rtctim1BcdSpec> {
        HourslowdigitW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Hours high digit (0 to 2)"]
    #[inline(always)]
    #[must_use]
    pub fn hourshighdigit(&mut self) -> HourshighdigitW<Rtctim1BcdSpec> {
        HourshighdigitW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Day of week (0 to 6)"]
    #[inline(always)]
    #[must_use]
    pub fn dayofweek(&mut self) -> DayofweekW<Rtctim1BcdSpec> {
        DayofweekW::new(self, 8)
    }
}
#[doc = "Real-Time Clock Hour, Day of Week - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim1_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim1_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtctim1BcdSpec;
impl crate::RegisterSpec for Rtctim1BcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtctim1_bcd::R`](R) reader structure"]
impl crate::Readable for Rtctim1BcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtctim1_bcd::W`](W) writer structure"]
impl crate::Writable for Rtctim1BcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCTIM1_BCD to value 0"]
impl crate::Resettable for Rtctim1BcdSpec {
    const RESET_VALUE: u16 = 0;
}
