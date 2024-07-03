#[doc = "cfr1 register accessor"]
pub type Cfr1<'a, I> = crate::Reg<'a, cfr1::Cfr1Spec, I>;
#[doc = "Control Function Register 1"]
pub mod cfr1;
#[doc = "cfr2 register accessor"]
pub type Cfr2<'a, I> = crate::Reg<'a, cfr2::Cfr2Spec, I>;
#[doc = "Control Function Register 2"]
pub mod cfr2;
#[doc = "cfr3 register accessor"]
pub type Cfr3<'a, I> = crate::Reg<'a, cfr3::Cfr3Spec, I>;
#[doc = "Control Function Register 3"]
pub mod cfr3;
#[doc = "aux_dac_control register accessor"]
pub type AuxDacControl<'a, I> = crate::Reg<'a, aux_dac_control::AuxDacControlSpec, I>;
#[doc = "Auxiliary DAC Control Register"]
pub mod aux_dac_control;
#[doc = "io_update_rate register accessor"]
pub type IoUpdateRate<'a, I> = crate::Reg<'a, io_update_rate::IoUpdateRateSpec, I>;
#[doc = "I/O Update Rate Register"]
pub mod io_update_rate;
#[doc = "ftw register accessor: an alias for `Reg<FTW_SPEC>`"]
pub type Ftw<'a, I> = crate::Reg<'a, ftw::FtwSpec, I>;
#[doc = "Frequency Tuning Word Register"]
pub mod ftw;
#[doc = "pow register accessor"]
pub type Pow<'a, I> = crate::Reg<'a, pow::PowSpec, I>;
#[doc = "Phase Offset Word Register"]
pub mod pow;
#[doc = "asf register accessor"]
pub type Asf<'a, I> = crate::Reg<'a, asf::AsfSpec, I>;
#[doc = "Amplitude Scale Factor Register"]
pub mod asf;
#[doc = "multichip_sync register accessor"]
pub type MultichipSync<'a, I> = crate::Reg<'a, multichip_sync::MultichipSyncSpec, I>;
#[doc = "Multichip Sync Register"]
pub mod multichip_sync;
#[doc = "digital_ramp_limit register accessor"]
pub type DigitalRampLimit<'a, I> = crate::Reg<'a, digital_ramp_limit::DigitalRampLimitSpec, I>;
#[doc = "Digital Ramp Limit Register"]
pub mod digital_ramp_limit;
#[doc = "digital_ramp_step_size register accessor"]
pub type DigitalRampStepSize<'a, I> =
    crate::Reg<'a, digital_ramp_step_size::DigitalRampStepSizeSpec, I>;
#[doc = "Digital Ramp Step Size Register"]
pub mod digital_ramp_step_size;
#[doc = "digital_ramp_rate register accessor"]
pub type DigitalRampRate<'a, I> = crate::Reg<'a, digital_ramp_rate::DigitalRampRateSpec, I>;
#[doc = "Digital Ramp Rate Register"]
pub mod digital_ramp_rate;
#[doc = "ram_profile0 register accessor"]
pub type RamProfile0<'a, I> = crate::Reg<'a, ram_profile0::RamProfile0Spec, I>;
#[doc = "RAM Profile 0 Register"]
pub mod ram_profile0;
#[doc = "single_tone_profile0 register accessor"]
pub type SingleToneProfile0<'a, I> =
    crate::Reg<'a, single_tone_profile0::SingleToneProfile0Spec, I>;
#[doc = "Single Tone Profile 0 Register"]
pub mod single_tone_profile0;
#[doc = "ram_profile1 register accessor"]
pub type RamProfile1<'a, I> = crate::Reg<'a, ram_profile1::RamProfile1Spec, I>;
#[doc = "RAM Profile 1 Register"]
pub mod ram_profile1;
#[doc = "single_tone_profile1 register accessor"]
pub type SingleToneProfile1<'a, I> =
    crate::Reg<'a, single_tone_profile1::SingleToneProfile1Spec, I>;
#[doc = "Single Tone Profile 1 Register"]
pub mod single_tone_profile1;
#[doc = "ram_profile2 register accessor"]
pub type RamProfile2<'a, I> = crate::Reg<'a, ram_profile2::RamProfile2Spec, I>;
#[doc = "RAM Profile 2 Register"]
pub mod ram_profile2;
#[doc = "single_tone_profile2 register accessor"]
pub type SingleToneProfile2<'a, I> =
    crate::Reg<'a, single_tone_profile2::SingleToneProfile2Spec, I>;
#[doc = "Single Tone Profile 2 Register"]
pub mod single_tone_profile2;
#[doc = "ram_profile3 register accessor"]
pub type RamProfile3<'a, I> = crate::Reg<'a, ram_profile3::RamProfile3Spec, I>;
#[doc = "RAM Profile 3 Register"]
pub mod ram_profile3;
#[doc = "single_tone_profile3 register accessor"]
pub type SingleToneProfile3<'a, I> =
    crate::Reg<'a, single_tone_profile3::SingleToneProfile3Spec, I>;
#[doc = "Single Tone Profile 3 Register"]
pub mod single_tone_profile3;
#[doc = "ram_profile4 register accessor"]
pub type RamProfile4<'a, I> = crate::Reg<'a, ram_profile4::RamProfile4Spec, I>;
#[doc = "RAM Profile 4 Register"]
pub mod ram_profile4;
#[doc = "single_tone_profile4 register accessor"]
pub type SingleToneProfile4<'a, I> =
    crate::Reg<'a, single_tone_profile4::SingleToneProfile4Spec, I>;
#[doc = "Single Tone Profile 4 Register"]
pub mod single_tone_profile4;
#[doc = "ram_profile5 register accessor"]
pub type RamProfile5<'a, I> = crate::Reg<'a, ram_profile5::RamProfile5Spec, I>;
#[doc = "RAM Profile 5 Register"]
pub mod ram_profile5;
#[doc = "single_tone_profile5 register accessor"]
pub type SingleToneProfile5<'a, I> =
    crate::Reg<'a, single_tone_profile5::SingleToneProfile5Spec, I>;
#[doc = "Single Tone Profile 5 Register"]
pub mod single_tone_profile5;
#[doc = "ram_profile6 register accessor"]
pub type RamProfile6<'a, I> = crate::Reg<'a, ram_profile6::RamProfile6Spec, I>;
#[doc = "RAM Profile 6 Register"]
pub mod ram_profile6;
#[doc = "single_tone_profile6 register accessor"]
pub type SingleToneProfile6<'a, I> =
    crate::Reg<'a, single_tone_profile6::SingleToneProfile6Spec, I>;
#[doc = "Single Tone Profile 6 Register"]
pub mod single_tone_profile6;
#[doc = "ram_profile7 register accessor"]
pub type RamProfile7<'a, I> = crate::Reg<'a, ram_profile7::RamProfile7Spec, I>;
#[doc = "RAM Profile 7 Register"]
pub mod ram_profile7;
#[doc = "single_tone_profile7 register accessor"]
pub type SingleToneProfile7<'a, I> =
    crate::Reg<'a, single_tone_profile7::SingleToneProfile7Spec, I>;
#[doc = "Single Tone Profile 7 Register"]
pub mod single_tone_profile7;
#[doc = "ram register accessor"]
pub type Ram<'a, I> = crate::Reg<'a, ram::RamSpec, I>;
#[doc = "RAM Register"]
pub mod ram;
