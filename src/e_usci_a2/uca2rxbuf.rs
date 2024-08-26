#[doc = "Register `UCA2RXBUF` reader"]
pub type R = crate::R<Uca2rxbufSpec>;
#[doc = "Register `UCA2RXBUF` writer"]
pub type W = crate::W<Uca2rxbufSpec>;
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
    pub fn ucrxbuf(&mut self) -> UcrxbufW<Uca2rxbufSpec> {
        UcrxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca2rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca2rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca2rxbufSpec;
impl crate::RegisterSpec for Uca2rxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca2rxbuf::R`](R) reader structure"]
impl crate::Readable for Uca2rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`uca2rxbuf::W`](W) writer structure"]
impl crate::Writable for Uca2rxbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCA2RXBUF to value 0"]
impl crate::Resettable for Uca2rxbufSpec {
    const RESET_VALUE: u16 = 0;
}
