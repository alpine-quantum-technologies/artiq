#[doc = "Register `ram_profile4` reader"]
pub struct R(crate::R<RamProfile4Spec>);
impl core::ops::Deref for R {
    type Target = crate::R<RamProfile4Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RamProfile4Spec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RamProfile4Spec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ram_profile4` writer"]
pub struct W(crate::W<RamProfile4Spec>);
impl core::ops::Deref for W {
    type Target = crate::W<RamProfile4Spec>;
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
impl From<crate::W<RamProfile4Spec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RamProfile4Spec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode_control` reader - RAM profile 4 mode control"]
pub struct ModeControlR(crate::FieldReader<u8>);
impl ModeControlR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ModeControlR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode_control` writer - RAM profile 4 mode control"]
pub struct ModeControlW<'a> {
    w: &'a mut W,
}
impl<'a> ModeControlW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u64 & 7);
        self.w
    }
}
#[doc = "Field `zero_crossing` reader - Zero-crossing function enable"]
pub struct ZeroCrossingR(crate::FieldReader<bool>);
impl ZeroCrossingR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZeroCrossingR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `zero_crossing` writer - Zero-crossing function enable"]
pub struct ZeroCrossingW<'a> {
    w: &'a mut W,
}
impl<'a> ZeroCrossingW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u64 & 1) << 3);
        self.w
    }
}
#[doc = "Field `no_dwell_high` reader - No-dwell high"]
pub struct NoDwellHighR(crate::FieldReader<bool>);
impl NoDwellHighR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NoDwellHighR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `no_dwell_high` writer - No-dwell high"]
pub struct NoDwellHighW<'a> {
    w: &'a mut W,
}
impl<'a> NoDwellHighW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u64 & 1) << 5);
        self.w
    }
}
#[doc = "Field `waveform_start_address` reader - RAM profile 4 waveform start address"]
pub struct WaveformStartAddressR(crate::FieldReader<u16>);
impl WaveformStartAddressR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WaveformStartAddressR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `waveform_start_address` writer - RAM profile 4 waveform start address"]
pub struct WaveformStartAddressW<'a> {
    w: &'a mut W,
}
impl<'a> WaveformStartAddressW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 14)) | ((value as u64 & 0x03ff) << 14);
        self.w
    }
}
#[doc = "Field `waveform_stop_address` reader - RAM profile 4 waveform end address"]
pub struct WaveformStopAddressR(crate::FieldReader<u16>);
impl WaveformStopAddressR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WaveformStopAddressR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `waveform_stop_address` writer - RAM profile 4 waveform end address"]
pub struct WaveformStopAddressW<'a> {
    w: &'a mut W,
}
impl<'a> WaveformStopAddressW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 30)) | ((value as u64 & 0x03ff) << 30);
        self.w
    }
}
#[doc = "Field `address_step_rate` reader - RAM profile 4 address step rate"]
pub struct AddressStepRateR(crate::FieldReader<u16>);
impl AddressStepRateR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AddressStepRateR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `address_step_rate` writer - RAM profile 4 address step rate"]
pub struct AddressStepRateW<'a> {
    w: &'a mut W,
}
impl<'a> AddressStepRateW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 40)) | ((value as u64 & 0xffff) << 40);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RAM profile 4 mode control"]
    #[inline(always)]
    pub fn mode_control(&self) -> ModeControlR {
        ModeControlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Zero-crossing function enable"]
    #[inline(always)]
    pub fn zero_crossing(&self) -> ZeroCrossingR {
        ZeroCrossingR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - No-dwell high"]
    #[inline(always)]
    pub fn no_dwell_high(&self) -> NoDwellHighR {
        NoDwellHighR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 14:23 - RAM profile 4 waveform start address"]
    #[inline(always)]
    pub fn waveform_start_address(&self) -> WaveformStartAddressR {
        WaveformStartAddressR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:39 - RAM profile 4 waveform end address"]
    #[inline(always)]
    pub fn waveform_stop_address(&self) -> WaveformStopAddressR {
        WaveformStopAddressR::new(((self.bits >> 30) & 0x03ff) as u16)
    }
    #[doc = "Bits 40:55 - RAM profile 4 address step rate"]
    #[inline(always)]
    pub fn address_step_rate(&self) -> AddressStepRateR {
        AddressStepRateR::new(((self.bits >> 40) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - RAM profile 4 mode control"]
    #[inline(always)]
    pub fn mode_control(&mut self) -> ModeControlW {
        ModeControlW { w: self }
    }
    #[doc = "Bit 3 - Zero-crossing function enable"]
    #[inline(always)]
    pub fn zero_crossing(&mut self) -> ZeroCrossingW {
        ZeroCrossingW { w: self }
    }
    #[doc = "Bit 5 - No-dwell high"]
    #[inline(always)]
    pub fn no_dwell_high(&mut self) -> NoDwellHighW {
        NoDwellHighW { w: self }
    }
    #[doc = "Bits 14:23 - RAM profile 4 waveform start address"]
    #[inline(always)]
    pub fn waveform_start_address(&mut self) -> WaveformStartAddressW {
        WaveformStartAddressW { w: self }
    }
    #[doc = "Bits 30:39 - RAM profile 4 waveform end address"]
    #[inline(always)]
    pub fn waveform_stop_address(&mut self) -> WaveformStopAddressW {
        WaveformStopAddressW { w: self }
    }
    #[doc = "Bits 40:55 - RAM profile 4 address step rate"]
    #[inline(always)]
    pub fn address_step_rate(&mut self) -> AddressStepRateW {
        AddressStepRateW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Profile 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_profile4](index.html) module"]
pub struct RamProfile4Spec;
impl crate::RegisterSpec for RamProfile4Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [ram_profile4::R](R) reader structure"]
impl crate::Readable for RamProfile4Spec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_profile4::W](W) writer structure"]
impl crate::Writable for RamProfile4Spec {
    type Writer = W;
}
#[doc = "`reset()` method sets ram_profile4 to value 0"]
impl crate::Resettable for RamProfile4Spec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for RamProfile4Spec {
    #[inline(always)]
    fn address() -> u8 {
        0x12
    }
}
