#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtcctl0: Rtcctl0,
    rtcctl13: Rtcctl13,
    rtcocal: Rtcocal,
    rtctcmp: Rtctcmp,
    rtcps0ctl: Rtcps0ctl,
    rtcps1ctl: Rtcps1ctl,
    _reserved_6_rt0ps: [u8; 0x02],
    rtciv: Rtciv,
    _reserved_8_rtccnt1: [u8; 0x02],
    _reserved_9_rtccnt3: [u8; 0x02],
    _reserved_10_rtcdate: [u8; 0x02],
    _reserved_11_rtcyear: [u8; 0x02],
    _reserved_12_rtcaminhr: [u8; 0x02],
    _reserved_13_rtcadowday: [u8; 0x02],
    bin2bcd: Bin2bcd,
    bcd2bin: Bcd2bin,
}
impl RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    #[inline(always)]
    pub const fn rtcctl0(&self) -> &Rtcctl0 {
        &self.rtcctl0
    }
    #[doc = "0x02 - RTCCTL13 Register"]
    #[inline(always)]
    pub const fn rtcctl13(&self) -> &Rtcctl13 {
        &self.rtcctl13
    }
    #[doc = "0x04 - RTCOCAL Register"]
    #[inline(always)]
    pub const fn rtcocal(&self) -> &Rtcocal {
        &self.rtcocal
    }
    #[doc = "0x06 - RTCTCMP Register"]
    #[inline(always)]
    pub const fn rtctcmp(&self) -> &Rtctcmp {
        &self.rtctcmp
    }
    #[doc = "0x08 - Real-Time Clock Prescale Timer 0 Control Register"]
    #[inline(always)]
    pub const fn rtcps0ctl(&self) -> &Rtcps0ctl {
        &self.rtcps0ctl
    }
    #[doc = "0x0a - Real-Time Clock Prescale Timer 1 Control Register"]
    #[inline(always)]
    pub const fn rtcps1ctl(&self) -> &Rtcps1ctl {
        &self.rtcps1ctl
    }
    #[doc = "0x0c - Prescale timer 0 counter value"]
    #[inline(always)]
    pub const fn rt0ps(&self) -> &Rt0ps {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    #[inline(always)]
    pub const fn rtcps(&self) -> &Rtcps {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0d - Prescale timer 1 counter value"]
    #[inline(always)]
    pub const fn rt1ps(&self) -> &Rt1ps {
        unsafe { &*(self as *const Self).cast::<u8>().add(13).cast() }
    }
    #[doc = "0x0e - Real-Time Clock Interrupt Vector Register"]
    #[inline(always)]
    pub const fn rtciv(&self) -> &Rtciv {
        &self.rtciv
    }
    #[doc = "0x10 - The RTCCNT1 register is the count of RTCCNT1"]
    #[inline(always)]
    pub const fn rtccnt1(&self) -> &Rtccnt1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Real-Time Clock Counter 1 and 2 Register Counter Mode"]
    #[inline(always)]
    pub const fn rtccnt12(&self) -> &Rtccnt12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Real-Time Clock Seconds, Minutes Register - BCD Format"]
    #[inline(always)]
    pub const fn rtctim0_bcd(&self) -> &Rtctim0Bcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtctim0(&self) -> &Rtctim0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - The RTCCNT2 register is the count of RTCCNT2"]
    #[inline(always)]
    pub const fn rtccnt2(&self) -> &Rtccnt2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x12 - The RTCCNT3 register is the count of RTCCNT3"]
    #[inline(always)]
    pub const fn rtccnt3(&self) -> &Rtccnt3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - Real-Time Clock Counter 3 and 4 Register Counter Mode"]
    #[inline(always)]
    pub const fn rtccnt34(&self) -> &Rtccnt34 {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week - BCD Format"]
    #[inline(always)]
    pub const fn rtctim1_bcd(&self) -> &Rtctim1Bcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    #[inline(always)]
    pub const fn rtctim1(&self) -> &Rtctim1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x13 - The RTCCNT4 register is the count of RTCCNT4"]
    #[inline(always)]
    pub const fn rtccnt4(&self) -> &Rtccnt4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(19).cast() }
    }
    #[doc = "0x14 - Real-Time Clock Date - BCD Format"]
    #[inline(always)]
    pub const fn rtcdate_bcd(&self) -> &RtcdateBcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcdate(&self) -> &Rtcdate {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x16 - Real-Time Clock Year Register - BCD Format"]
    #[inline(always)]
    pub const fn rtcyear_bcd(&self) -> &RtcyearBcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcyear(&self) -> &Rtcyear {
        unsafe { &*(self as *const Self).cast::<u8>().add(22).cast() }
    }
    #[doc = "0x18 - Real-Time Clock Minutes, Hour Alarm - BCD Format"]
    #[inline(always)]
    pub const fn rtcaminhr_bcd(&self) -> &RtcaminhrBcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcaminhr(&self) -> &Rtcaminhr {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1a - Real-Time Clock Day of Week, Day of Month Alarm - BCD Format"]
    #[inline(always)]
    pub const fn rtcadowday_bcd(&self) -> &RtcadowdayBcd {
        unsafe { &*(self as *const Self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    #[inline(always)]
    pub const fn rtcadowday(&self) -> &Rtcadowday {
        unsafe { &*(self as *const Self).cast::<u8>().add(26).cast() }
    }
    #[doc = "0x1c - Binary-to-BCD Conversion Register"]
    #[inline(always)]
    pub const fn bin2bcd(&self) -> &Bin2bcd {
        &self.bin2bcd
    }
    #[doc = "0x1e - BCD-to-Binary Conversion Register"]
    #[inline(always)]
    pub const fn bcd2bin(&self) -> &Bcd2bin {
        &self.bcd2bin
    }
}
#[doc = "RTCCTL0 (rw) register accessor: RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl0`]
module"]
#[doc(alias = "RTCCTL0")]
pub type Rtcctl0 = crate::Reg<rtcctl0::Rtcctl0Spec>;
#[doc = "RTCCTL0 Register"]
pub mod rtcctl0;
#[doc = "RTCCTL13 (rw) register accessor: RTCCTL13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl13`]
module"]
#[doc(alias = "RTCCTL13")]
pub type Rtcctl13 = crate::Reg<rtcctl13::Rtcctl13Spec>;
#[doc = "RTCCTL13 Register"]
pub mod rtcctl13;
#[doc = "RTCOCAL (rw) register accessor: RTCOCAL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcocal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcocal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcocal`]
module"]
#[doc(alias = "RTCOCAL")]
pub type Rtcocal = crate::Reg<rtcocal::RtcocalSpec>;
#[doc = "RTCOCAL Register"]
pub mod rtcocal;
#[doc = "RTCTCMP (rw) register accessor: RTCTCMP Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctcmp`]
module"]
#[doc(alias = "RTCTCMP")]
pub type Rtctcmp = crate::Reg<rtctcmp::RtctcmpSpec>;
#[doc = "RTCTCMP Register"]
pub mod rtctcmp;
#[doc = "RTCPS0CTL (rw) register accessor: Real-Time Clock Prescale Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps0ctl`]
module"]
#[doc(alias = "RTCPS0CTL")]
pub type Rtcps0ctl = crate::Reg<rtcps0ctl::Rtcps0ctlSpec>;
#[doc = "Real-Time Clock Prescale Timer 0 Control Register"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL (rw) register accessor: Real-Time Clock Prescale Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps1ctl`]
module"]
#[doc(alias = "RTCPS1CTL")]
pub type Rtcps1ctl = crate::Reg<rtcps1ctl::Rtcps1ctlSpec>;
#[doc = "Real-Time Clock Prescale Timer 1 Control Register"]
pub mod rtcps1ctl;
#[doc = "RTCPS (rw) register accessor: Real-Time Clock Prescale Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcps`]
module"]
#[doc(alias = "RTCPS")]
pub type Rtcps = crate::Reg<rtcps::RtcpsSpec>;
#[doc = "Real-Time Clock Prescale Timer Counter Register"]
pub mod rtcps;
#[doc = "RTCIV (rw) register accessor: Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtciv`]
module"]
#[doc(alias = "RTCIV")]
pub type Rtciv = crate::Reg<rtciv::RtcivSpec>;
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCTIM0 (rw) register accessor: RTCTIM0 Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim0`]
module"]
#[doc(alias = "RTCTIM0")]
pub type Rtctim0 = crate::Reg<rtctim0::Rtctim0Spec>;
#[doc = "RTCTIM0 Register Hexadecimal Format"]
pub mod rtctim0;
#[doc = "RTCTIM0_BCD (rw) register accessor: Real-Time Clock Seconds, Minutes Register - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim0_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim0_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim0_bcd`]
module"]
#[doc(alias = "RTCTIM0_BCD")]
pub type Rtctim0Bcd = crate::Reg<rtctim0_bcd::Rtctim0BcdSpec>;
#[doc = "Real-Time Clock Seconds, Minutes Register - BCD Format"]
pub mod rtctim0_bcd;
#[doc = "RTCCNT12 (rw) register accessor: Real-Time Clock Counter 1 and 2 Register Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt12`]
module"]
#[doc(alias = "RTCCNT12")]
pub type Rtccnt12 = crate::Reg<rtccnt12::Rtccnt12Spec>;
#[doc = "Real-Time Clock Counter 1 and 2 Register Counter Mode"]
pub mod rtccnt12;
#[doc = "RTCTIM1 (rw) register accessor: Real-Time Clock Hour, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim1`]
module"]
#[doc(alias = "RTCTIM1")]
pub type Rtctim1 = crate::Reg<rtctim1::Rtctim1Spec>;
#[doc = "Real-Time Clock Hour, Day of Week"]
pub mod rtctim1;
#[doc = "RTCTIM1_BCD (rw) register accessor: Real-Time Clock Hour, Day of Week - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtctim1_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtctim1_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtctim1_bcd`]
module"]
#[doc(alias = "RTCTIM1_BCD")]
pub type Rtctim1Bcd = crate::Reg<rtctim1_bcd::Rtctim1BcdSpec>;
#[doc = "Real-Time Clock Hour, Day of Week - BCD Format"]
pub mod rtctim1_bcd;
#[doc = "RTCCNT34 (rw) register accessor: Real-Time Clock Counter 3 and 4 Register Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt34`]
module"]
#[doc(alias = "RTCCNT34")]
pub type Rtccnt34 = crate::Reg<rtccnt34::Rtccnt34Spec>;
#[doc = "Real-Time Clock Counter 3 and 4 Register Counter Mode"]
pub mod rtccnt34;
#[doc = "RTCDATE (rw) register accessor: RTCDATE - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcdate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcdate`]
module"]
#[doc(alias = "RTCDATE")]
pub type Rtcdate = crate::Reg<rtcdate::RtcdateSpec>;
#[doc = "RTCDATE - Hexadecimal Format"]
pub mod rtcdate;
#[doc = "RTCDATE_BCD (rw) register accessor: Real-Time Clock Date - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcdate_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcdate_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcdate_bcd`]
module"]
#[doc(alias = "RTCDATE_BCD")]
pub type RtcdateBcd = crate::Reg<rtcdate_bcd::RtcdateBcdSpec>;
#[doc = "Real-Time Clock Date - BCD Format"]
pub mod rtcdate_bcd;
#[doc = "RTCYEAR (rw) register accessor: RTCYEAR Register Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcyear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcyear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcyear`]
module"]
#[doc(alias = "RTCYEAR")]
pub type Rtcyear = crate::Reg<rtcyear::RtcyearSpec>;
#[doc = "RTCYEAR Register Hexadecimal Format"]
pub mod rtcyear;
#[doc = "RTCYEAR_BCD (rw) register accessor: Real-Time Clock Year Register - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcyear_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcyear_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcyear_bcd`]
module"]
#[doc(alias = "RTCYEAR_BCD")]
pub type RtcyearBcd = crate::Reg<rtcyear_bcd::RtcyearBcdSpec>;
#[doc = "Real-Time Clock Year Register - BCD Format"]
pub mod rtcyear_bcd;
#[doc = "RTCAMINHR (rw) register accessor: RTCMINHR - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcaminhr`]
module"]
#[doc(alias = "RTCAMINHR")]
pub type Rtcaminhr = crate::Reg<rtcaminhr::RtcaminhrSpec>;
#[doc = "RTCMINHR - Hexadecimal Format"]
pub mod rtcaminhr;
#[doc = "RTCAMINHR_BCD (rw) register accessor: Real-Time Clock Minutes, Hour Alarm - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcaminhr_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcaminhr_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcaminhr_bcd`]
module"]
#[doc(alias = "RTCAMINHR_BCD")]
pub type RtcaminhrBcd = crate::Reg<rtcaminhr_bcd::RtcaminhrBcdSpec>;
#[doc = "Real-Time Clock Minutes, Hour Alarm - BCD Format"]
pub mod rtcaminhr_bcd;
#[doc = "RTCADOWDAY (rw) register accessor: RTCADOWDAY - Hexadecimal Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcadowday`]
module"]
#[doc(alias = "RTCADOWDAY")]
pub type Rtcadowday = crate::Reg<rtcadowday::RtcadowdaySpec>;
#[doc = "RTCADOWDAY - Hexadecimal Format"]
pub mod rtcadowday;
#[doc = "RTCADOWDAY_BCD (rw) register accessor: Real-Time Clock Day of Week, Day of Month Alarm - BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcadowday_bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcadowday_bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcadowday_bcd`]
module"]
#[doc(alias = "RTCADOWDAY_BCD")]
pub type RtcadowdayBcd = crate::Reg<rtcadowday_bcd::RtcadowdayBcdSpec>;
#[doc = "Real-Time Clock Day of Week, Day of Month Alarm - BCD Format"]
pub mod rtcadowday_bcd;
#[doc = "BIN2BCD (rw) register accessor: Binary-to-BCD Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bin2bcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bin2bcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bin2bcd`]
module"]
#[doc(alias = "BIN2BCD")]
pub type Bin2bcd = crate::Reg<bin2bcd::Bin2bcdSpec>;
#[doc = "Binary-to-BCD Conversion Register"]
pub mod bin2bcd;
#[doc = "BCD2BIN (rw) register accessor: BCD-to-Binary Conversion Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcd2bin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcd2bin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcd2bin`]
module"]
#[doc(alias = "BCD2BIN")]
pub type Bcd2bin = crate::Reg<bcd2bin::Bcd2binSpec>;
#[doc = "BCD-to-Binary Conversion Register"]
pub mod bcd2bin;
#[doc = "RT0PS (rw) register accessor: Prescale timer 0 counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`rt0ps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rt0ps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt0ps`]
module"]
#[doc(alias = "RT0PS")]
pub type Rt0ps = crate::Reg<rt0ps::Rt0psSpec>;
#[doc = "Prescale timer 0 counter value"]
pub mod rt0ps;
#[doc = "RT1PS (rw) register accessor: Prescale timer 1 counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`rt1ps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rt1ps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt1ps`]
module"]
#[doc(alias = "RT1PS")]
pub type Rt1ps = crate::Reg<rt1ps::Rt1psSpec>;
#[doc = "Prescale timer 1 counter value"]
pub mod rt1ps;
#[doc = "RTCCNT1 (rw) register accessor: The RTCCNT1 register is the count of RTCCNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt1`]
module"]
#[doc(alias = "RTCCNT1")]
pub type Rtccnt1 = crate::Reg<rtccnt1::Rtccnt1Spec>;
#[doc = "The RTCCNT1 register is the count of RTCCNT1"]
pub mod rtccnt1;
#[doc = "RTCCNT2 (rw) register accessor: The RTCCNT2 register is the count of RTCCNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt2`]
module"]
#[doc(alias = "RTCCNT2")]
pub type Rtccnt2 = crate::Reg<rtccnt2::Rtccnt2Spec>;
#[doc = "The RTCCNT2 register is the count of RTCCNT2"]
pub mod rtccnt2;
#[doc = "RTCCNT3 (rw) register accessor: The RTCCNT3 register is the count of RTCCNT3\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt3`]
module"]
#[doc(alias = "RTCCNT3")]
pub type Rtccnt3 = crate::Reg<rtccnt3::Rtccnt3Spec>;
#[doc = "The RTCCNT3 register is the count of RTCCNT3"]
pub mod rtccnt3;
#[doc = "RTCCNT4 (rw) register accessor: The RTCCNT4 register is the count of RTCCNT4\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt4`]
module"]
#[doc(alias = "RTCCNT4")]
pub type Rtccnt4 = crate::Reg<rtccnt4::Rtccnt4Spec>;
#[doc = "The RTCCNT4 register is the count of RTCCNT4"]
pub mod rtccnt4;
