#[doc = "Register `TA2IV` reader"]
pub type R = crate::R<Ta2ivSpec>;
#[doc = "Register `TA2IV` writer"]
pub type W = crate::W<Ta2ivSpec>;
#[doc = "15:0\\]
TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Taiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    Taccr1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    Taccr2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    Taccr3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    Taccr4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    Taccr5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    Taccr6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    Taifg = 14,
}
impl From<Taiv> for u16 {
    #[inline(always)]
    fn from(variant: Taiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Taiv {
    type Ux = u16;
}
impl crate::IsEnum for Taiv {}
#[doc = "Field `TAIV` reader - 15:0\\]
TimerA interrupt vector value"]
pub type TaivR = crate::FieldReader<Taiv>;
impl TaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Taiv> {
        match self.bits {
            0 => Some(Taiv::None),
            2 => Some(Taiv::Taccr1),
            4 => Some(Taiv::Taccr2),
            6 => Some(Taiv::Taccr3),
            8 => Some(Taiv::Taccr4),
            10 => Some(Taiv::Taccr5),
            12 => Some(Taiv::Taccr6),
            14 => Some(Taiv::Taifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Taiv::None
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_taccr1(&self) -> bool {
        *self == Taiv::Taccr1
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    #[inline(always)]
    pub fn is_taccr2(&self) -> bool {
        *self == Taiv::Taccr2
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    #[inline(always)]
    pub fn is_taccr3(&self) -> bool {
        *self == Taiv::Taccr3
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    #[inline(always)]
    pub fn is_taccr4(&self) -> bool {
        *self == Taiv::Taccr4
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    #[inline(always)]
    pub fn is_taccr5(&self) -> bool {
        *self == Taiv::Taccr5
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    #[inline(always)]
    pub fn is_taccr6(&self) -> bool {
        *self == Taiv::Taccr6
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_taifg(&self) -> bool {
        *self == Taiv::Taifg
    }
}
#[doc = "Field `TAIV` writer - 15:0\\]
TimerA interrupt vector value"]
pub type TaivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Taiv>;
impl<'a, REG> TaivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::None)
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn taccr1(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr1)
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    #[inline(always)]
    pub fn taccr2(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr2)
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    #[inline(always)]
    pub fn taccr3(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr3)
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    #[inline(always)]
    pub fn taccr4(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr4)
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    #[inline(always)]
    pub fn taccr5(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr5)
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    #[inline(always)]
    pub fn taccr6(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taccr6)
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn taifg(self) -> &'a mut crate::W<REG> {
        self.variant(Taiv::Taifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TaivR {
        TaivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TimerA interrupt vector value"]
    #[inline(always)]
    #[must_use]
    pub fn taiv(&mut self) -> TaivW<Ta2ivSpec> {
        TaivW::new(self, 0)
    }
}
#[doc = "TimerAx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ta2iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ta2iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ta2ivSpec;
impl crate::RegisterSpec for Ta2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ta2iv::R`](R) reader structure"]
impl crate::Readable for Ta2ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ta2iv::W`](W) writer structure"]
impl crate::Writable for Ta2ivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TA2IV to value 0"]
impl crate::Resettable for Ta2ivSpec {
    const RESET_VALUE: u16 = 0;
}
