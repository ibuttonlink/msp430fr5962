#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pain: Pain,
    paout: Paout,
    padir: Padir,
    paren: Paren,
    _reserved4: [u8; 0x02],
    pasel0: Pasel0,
    pasel1: Pasel1,
    _reserved6: [u8; 0x08],
    paselc: Paselc,
    paies: Paies,
    paie: Paie,
    paifg: Paifg,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A Input"]
    #[inline(always)]
    pub const fn pain(&self) -> &Pain {
        &self.pain
    }
    #[doc = "0x02 - Port A Output"]
    #[inline(always)]
    pub const fn paout(&self) -> &Paout {
        &self.paout
    }
    #[doc = "0x04 - Port A Direction"]
    #[inline(always)]
    pub const fn padir(&self) -> &Padir {
        &self.padir
    }
    #[doc = "0x06 - Port A Resistor Enable"]
    #[inline(always)]
    pub const fn paren(&self) -> &Paren {
        &self.paren
    }
    #[doc = "0x0a - Port A Select 0"]
    #[inline(always)]
    pub const fn pasel0(&self) -> &Pasel0 {
        &self.pasel0
    }
    #[doc = "0x0c - Port A Select 1"]
    #[inline(always)]
    pub const fn pasel1(&self) -> &Pasel1 {
        &self.pasel1
    }
    #[doc = "0x16 - Port A Complement Select"]
    #[inline(always)]
    pub const fn paselc(&self) -> &Paselc {
        &self.paselc
    }
    #[doc = "0x18 - Port A Interrupt Edge Select"]
    #[inline(always)]
    pub const fn paies(&self) -> &Paies {
        &self.paies
    }
    #[doc = "0x1a - Port A Interrupt Enable"]
    #[inline(always)]
    pub const fn paie(&self) -> &Paie {
        &self.paie
    }
    #[doc = "0x1c - Port A Interrupt Flag"]
    #[inline(always)]
    pub const fn paifg(&self) -> &Paifg {
        &self.paifg
    }
}
#[doc = "PAIN (rw) register accessor: Port A Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pain`]
module"]
#[doc(alias = "PAIN")]
pub type Pain = crate::Reg<pain::PainSpec>;
#[doc = "Port A Input"]
pub mod pain;
#[doc = "PAOUT (rw) register accessor: Port A Output\n\nYou can [`read`](crate::Reg::read) this register and get [`paout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paout`]
module"]
#[doc(alias = "PAOUT")]
pub type Paout = crate::Reg<paout::PaoutSpec>;
#[doc = "Port A Output"]
pub mod paout;
#[doc = "PADIR (rw) register accessor: Port A Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`padir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padir`]
module"]
#[doc(alias = "PADIR")]
pub type Padir = crate::Reg<padir::PadirSpec>;
#[doc = "Port A Direction"]
pub mod padir;
#[doc = "PAREN (rw) register accessor: Port A Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paren`]
module"]
#[doc(alias = "PAREN")]
pub type Paren = crate::Reg<paren::ParenSpec>;
#[doc = "Port A Resistor Enable"]
pub mod paren;
#[doc = "PASEL0 (rw) register accessor: Port A Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pasel0`]
module"]
#[doc(alias = "PASEL0")]
pub type Pasel0 = crate::Reg<pasel0::Pasel0Spec>;
#[doc = "Port A Select 0"]
pub mod pasel0;
#[doc = "PASEL1 (rw) register accessor: Port A Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pasel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pasel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pasel1`]
module"]
#[doc(alias = "PASEL1")]
pub type Pasel1 = crate::Reg<pasel1::Pasel1Spec>;
#[doc = "Port A Select 1"]
pub mod pasel1;
#[doc = "PASELC (rw) register accessor: Port A Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paselc`]
module"]
#[doc(alias = "PASELC")]
pub type Paselc = crate::Reg<paselc::PaselcSpec>;
#[doc = "Port A Complement Select"]
pub mod paselc;
#[doc = "PAIES (rw) register accessor: Port A Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`paies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paies`]
module"]
#[doc(alias = "PAIES")]
pub type Paies = crate::Reg<paies::PaiesSpec>;
#[doc = "Port A Interrupt Edge Select"]
pub mod paies;
#[doc = "PAIE (rw) register accessor: Port A Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`paie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paie`]
module"]
#[doc(alias = "PAIE")]
pub type Paie = crate::Reg<paie::PaieSpec>;
#[doc = "Port A Interrupt Enable"]
pub mod paie;
#[doc = "PAIFG (rw) register accessor: Port A Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`paifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paifg`]
module"]
#[doc(alias = "PAIFG")]
pub type Paifg = crate::Reg<paifg::PaifgSpec>;
#[doc = "Port A Interrupt Flag"]
pub mod paifg;
