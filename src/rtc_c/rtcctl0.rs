#[doc = "Register `RTCCTL0` reader"]
pub type R = crate::R<Rtcctl0Spec>;
#[doc = "Register `RTCCTL0` writer"]
pub type W = crate::W<Rtcctl0Spec>;
#[doc = "Field `RTCRDYIFG` reader - 0:0\\]
Real-time clock ready interrupt flag"]
pub type RtcrdyifgR = crate::BitReader;
#[doc = "Field `RTCRDYIFG` writer - 0:0\\]
Real-time clock ready interrupt flag"]
pub type RtcrdyifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAIFG` reader - 1:1\\]
Real-time clock alarm interrupt flag"]
pub type RtcaifgR = crate::BitReader;
#[doc = "Field `RTCAIFG` writer - 1:1\\]
Real-time clock alarm interrupt flag"]
pub type RtcaifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTEVIFG` reader - 2:2\\]
Real-time clock time event interrupt flag"]
pub type RtctevifgR = crate::BitReader;
#[doc = "Field `RTCTEVIFG` writer - 2:2\\]
Real-time clock time event interrupt flag"]
pub type RtctevifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCOFIFG` reader - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
pub type RtcofifgR = crate::BitReader;
#[doc = "Field `RTCOFIFG` writer - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
pub type RtcofifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCRDYIE` reader - 4:4\\]
Real-time clock ready interrupt enable"]
pub type RtcrdyieR = crate::BitReader;
#[doc = "Field `RTCRDYIE` writer - 4:4\\]
Real-time clock ready interrupt enable"]
pub type RtcrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAIE` reader - 5:5\\]
Real-time clock alarm interrupt enable"]
pub type RtcaieR = crate::BitReader;
#[doc = "Field `RTCAIE` writer - 5:5\\]
Real-time clock alarm interrupt enable"]
pub type RtcaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCTEVIE` reader - 6:6\\]
Real-time clock time event interrupt enable"]
pub type RtctevieR = crate::BitReader;
#[doc = "Field `RTCTEVIE` writer - 6:6\\]
Real-time clock time event interrupt enable"]
pub type RtctevieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCOFIE` reader - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
pub type RtcofieR = crate::BitReader;
#[doc = "Field `RTCOFIE` writer - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
pub type RtcofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCKEY` reader - 15:8\\]
Real-time clock key"]
pub type RtckeyR = crate::FieldReader;
#[doc = "Field `RTCKEY` writer - 15:8\\]
Real-time clock key"]
pub type RtckeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RtcrdyifgR {
        RtcrdyifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RtcaifgR {
        RtcaifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RtctevifgR {
        RtctevifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RtcofifgR {
        RtcofifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RtcrdyieR {
        RtcrdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RtcaieR {
        RtcaieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RtctevieR {
        RtctevieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RtcofieR {
        RtcofieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&self) -> RtckeyR {
        RtckeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Real-time clock ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyifg(&mut self) -> RtcrdyifgW<Rtcctl0Spec> {
        RtcrdyifgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Real-time clock alarm interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaifg(&mut self) -> RtcaifgW<Rtcctl0Spec> {
        RtcaifgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Real-time clock time event interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevifg(&mut self) -> RtctevifgW<Rtcctl0Spec> {
        RtctevifgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcofifg(&mut self) -> RtcofifgW<Rtcctl0Spec> {
        RtcofifgW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Real-time clock ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrdyie(&mut self) -> RtcrdyieW<Rtcctl0Spec> {
        RtcrdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Real-time clock alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcaie(&mut self) -> RtcaieW<Rtcctl0Spec> {
        RtcaieW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Real-time clock time event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtctevie(&mut self) -> RtctevieW<Rtcctl0Spec> {
        RtctevieW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcofie(&mut self) -> RtcofieW<Rtcctl0Spec> {
        RtcofieW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Real-time clock key"]
    #[inline(always)]
    #[must_use]
    pub fn rtckey(&mut self) -> RtckeyW<Rtcctl0Spec> {
        RtckeyW::new(self, 8)
    }
}
#[doc = "RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtcctl0Spec;
impl crate::RegisterSpec for Rtcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl0::R`](R) reader structure"]
impl crate::Readable for Rtcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl0::W`](W) writer structure"]
impl crate::Writable for Rtcctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RTCCTL0 to value 0"]
impl crate::Resettable for Rtcctl0Spec {
    const RESET_VALUE: u16 = 0;
}
