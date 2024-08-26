#[doc = "Register `MPUIPC0` reader"]
pub type R = crate::R<Mpuipc0Spec>;
#[doc = "Register `MPUIPC0` writer"]
pub type W = crate::W<Mpuipc0Spec>;
#[doc = "5:5\\]
MPU IP Encapsulation segment Violation Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuipvs {
    #[doc = "0: Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    Mpuipvs0 = 0,
    #[doc = "1: Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC"]
    Mpuipvs1 = 1,
}
impl From<Mpuipvs> for bool {
    #[inline(always)]
    fn from(variant: Mpuipvs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUIPVS` reader - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
pub type MpuipvsR = crate::BitReader<Mpuipvs>;
impl MpuipvsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuipvs {
        match self.bits {
            false => Mpuipvs::Mpuipvs0,
            true => Mpuipvs::Mpuipvs1,
        }
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn is_mpuipvs_0(&self) -> bool {
        *self == Mpuipvs::Mpuipvs0
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn is_mpuipvs_1(&self) -> bool {
        *self == Mpuipvs::Mpuipvs1
    }
}
#[doc = "Field `MPUIPVS` writer - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
pub type MpuipvsW<'a, REG> = crate::BitWriter<'a, REG, Mpuipvs>;
impl<'a, REG> MpuipvsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a SNMI if enabled by MPUSEGIE = 1"]
    #[inline(always)]
    pub fn mpuipvs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuipvs::Mpuipvs0)
    }
    #[doc = "Violation in Main Memory Segment 1 asserts the MPUSEGPIFG bit and executes a PUC"]
    #[inline(always)]
    pub fn mpuipvs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuipvs::Mpuipvs1)
    }
}
#[doc = "6:6\\]
MPU IP Encapsulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuipena {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Enabled"]
    Enable = 1,
}
impl From<Mpuipena> for bool {
    #[inline(always)]
    fn from(variant: Mpuipena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUIPENA` reader - 6:6\\]
MPU IP Encapsulation Enable"]
pub type MpuipenaR = crate::BitReader<Mpuipena>;
impl MpuipenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuipena {
        match self.bits {
            false => Mpuipena::Disable,
            true => Mpuipena::Enable,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpuipena::Disable
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpuipena::Enable
    }
}
#[doc = "Field `MPUIPENA` writer - 6:6\\]
MPU IP Encapsulation Enable"]
pub type MpuipenaW<'a, REG> = crate::BitWriter<'a, REG, Mpuipena>;
impl<'a, REG> MpuipenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuipena::Disable)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuipena::Enable)
    }
}
#[doc = "7:7\\]
MPU IP Encapsulation Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpuiplock {
    #[doc = "0: Open"]
    Open = 0,
    #[doc = "1: Locked"]
    Lock = 1,
}
impl From<Mpuiplock> for bool {
    #[inline(always)]
    fn from(variant: Mpuiplock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPUIPLOCK` reader - 7:7\\]
MPU IP Encapsulation Lock"]
pub type MpuiplockR = crate::BitReader<Mpuiplock>;
impl MpuiplockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpuiplock {
        match self.bits {
            false => Mpuiplock::Open,
            true => Mpuiplock::Lock,
        }
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == Mpuiplock::Open
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Mpuiplock::Lock
    }
}
#[doc = "Field `MPUIPLOCK` writer - 7:7\\]
MPU IP Encapsulation Lock"]
pub type MpuiplockW<'a, REG> = crate::BitWriter<'a, REG, Mpuiplock>;
impl<'a, REG> MpuiplockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuiplock::Open)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Mpuiplock::Lock)
    }
}
impl R {
    #[doc = "Bit 5 - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
    #[inline(always)]
    pub fn mpuipvs(&self) -> MpuipvsR {
        MpuipvsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MPU IP Encapsulation Enable"]
    #[inline(always)]
    pub fn mpuipena(&self) -> MpuipenaR {
        MpuipenaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
MPU IP Encapsulation Lock"]
    #[inline(always)]
    pub fn mpuiplock(&self) -> MpuiplockR {
        MpuiplockR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
MPU IP Encapsulation segment Violation Select"]
    #[inline(always)]
    #[must_use]
    pub fn mpuipvs(&mut self) -> MpuipvsW<Mpuipc0Spec> {
        MpuipvsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
MPU IP Encapsulation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpuipena(&mut self) -> MpuipenaW<Mpuipc0Spec> {
        MpuipenaW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
MPU IP Encapsulation Lock"]
    #[inline(always)]
    #[must_use]
    pub fn mpuiplock(&mut self) -> MpuiplockW<Mpuipc0Spec> {
        MpuiplockW::new(self, 7)
    }
}
#[doc = "Memory Protection Unit IP Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mpuipc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpuipc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpuipc0Spec;
impl crate::RegisterSpec for Mpuipc0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpuipc0::R`](R) reader structure"]
impl crate::Readable for Mpuipc0Spec {}
#[doc = "`write(|w| ..)` method takes [`mpuipc0::W`](W) writer structure"]
impl crate::Writable for Mpuipc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MPUIPC0 to value 0"]
impl crate::Resettable for Mpuipc0Spec {
    const RESET_VALUE: u16 = 0;
}
