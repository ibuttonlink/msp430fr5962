#[doc = "Register `RTCTIM0_BCD` reader"]
pub type R = crate::R<Rtctim0BcdSpec>;
#[doc = "Register `RTCTIM0_BCD` writer"]
pub type W = crate::W<Rtctim0BcdSpec>;
#[doc = "Field `SECONDSLOWDIGIT` reader - 3:0\\]
Seconds low digit (0 to 9)"]
pub type SecondslowdigitR = crate::FieldReader;
#[doc = "Field `SECONDSLOWDIGIT` writer - 3:0\\]
Seconds low digit (0 to 9)"]
pub type SecondslowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECONDSHIGHDIGIT` reader - 6:4\\]
Seconds high digit (0 to 5)"]
pub type SecondshighdigitR = crate::FieldReader;
#[doc = "Field `SECONDSHIGHDIGIT` writer - 6:4\\]
Seconds high digit (0 to 5)"]
pub type SecondshighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINUTESLOWDIGIT` reader - 11:8\\]
Minutes low digit (0 to 9)"]
pub type MinuteslowdigitR = crate::FieldReader;
#[doc = "Field `MINUTESLOWDIGIT` writer - 11:8\\]
Minutes low digit (0 to 9)"]
pub type MinuteslowdigitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINUTESHIGHDIGIT` reader - 14:12\\]
Minutes high digit (0 to 5)"]
pub type MinuteshighdigitR = crate::FieldReader;
#[doc = "Field `MINUTESHIGHDIGIT` writer - 14:12\\]
Minutes high digit (0 to 5)"]
pub type MinuteshighdigitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Seconds low digit (0 to 9)"]
    #[inline(always)]
    pub fn secondslowdigit(&self) -> SecondslowdigitR {
        SecondslowdigitR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Seconds high digit (0 to 5)"]
    #[inline(always)]
    pub fn secondshighdigit(&self) -> SecondshighdigitR {
        SecondshighdigitR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    pub fn minuteslowdigit(&self) -> MinuteslowdigitR {
        MinuteslowdigitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    pub fn minuteshighdigit(&self) -> MinuteshighdigitR {
        MinuteshighdigitR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Seconds low digit (0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn secondslowdigit(&mut self) -> SecondslowdigitW<Rtctim0BcdSpec> {
        SecondslowdigitW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Seconds high digit (0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn secondshighdigit(&mut self) -> SecondshighdigitW<Rtctim0BcdSpec> {
        SecondshighdigitW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Minutes low digit (0 to 9)"]
    #[inline(always)]
    #[must_use]
    pub fn minuteslowdigit(&mut self) -> MinuteslowdigitW<Rtctim0BcdSpec> {
        MinuteslowdigitW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Minutes high digit (0 to 5)"]
    #[inline(always)]
    #[must_use]
    pub fn minuteshighdigit(&mut self) -> MinuteshighdigitW<Rtctim0BcdSpec> {
        MinuteshighdigitW::new(self, 12)
    }
}
#[doc = "Real-Time Clock Seconds, Minutes Register - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim0_bcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim0_bcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtctim0BcdSpec;
impl crate::RegisterSpec for Rtctim0BcdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtctim0_bcd::R`](R) reader structure"]
impl crate::Readable for Rtctim0BcdSpec {}
#[doc = "`write(|w| ..)` method takes [`rtctim0_bcd::W`](W) writer structure"]
impl crate::Writable for Rtctim0BcdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCTIM0_BCD to value 0"]
impl crate::Resettable for Rtctim0BcdSpec {
    const RESET_VALUE: u16 = 0;
}
