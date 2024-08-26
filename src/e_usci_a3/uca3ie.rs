#[doc = "Register `UCA3IE` reader"]
pub type R = crate::R<Uca3ieSpec>;
#[doc = "Register `UCA3IE` writer"]
pub type W = crate::W<Uca3ieSpec>;
#[doc = "0:0\\]
Receive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxie {
    #[doc = "0: Interrupt disabled"]
    Ucrxie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucrxie1 = 1,
}
impl From<Ucrxie> for bool {
    #[inline(always)]
    fn from(variant: Ucrxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE` reader - 0:0\\]
Receive interrupt enable"]
pub type UcrxieR = crate::BitReader<Ucrxie>;
impl UcrxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxie {
        match self.bits {
            false => Ucrxie::Ucrxie0,
            true => Ucrxie::Ucrxie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucrxie_0(&self) -> bool {
        *self == Ucrxie::Ucrxie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucrxie_1(&self) -> bool {
        *self == Ucrxie::Ucrxie1
    }
}
#[doc = "Field `UCRXIE` writer - 0:0\\]
Receive interrupt enable"]
pub type UcrxieW<'a, REG> = crate::BitWriter<'a, REG, Ucrxie>;
impl<'a, REG> UcrxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie::Ucrxie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxie::Ucrxie1)
    }
}
#[doc = "1:1\\]
Transmit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxie {
    #[doc = "0: Interrupt disabled"]
    Uctxie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxie1 = 1,
}
impl From<Uctxie> for bool {
    #[inline(always)]
    fn from(variant: Uctxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE` reader - 1:1\\]
Transmit interrupt enable"]
pub type UctxieR = crate::BitReader<Uctxie>;
impl UctxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxie {
        match self.bits {
            false => Uctxie::Uctxie0,
            true => Uctxie::Uctxie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxie_0(&self) -> bool {
        *self == Uctxie::Uctxie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxie_1(&self) -> bool {
        *self == Uctxie::Uctxie1
    }
}
#[doc = "Field `UCTXIE` writer - 1:1\\]
Transmit interrupt enable"]
pub type UctxieW<'a, REG> = crate::BitWriter<'a, REG, Uctxie>;
impl<'a, REG> UctxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie::Uctxie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxie::Uctxie1)
    }
}
#[doc = "2:2\\]
Start bit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsttie {
    #[doc = "0: Interrupt disabled"]
    Ucsttie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ucsttie1 = 1,
}
impl From<Ucsttie> for bool {
    #[inline(always)]
    fn from(variant: Ucsttie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIE` reader - 2:2\\]
Start bit interrupt enable"]
pub type UcsttieR = crate::BitReader<Ucsttie>;
impl UcsttieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsttie {
        match self.bits {
            false => Ucsttie::Ucsttie0,
            true => Ucsttie::Ucsttie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ucsttie_0(&self) -> bool {
        *self == Ucsttie::Ucsttie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ucsttie_1(&self) -> bool {
        *self == Ucsttie::Ucsttie1
    }
}
#[doc = "Field `UCSTTIE` writer - 2:2\\]
Start bit interrupt enable"]
pub type UcsttieW<'a, REG> = crate::BitWriter<'a, REG, Ucsttie>;
impl<'a, REG> UcsttieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucsttie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttie::Ucsttie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucsttie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsttie::Ucsttie1)
    }
}
#[doc = "3:3\\]
Transmit complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxcptie {
    #[doc = "0: Interrupt disabled"]
    Uctxcptie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Uctxcptie1 = 1,
}
impl From<Uctxcptie> for bool {
    #[inline(always)]
    fn from(variant: Uctxcptie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXCPTIE` reader - 3:3\\]
Transmit complete interrupt enable"]
pub type UctxcptieR = crate::BitReader<Uctxcptie>;
impl UctxcptieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxcptie {
        match self.bits {
            false => Uctxcptie::Uctxcptie0,
            true => Uctxcptie::Uctxcptie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_uctxcptie_0(&self) -> bool {
        *self == Uctxcptie::Uctxcptie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_uctxcptie_1(&self) -> bool {
        *self == Uctxcptie::Uctxcptie1
    }
}
#[doc = "Field `UCTXCPTIE` writer - 3:3\\]
Transmit complete interrupt enable"]
pub type UctxcptieW<'a, REG> = crate::BitWriter<'a, REG, Uctxcptie>;
impl<'a, REG> UctxcptieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxcptie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxcptie::Uctxcptie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxcptie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxcptie::Uctxcptie1)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UcrxieR {
        UcrxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UctxieR {
        UctxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UcsttieR {
        UcsttieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&self) -> UctxcptieR {
        UctxcptieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie(&mut self) -> UcrxieW<Uca3ieSpec> {
        UcrxieW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie(&mut self) -> UctxieW<Uca3ieSpec> {
        UctxieW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Start bit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttie(&mut self) -> UcsttieW<Uca3ieSpec> {
        UcsttieW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxcptie(&mut self) -> UctxcptieW<Uca3ieSpec> {
        UctxcptieW::new(self, 3)
    }
}
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca3ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca3ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca3ieSpec;
impl crate::RegisterSpec for Uca3ieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca3ie::R`](R) reader structure"]
impl crate::Readable for Uca3ieSpec {}
#[doc = "`write(|w| ..)` method takes [`uca3ie::W`](W) writer structure"]
impl crate::Writable for Uca3ieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCA3IE to value 0"]
impl crate::Resettable for Uca3ieSpec {
    const RESET_VALUE: u16 = 0;
}
