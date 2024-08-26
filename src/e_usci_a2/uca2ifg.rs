#[doc = "Register `UCA2IFG` reader"]
pub type R = crate::R<Uca2ifgSpec>;
#[doc = "Register `UCA2IFG` writer"]
pub type W = crate::W<Uca2ifgSpec>;
#[doc = "0:0\\]
Receive interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxifg {
    #[doc = "0: No interrupt pending"]
    Ucrxifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucrxifg1 = 1,
}
impl From<Ucrxifg> for bool {
    #[inline(always)]
    fn from(variant: Ucrxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG` reader - 0:0\\]
Receive interrupt flag"]
pub type UcrxifgR = crate::BitReader<Ucrxifg>;
impl UcrxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxifg {
        match self.bits {
            false => Ucrxifg::Ucrxifg0,
            true => Ucrxifg::Ucrxifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg_0(&self) -> bool {
        *self == Ucrxifg::Ucrxifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucrxifg_1(&self) -> bool {
        *self == Ucrxifg::Ucrxifg1
    }
}
#[doc = "Field `UCRXIFG` writer - 0:0\\]
Receive interrupt flag"]
pub type UcrxifgW<'a, REG> = crate::BitWriter<'a, REG, Ucrxifg>;
impl<'a, REG> UcrxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg::Ucrxifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxifg::Ucrxifg1)
    }
}
#[doc = "1:1\\]
Transmit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxifg {
    #[doc = "0: No interrupt pending"]
    Uctxifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxifg1 = 1,
}
impl From<Uctxifg> for bool {
    #[inline(always)]
    fn from(variant: Uctxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG` reader - 1:1\\]
Transmit interrupt flag"]
pub type UctxifgR = crate::BitReader<Uctxifg>;
impl UctxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxifg {
        match self.bits {
            false => Uctxifg::Uctxifg0,
            true => Uctxifg::Uctxifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg_0(&self) -> bool {
        *self == Uctxifg::Uctxifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxifg_1(&self) -> bool {
        *self == Uctxifg::Uctxifg1
    }
}
#[doc = "Field `UCTXIFG` writer - 1:1\\]
Transmit interrupt flag"]
pub type UctxifgW<'a, REG> = crate::BitWriter<'a, REG, Uctxifg>;
impl<'a, REG> UctxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg::Uctxifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxifg::Uctxifg1)
    }
}
#[doc = "2:2\\]
Start bit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsttifg {
    #[doc = "0: No interrupt pending"]
    Ucsttifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ucsttifg1 = 1,
}
impl From<Ucsttifg> for bool {
    #[inline(always)]
    fn from(variant: Ucsttifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIFG` reader - 2:2\\]
Start bit interrupt flag"]
pub type UcsttifgR = crate::BitReader<Ucsttifg>;
impl UcsttifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsttifg {
        match self.bits {
            false => Ucsttifg::Ucsttifg0,
            true => Ucsttifg::Ucsttifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ucsttifg_0(&self) -> bool {
        *self == Ucsttifg::Ucsttifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ucsttifg_1(&self) -> bool {
        *self == Ucsttifg::Ucsttifg1
    }
}
#[doc = "Field `UCSTTIFG` writer - 2:2\\]
Start bit interrupt flag"]
pub type UcsttifgW<'a, REG> = crate::BitWriter<'a, REG, Ucsttifg>;
impl<'a, REG> UcsttifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttifg::Ucsttifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttifg::Ucsttifg1)
    }
}
#[doc = "3:3\\]
Transmit ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxcptifg {
    #[doc = "0: No interrupt pending"]
    Uctxcptifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Uctxcptifg1 = 1,
}
impl From<Uctxcptifg> for bool {
    #[inline(always)]
    fn from(variant: Uctxcptifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXCPTIFG` reader - 3:3\\]
Transmit ready interrupt enable"]
pub type UctxcptifgR = crate::BitReader<Uctxcptifg>;
impl UctxcptifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxcptifg {
        match self.bits {
            false => Uctxcptifg::Uctxcptifg0,
            true => Uctxcptifg::Uctxcptifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_uctxcptifg_0(&self) -> bool {
        *self == Uctxcptifg::Uctxcptifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_uctxcptifg_1(&self) -> bool {
        *self == Uctxcptifg::Uctxcptifg1
    }
}
#[doc = "Field `UCTXCPTIFG` writer - 3:3\\]
Transmit ready interrupt enable"]
pub type UctxcptifgW<'a, REG> = crate::BitWriter<'a, REG, Uctxcptifg>;
impl<'a, REG> UctxcptifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxcptifg::Uctxcptifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxcptifg::Uctxcptifg1)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UcrxifgR {
        UcrxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UctxifgR {
        UctxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Start bit interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UcsttifgR {
        UcsttifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit ready interrupt enable"]
    #[inline(always)]
    pub fn uctxcptifg(&self) -> UctxcptifgR {
        UctxcptifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UcrxifgW<Uca2ifgSpec> {
        UcrxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UctxifgW<Uca2ifgSpec> {
        UctxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Start bit interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttifg(&mut self) -> UcsttifgW<Uca2ifgSpec> {
        UcsttifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxcptifg(&mut self) -> UctxcptifgW<Uca2ifgSpec> {
        UctxcptifgW::new(self, 3)
    }
}
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca2ifgSpec;
impl crate::RegisterSpec for Uca2ifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca2ifg::R`](R) reader structure"]
impl crate::Readable for Uca2ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`uca2ifg::W`](W) writer structure"]
impl crate::Writable for Uca2ifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCA2IFG to value 0"]
impl crate::Resettable for Uca2ifgSpec {
    const RESET_VALUE: u16 = 0;
}
