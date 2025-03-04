#[doc = "Register `INTFLAGD` reader"]
pub struct R(crate::R<INTFLAGD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGD` writer"]
pub struct W(crate::W<INTFLAGD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGD_SPEC>;
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
impl From<crate::W<INTFLAGD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERCOM4_` reader - SERCOM4"]
pub struct SERCOM4__R(crate::FieldReader<bool, bool>);
impl SERCOM4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM4_` writer - SERCOM4"]
pub struct SERCOM4__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM4__W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SERCOM5_` reader - SERCOM5"]
pub struct SERCOM5__R(crate::FieldReader<bool, bool>);
impl SERCOM5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM5_` writer - SERCOM5"]
pub struct SERCOM5__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM5__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ADC0_` reader - ADC0"]
pub struct ADC0__R(crate::FieldReader<bool, bool>);
impl ADC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_` writer - ADC0"]
pub struct ADC0__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ADC1_` reader - ADC1"]
pub struct ADC1__R(crate::FieldReader<bool, bool>);
impl ADC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_` writer - ADC1"]
pub struct ADC1__W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DAC_` reader - DAC"]
pub struct DAC__R(crate::FieldReader<bool, bool>);
impl DAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_` writer - DAC"]
pub struct DAC__W<'a> {
    w: &'a mut W,
}
impl<'a> DAC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PCC_` reader - PCC"]
pub struct PCC__R(crate::FieldReader<bool, bool>);
impl PCC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCC_` writer - PCC"]
pub struct PCC__W<'a> {
    w: &'a mut W,
}
impl<'a> PCC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&mut self) -> SERCOM4__W {
        SERCOM4__W { w: self }
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    pub fn sercom5_(&mut self) -> SERCOM5__W {
        SERCOM5__W { w: self }
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&mut self) -> ADC0__W {
        ADC0__W { w: self }
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&mut self) -> ADC1__W {
        ADC1__W { w: self }
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    pub fn dac_(&mut self) -> DAC__W {
        DAC__W { w: self }
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    pub fn pcc_(&mut self) -> PCC__W {
        PCC__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagd](index.html) module"]
pub struct INTFLAGD_SPEC;
impl crate::RegisterSpec for INTFLAGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagd::R](R) reader structure"]
impl crate::Readable for INTFLAGD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagd::W](W) writer structure"]
impl crate::Writable for INTFLAGD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGD to value 0"]
impl crate::Resettable for INTFLAGD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
