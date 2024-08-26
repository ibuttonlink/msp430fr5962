#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x60],
    pdin: Pdin,
    pdout: Pdout,
    pddir: Pddir,
    pdren: Pdren,
    _reserved4: [u8; 0x02],
    pdsel0: Pdsel0,
    pdsel1: Pdsel1,
    _reserved6: [u8; 0x08],
    pdselc: Pdselc,
    pdies: Pdies,
    pdie: Pdie,
    pdifg: Pdifg,
}
impl RegisterBlock {
    #[doc = "0x60 - Port D Input"]
    #[inline(always)]
    pub const fn pdin(&self) -> &Pdin {
        &self.pdin
    }
    #[doc = "0x62 - Port D Output"]
    #[inline(always)]
    pub const fn pdout(&self) -> &Pdout {
        &self.pdout
    }
    #[doc = "0x64 - Port D Direction"]
    #[inline(always)]
    pub const fn pddir(&self) -> &Pddir {
        &self.pddir
    }
    #[doc = "0x66 - Port D Resistor Enable"]
    #[inline(always)]
    pub const fn pdren(&self) -> &Pdren {
        &self.pdren
    }
    #[doc = "0x6a - Port D Select 0"]
    #[inline(always)]
    pub const fn pdsel0(&self) -> &Pdsel0 {
        &self.pdsel0
    }
    #[doc = "0x6c - Port D Select 1"]
    #[inline(always)]
    pub const fn pdsel1(&self) -> &Pdsel1 {
        &self.pdsel1
    }
    #[doc = "0x76 - Port D Complement Select"]
    #[inline(always)]
    pub const fn pdselc(&self) -> &Pdselc {
        &self.pdselc
    }
    #[doc = "0x78 - Port D Interrupt Edge Select"]
    #[inline(always)]
    pub const fn pdies(&self) -> &Pdies {
        &self.pdies
    }
    #[doc = "0x7a - Port D Interrupt Enable"]
    #[inline(always)]
    pub const fn pdie(&self) -> &Pdie {
        &self.pdie
    }
    #[doc = "0x7c - Port D Interrupt Flag"]
    #[inline(always)]
    pub const fn pdifg(&self) -> &Pdifg {
        &self.pdifg
    }
}
#[doc = "PDIN (rw) register accessor: Port D Input\n\nYou can [`read`](crate::Reg::read) this register and get [`pdin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdin`]
module"]
#[doc(alias = "PDIN")]
pub type Pdin = crate::Reg<pdin::PdinSpec>;
#[doc = "Port D Input"]
pub mod pdin;
#[doc = "PDOUT (rw) register accessor: Port D Output\n\nYou can [`read`](crate::Reg::read) this register and get [`pdout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdout`]
module"]
#[doc(alias = "PDOUT")]
pub type Pdout = crate::Reg<pdout::PdoutSpec>;
#[doc = "Port D Output"]
pub mod pdout;
#[doc = "PDDIR (rw) register accessor: Port D Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`pddir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pddir`]
module"]
#[doc(alias = "PDDIR")]
pub type Pddir = crate::Reg<pddir::PddirSpec>;
#[doc = "Port D Direction"]
pub mod pddir;
#[doc = "PDREN (rw) register accessor: Port D Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdren`]
module"]
#[doc(alias = "PDREN")]
pub type Pdren = crate::Reg<pdren::PdrenSpec>;
#[doc = "Port D Resistor Enable"]
pub mod pdren;
#[doc = "PDSEL0 (rw) register accessor: Port D Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsel0`]
module"]
#[doc(alias = "PDSEL0")]
pub type Pdsel0 = crate::Reg<pdsel0::Pdsel0Spec>;
#[doc = "Port D Select 0"]
pub mod pdsel0;
#[doc = "PDSEL1 (rw) register accessor: Port D Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsel1`]
module"]
#[doc(alias = "PDSEL1")]
pub type Pdsel1 = crate::Reg<pdsel1::Pdsel1Spec>;
#[doc = "Port D Select 1"]
pub mod pdsel1;
#[doc = "PDSELC (rw) register accessor: Port D Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdselc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdselc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdselc`]
module"]
#[doc(alias = "PDSELC")]
pub type Pdselc = crate::Reg<pdselc::PdselcSpec>;
#[doc = "Port D Complement Select"]
pub mod pdselc;
#[doc = "PDIES (rw) register accessor: Port D Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdies`]
module"]
#[doc(alias = "PDIES")]
pub type Pdies = crate::Reg<pdies::PdiesSpec>;
#[doc = "Port D Interrupt Edge Select"]
pub mod pdies;
#[doc = "PDIE (rw) register accessor: Port D Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdie`]
module"]
#[doc(alias = "PDIE")]
pub type Pdie = crate::Reg<pdie::PdieSpec>;
#[doc = "Port D Interrupt Enable"]
pub mod pdie;
#[doc = "PDIFG (rw) register accessor: Port D Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pdifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdifg`]
module"]
#[doc(alias = "PDIFG")]
pub type Pdifg = crate::Reg<pdifg::PdifgSpec>;
#[doc = "Port D Interrupt Flag"]
pub mod pdifg;
