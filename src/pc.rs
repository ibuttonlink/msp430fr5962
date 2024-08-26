#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    pcin: Pcin,
    pcout: Pcout,
    pcdir: Pcdir,
    pcren: Pcren,
    _reserved4: [u8; 0x02],
    pcsel0: Pcsel0,
    pcsel1: Pcsel1,
    _reserved6: [u8; 0x08],
    pcselc: Pcselc,
    pcies: Pcies,
    pcie: Pcie,
    pcifg: Pcifg,
}
impl RegisterBlock {
    #[doc = "0x40 - Port C Input"]
    #[inline(always)]
    pub const fn pcin(&self) -> &Pcin {
        &self.pcin
    }
    #[doc = "0x42 - Port C Output"]
    #[inline(always)]
    pub const fn pcout(&self) -> &Pcout {
        &self.pcout
    }
    #[doc = "0x44 - Port C Direction"]
    #[inline(always)]
    pub const fn pcdir(&self) -> &Pcdir {
        &self.pcdir
    }
    #[doc = "0x46 - Port C Resistor Enable"]
    #[inline(always)]
    pub const fn pcren(&self) -> &Pcren {
        &self.pcren
    }
    #[doc = "0x4a - Port C Select 0"]
    #[inline(always)]
    pub const fn pcsel0(&self) -> &Pcsel0 {
        &self.pcsel0
    }
    #[doc = "0x4c - Port C Select 1"]
    #[inline(always)]
    pub const fn pcsel1(&self) -> &Pcsel1 {
        &self.pcsel1
    }
    #[doc = "0x56 - Port C Complement Select"]
    #[inline(always)]
    pub const fn pcselc(&self) -> &Pcselc {
        &self.pcselc
    }
    #[doc = "0x58 - Port C Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pcies(&self) -> &Pcies {
        &self.pcies
    }
    #[doc = "0x5a - Port C Interrupt Enable"]
    #[inline(always)]
    pub const fn pcie(&self) -> &Pcie {
        &self.pcie
    }
    #[doc = "0x5c - Port C Interrupt Flag"]
    #[inline(always)]
    pub const fn pcifg(&self) -> &Pcifg {
        &self.pcifg
    }
}
#[doc = "PCIN (rw) register accessor: Port C Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pcin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcin`]
module"]
#[doc(alias = "PCIN")]
pub type Pcin = crate::Reg<pcin::PcinSpec>;
#[doc = "Port C Input"]
pub mod pcin;
#[doc = "PCOUT (rw) register accessor: Port C Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pcout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcout`]
module"]
#[doc(alias = "PCOUT")]
pub type Pcout = crate::Reg<pcout::PcoutSpec>;
#[doc = "Port C Output"]
pub mod pcout;
#[doc = "PCDIR (rw) register accessor: Port C Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdir`]
module"]
#[doc(alias = "PCDIR")]
pub type Pcdir = crate::Reg<pcdir::PcdirSpec>;
#[doc = "Port C Direction"]
pub mod pcdir;
#[doc = "PCREN (rw) register accessor: Port C Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcren`]
module"]
#[doc(alias = "PCREN")]
pub type Pcren = crate::Reg<pcren::PcrenSpec>;
#[doc = "Port C Resistor Enable"]
pub mod pcren;
#[doc = "PCSEL0 (rw) register accessor: Port C Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsel0`]
module"]
#[doc(alias = "PCSEL0")]
pub type Pcsel0 = crate::Reg<pcsel0::Pcsel0Spec>;
#[doc = "Port C Select 0"]
pub mod pcsel0;
#[doc = "PCSEL1 (rw) register accessor: Port C Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsel1`]
module"]
#[doc(alias = "PCSEL1")]
pub type Pcsel1 = crate::Reg<pcsel1::Pcsel1Spec>;
#[doc = "Port C Select 1"]
pub mod pcsel1;
#[doc = "PCSELC (rw) register accessor: Port C Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcselc`]
module"]
#[doc(alias = "PCSELC")]
pub type Pcselc = crate::Reg<pcselc::PcselcSpec>;
#[doc = "Port C Complement Select"]
pub mod pcselc;
#[doc = "PCIES (rw) register accessor: Port C Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcies`]
module"]
#[doc(alias = "PCIES")]
pub type Pcies = crate::Reg<pcies::PciesSpec>;
#[doc = "Port C Interrupt Edge Select"]
pub mod pcies;
#[doc = "PCIE (rw) register accessor: Port C Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie`]
module"]
#[doc(alias = "PCIE")]
pub type Pcie = crate::Reg<pcie::PcieSpec>;
#[doc = "Port C Interrupt Enable"]
pub mod pcie;
#[doc = "PCIFG (rw) register accessor: Port C Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pcifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcifg`]
module"]
#[doc(alias = "PCIFG")]
pub type Pcifg = crate::Reg<pcifg::PcifgSpec>;
#[doc = "Port C Interrupt Flag"]
pub mod pcifg;
