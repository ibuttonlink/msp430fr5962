#[doc = "Register `TB0IV` reader"]
pub type R = crate::R<Tb0ivSpec>;
#[doc = "Register `TB0IV` writer"]
pub type W = crate::W<Tb0ivSpec>;
#[doc = "15:0\\]
Timer_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Tbiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    Tbccr1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    Tbccr2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    Tbccr3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    Tbccr4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    Tbccr5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    Tbccr6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    Tbifg = 14,
}
impl From<Tbiv> for u16 {
    #[inline(always)]
    fn from(variant: Tbiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbiv {
    type Ux = u16;
}
impl crate::IsEnum for Tbiv {}
#[doc = "Field `TBIV` reader - 15:0\\]
Timer_B interrupt vector value"]
pub type TbivR = crate::FieldReader<Tbiv>;
impl TbivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tbiv> {
        match self.bits {
            0 => Some(Tbiv::None),
            2 => Some(Tbiv::Tbccr1),
            4 => Some(Tbiv::Tbccr2),
            6 => Some(Tbiv::Tbccr3),
            8 => Some(Tbiv::Tbccr4),
            10 => Some(Tbiv::Tbccr5),
            12 => Some(Tbiv::Tbccr6),
            14 => Some(Tbiv::Tbifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Tbiv::None
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == Tbiv::Tbccr1
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == Tbiv::Tbccr2
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    #[inline(always)]
    pub fn is_tbccr3(&self) -> bool {
        *self == Tbiv::Tbccr3
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    #[inline(always)]
    pub fn is_tbccr4(&self) -> bool {
        *self == Tbiv::Tbccr4
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    #[inline(always)]
    pub fn is_tbccr5(&self) -> bool {
        *self == Tbiv::Tbccr5
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    #[inline(always)]
    pub fn is_tbccr6(&self) -> bool {
        *self == Tbiv::Tbccr6
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == Tbiv::Tbifg
    }
}
#[doc = "Field `TBIV` writer - 15:0\\]
Timer_B interrupt vector value"]
pub type TbivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Tbiv>;
impl<'a, REG> TbivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::None)
    }
    #[doc = "Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn tbccr1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr1)
    }
    #[doc = "Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    #[inline(always)]
    pub fn tbccr2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr2)
    }
    #[doc = "Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    #[inline(always)]
    pub fn tbccr3(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr3)
    }
    #[doc = "Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    #[inline(always)]
    pub fn tbccr4(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr4)
    }
    #[doc = "Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    #[inline(always)]
    pub fn tbccr5(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr5)
    }
    #[doc = "Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    #[inline(always)]
    pub fn tbccr6(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbccr6)
    }
    #[doc = "Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn tbifg(self) -> &'a mut crate::W<REG> {
        self.variant(Tbiv::Tbifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timer_B interrupt vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TbivR {
        TbivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timer_B interrupt vector value"]
    #[inline(always)]
    #[must_use]
    pub fn tbiv(&mut self) -> TbivW<Tb0ivSpec> {
        TbivW::new(self, 0)
    }
}
#[doc = "Timer_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0ivSpec;
impl crate::RegisterSpec for Tb0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0iv::R`](R) reader structure"]
impl crate::Readable for Tb0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`tb0iv::W`](W) writer structure"]
impl crate::Writable for Tb0ivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TB0IV to value 0"]
impl crate::Resettable for Tb0ivSpec {
    const RESET_VALUE: u16 = 0;
}
