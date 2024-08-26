#[doc = "Register `UCB0IV` reader"]
pub type R = crate::R<Ucb0ivSpec>;
#[doc = "Register `UCB0IV` writer"]
pub type W = crate::W<Ucb0ivSpec>;
#[doc = "15:0\\]
eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Uciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    Ucalifg = 2,
    #[doc = "4: Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    Ucnackifg = 4,
    #[doc = "6: Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    Ucsttifg = 6,
    #[doc = "8: Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    Ucstpifg = 8,
    #[doc = "10: Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    Ucrxifg3 = 10,
    #[doc = "12: Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    Uctxifg3 = 12,
    #[doc = "14: Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    Ucrxifg2 = 14,
    #[doc = "16: Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    Uctxifg2 = 16,
    #[doc = "18: Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    Ucrxifg1 = 18,
    #[doc = "20: Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    Uctxifg1 = 20,
    #[doc = "22: Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    Ucrxifg0 = 22,
    #[doc = "24: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    Uctxifg0 = 24,
    #[doc = "26: Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    Ucbcntifg = 26,
    #[doc = "28: Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    Uccltoifg = 28,
    #[doc = "30: Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    Ucbit9ifg = 30,
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
eUSCI_B interrupt vector value"]
pub type UcivR = crate::FieldReader<Uciv>;
impl UcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uciv> {
        match self.bits {
            0 => Some(Uciv::None),
            2 => Some(Uciv::Ucalifg),
            4 => Some(Uciv::Ucnackifg),
            6 => Some(Uciv::Ucsttifg),
            8 => Some(Uciv::Ucstpifg),
            10 => Some(Uciv::Ucrxifg3),
            12 => Some(Uciv::Uctxifg3),
            14 => Some(Uciv::Ucrxifg2),
            16 => Some(Uciv::Uctxifg2),
            18 => Some(Uciv::Ucrxifg1),
            20 => Some(Uciv::Uctxifg1),
            22 => Some(Uciv::Ucrxifg0),
            24 => Some(Uciv::Uctxifg0),
            26 => Some(Uciv::Ucbcntifg),
            28 => Some(Uciv::Uccltoifg),
            30 => Some(Uciv::Ucbit9ifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Uciv::None
    }
    #[doc = "Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ucalifg(&self) -> bool {
        *self == Uciv::Ucalifg
    }
    #[doc = "Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    #[inline(always)]
    pub fn is_ucnackifg(&self) -> bool {
        *self == Uciv::Ucnackifg
    }
    #[doc = "Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn is_ucsttifg(&self) -> bool {
        *self == Uciv::Ucsttifg
    }
    #[doc = "Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    #[inline(always)]
    pub fn is_ucstpifg(&self) -> bool {
        *self == Uciv::Ucstpifg
    }
    #[doc = "Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    #[inline(always)]
    pub fn is_ucrxifg3(&self) -> bool {
        *self == Uciv::Ucrxifg3
    }
    #[doc = "Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    #[inline(always)]
    pub fn is_uctxifg3(&self) -> bool {
        *self == Uciv::Uctxifg3
    }
    #[doc = "Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    #[inline(always)]
    pub fn is_ucrxifg2(&self) -> bool {
        *self == Uciv::Ucrxifg2
    }
    #[doc = "Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    #[inline(always)]
    pub fn is_uctxifg2(&self) -> bool {
        *self == Uciv::Uctxifg2
    }
    #[doc = "Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    #[inline(always)]
    pub fn is_ucrxifg1(&self) -> bool {
        *self == Uciv::Ucrxifg1
    }
    #[doc = "Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    #[inline(always)]
    pub fn is_uctxifg1(&self) -> bool {
        *self == Uciv::Uctxifg1
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    #[inline(always)]
    pub fn is_ucrxifg0(&self) -> bool {
        *self == Uciv::Ucrxifg0
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    #[inline(always)]
    pub fn is_uctxifg0(&self) -> bool {
        *self == Uciv::Uctxifg0
    }
    #[doc = "Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    #[inline(always)]
    pub fn is_ucbcntifg(&self) -> bool {
        *self == Uciv::Ucbcntifg
    }
    #[doc = "Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    #[inline(always)]
    pub fn is_uccltoifg(&self) -> bool {
        *self == Uciv::Uccltoifg
    }
    #[doc = "Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    #[inline(always)]
    pub fn is_ucbit9ifg(&self) -> bool {
        *self == Uciv::Ucbit9ifg
    }
}
#[doc = "Field `UCIV` writer - 15:0\\]
eUSCI_B interrupt vector value"]
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
    #[doc = "Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucalifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucalifg)
    }
    #[doc = "Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    #[inline(always)]
    pub fn ucnackifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucnackifg)
    }
    #[doc = "Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    #[inline(always)]
    pub fn ucsttifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucsttifg)
    }
    #[doc = "Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    #[inline(always)]
    pub fn ucstpifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucstpifg)
    }
    #[doc = "Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    #[inline(always)]
    pub fn ucrxifg3(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg3)
    }
    #[doc = "Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    #[inline(always)]
    pub fn uctxifg3(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg3)
    }
    #[doc = "Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    #[inline(always)]
    pub fn ucrxifg2(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg2)
    }
    #[doc = "Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    #[inline(always)]
    pub fn uctxifg2(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg2)
    }
    #[doc = "Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    #[inline(always)]
    pub fn ucrxifg1(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg1)
    }
    #[doc = "Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    #[inline(always)]
    pub fn uctxifg1(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg1)
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    #[inline(always)]
    pub fn ucrxifg0(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg0)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    #[inline(always)]
    pub fn uctxifg0(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg0)
    }
    #[doc = "Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    #[inline(always)]
    pub fn ucbcntifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucbcntifg)
    }
    #[doc = "Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    #[inline(always)]
    pub fn uccltoifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uccltoifg)
    }
    #[doc = "Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    #[inline(always)]
    pub fn ucbit9ifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucbit9ifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UcivR {
        UcivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
eUSCI_B interrupt vector value"]
    #[inline(always)]
    #[must_use]
    pub fn uciv(&mut self) -> UcivW<Ucb0ivSpec> {
        UcivW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ivSpec;
impl crate::RegisterSpec for Ucb0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0iv::R`](R) reader structure"]
impl crate::Readable for Ucb0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0iv::W`](W) writer structure"]
impl crate::Writable for Ucb0ivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0IV to value 0"]
impl crate::Resettable for Ucb0ivSpec {
    const RESET_VALUE: u16 = 0;
}
