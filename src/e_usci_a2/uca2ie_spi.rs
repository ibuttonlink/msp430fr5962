#[doc = "Register `UCA2IE_SPI` reader"]
pub type R = crate::R<Uca2ieSpiSpec>;
#[doc = "Register `UCA2IE_SPI` writer"]
pub type W = crate::W<Uca2ieSpiSpec>;
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
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie(&mut self) -> UcrxieW<Uca2ieSpiSpec> {
        UcrxieW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie(&mut self) -> UctxieW<Uca2ieSpiSpec> {
        UctxieW::new(self, 1)
    }
}
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2ie_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2ie_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca2ieSpiSpec;
impl crate::RegisterSpec for Uca2ieSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca2ie_spi::R`](R) reader structure"]
impl crate::Readable for Uca2ieSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca2ie_spi::W`](W) writer structure"]
impl crate::Writable for Uca2ieSpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCA2IE_SPI to value 0"]
impl crate::Resettable for Uca2ieSpiSpec {
    const RESET_VALUE: u16 = 0;
}
