#[doc = "Register `digital_ramp_limit` reader"]
pub struct R(crate::R<DigitalRampLimitSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<DigitalRampLimitSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DigitalRampLimitSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DigitalRampLimitSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `digital_ramp_limit` writer"]
pub struct W(crate::W<DigitalRampLimitSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<DigitalRampLimitSpec>;
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
impl From<crate::W<DigitalRampLimitSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DigitalRampLimitSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lower_limit` reader - Digital ramp lower limit"]
pub struct LowerLimitR(crate::FieldReader<u32>);
impl LowerLimitR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LowerLimitR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lower_limit` writer - Digital ramp lower limit"]
pub struct LowerLimitW<'a> {
    w: &'a mut W,
}
impl<'a> LowerLimitW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u64 & 0xffff_ffff);
        self.w
    }
}
#[doc = "Field `upper_limit` reader - Digital ramp upper limit"]
pub struct UpperLimitR(crate::FieldReader<u32>);
impl UpperLimitR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UpperLimitR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `upper_limit` writer - Digital ramp upper limit"]
pub struct UpperLimitW<'a> {
    w: &'a mut W,
}
impl<'a> UpperLimitW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff_ffff << 32)) | ((value as u64 & 0xffff_ffff) << 32);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital ramp lower limit"]
    #[inline(always)]
    pub fn lower_limit(&self) -> LowerLimitR {
        LowerLimitR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 32:63 - Digital ramp upper limit"]
    #[inline(always)]
    pub fn upper_limit(&self) -> UpperLimitR {
        UpperLimitR::new(((self.bits >> 32) & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital ramp lower limit"]
    #[inline(always)]
    pub fn lower_limit(&mut self) -> LowerLimitW {
        LowerLimitW { w: self }
    }
    #[doc = "Bits 32:63 - Digital ramp upper limit"]
    #[inline(always)]
    pub fn upper_limit(&mut self) -> UpperLimitW {
        UpperLimitW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Ramp Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digital_ramp_limit](index.html) module"]
pub struct DigitalRampLimitSpec;
impl crate::RegisterSpec for DigitalRampLimitSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [digital_ramp_limit::R](R) reader structure"]
impl crate::Readable for DigitalRampLimitSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [digital_ramp_limit::W](W) writer structure"]
impl crate::Writable for DigitalRampLimitSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets digital_ramp_limit to value 0"]
impl crate::Resettable for DigitalRampLimitSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for DigitalRampLimitSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x0b
    }
}
