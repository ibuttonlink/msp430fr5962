#[doc = "Register `PMMIFG` reader"]
pub type R = crate::R<PmmifgSpec>;
#[doc = "Register `PMMIFG` writer"]
pub type W = crate::W<PmmifgSpec>;
#[doc = "8:8\\]
PMM software brownout reset interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmborifg {
    #[doc = "0: Reset not due to PMMSWBOR"]
    Pmmborifg0 = 0,
    #[doc = "1: Reset due to PMMSWBOR"]
    Pmmborifg1 = 1,
}
impl From<Pmmborifg> for bool {
    #[inline(always)]
    fn from(variant: Pmmborifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMBORIFG` reader - 8:8\\]
PMM software brownout reset interrupt flag."]
pub type PmmborifgR = crate::BitReader<Pmmborifg>;
impl PmmborifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmborifg {
        match self.bits {
            false => Pmmborifg::Pmmborifg0,
            true => Pmmborifg::Pmmborifg1,
        }
    }
    #[doc = "Reset not due to PMMSWBOR"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == Pmmborifg::Pmmborifg0
    }
    #[doc = "Reset due to PMMSWBOR"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == Pmmborifg::Pmmborifg1
    }
}
#[doc = "Field `PMMBORIFG` writer - 8:8\\]
PMM software brownout reset interrupt flag."]
pub type PmmborifgW<'a, REG> = crate::BitWriter<'a, REG, Pmmborifg>;
impl<'a, REG> PmmborifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmborifg::Pmmborifg0)
    }
    #[doc = "Reset due to PMMSWBOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmborifg::Pmmborifg1)
    }
}
#[doc = "9:9\\]
PMM reset pin interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmrstifg {
    #[doc = "0: Reset not due to reset pin"]
    Pmmborifg0 = 0,
    #[doc = "1: Reset due to reset pin"]
    Pmmborifg1 = 1,
}
impl From<Pmmrstifg> for bool {
    #[inline(always)]
    fn from(variant: Pmmrstifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMRSTIFG` reader - 9:9\\]
PMM reset pin interrupt flag."]
pub type PmmrstifgR = crate::BitReader<Pmmrstifg>;
impl PmmrstifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmrstifg {
        match self.bits {
            false => Pmmrstifg::Pmmborifg0,
            true => Pmmrstifg::Pmmborifg1,
        }
    }
    #[doc = "Reset not due to reset pin"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == Pmmrstifg::Pmmborifg0
    }
    #[doc = "Reset due to reset pin"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == Pmmrstifg::Pmmborifg1
    }
}
#[doc = "Field `PMMRSTIFG` writer - 9:9\\]
PMM reset pin interrupt flag."]
pub type PmmrstifgW<'a, REG> = crate::BitWriter<'a, REG, Pmmrstifg>;
impl<'a, REG> PmmrstifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmrstifg::Pmmborifg0)
    }
    #[doc = "Reset due to reset pin"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmrstifg::Pmmborifg1)
    }
}
#[doc = "10:10\\]
PMM software POR interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmporifg {
    #[doc = "0: Reset not due to PMMSWPOR"]
    Pmmborifg0 = 0,
    #[doc = "1: Reset due to PMMSWPOR"]
    Pmmborifg1 = 1,
}
impl From<Pmmporifg> for bool {
    #[inline(always)]
    fn from(variant: Pmmporifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMPORIFG` reader - 10:10\\]
PMM software POR interrupt flag."]
pub type PmmporifgR = crate::BitReader<Pmmporifg>;
impl PmmporifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmporifg {
        match self.bits {
            false => Pmmporifg::Pmmborifg0,
            true => Pmmporifg::Pmmborifg1,
        }
    }
    #[doc = "Reset not due to PMMSWPOR"]
    #[inline(always)]
    pub fn is_pmmborifg_0(&self) -> bool {
        *self == Pmmporifg::Pmmborifg0
    }
    #[doc = "Reset due to PMMSWPOR"]
    #[inline(always)]
    pub fn is_pmmborifg_1(&self) -> bool {
        *self == Pmmporifg::Pmmborifg1
    }
}
#[doc = "Field `PMMPORIFG` writer - 10:10\\]
PMM software POR interrupt flag."]
pub type PmmporifgW<'a, REG> = crate::BitWriter<'a, REG, Pmmporifg>;
impl<'a, REG> PmmporifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmporifg::Pmmborifg0)
    }
    #[doc = "Reset due to PMMSWPOR"]
    #[inline(always)]
    pub fn pmmborifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmporifg::Pmmborifg1)
    }
}
#[doc = "13:13\\]
High-side SVS interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svshifg {
    #[doc = "0: Reset not due to SVSH"]
    Svshifg0 = 0,
    #[doc = "1: Reset due to SVSH"]
    Svshifg1 = 1,
}
impl From<Svshifg> for bool {
    #[inline(always)]
    fn from(variant: Svshifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSHIFG` reader - 13:13\\]
High-side SVS interrupt flag."]
pub type SvshifgR = crate::BitReader<Svshifg>;
impl SvshifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svshifg {
        match self.bits {
            false => Svshifg::Svshifg0,
            true => Svshifg::Svshifg1,
        }
    }
    #[doc = "Reset not due to SVSH"]
    #[inline(always)]
    pub fn is_svshifg_0(&self) -> bool {
        *self == Svshifg::Svshifg0
    }
    #[doc = "Reset due to SVSH"]
    #[inline(always)]
    pub fn is_svshifg_1(&self) -> bool {
        *self == Svshifg::Svshifg1
    }
}
#[doc = "Field `SVSHIFG` writer - 13:13\\]
High-side SVS interrupt flag."]
pub type SvshifgW<'a, REG> = crate::BitWriter<'a, REG, Svshifg>;
impl<'a, REG> SvshifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not due to SVSH"]
    #[inline(always)]
    pub fn svshifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Svshifg::Svshifg0)
    }
    #[doc = "Reset due to SVSH"]
    #[inline(always)]
    pub fn svshifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Svshifg::Svshifg1)
    }
}
#[doc = "15:15\\]
LPMx.5 flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmlpm5ifg {
    #[doc = "0: Reset not due to wake-up from LPMx.5"]
    Pmmlpm5ifg0 = 0,
    #[doc = "1: Reset due to wake-up from LPMx.5"]
    Pmmlpm5ifg1 = 1,
}
impl From<Pmmlpm5ifg> for bool {
    #[inline(always)]
    fn from(variant: Pmmlpm5ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMLPM5IFG` reader - 15:15\\]
LPMx.5 flag."]
pub type Pmmlpm5ifgR = crate::BitReader<Pmmlpm5ifg>;
impl Pmmlpm5ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmlpm5ifg {
        match self.bits {
            false => Pmmlpm5ifg::Pmmlpm5ifg0,
            true => Pmmlpm5ifg::Pmmlpm5ifg1,
        }
    }
    #[doc = "Reset not due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_0(&self) -> bool {
        *self == Pmmlpm5ifg::Pmmlpm5ifg0
    }
    #[doc = "Reset due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn is_pmmlpm5ifg_1(&self) -> bool {
        *self == Pmmlpm5ifg::Pmmlpm5ifg1
    }
}
#[doc = "Field `PMMLPM5IFG` writer - 15:15\\]
LPMx.5 flag."]
pub type Pmmlpm5ifgW<'a, REG> = crate::BitWriter<'a, REG, Pmmlpm5ifg>;
impl<'a, REG> Pmmlpm5ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmlpm5ifg::Pmmlpm5ifg0)
    }
    #[doc = "Reset due to wake-up from LPMx.5"]
    #[inline(always)]
    pub fn pmmlpm5ifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmlpm5ifg::Pmmlpm5ifg1)
    }
}
impl R {
    #[doc = "Bit 8 - 8:8\\]
PMM software brownout reset interrupt flag."]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PmmborifgR {
        PmmborifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
PMM reset pin interrupt flag."]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PmmrstifgR {
        PmmrstifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
PMM software POR interrupt flag."]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PmmporifgR {
        PmmporifgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
High-side SVS interrupt flag."]
    #[inline(always)]
    pub fn svshifg(&self) -> SvshifgR {
        SvshifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
LPMx.5 flag."]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> Pmmlpm5ifgR {
        Pmmlpm5ifgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
PMM software brownout reset interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn pmmborifg(&mut self) -> PmmborifgW<PmmifgSpec> {
        PmmborifgW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
PMM reset pin interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn pmmrstifg(&mut self) -> PmmrstifgW<PmmifgSpec> {
        PmmrstifgW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
PMM software POR interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn pmmporifg(&mut self) -> PmmporifgW<PmmifgSpec> {
        PmmporifgW::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
High-side SVS interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn svshifg(&mut self) -> SvshifgW<PmmifgSpec> {
        SvshifgW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
LPMx.5 flag."]
    #[inline(always)]
    #[must_use]
    pub fn pmmlpm5ifg(&mut self) -> Pmmlpm5ifgW<PmmifgSpec> {
        Pmmlpm5ifgW::new(self, 15)
    }
}
#[doc = "PMM interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmmifgSpec;
impl crate::RegisterSpec for PmmifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmifg::R`](R) reader structure"]
impl crate::Readable for PmmifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmmifg::W`](W) writer structure"]
impl crate::Writable for PmmifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMMIFG to value 0"]
impl crate::Resettable for PmmifgSpec {
    const RESET_VALUE: u16 = 0;
}
