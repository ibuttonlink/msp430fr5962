#[doc = "Register `UCB3RXBUF` reader"]
pub type R = crate::R<Ucb3rxbufSpec>;
#[doc = "Register `UCB3RXBUF` writer"]
pub type W = crate::W<Ucb3rxbufSpec>;
#[doc = "Field `UCRXBUF` reader - 7:0\\]
Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
#[doc = "Field `UCRXBUF` writer - 7:0\\]
Receive data buffer"]
pub type UcrxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receive data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxbuf(&mut self) -> UcrxbufW<Ucb3rxbufSpec> {
        UcrxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3rxbufSpec;
impl crate::RegisterSpec for Ucb3rxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3rxbuf::R`](R) reader structure"]
impl crate::Readable for Ucb3rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3rxbuf::W`](W) writer structure"]
impl crate::Writable for Ucb3rxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCB3RXBUF to value 0"]
impl crate::Resettable for Ucb3rxbufSpec {
    const RESET_VALUE: u16 = 0;
}
