#[doc = "Register `UCB0IFG_SPI` reader"]
pub type R = crate::R<Ucb0ifgSpiSpec>;
#[doc = "Register `UCB0IFG_SPI` writer"]
pub type W = crate::W<Ucb0ifgSpiSpec>;
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
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UcrxifgW<Ucb0ifgSpiSpec> {
        UcrxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UctxifgW<Ucb0ifgSpiSpec> {
        UctxifgW::new(self, 1)
    }
}
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0ifg_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0ifg_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0ifgSpiSpec;
impl crate::RegisterSpec for Ucb0ifgSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ifg_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0ifgSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0ifg_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0ifgSpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0IFG_SPI to value 0"]
impl crate::Resettable for Ucb0ifgSpiSpec {
    const RESET_VALUE: u16 = 0;
}
