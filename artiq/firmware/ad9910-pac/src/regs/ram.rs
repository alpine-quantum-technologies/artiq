#[doc = "Register `ram` reader"]
pub struct R(crate::R<RamSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<RamSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RamSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RamSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ram` writer"]
pub struct W(crate::W<RamSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<RamSpec>;
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
impl From<crate::W<RamSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RamSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `word` reader - RAM word"]
pub struct WordR(crate::FieldReader<u32>);
impl WordR {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WordR {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `word` writer - RAM word"]
pub struct WordW<'a> {
    w: &'a mut W,
}
impl<'a> WordW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM word"]
    #[inline(always)]
    pub fn word(&self) -> WordR {
        WordR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RAM word"]
    #[inline(always)]
    pub fn word(&mut self) -> WordW {
        WordW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](index.html) module"]
pub struct RamSpec;
impl crate::RegisterSpec for RamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram::R](R) reader structure"]
impl crate::Readable for RamSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram::W](W) writer structure"]
impl crate::Writable for RamSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets ram to value 0"]
impl crate::Resettable for RamSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

impl crate::Addressable for RamSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x16
    }
}
