use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[doc = "Register `multichip_sync` reader"]
pub struct R(crate::R<MultichipSyncSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<MultichipSyncSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MultichipSyncSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MultichipSyncSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `multichip_sync` writer"]
pub struct W(crate::W<MultichipSyncSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<MultichipSyncSpec>;
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
impl From<crate::W<MultichipSyncSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MultichipSyncSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_sync_receiver_delay` reader - Input sync receiver delay"]
pub struct InputSyncReceiverDelayR(crate::FieldReader<u8>);
impl InputSyncReceiverDelayR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for InputSyncReceiverDelayR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `input_sync_receiver_delay` writer - Input sync receiver delay"]
pub struct InputSyncReceiverDelayW<'a> {
    w: &'a mut W,
}
impl<'a> InputSyncReceiverDelayW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `output_sync_generator_delay` reader - Output sync generator delay"]
pub struct OutputSyncGeneratorDelayR(crate::FieldReader<u8>);
impl OutputSyncGeneratorDelayR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OutputSyncGeneratorDelayR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `output_sync_generator_delay` writer - Output sync generator delay"]
pub struct OutputSyncGeneratorDelayW<'a> {
    w: &'a mut W,
}
impl<'a> OutputSyncGeneratorDelayW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `sync_state_preset_value` reader - Sync state preset value"]
pub struct SyncStatePresetValueR(crate::FieldReader<u8>);
impl SyncStatePresetValueR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncStatePresetValueR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_state_preset_value` writer - Sync state preset value"]
pub struct SyncStatePresetValueW<'a> {
    w: &'a mut W,
}
impl<'a> SyncStatePresetValueW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}

#[doc = "Sync generator polarity"]
#[derive(
    Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum SyncGeneratorPolarityA {
    #[doc = "Synchronization clock generator coincident with the rising edge of SYSCLK"]
    RisingEdge = 0,

    #[doc = "Synchronization clock generator coincident with the falling edge of SYSCLK"]
    FallingEdge = 1,
}

impl Default for SyncGeneratorPolarityA {
    fn default() -> Self {
        Self::RisingEdge
    }
}

impl From<SyncGeneratorPolarityA> for bool {
    fn from(variant: SyncGeneratorPolarityA) -> Self {
        let value: u8 = variant.into();
        value != 0
    }
}

impl From<bool> for SyncGeneratorPolarityA {
    fn from(bit: bool) -> Self {
        if bit {
            Self::FallingEdge
        } else {
            Self::RisingEdge
        }
    }
}

#[doc = "Field `sync_generator_polarity` reader - Sync generator polarity"]
pub struct SyncGeneratorPolarityR(crate::FieldReader<bool>);
impl SyncGeneratorPolarityR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }

    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SyncGeneratorPolarityA> {
        Some(self.bits.into())
    }
}
impl core::ops::Deref for SyncGeneratorPolarityR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_generator_polarity` writer - Sync generator polarity"]
pub struct SyncGeneratorPolarityW<'a> {
    w: &'a mut W,
}
impl<'a> SyncGeneratorPolarityW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
    #[doc = "Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SyncGeneratorPolarityA) -> &'a mut W {
        self.bit(variant.into())
    }
}
#[doc = "Field `sync_generator_enable` reader - Sync generator enable"]
pub struct SyncGeneratorEnableR(crate::FieldReader<bool>);
impl SyncGeneratorEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncGeneratorEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_generator_enable` writer - Sync generator enable"]
pub struct SyncGeneratorEnableW<'a> {
    w: &'a mut W,
}
impl<'a> SyncGeneratorEnableW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `sync_receiver_enable` reader - Sync receiver enable"]
pub struct SyncReceiverEnableR(crate::FieldReader<bool>);
impl SyncReceiverEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncReceiverEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_receiver_enable` writer - Sync receiver enable"]
pub struct SyncReceiverEnableW<'a> {
    w: &'a mut W,
}
impl<'a> SyncReceiverEnableW<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `sync_validation_delay` reader - Sync validation delay"]
pub struct SyncValidationDelayR(crate::FieldReader<u8>);
impl SyncValidationDelayR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SyncValidationDelayR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sync_validation_delay` writer - Sync validation delay"]
pub struct SyncValidationDelayW<'a> {
    w: &'a mut W,
}
impl<'a> SyncValidationDelayW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Input sync receiver delay"]
    #[inline(always)]
    pub fn input_sync_receiver_delay(&self) -> InputSyncReceiverDelayR {
        InputSyncReceiverDelayR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Output sync generator delay"]
    #[inline(always)]
    pub fn output_sync_generator_delay(&self) -> OutputSyncGeneratorDelayR {
        OutputSyncGeneratorDelayR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23 - Sync state preset value"]
    #[inline(always)]
    pub fn sync_state_preset_value(&self) -> SyncStatePresetValueR {
        SyncStatePresetValueR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - Sync generator polarity"]
    #[inline(always)]
    pub fn sync_generator_polarity(&self) -> SyncGeneratorPolarityR {
        SyncGeneratorPolarityR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Sync generator enable"]
    #[inline(always)]
    pub fn sync_generator_enable(&self) -> SyncGeneratorEnableR {
        SyncGeneratorEnableR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Sync receiver enable"]
    #[inline(always)]
    pub fn sync_receiver_enable(&self) -> SyncReceiverEnableR {
        SyncReceiverEnableR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Sync validation delay"]
    #[inline(always)]
    pub fn sync_validation_delay(&self) -> SyncValidationDelayR {
        SyncValidationDelayR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Input sync receiver delay"]
    #[inline(always)]
    pub fn input_sync_receiver_delay(&mut self) -> InputSyncReceiverDelayW {
        InputSyncReceiverDelayW { w: self }
    }
    #[doc = "Bits 11:15 - Output sync generator delay"]
    #[inline(always)]
    pub fn output_sync_generator_delay(&mut self) -> OutputSyncGeneratorDelayW {
        OutputSyncGeneratorDelayW { w: self }
    }
    #[doc = "Bits 18:23 - Sync state preset value"]
    #[inline(always)]
    pub fn sync_state_preset_value(&mut self) -> SyncStatePresetValueW {
        SyncStatePresetValueW { w: self }
    }
    #[doc = "Bit 25 - Sync generator polarity"]
    #[inline(always)]
    pub fn sync_generator_polarity(&mut self) -> SyncGeneratorPolarityW {
        SyncGeneratorPolarityW { w: self }
    }
    #[doc = "Bit 26 - Sync generator enable"]
    #[inline(always)]
    pub fn sync_generator_enable(&mut self) -> SyncGeneratorEnableW {
        SyncGeneratorEnableW { w: self }
    }
    #[doc = "Bit 27 - Sync receiver enable"]
    #[inline(always)]
    pub fn sync_receiver_enable(&mut self) -> SyncReceiverEnableW {
        SyncReceiverEnableW { w: self }
    }
    #[doc = "Bits 28:31 - Sync validation delay"]
    #[inline(always)]
    pub fn sync_validation_delay(&mut self) -> SyncValidationDelayW {
        SyncValidationDelayW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multichip Sync Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multichip_sync](index.html) module"]
pub struct MultichipSyncSpec;
impl crate::RegisterSpec for MultichipSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [multichip_sync::R](R) reader structure"]
impl crate::Readable for MultichipSyncSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [multichip_sync::W](W) writer structure"]
impl crate::Writable for MultichipSyncSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets multichip_sync to value 0"]
impl crate::Resettable for MultichipSyncSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for MultichipSyncSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x0a
    }
}
