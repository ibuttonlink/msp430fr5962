#[doc = "Register `PDIFG` reader"]
pub type R = crate::R<PdifgSpec>;
#[doc = "Register `PDIFG` writer"]
pub type W = crate::W<PdifgSpec>;
#[doc = "Field `P0` reader - P0"]
pub type P0R = crate::BitReader;
#[doc = "Field `P0` writer - P0"]
pub type P0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1` reader - P1"]
pub type P1R = crate::BitReader;
#[doc = "Field `P1` writer - P1"]
pub type P1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P2` reader - P2"]
pub type P2R = crate::BitReader;
#[doc = "Field `P2` writer - P2"]
pub type P2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3` reader - P3"]
pub type P3R = crate::BitReader;
#[doc = "Field `P3` writer - P3"]
pub type P3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4` reader - P4"]
pub type P4R = crate::BitReader;
#[doc = "Field `P4` writer - P4"]
pub type P4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5` reader - P5"]
pub type P5R = crate::BitReader;
#[doc = "Field `P5` writer - P5"]
pub type P5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P6` reader - P6"]
pub type P6R = crate::BitReader;
#[doc = "Field `P6` writer - P6"]
pub type P6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7` reader - P7"]
pub type P7R = crate::BitReader;
#[doc = "Field `P7` writer - P7"]
pub type P7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P8` reader - P8"]
pub type P8R = crate::BitReader;
#[doc = "Field `P8` writer - P8"]
pub type P8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P9` reader - P9"]
pub type P9R = crate::BitReader;
#[doc = "Field `P9` writer - P9"]
pub type P9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P10` reader - P10"]
pub type P10R = crate::BitReader;
#[doc = "Field `P10` writer - P10"]
pub type P10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` reader - P11"]
pub type P11R = crate::BitReader;
#[doc = "Field `P11` writer - P11"]
pub type P11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` reader - P12"]
pub type P12R = crate::BitReader;
#[doc = "Field `P12` writer - P12"]
pub type P12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` reader - P13"]
pub type P13R = crate::BitReader;
#[doc = "Field `P13` writer - P13"]
pub type P13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` reader - P14"]
pub type P14R = crate::BitReader;
#[doc = "Field `P14` writer - P14"]
pub type P14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` reader - P15"]
pub type P15R = crate::BitReader;
#[doc = "Field `P15` writer - P15"]
pub type P15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&self) -> P2R {
        P2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&self) -> P3R {
        P3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&self) -> P4R {
        P4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&self) -> P5R {
        P5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&self) -> P6R {
        P6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&self) -> P7R {
        P7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - P8"]
    #[inline(always)]
    pub fn p8(&self) -> P8R {
        P8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - P9"]
    #[inline(always)]
    pub fn p9(&self) -> P9R {
        P9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - P10"]
    #[inline(always)]
    pub fn p10(&self) -> P10R {
        P10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - P11"]
    #[inline(always)]
    pub fn p11(&self) -> P11R {
        P11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - P12"]
    #[inline(always)]
    pub fn p12(&self) -> P12R {
        P12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - P13"]
    #[inline(always)]
    pub fn p13(&self) -> P13R {
        P13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - P14"]
    #[inline(always)]
    pub fn p14(&self) -> P14R {
        P14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - P15"]
    #[inline(always)]
    pub fn p15(&self) -> P15R {
        P15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<PdifgSpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<PdifgSpec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<PdifgSpec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<PdifgSpec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<PdifgSpec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<PdifgSpec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<PdifgSpec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<PdifgSpec> {
        P7W::new(self, 7)
    }
    #[doc = "Bit 8 - P8"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8W<PdifgSpec> {
        P8W::new(self, 8)
    }
    #[doc = "Bit 9 - P9"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9W<PdifgSpec> {
        P9W::new(self, 9)
    }
    #[doc = "Bit 10 - P10"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10W<PdifgSpec> {
        P10W::new(self, 10)
    }
    #[doc = "Bit 11 - P11"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11W<PdifgSpec> {
        P11W::new(self, 11)
    }
    #[doc = "Bit 12 - P12"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12W<PdifgSpec> {
        P12W::new(self, 12)
    }
    #[doc = "Bit 13 - P13"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13W<PdifgSpec> {
        P13W::new(self, 13)
    }
    #[doc = "Bit 14 - P14"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14W<PdifgSpec> {
        P14W::new(self, 14)
    }
    #[doc = "Bit 15 - P15"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15W<PdifgSpec> {
        P15W::new(self, 15)
    }
}
#[doc = "Port D Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pdifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdifgSpec;
impl crate::RegisterSpec for PdifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pdifg::R`](R) reader structure"]
impl crate::Readable for PdifgSpec {}
#[doc = "`write(|w| ..)` method takes [`pdifg::W`](W) writer structure"]
impl crate::Writable for PdifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PDIFG to value 0"]
impl crate::Resettable for PdifgSpec {
    const RESET_VALUE: u16 = 0;
}
