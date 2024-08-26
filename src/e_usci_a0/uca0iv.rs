#[doc = "Register `UCA0IV` reader"]
pub type R = crate::R<Uca0ivSpec>;
#[doc = "Register `UCA0IV` writer"]
pub type W = crate::W<Uca0ivSpec>;
#[doc = "15:0\\]
eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Uciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    Ucrxifg = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    Uctxifg = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    Ucsttifg = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    Uctxcptifg = 8,
}
impl From<Uciv> for u16 {
    #[inline(always)]
    fn from(variant: Uciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uciv {
    type Ux = u16;
}
impl crate::IsEnum for Uciv {}
#[doc = "Field `UCIV` reader - 15:0\\]
eUSCI_A interrupt vector value"]
pub type UcivR = crate::FieldReader<Uciv>;
impl UcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uciv> {
        match self.bits {
            0 => Some(Uciv::None),
            2 => Some(Uciv::Ucrxifg),
            4 => Some(Uciv::Uctxifg),
            6 => Some(Uciv::Ucsttifg),
            8 => Some(Uciv::Uctxcptifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Uciv::None
    }
    #[doc = "Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == Uciv::Ucrxifg
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == Uciv::Uctxifg
    }
    #[doc = "Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == Uciv::Ucsttifg
    }
    #[doc = "Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_uctxcptifg(&self) -> bool {
        *self == Uciv::Uctxcptifg
    }
}
#[doc = "Field `UCIV` writer - 15:0\\]
eUSCI_A interrupt vector value"]
pub type UcivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Uciv>;
impl<'a, REG> UcivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::None)
    }
    #[doc = "Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucrxifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    #[inline(always)]
    pub fn uctxifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg)
    }
    #[doc = "Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn ucsttifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucsttifg)
    }
    #[doc = "Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn uctxcptifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxcptifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UcivR {
        UcivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_A interrupt vector value"]
    #[inline(always)]
    #[must_use]
    pub fn uciv(&mut self) -> UcivW<Uca0ivSpec> {
        UcivW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ivSpec;
impl crate::RegisterSpec for Uca0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0iv::R`](R) reader structure"]
impl crate::Readable for Uca0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0iv::W`](W) writer structure"]
impl crate::Writable for Uca0ivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCA0IV to value 0"]
impl crate::Resettable for Uca0ivSpec {
    const RESET_VALUE: u16 = 0;
}
