#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    pein: Pein,
    peout: Peout,
    pedir: Pedir,
    peren: Peren,
    _reserved4: [u8; 0x02],
    pesel0: Pesel0,
    pesel1: Pesel1,
    _reserved6: [u8; 0x08],
    peselc: Peselc,
    peies: Peies,
    peie: Peie,
    peifg: Peifg,
}
impl RegisterBlock {
    #[doc = "0x80 - Port E Input"]
    #[inline(always)]
    pub const fn pein(&self) -> &Pein {
        &self.pein
    }
    #[doc = "0x82 - Port E Output"]
    #[inline(always)]
    pub const fn peout(&self) -> &Peout {
        &self.peout
    }
    #[doc = "0x84 - Port E Direction"]
    #[inline(always)]
    pub const fn pedir(&self) -> &Pedir {
        &self.pedir
    }
    #[doc = "0x86 - Port E Resistor Enable"]
    #[inline(always)]
    pub const fn peren(&self) -> &Peren {
        &self.peren
    }
    #[doc = "0x8a - Port E Select 0"]
    #[inline(always)]
    pub const fn pesel0(&self) -> &Pesel0 {
        &self.pesel0
    }
    #[doc = "0x8c - Port E Select 1"]
    #[inline(always)]
    pub const fn pesel1(&self) -> &Pesel1 {
        &self.pesel1
    }
    #[doc = "0x96 - Port E Complement Select"]
    #[inline(always)]
    pub const fn peselc(&self) -> &Peselc {
        &self.peselc
    }
    #[doc = "0x98 - Port E Interrupt Edge Select"]
    #[inline(always)]
    pub const fn peies(&self) -> &Peies {
        &self.peies
    }
    #[doc = "0x9a - Port E Interrupt Enable"]
    #[inline(always)]
    pub const fn peie(&self) -> &Peie {
        &self.peie
    }
    #[doc = "0x9c - Port E Interrupt Flag"]
    #[inline(always)]
    pub const fn peifg(&self) -> &Peifg {
        &self.peifg
    }
}
#[doc = "PEIN (rw) register accessor: Port E Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pein::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pein::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pein`]
module"]
#[doc(alias = "PEIN")]
pub type Pein = crate::Reg<pein::PeinSpec>;
#[doc = "Port E Input"]
pub mod pein;
#[doc = "PEOUT (rw) register accessor: Port E Output\n\nYou can [`read`](crate::Reg::read) this register and get [`peout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peout`]
module"]
#[doc(alias = "PEOUT")]
pub type Peout = crate::Reg<peout::PeoutSpec>;
#[doc = "Port E Output"]
pub mod peout;
#[doc = "PEDIR (rw) register accessor: Port E Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pedir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pedir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pedir`]
module"]
#[doc(alias = "PEDIR")]
pub type Pedir = crate::Reg<pedir::PedirSpec>;
#[doc = "Port E Direction"]
pub mod pedir;
#[doc = "PEREN (rw) register accessor: Port E Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peren`]
module"]
#[doc(alias = "PEREN")]
pub type Peren = crate::Reg<peren::PerenSpec>;
#[doc = "Port E Resistor Enable"]
pub mod peren;
#[doc = "PESEL0 (rw) register accessor: Port E Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pesel0`]
module"]
#[doc(alias = "PESEL0")]
pub type Pesel0 = crate::Reg<pesel0::Pesel0Spec>;
#[doc = "Port E Select 0"]
pub mod pesel0;
#[doc = "PESEL1 (rw) register accessor: Port E Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pesel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pesel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pesel1`]
module"]
#[doc(alias = "PESEL1")]
pub type Pesel1 = crate::Reg<pesel1::Pesel1Spec>;
#[doc = "Port E Select 1"]
pub mod pesel1;
#[doc = "PESELC (rw) register accessor: Port E Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peselc`]
module"]
#[doc(alias = "PESELC")]
pub type Peselc = crate::Reg<peselc::PeselcSpec>;
#[doc = "Port E Complement Select"]
pub mod peselc;
#[doc = "PEIES (rw) register accessor: Port E Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`peies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peies`]
module"]
#[doc(alias = "PEIES")]
pub type Peies = crate::Reg<peies::PeiesSpec>;
#[doc = "Port E Interrupt Edge Select"]
pub mod peies;
#[doc = "PEIE (rw) register accessor: Port E Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`peie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peie`]
module"]
#[doc(alias = "PEIE")]
pub type Peie = crate::Reg<peie::PeieSpec>;
#[doc = "Port E Interrupt Enable"]
pub mod peie;
#[doc = "PEIFG (rw) register accessor: Port E Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`peifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peifg`]
module"]
#[doc(alias = "PEIFG")]
pub type Peifg = crate::Reg<peifg::PeifgSpec>;
#[doc = "Port E Interrupt Flag"]
pub mod peifg;
