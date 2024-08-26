#[doc = "Register `UCB3IV_SPI` reader"]
pub type R = crate::R<Ucb3ivSpiSpec>;
#[doc = "Register `UCB3IV_SPI` writer"]
pub type W = crate::W<Ucb3ivSpiSpec>;
#[doc = "15:0\\]
eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Uciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    Ucrxifg = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    Uctxifg = 4,
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
            2 => Some(Uciv::Ucrxifg),
            4 => Some(Uciv::Uctxifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Uciv::None
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == Uciv::Ucrxifg
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == Uciv::Uctxifg
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
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucrxifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Ucrxifg)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn uctxifg(self) -> &'a mut crate::W<REG> {
        self.variant(Uciv::Uctxifg)
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
    pub fn uciv(&mut self) -> UcivW<Ucb3ivSpiSpec> {
        UcivW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3iv_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3iv_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3ivSpiSpec;
impl crate::RegisterSpec for Ucb3ivSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3iv_spi::R`](R) reader structure"]
impl crate::Readable for Ucb3ivSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3iv_spi::W`](W) writer structure"]
impl crate::Writable for Ucb3ivSpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB3IV_SPI to value 0"]
impl crate::Resettable for Ucb3ivSpiSpec {
    const RESET_VALUE: u16 = 0;
}
