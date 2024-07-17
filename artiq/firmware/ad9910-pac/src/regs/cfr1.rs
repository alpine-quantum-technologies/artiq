#[doc = "Register `cfr1` reader"]
pub struct R(crate::R<Cfr1Spec>);
impl core::ops::Deref for R {
    type Target = crate::R<Cfr1Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Cfr1Spec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Cfr1Spec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cfr1` writer"]
pub struct W(crate::W<Cfr1Spec>);
impl core::ops::Deref for W {
    type Target = crate::W<Cfr1Spec>;
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
impl From<crate::W<Cfr1Spec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Cfr1Spec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lsb_first` reader - LSB first"]
pub struct LsbFirstR(crate::FieldReader<bool>);
impl LsbFirstR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LsbFirstR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lsb_first` writer - LSB first"]
pub struct LsbFirstW<'a> {
    w: &'a mut W,
}
impl<'a> LsbFirstW<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `sdio_input_only` reader - SDIO input only"]
pub struct SdioInputOnlyR(crate::FieldReader<bool>);
impl SdioInputOnlyR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SdioInputOnlyR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdio_input_only` writer - SDIO input only"]
pub struct SdioInputOnlyW<'a> {
    w: &'a mut W,
}
impl<'a> SdioInputOnlyW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `external_power_down_ctrl` reader - External power-down control"]
pub struct ExternalPowerDownCtrlR(crate::FieldReader<bool>);
impl ExternalPowerDownCtrlR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ExternalPowerDownCtrlR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `external_power_down_ctrl` writer - External power-down control"]
pub struct ExternalPowerDownCtrlW<'a> {
    w: &'a mut W,
}
impl<'a> ExternalPowerDownCtrlW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `aux_dac_power_down` reader - Aux DAC power-down"]
pub struct AuxDacPowerDownR(crate::FieldReader<bool>);
impl AuxDacPowerDownR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AuxDacPowerDownR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `aux_dac_power_down` writer - Aux DAC power-down"]
pub struct AuxDacPowerDownW<'a> {
    w: &'a mut W,
}
impl<'a> AuxDacPowerDownW<'a> {
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
#[doc = "Field `refclk_input_power_down` reader - REFCLK input power-down"]
pub struct RefclkInputPowerDownR(crate::FieldReader<bool>);
impl RefclkInputPowerDownR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RefclkInputPowerDownR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `refclk_input_power_down` writer - REFCLK input power-down"]
pub struct RefclkInputPowerDownW<'a> {
    w: &'a mut W,
}
impl<'a> RefclkInputPowerDownW<'a> {
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
#[doc = "Field `dac_power_down` reader - DAC power-down"]
pub struct DacPowerDownR(crate::FieldReader<bool>);
impl DacPowerDownR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DacPowerDownR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dac_power_down` writer - DAC power-down"]
pub struct DacPowerDownW<'a> {
    w: &'a mut W,
}
impl<'a> DacPowerDownW<'a> {
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
#[doc = "Field `digital_power_down` reader - Digital power-down"]
pub struct DigitalPowerDownR(crate::FieldReader<bool>);
impl DigitalPowerDownR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DigitalPowerDownR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `digital_power_down` writer - Digital power-down"]
pub struct DigitalPowerDownW<'a> {
    w: &'a mut W,
}
impl<'a> DigitalPowerDownW<'a> {
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
#[doc = "Field `select_auto_osk` reader - Select auto OSK"]
pub struct SelectAutoOskR(crate::FieldReader<bool>);
impl SelectAutoOskR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SelectAutoOskR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `select_auto_osk` writer - Select auto OSK"]
pub struct SelectAutoOskW<'a> {
    w: &'a mut W,
}
impl<'a> SelectAutoOskW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `osk_enable` reader - OSK enable"]
pub struct OskEnableR(crate::FieldReader<bool>);
impl OskEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OskEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osk_enable` writer - OSK enable"]
pub struct OskEnableW<'a> {
    w: &'a mut W,
}
impl<'a> OskEnableW<'a> {
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
#[doc = "Field `load_arr_at_io_update` reader - Load ARR @ I/O update"]
pub struct LoadArrAtIoUpdateR(crate::FieldReader<bool>);
impl LoadArrAtIoUpdateR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LoadArrAtIoUpdateR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `load_arr_at_io_update` writer - Load ARR @ I/O update"]
pub struct LoadArrAtIoUpdateW<'a> {
    w: &'a mut W,
}
impl<'a> LoadArrAtIoUpdateW<'a> {
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
#[doc = "Field `clear_phase_accumulator` reader - Clear phase accumulator"]
pub struct ClearPhaseAccumulatorR(crate::FieldReader<bool>);
impl ClearPhaseAccumulatorR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ClearPhaseAccumulatorR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clear_phase_accumulator` writer - Clear phase accumulator"]
pub struct ClearPhaseAccumulatorW<'a> {
    w: &'a mut W,
}
impl<'a> ClearPhaseAccumulatorW<'a> {
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
#[doc = "Field `clear_digital_ramp_accumulator` reader - Clear digital ramp accumulator"]
pub struct ClearDigitalRampAccumulatorR(crate::FieldReader<bool>);
impl ClearDigitalRampAccumulatorR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ClearDigitalRampAccumulatorR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clear_digital_ramp_accumulator` writer - Clear digital ramp accumulator"]
pub struct ClearDigitalRampAccumulatorW<'a> {
    w: &'a mut W,
}
impl<'a> ClearDigitalRampAccumulatorW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `autoclear_phase_accumulator` reader - Autoclear phase accumulator"]
pub struct AutoclearPhaseAccumulatorR(crate::FieldReader<bool>);
impl AutoclearPhaseAccumulatorR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AutoclearPhaseAccumulatorR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `autoclear_phase_accumulator` writer - Autoclear phase accumulator"]
pub struct AutoclearPhaseAccumulatorW<'a> {
    w: &'a mut W,
}
impl<'a> AutoclearPhaseAccumulatorW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `autoclear_digital_ramp_accumulator` reader - Autoclear digital ramp accumulator"]
pub struct AutoclearDigitalRampAccumulatorR(crate::FieldReader<bool>);
impl AutoclearDigitalRampAccumulatorR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AutoclearDigitalRampAccumulatorR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `autoclear_digital_ramp_accumulator` writer - Autoclear digital ramp accumulator"]
pub struct AutoclearDigitalRampAccumulatorW<'a> {
    w: &'a mut W,
}
impl<'a> AutoclearDigitalRampAccumulatorW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `load_lrr_at_io_update` reader - Load LRR @ I/O update"]
pub struct LoadLrrAtIoUpdateR(crate::FieldReader<bool>);
impl LoadLrrAtIoUpdateR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LoadLrrAtIoUpdateR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `load_lrr_at_io_update` writer - Load LRR @ I/O update"]
pub struct LoadLrrAtIoUpdateW<'a> {
    w: &'a mut W,
}
impl<'a> LoadLrrAtIoUpdateW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `select_dds_sine_output` reader - Select DDS sine output"]
pub struct SelectDdsSineOutputR(crate::FieldReader<bool>);
impl SelectDdsSineOutputR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SelectDdsSineOutputR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `select_dds_sine_output` writer - Select DDS sine output"]
pub struct SelectDdsSineOutputW<'a> {
    w: &'a mut W,
}
impl<'a> SelectDdsSineOutputW<'a> {
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
#[doc = "Field `internal_profile_control` reader - Internal profile control"]
pub struct InternalProfileControlR(crate::FieldReader<u8>);
impl InternalProfileControlR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for InternalProfileControlR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `internal_profile_control` writer - Internal profile control"]
pub struct InternalProfileControlW<'a> {
    w: &'a mut W,
}
impl<'a> InternalProfileControlW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 17)) | ((value as u32 & 0x0f) << 17);
        self.w
    }
}
#[doc = "Field `inverse_sinc_filter_enable` reader - Inverse sinc filter enable"]
pub struct InverseSincFilterEnableR(crate::FieldReader<bool>);
impl InverseSincFilterEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for InverseSincFilterEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inverse_sinc_filter_enable` writer - Inverse sinc filter enable"]
pub struct InverseSincFilterEnableW<'a> {
    w: &'a mut W,
}
impl<'a> InverseSincFilterEnableW<'a> {
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
#[doc = "Field `manual_osk_external_control` reader - Manuel OSK external control"]
pub struct ManualOskExternalControlR(crate::FieldReader<bool>);
impl ManualOskExternalControlR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ManualOskExternalControlR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `manual_osk_external_control` writer - Manuel OSK external control"]
pub struct ManualOskExternalControlW<'a> {
    w: &'a mut W,
}
impl<'a> ManualOskExternalControlW<'a> {
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
#[doc = "Field `ram_playback_destination` reader - RAM playback destination"]
pub struct RamPlaybackDestinationR(crate::FieldReader<u8>);
impl RamPlaybackDestinationR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RamPlaybackDestinationR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ram_playback_destination` writer - RAM playback destination"]
pub struct RamPlaybackDestinationW<'a> {
    w: &'a mut W,
}
impl<'a> RamPlaybackDestinationW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `ram_enable` reader - RAM enable"]
pub struct RamEnableR(crate::FieldReader<bool>);
impl RamEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RamEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ram_enable` writer - RAM enable"]
pub struct RamEnableW<'a> {
    w: &'a mut W,
}
impl<'a> RamEnableW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSB first"]
    #[inline(always)]
    pub fn lsb_first(&self) -> LsbFirstR {
        LsbFirstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIO input only"]
    #[inline(always)]
    pub fn sdio_input_only(&self) -> SdioInputOnlyR {
        SdioInputOnlyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - External power-down control"]
    #[inline(always)]
    pub fn external_power_down_ctrl(&self) -> ExternalPowerDownCtrlR {
        ExternalPowerDownCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Aux DAC power-down"]
    #[inline(always)]
    pub fn aux_dac_power_down(&self) -> AuxDacPowerDownR {
        AuxDacPowerDownR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REFCLK input power-down"]
    #[inline(always)]
    pub fn refclk_input_power_down(&self) -> RefclkInputPowerDownR {
        RefclkInputPowerDownR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC power-down"]
    #[inline(always)]
    pub fn dac_power_down(&self) -> DacPowerDownR {
        DacPowerDownR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital power-down"]
    #[inline(always)]
    pub fn digital_power_down(&self) -> DigitalPowerDownR {
        DigitalPowerDownR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select auto OSK"]
    #[inline(always)]
    pub fn select_auto_osk(&self) -> SelectAutoOskR {
        SelectAutoOskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OSK enable"]
    #[inline(always)]
    pub fn osk_enable(&self) -> OskEnableR {
        OskEnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Load ARR @ I/O update"]
    #[inline(always)]
    pub fn load_arr_at_io_update(&self) -> LoadArrAtIoUpdateR {
        LoadArrAtIoUpdateR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear phase accumulator"]
    #[inline(always)]
    pub fn clear_phase_accumulator(&self) -> ClearPhaseAccumulatorR {
        ClearPhaseAccumulatorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clear digital ramp accumulator"]
    #[inline(always)]
    pub fn clear_digital_ramp_accumulator(&self) -> ClearDigitalRampAccumulatorR {
        ClearDigitalRampAccumulatorR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Autoclear phase accumulator"]
    #[inline(always)]
    pub fn autoclear_phase_accumulator(&self) -> AutoclearPhaseAccumulatorR {
        AutoclearPhaseAccumulatorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Autoclear digital ramp accumulator"]
    #[inline(always)]
    pub fn autoclear_digital_ramp_accumulator(&self) -> AutoclearDigitalRampAccumulatorR {
        AutoclearDigitalRampAccumulatorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Load LRR @ I/O update"]
    #[inline(always)]
    pub fn load_lrr_at_io_update(&self) -> LoadLrrAtIoUpdateR {
        LoadLrrAtIoUpdateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select DDS sine output"]
    #[inline(always)]
    pub fn select_dds_sine_output(&self) -> SelectDdsSineOutputR {
        SelectDdsSineOutputR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Internal profile control"]
    #[inline(always)]
    pub fn internal_profile_control(&self) -> InternalProfileControlR {
        InternalProfileControlR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Inverse sinc filter enable"]
    #[inline(always)]
    pub fn inverse_sinc_filter_enable(&self) -> InverseSincFilterEnableR {
        InverseSincFilterEnableR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Manuel OSK external control"]
    #[inline(always)]
    pub fn manual_osk_external_control(&self) -> ManualOskExternalControlR {
        ManualOskExternalControlR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 29:30 - RAM playback destination"]
    #[inline(always)]
    pub fn ram_playback_destination(&self) -> RamPlaybackDestinationR {
        RamPlaybackDestinationR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - RAM enable"]
    #[inline(always)]
    pub fn ram_enable(&self) -> RamEnableR {
        RamEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSB first"]
    #[inline(always)]
    pub fn lsb_first(&mut self) -> LsbFirstW {
        LsbFirstW { w: self }
    }
    #[doc = "Bit 1 - SDIO input only"]
    #[inline(always)]
    pub fn sdio_input_only(&mut self) -> SdioInputOnlyW {
        SdioInputOnlyW { w: self }
    }
    #[doc = "Bit 3 - External power-down control"]
    #[inline(always)]
    pub fn external_power_down_ctrl(&mut self) -> ExternalPowerDownCtrlW {
        ExternalPowerDownCtrlW { w: self }
    }
    #[doc = "Bit 4 - Aux DAC power-down"]
    #[inline(always)]
    pub fn aux_dac_power_down(&mut self) -> AuxDacPowerDownW {
        AuxDacPowerDownW { w: self }
    }
    #[doc = "Bit 5 - REFCLK input power-down"]
    #[inline(always)]
    pub fn refclk_input_power_down(&mut self) -> RefclkInputPowerDownW {
        RefclkInputPowerDownW { w: self }
    }
    #[doc = "Bit 6 - DAC power-down"]
    #[inline(always)]
    pub fn dac_power_down(&mut self) -> DacPowerDownW {
        DacPowerDownW { w: self }
    }
    #[doc = "Bit 7 - Digital power-down"]
    #[inline(always)]
    pub fn digital_power_down(&mut self) -> DigitalPowerDownW {
        DigitalPowerDownW { w: self }
    }
    #[doc = "Bit 8 - Select auto OSK"]
    #[inline(always)]
    pub fn select_auto_osk(&mut self) -> SelectAutoOskW {
        SelectAutoOskW { w: self }
    }
    #[doc = "Bit 9 - OSK enable"]
    #[inline(always)]
    pub fn osk_enable(&mut self) -> OskEnableW {
        OskEnableW { w: self }
    }
    #[doc = "Bit 10 - Load ARR @ I/O update"]
    #[inline(always)]
    pub fn load_arr_at_io_update(&mut self) -> LoadArrAtIoUpdateW {
        LoadArrAtIoUpdateW { w: self }
    }
    #[doc = "Bit 11 - Clear phase accumulator"]
    #[inline(always)]
    pub fn clear_phase_accumulator(&mut self) -> ClearPhaseAccumulatorW {
        ClearPhaseAccumulatorW { w: self }
    }
    #[doc = "Bit 12 - Clear digital ramp accumulator"]
    #[inline(always)]
    pub fn clear_digital_ramp_accumulator(&mut self) -> ClearDigitalRampAccumulatorW {
        ClearDigitalRampAccumulatorW { w: self }
    }
    #[doc = "Bit 13 - Autoclear phase accumulator"]
    #[inline(always)]
    pub fn autoclear_phase_accumulator(&mut self) -> AutoclearPhaseAccumulatorW {
        AutoclearPhaseAccumulatorW { w: self }
    }
    #[doc = "Bit 14 - Autoclear digital ramp accumulator"]
    #[inline(always)]
    pub fn autoclear_digital_ramp_accumulator(&mut self) -> AutoclearDigitalRampAccumulatorW {
        AutoclearDigitalRampAccumulatorW { w: self }
    }
    #[doc = "Bit 15 - Load LRR @ I/O update"]
    #[inline(always)]
    pub fn load_lrr_at_io_update(&mut self) -> LoadLrrAtIoUpdateW {
        LoadLrrAtIoUpdateW { w: self }
    }
    #[doc = "Bit 16 - Select DDS sine output"]
    #[inline(always)]
    pub fn select_dds_sine_output(&mut self) -> SelectDdsSineOutputW {
        SelectDdsSineOutputW { w: self }
    }
    #[doc = "Bits 17:20 - Internal profile control"]
    #[inline(always)]
    pub fn internal_profile_control(&mut self) -> InternalProfileControlW {
        InternalProfileControlW { w: self }
    }
    #[doc = "Bit 22 - Inverse sinc filter enable"]
    #[inline(always)]
    pub fn inverse_sinc_filter_enable(&mut self) -> InverseSincFilterEnableW {
        InverseSincFilterEnableW { w: self }
    }
    #[doc = "Bit 23 - Manuel OSK external control"]
    #[inline(always)]
    pub fn manual_osk_external_control(&mut self) -> ManualOskExternalControlW {
        ManualOskExternalControlW { w: self }
    }
    #[doc = "Bits 29:30 - RAM playback destination"]
    #[inline(always)]
    pub fn ram_playback_destination(&mut self) -> RamPlaybackDestinationW {
        RamPlaybackDestinationW { w: self }
    }
    #[doc = "Bit 31 - RAM enable"]
    #[inline(always)]
    pub fn ram_enable(&mut self) -> RamEnableW {
        RamEnableW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Function Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr1](index.html) module"]
pub struct Cfr1Spec;
impl crate::RegisterSpec for Cfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr1::R](R) reader structure"]
impl crate::Readable for Cfr1Spec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr1::W](W) writer structure"]
impl crate::Writable for Cfr1Spec {
    type Writer = W;
}
#[doc = "`reset()` method sets cfr1 to value 0"]
impl crate::Resettable for Cfr1Spec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
impl crate::Addressable for Cfr1Spec {
    #[inline(always)]
    fn address() -> u8 {
        0
    }
}
