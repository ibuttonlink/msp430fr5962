#[doc = "Register `RTCIV` reader"]
pub type R = crate::R<RtcivSpec>;
#[doc = "Register `RTCIV` writer"]
pub type W = crate::W<RtcivSpec>;
#[doc = "15:0\\]
Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Rtciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    Rtcofifg = 2,
    #[doc = "4: Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    Rtcrdyifg = 4,
    #[doc = "6: Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    Rtctevifg = 6,
    #[doc = "8: Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    Rtcaifg = 8,
    #[doc = "10: Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    Rt0psifg = 10,
    #[doc = "12: Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    Rt1psifg = 12,
}
impl From<Rtciv> for u16 {
    #[inline(always)]
    fn from(variant: Rtciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtciv {
    type Ux = u16;
}
impl crate::IsEnum for Rtciv {}
#[doc = "Field `RTCIV` reader - 15:0\\]
Real-time clock interrupt vector value"]
pub type RtcivR = crate::FieldReader<Rtciv>;
impl RtcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtciv> {
        match self.bits {
            0 => Some(Rtciv::None),
            2 => Some(Rtciv::Rtcofifg),
            4 => Some(Rtciv::Rtcrdyifg),
            6 => Some(Rtciv::Rtctevifg),
            8 => Some(Rtciv::Rtcaifg),
            10 => Some(Rtciv::Rt0psifg),
            12 => Some(Rtciv::Rt1psifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rtciv::None
    }
    #[doc = "Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_rtcofifg(&self) -> bool {
        *self == Rtciv::Rtcofifg
    }
    #[doc = "Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    #[inline(always)]
    pub fn is_rtcrdyifg(&self) -> bool {
        *self == Rtciv::Rtcrdyifg
    }
    #[doc = "Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    #[inline(always)]
    pub fn is_rtctevifg(&self) -> bool {
        *self == Rtciv::Rtctevifg
    }
    #[doc = "Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    #[inline(always)]
    pub fn is_rtcaifg(&self) -> bool {
        *self == Rtciv::Rtcaifg
    }
    #[doc = "Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    #[inline(always)]
    pub fn is_rt0psifg(&self) -> bool {
        *self == Rtciv::Rt0psifg
    }
    #[doc = "Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    #[inline(always)]
    pub fn is_rt1psifg(&self) -> bool {
        *self == Rtciv::Rt1psifg
    }
}
#[doc = "Field `RTCIV` writer - 15:0\\]
Real-time clock interrupt vector value"]
pub type RtcivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Rtciv>;
impl<'a, REG> RtcivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::None)
    }
    #[doc = "Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn rtcofifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rtcofifg)
    }
    #[doc = "Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    #[inline(always)]
    pub fn rtcrdyifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rtcrdyifg)
    }
    #[doc = "Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    #[inline(always)]
    pub fn rtctevifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rtctevifg)
    }
    #[doc = "Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    #[inline(always)]
    pub fn rtcaifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rtcaifg)
    }
    #[doc = "Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    #[inline(always)]
    pub fn rt0psifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rt0psifg)
    }
    #[doc = "Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    #[inline(always)]
    pub fn rt1psifg(self) -> &'a mut crate::W<REG> {
        self.variant(Rtciv::Rt1psifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RtcivR {
        RtcivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Real-time clock interrupt vector value"]
    #[inline(always)]
    #[must_use]
    pub fn rtciv(&mut self) -> RtcivW<RtcivSpec> {
        RtcivW::new(self, 0)
    }
}
#[doc = "Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcivSpec;
impl crate::RegisterSpec for RtcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtciv::R`](R) reader structure"]
impl crate::Readable for RtcivSpec {}
#[doc = "`write(|w| ..)` method takes [`rtciv::W`](W) writer structure"]
impl crate::Writable for RtcivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RtcivSpec {
    const RESET_VALUE: u16 = 0;
}
