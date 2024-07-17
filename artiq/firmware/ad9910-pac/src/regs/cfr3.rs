#[doc = "Register `cfr3` reader"]
pub struct R(crate::R<Cfr3Spec>);
impl core::ops::Deref for R {
    type Target = crate::R<Cfr3Spec>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Cfr3Spec>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Cfr3Spec>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cfr3` writer"]
pub struct W(crate::W<Cfr3Spec>);
impl core::ops::Deref for W {
    type Target = crate::W<Cfr3Spec>;
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
impl From<crate::W<Cfr3Spec>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Cfr3Spec>) -> Self {
        W(writer)
    }
}
#[doc = "Field `n` reader - Divide modulus of the REFCLK PLL feedback divider"]
pub struct NR(crate::FieldReader<u8>);
impl NR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `n` writer - Divide modulus of the REFCLK PLL feedback divider"]
pub struct NW<'a> {
    w: &'a mut W,
}
impl<'a> NW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
#[doc = "Field `pll_enable` reader - PLL enable"]
pub struct PllEnableR(crate::FieldReader<bool>);
impl PllEnableR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PllEnableR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pll_enable` writer - PLL enable"]
pub struct PllEnableW<'a> {
    w: &'a mut W,
}
impl<'a> PllEnableW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `pfd_reset` reader - PFD reset"]
pub struct PfdResetR(crate::FieldReader<bool>);
impl PfdResetR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PfdResetR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pfd_reset` writer - PFD reset"]
pub struct PfdResetW<'a> {
    w: &'a mut W,
}
impl<'a> PfdResetW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `refclk_input_divider_resetb` reader - REFCLK input divider ResetB"]
pub struct RefclkInputDividerResetbR(crate::FieldReader<bool>);
impl RefclkInputDividerResetbR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RefclkInputDividerResetbR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `refclk_input_divider_resetb` writer - REFCLK input divider ResetB"]
pub struct RefclkInputDividerResetbW<'a> {
    w: &'a mut W,
}
impl<'a> RefclkInputDividerResetbW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `refclk_input_divider_bypass` reader - REFCLK input divider bypass"]
pub struct RefclkInputDividerBypassR(crate::FieldReader<bool>);
impl RefclkInputDividerBypassR {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RefclkInputDividerBypassR {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `refclk_input_divider_bypass` writer - REFCLK input divider bypass"]
pub struct RefclkInputDividerBypassW<'a> {
    w: &'a mut W,
}
impl<'a> RefclkInputDividerBypassW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `i_cp` reader - Charge pump current in the REFCLK PLL"]
pub struct ICpR(crate::FieldReader<u8>);
impl ICpR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICpR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i_cp` writer - Charge pump current in the REFCLK PLL"]
pub struct ICpW<'a> {
    w: &'a mut W,
}
impl<'a> ICpW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 19)) | ((value as u32 & 7) << 19);
        self.w
    }
}
#[doc = "Field `vco_sel` reader - Frequency band selection for the REFCLK PLL VCO"]
pub struct VcoSelR(crate::FieldReader<u8>);
impl VcoSelR {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VcoSelR {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_sel` writer - Frequency band selection for the REFCLK PLL VCO"]
pub struct VcoSelW<'a> {
    w: &'a mut W,
}
impl<'a> VcoSelW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `drv0` reader - REFCLK_OUT pin control"]
pub struct Drv0R(crate::FieldReader<u8>);
impl Drv0R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Self(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Drv0R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `drv0` writer - REFCLK_OUT pin control"]
pub struct Drv0W<'a> {
    w: &'a mut W,
}
impl<'a> Drv0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Divide modulus of the REFCLK PLL feedback divider"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - PLL enable"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PllEnableR {
        PllEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PFD reset"]
    #[inline(always)]
    pub fn pfd_reset(&self) -> PfdResetR {
        PfdResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - REFCLK input divider ResetB"]
    #[inline(always)]
    pub fn refclk_input_divider_resetb(&self) -> RefclkInputDividerResetbR {
        RefclkInputDividerResetbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REFCLK input divider bypass"]
    #[inline(always)]
    pub fn refclk_input_divider_bypass(&self) -> RefclkInputDividerBypassR {
        RefclkInputDividerBypassR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Charge pump current in the REFCLK PLL"]
    #[inline(always)]
    pub fn i_cp(&self) -> ICpR {
        ICpR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Frequency band selection for the REFCLK PLL VCO"]
    #[inline(always)]
    pub fn vco_sel(&self) -> VcoSelR {
        VcoSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - REFCLK_OUT pin control"]
    #[inline(always)]
    pub fn drv0(&self) -> Drv0R {
        Drv0R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Divide modulus of the REFCLK PLL feedback divider"]
    #[inline(always)]
    pub fn n(&mut self) -> NW {
        NW { w: self }
    }
    #[doc = "Bit 8 - PLL enable"]
    #[inline(always)]
    pub fn pll_enable(&mut self) -> PllEnableW {
        PllEnableW { w: self }
    }
    #[doc = "Bit 10 - PFD reset"]
    #[inline(always)]
    pub fn pfd_reset(&mut self) -> PfdResetW {
        PfdResetW { w: self }
    }
    #[doc = "Bit 14 - REFCLK input divider ResetB"]
    #[inline(always)]
    pub fn refclk_input_divider_resetb(&mut self) -> RefclkInputDividerResetbW {
        RefclkInputDividerResetbW { w: self }
    }
    #[doc = "Bit 15 - REFCLK input divider bypass"]
    #[inline(always)]
    pub fn refclk_input_divider_bypass(&mut self) -> RefclkInputDividerBypassW {
        RefclkInputDividerBypassW { w: self }
    }
    #[doc = "Bits 19:21 - Charge pump current in the REFCLK PLL"]
    #[inline(always)]
    pub fn i_cp(&mut self) -> ICpW {
        ICpW { w: self }
    }
    #[doc = "Bits 24:26 - Frequency band selection for the REFCLK PLL VCO"]
    #[inline(always)]
    pub fn vco_sel(&mut self) -> VcoSelW {
        VcoSelW { w: self }
    }
    #[doc = "Bits 28:29 - REFCLK_OUT pin control"]
    #[inline(always)]
    pub fn drv0(&mut self) -> Drv0W {
        Drv0W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Function Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr3](index.html) module"]
pub struct Cfr3Spec;
impl crate::RegisterSpec for Cfr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr3::R](R) reader structure"]
impl crate::Readable for Cfr3Spec {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr3::W](W) writer structure"]
impl crate::Writable for Cfr3Spec {
    type Writer = W;
}
#[doc = "`reset()` method sets cfr3 to value 0x1f3f_4000"]
impl crate::Resettable for Cfr3Spec {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f3f_4000
    }
}

impl crate::Addressable for Cfr3Spec {
    #[inline(always)]
    fn address() -> u8 {
        0x02
    }
}
