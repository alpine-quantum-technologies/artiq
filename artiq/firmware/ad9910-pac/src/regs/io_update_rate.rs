#[doc = "Register `io_update_rate` reader"]
pub struct R(crate::R<IoUpdateRateSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<IoUpdateRateSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IoUpdateRateSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IoUpdateRateSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `io_update_rate` writer"]
pub struct W(crate::W<IoUpdateRateSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<IoUpdateRateSpec>;
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
impl From<crate::W<IoUpdateRateSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IoUpdateRateSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_update_rate` reader - I/O update rate"]
pub struct IoUpdateRateR(crate::FieldReader<u32>);
impl IoUpdateRateR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IoUpdateRateR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_update_rate` writer - I/O update rate"]
pub struct IoUpdateRateW<'a> {
    w: &'a mut W,
}
impl<'a> IoUpdateRateW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - I/O update rate"]
    #[inline(always)]
    pub fn io_update_rate(&self) -> IoUpdateRateR {
        IoUpdateRateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - I/O update rate"]
    #[inline(always)]
    pub fn io_update_rate(&mut self) -> IoUpdateRateW {
        IoUpdateRateW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Update Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_update_rate](index.html) module"]
pub struct IoUpdateRateSpec;
impl crate::RegisterSpec for IoUpdateRateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_update_rate::R](R) reader structure"]
impl crate::Readable for IoUpdateRateSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_update_rate::W](W) writer structure"]
impl crate::Writable for IoUpdateRateSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets io_update_rate to value 0xffff_ffff"]
impl crate::Resettable for IoUpdateRateSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}

impl crate::Addressable for IoUpdateRateSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x04
    }
}
