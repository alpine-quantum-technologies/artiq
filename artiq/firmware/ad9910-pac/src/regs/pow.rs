#[doc = "Register `pow` reader"]
pub struct R(crate::R<PowSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<PowSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PowSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PowSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pow` writer"]
pub struct W(crate::W<PowSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<PowSpec>;
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
impl From<crate::W<PowSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PowSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pow` reader - Phase offset word"]
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
#[doc = "Field `pow` writer - Phase offset word"]
pub struct PowW<'a> {
    w: &'a mut W,
}
impl<'a> PowW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Phase offset word"]
    #[inline(always)]
    pub fn pow(&self) -> PowR {
        PowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Phase offset word"]
    #[inline(always)]
    pub fn pow(&mut self) -> PowW {
        PowW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase Offset Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pow](index.html) module"]
pub struct PowSpec;
impl crate::RegisterSpec for PowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [pow::R](R) reader structure"]
impl crate::Readable for PowSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pow::W](W) writer structure"]
impl crate::Writable for PowSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets pow to value 0"]
impl crate::Resettable for PowSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for PowSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x08
    }
}
