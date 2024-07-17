#[doc = "Register `digital_ramp_step_size` reader"]
pub struct R(crate::R<DigitalRampStepSizeSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<DigitalRampStepSizeSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DigitalRampStepSizeSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DigitalRampStepSizeSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `digital_ramp_step_size` writer"]
pub struct W(crate::W<DigitalRampStepSizeSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<DigitalRampStepSizeSpec>;
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
impl From<crate::W<DigitalRampStepSizeSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DigitalRampStepSizeSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `increment_step_size` reader - Digital ramp increment step size"]
pub struct IncrementStepSizeR(crate::FieldReader<u32>);
impl IncrementStepSizeR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IncrementStepSizeR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `increment_step_size` writer - Digital ramp increment step size"]
pub struct IncrementStepSizeW<'a> {
    w: &'a mut W,
}
impl<'a> IncrementStepSizeW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u64 & 0xffff_ffff);
        self.w
    }
}
#[doc = "Field `decrement_step_size` reader - Digital ramp decrement step size"]
pub struct DecrementStepSizeR(crate::FieldReader<u32>);
impl DecrementStepSizeR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DecrementStepSizeR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `decrement_step_size` writer - Digital ramp decrement step size"]
pub struct DecrementStepSizeW<'a> {
    w: &'a mut W,
}
impl<'a> DecrementStepSizeW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff_ffff << 32)) | ((value as u64 & 0xffff_ffff) << 32);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital ramp increment step size"]
    #[inline(always)]
    pub fn increment_step_size(&self) -> IncrementStepSizeR {
        IncrementStepSizeR::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bits 32:63 - Digital ramp decrement step size"]
    #[inline(always)]
    pub fn decrement_step_size(&self) -> DecrementStepSizeR {
        DecrementStepSizeR::new(((self.bits >> 32) & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital ramp increment step size"]
    #[inline(always)]
    pub fn increment_step_size(&mut self) -> IncrementStepSizeW {
        IncrementStepSizeW { w: self }
    }
    #[doc = "Bits 32:63 - Digital ramp decrement step size"]
    #[inline(always)]
    pub fn decrement_step_size(&mut self) -> DecrementStepSizeW {
        DecrementStepSizeW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Ramp Step Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digital_ramp_step_size](index.html) module"]
pub struct DigitalRampStepSizeSpec;
impl crate::RegisterSpec for DigitalRampStepSizeSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [digital_ramp_step_size::R](R) reader structure"]
impl crate::Readable for DigitalRampStepSizeSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [digital_ramp_step_size::W](W) writer structure"]
impl crate::Writable for DigitalRampStepSizeSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets digital_ramp_step_size to value 0"]
impl crate::Resettable for DigitalRampStepSizeSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for DigitalRampStepSizeSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x0c
    }
}
