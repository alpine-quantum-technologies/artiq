#[doc = "Register `asf` reader"]
pub struct R(crate::R<AsfSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<AsfSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AsfSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AsfSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asf` writer"]
pub struct W(crate::W<AsfSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<AsfSpec>;
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
impl From<crate::W<AsfSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AsfSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `amplitude_step_size` reader - Amplitude step size"]
pub struct AmplitudeStepSizeR(crate::FieldReader<u8>);
impl AmplitudeStepSizeR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AmplitudeStepSizeR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `amplitude_step_size` writer - Amplitude step size"]
pub struct AmplitudeStepSizeW<'a> {
    w: &'a mut W,
}
impl<'a> AmplitudeStepSizeW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `asf` reader - Amplitude scale factor"]
pub struct AsfR(crate::FieldReader<u16>);
impl AsfR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AsfR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `asf` writer - Amplitude scale factor"]
pub struct AsfW<'a> {
    w: &'a mut W,
}
impl<'a> AsfW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `amplitude_ramp_rate` reader - Amplitude ramp rate"]
pub struct AmplitudeRampRateR(crate::FieldReader<u16>);
impl AmplitudeRampRateR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AmplitudeRampRateR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `amplitude_ramp_rate` writer - Amplitude ramp rate"]
pub struct AmplitudeRampRateW<'a> {
    w: &'a mut W,
}
impl<'a> AmplitudeRampRateW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Amplitude step size"]
    #[inline(always)]
    pub fn amplitude_step_size(&self) -> AmplitudeStepSizeR {
        AmplitudeStepSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - Amplitude scale factor"]
    #[inline(always)]
    pub fn asf(&self) -> AsfR {
        AsfR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:31 - Amplitude ramp rate"]
    #[inline(always)]
    pub fn amplitude_ramp_rate(&self) -> AmplitudeRampRateR {
        AmplitudeRampRateR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amplitude step size"]
    #[inline(always)]
    pub fn amplitude_step_size(&mut self) -> AmplitudeStepSizeW {
        AmplitudeStepSizeW { w: self }
    }
    #[doc = "Bits 2:15 - Amplitude scale factor"]
    #[inline(always)]
    pub fn asf(&mut self) -> AsfW {
        AsfW { w: self }
    }
    #[doc = "Bits 16:31 - Amplitude ramp rate"]
    #[inline(always)]
    pub fn amplitude_ramp_rate(&mut self) -> AmplitudeRampRateW {
        AmplitudeRampRateW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amplitude Scale Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asf](index.html) module"]
pub struct AsfSpec;
impl crate::RegisterSpec for AsfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [asf::R](R) reader structure"]
impl crate::Readable for AsfSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asf::W](W) writer structure"]
impl crate::Writable for AsfSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets asf to value 0"]
impl crate::Resettable for AsfSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for AsfSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x09
    }
}
