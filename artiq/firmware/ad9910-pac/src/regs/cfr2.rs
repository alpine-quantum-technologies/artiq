#[doc = "Register `cfr2` reader"]
pub struct R(crate::R<Cfr2Spec>);
impl core::ops::Deref for R {
    type Target = crate::R<Cfr2Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Cfr2Spec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Cfr2Spec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cfr2` writer"]
pub struct W(crate::W<Cfr2Spec>);
impl core::ops::Deref for W {
    type Target = crate::W<Cfr2Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<Cfr2Spec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Cfr2Spec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fm_gain` reader - FM gain"]
pub struct FmGainR(crate::FieldReader<u8>);
impl FmGainR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FmGainR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fm_gain` writer - FM gain"]
pub struct FmGainW<'a> {
    w: &'a mut W,
}
impl<'a> FmGainW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `parallel_data_port_enable` reader - Parallel data port enable"]
pub struct ParallelDataPortEnableR(crate::FieldReader<bool>);
impl ParallelDataPortEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ParallelDataPortEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parallel_data_port_enable` writer - Parallel data port enable"]
pub struct ParallelDataPortEnableW<'a> {
    w: &'a mut W,
}
impl<'a> ParallelDataPortEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `sync_timing_validation_disable` reader - Sync timing validation disable"]
pub struct SyncTimingValidationDisableR(crate::FieldReader<bool>);
impl SyncTimingValidationDisableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncTimingValidationDisableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_timing_validation_disable` writer - Sync timing validation disable"]
pub struct SyncTimingValidationDisableW<'a> {
    w: &'a mut W,
}
impl<'a> SyncTimingValidationDisableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `data_assembler_hold_last_value` reader - Data assembler hold last value"]
pub struct DataAssemblerHoldLastValueR(crate::FieldReader<bool>);
impl DataAssemblerHoldLastValueR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DataAssemblerHoldLastValueR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data_assembler_hold_last_value` writer - Data assembler hold last value"]
pub struct DataAssemblerHoldLastValueW<'a> {
    w: &'a mut W,
}
impl<'a> DataAssemblerHoldLastValueW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `matched_latency_enable` reader - Matched latency enable"]
pub struct MatchedLatencyEnableR(crate::FieldReader<bool>);
impl MatchedLatencyEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MatchedLatencyEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `matched_latency_enable` writer - Matched latency enable"]
pub struct MatchedLatencyEnableW<'a> {
    w: &'a mut W,
}
impl<'a> MatchedLatencyEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `tx_enable_invert` reader - TxEnable invert"]
pub struct TxEnableInvertR(crate::FieldReader<bool>);
impl TxEnableInvertR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TxEnableInvertR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_enable_invert` writer - TxEnable invert"]
pub struct TxEnableInvertW<'a> {
    w: &'a mut W,
}
impl<'a> TxEnableInvertW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `pdclk_invert` reader - PDCLK invert"]
pub struct PdclkInvertR(crate::FieldReader<bool>);
impl PdclkInvertR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PdclkInvertR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pdclk_invert` writer - PDCLK invert"]
pub struct PdclkInvertW<'a> {
    w: &'a mut W,
}
impl<'a> PdclkInvertW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `pdclk_enable` reader - PDCLK enable"]
pub struct PdclkEnableR(crate::FieldReader<bool>);
impl PdclkEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PdclkEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pdclk_enable` writer - PDCLK enable"]
pub struct PdclkEnableW<'a> {
    w: &'a mut W,
}
impl<'a> PdclkEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `io_update_rate_control` reader - I/O update rate control"]
pub struct IoUpdateRateControlR(crate::FieldReader<u8>);
impl IoUpdateRateControlR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IoUpdateRateControlR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_update_rate_control` writer - I/O update rate control"]
pub struct IoUpdateRateControlW<'a> {
    w: &'a mut W,
}
impl<'a> IoUpdateRateControlW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `read_effective_ftw` reader - Read effective FTW"]
pub struct ReadEffectiveFtwR(crate::FieldReader<bool>);
impl ReadEffectiveFtwR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ReadEffectiveFtwR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `read_effective_ftw` writer - Read effective FTW"]
pub struct ReadEffectiveFtwW<'a> {
    w: &'a mut W,
}
impl<'a> ReadEffectiveFtwW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `digital_ramp_no_dwell_low` reader - Digital ramp no-dwell low"]
pub struct DigitalRampNoDwellLowR(crate::FieldReader<bool>);
impl DigitalRampNoDwellLowR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DigitalRampNoDwellLowR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digital_ramp_no_dwell_low` writer - Digital ramp no-dwell low"]
pub struct DigitalRampNoDwellLowW<'a> {
    w: &'a mut W,
}
impl<'a> DigitalRampNoDwellLowW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `digital_ramp_no_dwell_high` reader - Digital ramp no-dwell high"]
pub struct DigitalRampNoDwellHighR(crate::FieldReader<bool>);
impl DigitalRampNoDwellHighR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DigitalRampNoDwellHighR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digital_ramp_no_dwell_high` writer - Digital ramp no-dwell high"]
pub struct DigitalRampNoDwellHighW<'a> {
    w: &'a mut W,
}
impl<'a> DigitalRampNoDwellHighW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `digital_ramp_enable` reader - Digital ramp enable"]
pub struct DigitalRampEnableR(crate::FieldReader<bool>);
impl DigitalRampEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DigitalRampEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digital_ramp_enable` writer - Digital ramp enable"]
pub struct DigitalRampEnableW<'a> {
    w: &'a mut W,
}
impl<'a> DigitalRampEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `digital_ramp_destination` reader - Digital ramp destination"]
pub struct DigitalRampDestinationR(crate::FieldReader<u8>);
impl DigitalRampDestinationR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DigitalRampDestinationR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digital_ramp_destination` writer - Digital ramp destination"]
pub struct DigitalRampDestinationW<'a> {
    w: &'a mut W,
}
impl<'a> DigitalRampDestinationW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `sync_clk_enable` reader - SYNC_CLK enable"]
pub struct SyncClkEnableR(crate::FieldReader<bool>);
impl SyncClkEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncClkEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_clk_enable` writer - SYNC_CLK enable"]
pub struct SyncClkEnableW<'a> {
    w: &'a mut W,
}
impl<'a> SyncClkEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `internal_io_update_active` reader - Internal I/O update active"]
pub struct InternalIoUpdateActiveR(crate::FieldReader<bool>);
impl InternalIoUpdateActiveR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for InternalIoUpdateActiveR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `internal_io_update_active` writer - Internal I/O update active"]
pub struct InternalIoUpdateActiveW<'a> {
    w: &'a mut W,
}
impl<'a> InternalIoUpdateActiveW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `enable_amplitude_scale_from_single_tone_profiles` reader - Enable amplitude scale from single tone profiles"]
pub struct EnableAmplitudeScaleFromSingleToneProfilesR(crate::FieldReader<bool>);
impl EnableAmplitudeScaleFromSingleToneProfilesR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EnableAmplitudeScaleFromSingleToneProfilesR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable_amplitude_scale_from_single_tone_profiles` writer - Enable amplitude scale from single tone profiles"]
pub struct EnableAmplitudeScaleFromSingleToneProfilesW<'a> {
    w: &'a mut W,
}
impl<'a> EnableAmplitudeScaleFromSingleToneProfilesW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - FM gain"]
    #[inline(always)]
    pub fn fm_gain(&self) -> FmGainR {
        FmGainR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Parallel data port enable"]
    #[inline(always)]
    pub fn parallel_data_port_enable(&self) -> ParallelDataPortEnableR {
        ParallelDataPortEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sync timing validation disable"]
    #[inline(always)]
    pub fn sync_timing_validation_disable(&self) -> SyncTimingValidationDisableR {
        SyncTimingValidationDisableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data assembler hold last value"]
    #[inline(always)]
    pub fn data_assembler_hold_last_value(&self) -> DataAssemblerHoldLastValueR {
        DataAssemblerHoldLastValueR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Matched latency enable"]
    #[inline(always)]
    pub fn matched_latency_enable(&self) -> MatchedLatencyEnableR {
        MatchedLatencyEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - TxEnable invert"]
    #[inline(always)]
    pub fn tx_enable_invert(&self) -> TxEnableInvertR {
        TxEnableInvertR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDCLK invert"]
    #[inline(always)]
    pub fn pdclk_invert(&self) -> PdclkInvertR {
        PdclkInvertR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDCLK enable"]
    #[inline(always)]
    pub fn pdclk_enable(&self) -> PdclkEnableR {
        PdclkEnableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - I/O update rate control"]
    #[inline(always)]
    pub fn io_update_rate_control(&self) -> IoUpdateRateControlR {
        IoUpdateRateControlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read effective FTW"]
    #[inline(always)]
    pub fn read_effective_ftw(&self) -> ReadEffectiveFtwR {
        ReadEffectiveFtwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital ramp no-dwell low"]
    #[inline(always)]
    pub fn digital_ramp_no_dwell_low(&self) -> DigitalRampNoDwellLowR {
        DigitalRampNoDwellLowR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital ramp no-dwell high"]
    #[inline(always)]
    pub fn digital_ramp_no_dwell_high(&self) -> DigitalRampNoDwellHighR {
        DigitalRampNoDwellHighR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital ramp enable"]
    #[inline(always)]
    pub fn digital_ramp_enable(&self) -> DigitalRampEnableR {
        DigitalRampEnableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Digital ramp destination"]
    #[inline(always)]
    pub fn digital_ramp_destination(&self) -> DigitalRampDestinationR {
        DigitalRampDestinationR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - SYNC_CLK enable"]
    #[inline(always)]
    pub fn sync_clk_enable(&self) -> SyncClkEnableR {
        SyncClkEnableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal I/O update active"]
    #[inline(always)]
    pub fn internal_io_update_active(&self) -> InternalIoUpdateActiveR {
        InternalIoUpdateActiveR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable amplitude scale from single tone profiles"]
    #[inline(always)]
    pub fn enable_amplitude_scale_from_single_tone_profiles(
        &self,
    ) -> EnableAmplitudeScaleFromSingleToneProfilesR {
        EnableAmplitudeScaleFromSingleToneProfilesR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FM gain"]
    #[inline(always)]
    pub fn fm_gain(&mut self) -> FmGainW {
        FmGainW { w: self }
    }
    #[doc = "Bit 4 - Parallel data port enable"]
    #[inline(always)]
    pub fn parallel_data_port_enable(&mut self) -> ParallelDataPortEnableW {
        ParallelDataPortEnableW { w: self }
    }
    #[doc = "Bit 5 - Sync timing validation disable"]
    #[inline(always)]
    pub fn sync_timing_validation_disable(&mut self) -> SyncTimingValidationDisableW {
        SyncTimingValidationDisableW { w: self }
    }
    #[doc = "Bit 6 - Data assembler hold last value"]
    #[inline(always)]
    pub fn data_assembler_hold_last_value(&mut self) -> DataAssemblerHoldLastValueW {
        DataAssemblerHoldLastValueW { w: self }
    }
    #[doc = "Bit 7 - Matched latency enable"]
    #[inline(always)]
    pub fn matched_latency_enable(&mut self) -> MatchedLatencyEnableW {
        MatchedLatencyEnableW { w: self }
    }
    #[doc = "Bit 9 - TxEnable invert"]
    #[inline(always)]
    pub fn tx_enable_invert(&mut self) -> TxEnableInvertW {
        TxEnableInvertW { w: self }
    }
    #[doc = "Bit 10 - PDCLK invert"]
    #[inline(always)]
    pub fn pdclk_invert(&mut self) -> PdclkInvertW {
        PdclkInvertW { w: self }
    }
    #[doc = "Bit 11 - PDCLK enable"]
    #[inline(always)]
    pub fn pdclk_enable(&mut self) -> PdclkEnableW {
        PdclkEnableW { w: self }
    }
    #[doc = "Bits 14:15 - I/O update rate control"]
    #[inline(always)]
    pub fn io_update_rate_control(&mut self) -> IoUpdateRateControlW {
        IoUpdateRateControlW { w: self }
    }
    #[doc = "Bit 16 - Read effective FTW"]
    #[inline(always)]
    pub fn read_effective_ftw(&mut self) -> ReadEffectiveFtwW {
        ReadEffectiveFtwW { w: self }
    }
    #[doc = "Bit 17 - Digital ramp no-dwell low"]
    #[inline(always)]
    pub fn digital_ramp_no_dwell_low(&mut self) -> DigitalRampNoDwellLowW {
        DigitalRampNoDwellLowW { w: self }
    }
    #[doc = "Bit 18 - Digital ramp no-dwell high"]
    #[inline(always)]
    pub fn digital_ramp_no_dwell_high(&mut self) -> DigitalRampNoDwellHighW {
        DigitalRampNoDwellHighW { w: self }
    }
    #[doc = "Bit 19 - Digital ramp enable"]
    #[inline(always)]
    pub fn digital_ramp_enable(&mut self) -> DigitalRampEnableW {
        DigitalRampEnableW { w: self }
    }
    #[doc = "Bits 20:21 - Digital ramp destination"]
    #[inline(always)]
    pub fn digital_ramp_destination(&mut self) -> DigitalRampDestinationW {
        DigitalRampDestinationW { w: self }
    }
    #[doc = "Bit 22 - SYNC_CLK enable"]
    #[inline(always)]
    pub fn sync_clk_enable(&mut self) -> SyncClkEnableW {
        SyncClkEnableW { w: self }
    }
    #[doc = "Bit 23 - Internal I/O update active"]
    #[inline(always)]
    pub fn internal_io_update_active(&mut self) -> InternalIoUpdateActiveW {
        InternalIoUpdateActiveW { w: self }
    }
    #[doc = "Bit 24 - Enable amplitude scale from single tone profiles"]
    #[inline(always)]
    pub fn enable_amplitude_scale_from_single_tone_profiles(
        &mut self,
    ) -> EnableAmplitudeScaleFromSingleToneProfilesW {
        EnableAmplitudeScaleFromSingleToneProfilesW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Function Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr2](index.html) module"]
pub struct Cfr2Spec;
impl crate::RegisterSpec for Cfr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr2::R](R) reader structure"]
impl crate::Readable for Cfr2Spec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr2::W](W) writer structure"]
impl crate::Writable for Cfr2Spec {
    type Writer = W;
}
#[doc = "`reset()` method sets cfr2 to value 0x0040_0820"]
impl crate::Resettable for Cfr2Spec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0820
    }
}

impl crate::Addressable for Cfr2Spec {
    #[inline(always)]
    fn address() -> u8 {
        0x01
    }
}
