#[doc = "Register `SYSUNIV` reader"]
pub type R = crate::R<SysunivSpec>;
#[doc = "Register `SYSUNIV` writer"]
pub type W = crate::W<SysunivSpec>;
#[doc = "15:0\\]
User NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sysuniv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: NMIIFG NMI pin"]
    Nmiifg = 2,
    #[doc = "4: OFIFG oscillator fault"]
    Ofifg = 4,
}
impl From<Sysuniv> for u16 {
    #[inline(always)]
    fn from(variant: Sysuniv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysuniv {
    type Ux = u16;
}
impl crate::IsEnum for Sysuniv {}
#[doc = "Field `SYSUNIV` reader - 15:0\\]
User NMI vector"]
pub type SysunivR = crate::FieldReader<Sysuniv>;
impl SysunivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysuniv> {
        match self.bits {
            0 => Some(Sysuniv::None),
            2 => Some(Sysuniv::Nmiifg),
            4 => Some(Sysuniv::Ofifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sysuniv::None
    }
    #[doc = "NMIIFG NMI pin"]
    #[inline(always)]
    pub fn is_nmiifg(&self) -> bool {
        *self == Sysuniv::Nmiifg
    }
    #[doc = "OFIFG oscillator fault"]
    #[inline(always)]
    pub fn is_ofifg(&self) -> bool {
        *self == Sysuniv::Ofifg
    }
}
#[doc = "Field `SYSUNIV` writer - 15:0\\]
User NMI vector"]
pub type SysunivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Sysuniv>;
impl<'a, REG> SysunivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sysuniv::None)
    }
    #[doc = "NMIIFG NMI pin"]
    #[inline(always)]
    pub fn nmiifg(self) -> &'a mut crate::W<REG> {
        self.variant(Sysuniv::Nmiifg)
    }
    #[doc = "OFIFG oscillator fault"]
    #[inline(always)]
    pub fn ofifg(self) -> &'a mut crate::W<REG> {
        self.variant(Sysuniv::Ofifg)
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&self) -> SysunivR {
        SysunivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User NMI vector"]
    #[inline(always)]
    #[must_use]
    pub fn sysuniv(&mut self) -> SysunivW<SysunivSpec> {
        SysunivW::new(self, 0)
    }
}
#[doc = "User NMI Vector Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysuniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysuniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysunivSpec;
impl crate::RegisterSpec for SysunivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysuniv::R`](R) reader structure"]
impl crate::Readable for SysunivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysuniv::W`](W) writer structure"]
impl crate::Writable for SysunivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SYSUNIV to value 0"]
impl crate::Resettable for SysunivSpec {
    const RESET_VALUE: u16 = 0;
}
