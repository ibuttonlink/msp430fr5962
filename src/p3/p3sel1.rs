#[doc = "Register `P3SEL1` reader"]
pub type R = crate::R<P3sel1Spec>;
#[doc = "Register `P3SEL1` writer"]
pub type W = crate::W<P3sel1Spec>;
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
}
impl W {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<P3sel1Spec> {
        P0W::new(self, 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<P3sel1Spec> {
        P1W::new(self, 1)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2W<P3sel1Spec> {
        P2W::new(self, 2)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3W<P3sel1Spec> {
        P3W::new(self, 3)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4W<P3sel1Spec> {
        P4W::new(self, 4)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5W<P3sel1Spec> {
        P5W::new(self, 5)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6W<P3sel1Spec> {
        P6W::new(self, 6)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7W<P3sel1Spec> {
        P7W::new(self, 7)
    }
}
#[doc = "Port 3 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3sel1Spec;
impl crate::RegisterSpec for P3sel1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3sel1::R`](R) reader structure"]
impl crate::Readable for P3sel1Spec {}
#[doc = "`write(|w| ..)` method takes [`p3sel1::W`](W) writer structure"]
impl crate::Writable for P3sel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets P3SEL1 to value 0"]
impl crate::Resettable for P3sel1Spec {
    const RESET_VALUE: u8 = 0;
}
