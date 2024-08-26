#[doc = "Register `UCB0STATW` reader"]
pub type R = crate::R<Ucb0statwSpec>;
#[doc = "Register `UCB0STATW` writer"]
pub type W = crate::W<Ucb0statwSpec>;
#[doc = "4:4\\]
Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbbusy {
    #[doc = "0: Bus inactive"]
    Idle = 0,
    #[doc = "1: Bus busy"]
    Busy = 1,
}
impl From<Ucbbusy> for bool {
    #[inline(always)]
    fn from(variant: Ucbbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBBUSY` reader - 4:4\\]
Bus busy"]
pub type UcbbusyR = crate::BitReader<Ucbbusy>;
impl UcbbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbbusy {
        match self.bits {
            false => Ucbbusy::Idle,
            true => Ucbbusy::Busy,
        }
    }
    #[doc = "Bus inactive"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Ucbbusy::Idle
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ucbbusy::Busy
    }
}
#[doc = "Field `UCBBUSY` writer - 4:4\\]
Bus busy"]
pub type UcbbusyW<'a, REG> = crate::BitWriter<'a, REG, Ucbbusy>;
impl<'a, REG> UcbbusyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus inactive"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbbusy::Idle)
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbbusy::Busy)
    }
}
#[doc = "5:5\\]
General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucgc {
    #[doc = "0: No general call address received"]
    Ucgc0 = 0,
    #[doc = "1: General call address received"]
    Ucgc1 = 1,
}
impl From<Ucgc> for bool {
    #[inline(always)]
    fn from(variant: Ucgc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCGC` reader - 5:5\\]
General call address received"]
pub type UcgcR = crate::BitReader<Ucgc>;
impl UcgcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucgc {
        match self.bits {
            false => Ucgc::Ucgc0,
            true => Ucgc::Ucgc1,
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == Ucgc::Ucgc0
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == Ucgc::Ucgc1
    }
}
#[doc = "Field `UCGC` writer - 5:5\\]
General call address received"]
pub type UcgcW<'a, REG> = crate::BitWriter<'a, REG, Ucgc>;
impl<'a, REG> UcgcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn ucgc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucgc::Ucgc0)
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn ucgc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucgc::Ucgc1)
    }
}
#[doc = "6:6\\]
SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucscllow {
    #[doc = "0: SCL is not held low"]
    Ucscllow0 = 0,
    #[doc = "1: SCL is held low"]
    Ucscllow1 = 1,
}
impl From<Ucscllow> for bool {
    #[inline(always)]
    fn from(variant: Ucscllow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSCLLOW` reader - 6:6\\]
SCL low"]
pub type UcscllowR = crate::BitReader<Ucscllow>;
impl UcscllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucscllow {
        match self.bits {
            false => Ucscllow::Ucscllow0,
            true => Ucscllow::Ucscllow1,
        }
    }
    #[doc = "SCL is not held low"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == Ucscllow::Ucscllow0
    }
    #[doc = "SCL is held low"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == Ucscllow::Ucscllow1
    }
}
#[doc = "Field `UCSCLLOW` writer - 6:6\\]
SCL low"]
pub type UcscllowW<'a, REG> = crate::BitWriter<'a, REG, Ucscllow>;
impl<'a, REG> UcscllowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCL is not held low"]
    #[inline(always)]
    pub fn ucscllow_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucscllow::Ucscllow0)
    }
    #[doc = "SCL is held low"]
    #[inline(always)]
    pub fn ucscllow_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucscllow::Ucscllow1)
    }
}
#[doc = "Field `UCBCNT` reader - 15:8\\]
Hardware byte counter value"]
pub type UcbcntR = crate::FieldReader;
#[doc = "Field `UCBCNT` writer - 15:8\\]
Hardware byte counter value"]
pub type UcbcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UcbbusyR {
        UcbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UcgcR {
        UcgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UcscllowR {
        UcscllowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UcbcntR {
        UcbcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Bus busy"]
    #[inline(always)]
    #[must_use]
    pub fn ucbbusy(&mut self) -> UcbbusyW<Ucb0statwSpec> {
        UcbbusyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
General call address received"]
    #[inline(always)]
    #[must_use]
    pub fn ucgc(&mut self) -> UcgcW<Ucb0statwSpec> {
        UcgcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
SCL low"]
    #[inline(always)]
    #[must_use]
    pub fn ucscllow(&mut self) -> UcscllowW<Ucb0statwSpec> {
        UcscllowW::new(self, 6)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Hardware byte counter value"]
    #[inline(always)]
    #[must_use]
    pub fn ucbcnt(&mut self) -> UcbcntW<Ucb0statwSpec> {
        UcbcntW::new(self, 8)
    }
}
#[doc = "eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0statw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0statw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0statwSpec;
impl crate::RegisterSpec for Ucb0statwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0statw::R`](R) reader structure"]
impl crate::Readable for Ucb0statwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0statw::W`](W) writer structure"]
impl crate::Writable for Ucb0statwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB0STATW to value 0"]
impl crate::Resettable for Ucb0statwSpec {
    const RESET_VALUE: u16 = 0;
}
