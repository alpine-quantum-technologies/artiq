#[doc = "Register `single_tone_profile3` reader"]
pub struct R(crate::R<SingleToneProfile3Spec>);
impl core::ops::Deref for R {
    type Target = crate::R<SingleToneProfile3Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SingleToneProfile3Spec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SingleToneProfile3Spec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `single_tone_profile3` writer"]
pub struct W(crate::W<SingleToneProfile3Spec>);
impl core::ops::Deref for W {
    type Target = crate::W<SingleToneProfile3Spec>;
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
impl From<crate::W<SingleToneProfile3Spec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SingleToneProfile3Spec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ftw` reader - Frequency tuning word 3"]
pub struct FtwR(crate::FieldReader<u32>);
impl FtwR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FtwR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ftw` writer - Frequency tuning word 3"]
pub struct FtwW<'a> {
    w: &'a mut W,
}
impl<'a> FtwW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u64 & 0xffff_ffff);
        self.w
    }
}
#[doc = "Field `pow` reader - Phase offset word 3"]
pub struct PowR(crate::FieldReader<u16>);
impl PowR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PowR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pow` writer - Phase offset word 3"]
pub struct PowW<'a> {
    w: &'a mut W,
}
impl<'a> PowW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 32)) | ((value as u64 & 0xffff) << 32);
        self.w
    }
}
#[doc = "Field `asf` reader - Amplitude scale factor 3"]
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
#[doc = "Field `asf` writer - Amplitude scale factor 3"]
pub struct AsfW<'a> {
    w: &'a mut W,
}
impl<'a> AsfW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 48)) | ((value as u64 & 0x3fff) << 48);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Frequency tuning word 3"]
    #[inline(always)]
    pub fn ftw(&self) -> FtwR {
        FtwR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 32:47 - Phase offset word 3"]
    #[inline(always)]
    pub fn pow(&self) -> PowR {
        PowR::new(((self.bits >> 32) & 0xffff) as u16)
    }
    #[doc = "Bits 48:61 - Amplitude scale factor 3"]
    #[inline(always)]
    pub fn asf(&self) -> AsfR {
        AsfR::new(((self.bits >> 48) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frequency tuning word 3"]
    #[inline(always)]
    pub fn ftw(&mut self) -> FtwW {
        FtwW { w: self }
    }
    #[doc = "Bits 32:47 - Phase offset word 3"]
    #[inline(always)]
    pub fn pow(&mut self) -> PowW {
        PowW { w: self }
    }
    #[doc = "Bits 48:61 - Amplitude scale factor 3"]
    #[inline(always)]
    pub fn asf(&mut self) -> AsfW {
        AsfW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Tone Profile 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [single_tone_profile3](index.html) module"]
pub struct SingleToneProfile3Spec;
impl crate::RegisterSpec for SingleToneProfile3Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [single_tone_profile3::R](R) reader structure"]
impl crate::Readable for SingleToneProfile3Spec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [single_tone_profile3::W](W) writer structure"]
impl crate::Writable for SingleToneProfile3Spec {
    type Writer = W;
}
#[doc = "`reset()` method sets single_tone_profile3 to value 0"]
impl crate::Resettable for SingleToneProfile3Spec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for SingleToneProfile3Spec {
    #[inline(always)]
    fn address() -> u8 {
        0x11
    }
}
