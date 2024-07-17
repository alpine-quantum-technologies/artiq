#[doc = "Register `ftw` reader"]
pub struct R(crate::R<FtwSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<FtwSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FtwSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FtwSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ftw` writer"]
pub struct W(crate::W<FtwSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<FtwSpec>;
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
impl From<crate::W<FtwSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FtwSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ftw` reader - Frequency tuning word"]
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
#[doc = "Field `ftw` writer - Frequency tuning word"]
pub struct FtwW<'a> {
    w: &'a mut W,
}
impl<'a> FtwW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Frequency tuning word"]
    #[inline(always)]
    pub fn ftw(&self) -> FtwR {
        FtwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frequency tuning word"]
    #[inline(always)]
    pub fn ftw(&mut self) -> FtwW {
        FtwW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Tuning Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftw](index.html) module"]
pub struct FtwSpec;
impl crate::RegisterSpec for FtwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftw::R](R) reader structure"]
impl crate::Readable for FtwSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftw::W](W) writer structure"]
impl crate::Writable for FtwSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets ftw to value 0"]
impl crate::Resettable for FtwSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for FtwSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x07
    }
}
