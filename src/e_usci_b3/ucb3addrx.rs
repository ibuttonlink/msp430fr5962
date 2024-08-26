#[doc = "Register `UCB3ADDRX` reader"]
pub type R = crate::R<Ucb3addrxSpec>;
#[doc = "Register `UCB3ADDRX` writer"]
pub type W = crate::W<Ucb3addrxSpec>;
#[doc = "Field `ADDRX` reader - 9:0\\]
Received Address Register"]
pub type AddrxR = crate::FieldReader<u16>;
#[doc = "Field `ADDRX` writer - 9:0\\]
Received Address Register"]
pub type AddrxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> AddrxR {
        AddrxR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Received Address Register"]
    #[inline(always)]
    #[must_use]
    pub fn addrx(&mut self) -> AddrxW<Ucb3addrxSpec> {
        AddrxW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3addrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3addrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3addrxSpec;
impl crate::RegisterSpec for Ucb3addrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3addrx::R`](R) reader structure"]
impl crate::Readable for Ucb3addrxSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3addrx::W`](W) writer structure"]
impl crate::Writable for Ucb3addrxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB3ADDRX to value 0"]
impl crate::Resettable for Ucb3addrxSpec {
    const RESET_VALUE: u16 = 0;
}
