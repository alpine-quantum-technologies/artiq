#[doc = "Register `aux_dac_control` reader"]
pub struct R(crate::R<AuxDacControlSpec>);
impl core::ops::Deref for R {
    type Target = crate::R<AuxDacControlSpec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AuxDacControlSpec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AuxDacControlSpec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aux_dac_control` writer"]
pub struct W(crate::W<AuxDacControlSpec>);
impl core::ops::Deref for W {
    type Target = crate::W<AuxDacControlSpec>;
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
impl From<crate::W<AuxDacControlSpec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AuxDacControlSpec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fsc` reader - Full scale output current of the main DAC"]
pub struct FscR(crate::FieldReader<u8>);
impl FscR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FscR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fsc` writer - Full scale output current of the main DAC"]
pub struct FscW<'a> {
    w: &'a mut W,
}
impl<'a> FscW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Full scale output current of the main DAC"]
    #[inline(always)]
    pub fn fsc(&self) -> FscR {
        FscR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Full scale output current of the main DAC"]
    #[inline(always)]
    pub fn fsc(&mut self) -> FscW {
        FscW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux_dac_control](index.html) module"]
pub struct AuxDacControlSpec;
impl crate::RegisterSpec for AuxDacControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux_dac_control::R](R) reader structure"]
impl crate::Readable for AuxDacControlSpec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux_dac_control::W](W) writer structure"]
impl crate::Writable for AuxDacControlSpec {
    type Writer = W;
}
#[doc = "`reset()` method sets aux_dac_control to value 0x7f"]
impl crate::Resettable for AuxDacControlSpec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}

impl crate::Addressable for AuxDacControlSpec {
    #[inline(always)]
    fn address() -> u8 {
        0x03
    }
}
