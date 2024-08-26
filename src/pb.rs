#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    pbin: Pbin,
    pbout: Pbout,
    pbdir: Pbdir,
    pbren: Pbren,
    _reserved4: [u8; 0x02],
    pbsel0: Pbsel0,
    pbsel1: Pbsel1,
    _reserved6: [u8; 0x08],
    pbselc: Pbselc,
    pbies: Pbies,
    pbie: Pbie,
    pbifg: Pbifg,
}
impl RegisterBlock {
    #[doc = "0x20 - Port B Input"]
    #[inline(always)]
    pub const fn pbin(&self) -> &Pbin {
        &self.pbin
    }
    #[doc = "0x22 - Port B Output"]
    #[inline(always)]
    pub const fn pbout(&self) -> &Pbout {
        &self.pbout
    }
    #[doc = "0x24 - Port B Direction"]
    #[inline(always)]
    pub const fn pbdir(&self) -> &Pbdir {
        &self.pbdir
    }
    #[doc = "0x26 - Port B Resistor Enable"]
    #[inline(always)]
    pub const fn pbren(&self) -> &Pbren {
        &self.pbren
    }
    #[doc = "0x2a - Port B Select 0"]
    #[inline(always)]
    pub const fn pbsel0(&self) -> &Pbsel0 {
        &self.pbsel0
    }
    #[doc = "0x2c - Port B Select 1"]
    #[inline(always)]
    pub const fn pbsel1(&self) -> &Pbsel1 {
        &self.pbsel1
    }
    #[doc = "0x36 - Port B Complement Select"]
    #[inline(always)]
    pub const fn pbselc(&self) -> &Pbselc {
        &self.pbselc
    }
    #[doc = "0x38 - Port B Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pbies(&self) -> &Pbies {
        &self.pbies
    }
    #[doc = "0x3a - Port B Interrupt Enable"]
    #[inline(always)]
    pub const fn pbie(&self) -> &Pbie {
        &self.pbie
    }
    #[doc = "0x3c - Port B Interrupt Flag"]
    #[inline(always)]
    pub const fn pbifg(&self) -> &Pbifg {
        &self.pbifg
    }
}
#[doc = "PBIN (rw) register accessor: Port B Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pbin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbin`]
module"]
#[doc(alias = "PBIN")]
pub type Pbin = crate::Reg<pbin::PbinSpec>;
#[doc = "Port B Input"]
pub mod pbin;
#[doc = "PBOUT (rw) register accessor: Port B Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pbout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbout`]
module"]
#[doc(alias = "PBOUT")]
pub type Pbout = crate::Reg<pbout::PboutSpec>;
#[doc = "Port B Output"]
pub mod pbout;
#[doc = "PBDIR (rw) register accessor: Port B Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pbdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbdir`]
module"]
#[doc(alias = "PBDIR")]
pub type Pbdir = crate::Reg<pbdir::PbdirSpec>;
#[doc = "Port B Direction"]
pub mod pbdir;
#[doc = "PBREN (rw) register accessor: Port B Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbren`]
module"]
#[doc(alias = "PBREN")]
pub type Pbren = crate::Reg<pbren::PbrenSpec>;
#[doc = "Port B Resistor Enable"]
pub mod pbren;
#[doc = "PBSEL0 (rw) register accessor: Port B Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbsel0`]
module"]
#[doc(alias = "PBSEL0")]
pub type Pbsel0 = crate::Reg<pbsel0::Pbsel0Spec>;
#[doc = "Port B Select 0"]
pub mod pbsel0;
#[doc = "PBSEL1 (rw) register accessor: Port B Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbsel1`]
module"]
#[doc(alias = "PBSEL1")]
pub type Pbsel1 = crate::Reg<pbsel1::Pbsel1Spec>;
#[doc = "Port B Select 1"]
pub mod pbsel1;
#[doc = "PBSELC (rw) register accessor: Port B Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbselc`]
module"]
#[doc(alias = "PBSELC")]
pub type Pbselc = crate::Reg<pbselc::PbselcSpec>;
#[doc = "Port B Complement Select"]
pub mod pbselc;
#[doc = "PBIES (rw) register accessor: Port B Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pbies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbies`]
module"]
#[doc(alias = "PBIES")]
pub type Pbies = crate::Reg<pbies::PbiesSpec>;
#[doc = "Port B Interrupt Edge Select"]
pub mod pbies;
#[doc = "PBIE (rw) register accessor: Port B Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pbie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbie`]
module"]
#[doc(alias = "PBIE")]
pub type Pbie = crate::Reg<pbie::PbieSpec>;
#[doc = "Port B Interrupt Enable"]
pub mod pbie;
#[doc = "PBIFG (rw) register accessor: Port B Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pbifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbifg`]
module"]
#[doc(alias = "PBIFG")]
pub type Pbifg = crate::Reg<pbifg::PbifgSpec>;
#[doc = "Port B Interrupt Flag"]
pub mod pbifg;
