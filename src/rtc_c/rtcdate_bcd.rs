#[doc = "Register `RTCDATE_BCD` reader"]
pub type R = crate::R<RtcdateBcdSpec>;
#[doc = "Register `RTCDATE_BCD` writer"]
pub type W = crate::W<RtcdateBcdSpec>;
#[doc = "Field `DAYLOWDIGIT` reader - 3:0\\]
Day of month low digit (0 to 9)"]
pub type DaylowdigitR = crate::FieldReader;
#[doc = "Field `DAYLOWDIGIT` writer - 3:0\\]
Day of month low digit (0 to 9)"]
pub type DaylowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYHIGHDIGIT` reader - 5:4\\]
Day of month high digit (0 to 3)"]
pub type DayhighdigitR = crate::FieldReader;
#[doc = "Field `DAYHIGHDIGIT` writer - 5:4\\]
Day of month high digit (0 to 3)"]
pub type DayhighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHLOWDIGIT` reader - 11:8\\]
Month low digit (0 to 9)"]
pub type MonthlowdigitR = crate::FieldReader;
#[doc = "Field `MONTHLOWDIGIT` writer - 11:8\\]
Month low digit (0 to 9)"]
pub type MonthlowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHHIGHDIGIT` reader - 12:12\\]
Month high digit (0 or 1)"]
pub type MonthhighdigitR = crate::BitReader;
#[doc = "Field `MONTHHIGHDIGIT` writer - 12:12\\]
Month high digit (0 or 1)"]
pub type MonthhighdigitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    pub fn daylowdigit(&self) -> DaylowdigitR {
        DaylowdigitR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    pub fn dayhighdigit(&self) -> DayhighdigitR {
        DayhighdigitR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Month low digit (0 to 9)"]
    #[inline(always)]
    pub fn monthlowdigit(&self) -> MonthlowdigitR {
        MonthlowdigitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Month high digit (0 or 1)"]
    #[inline(always)]
    pub fn monthhighdigit(&self) -> MonthhighdigitR {
        MonthhighdigitR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Day of month low digit (0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn daylowdigit(&mut self) -> DaylowdigitW<RtcdateBcdSpec> {
        DaylowdigitW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Day of month high digit (0 to 3)"]
    #[inline(always)]
    #[must_use]
    pub fn dayhighdigit(&mut self) -> DayhighdigitW<RtcdateBcdSpec> {
        DayhighdigitW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Month low digit (0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn monthlowdigit(&mut self) -> MonthlowdigitW<RtcdateBcdSpec> {
        MonthlowdigitW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Month high digit (0 or 1)"]
    #[inline(always)]
    #[must_use]
    pub fn monthhighdigit(&mut self) -> MonthhighdigitW<RtcdateBcdSpec> {
        MonthhighdigitW::new(self, 12)
    }
}
#[doc = "Real-Time Clock Date - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcdate_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdate_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcdateBcdSpec;
impl crate::RegisterSpec for RtcdateBcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcdate_bcd::R`](R) reader structure"]
impl crate::Readable for RtcdateBcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcdate_bcd::W`](W) writer structure"]
impl crate::Writable for RtcdateBcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCDATE_BCD to value 0"]
impl crate::Resettable for RtcdateBcdSpec {
    const RESET_VALUE: u16 = 0;
}
