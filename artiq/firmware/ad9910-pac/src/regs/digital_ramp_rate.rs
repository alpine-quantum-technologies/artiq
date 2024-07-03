#[doc = "Register `digital_ramp_rate` reader"]
pub struct R(crate::R<DigitalRampRateSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<DigitalRampRateSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DigitalRampRateSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DigitalRampRateSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `digital_ramp_rate` writer"]
pub struct W(crate::W<DigitalRampRateSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<DigitalRampRateSpec>;
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
impl From<crate::W<DigitalRampRateSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DigitalRampRateSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `positive_slope_rate` reader - Digital ramp positive slope rate"]
pub struct PositiveSlopeRateR(crate::FieldReader<u16>);
impl PositiveSlopeRateR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PositiveSlopeRateR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `positive_slope_rate` writer - Digital ramp positive slope rate"]
pub struct PositiveSlopeRateW<'a> {
    w: &'a mut W,
}
impl<'a> PositiveSlopeRateW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `negative_slope_rate` reader - Digital ramp negative slope rate"]
pub struct NegativeSlopeRateR(crate::FieldReader<u16>);
impl NegativeSlopeRateR {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NegativeSlopeRateR {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `negative_slope_rate` writer - Digital ramp negative slope rate"]
pub struct NegativeSlopeRateW<'a> {
    w: &'a mut W,
}
impl<'a> NegativeSlopeRateW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Digital ramp positive slope rate"]
    #[inline(always)]
    pub fn positive_slope_rate(&self) -> PositiveSlopeRateR {
        PositiveSlopeRateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Digital ramp negative slope rate"]
    #[inline(always)]
    pub fn negative_slope_rate(&self) -> NegativeSlopeRateR {
        NegativeSlopeRateR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Digital ramp positive slope rate"]
    #[inline(always)]
    pub fn positive_slope_rate(&mut self) -> PositiveSlopeRateW {
        PositiveSlopeRateW { w: self }
    }
    #[doc = "Bits 16:31 - Digital ramp negative slope rate"]
    #[inline(always)]
    pub fn negative_slope_rate(&mut self) -> NegativeSlopeRateW {
        NegativeSlopeRateW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Ramp Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digital_ramp_rate](index.html) module"]
pub struct DigitalRampRateSpec;
impl crate::RegisterSpec for DigitalRampRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [digital_ramp_rate::R](R) reader structure"]
impl crate::Readable for DigitalRampRateSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [digital_ramp_rate::W](W) writer structure"]
impl crate::Writable for DigitalRampRateSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets digital_ramp_rate to value 0"]
impl crate::Resettable for DigitalRampRateSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for DigitalRampRateSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x0d
    }
}
