#[doc = "Register `COUNT_DITH4` reader"]
pub struct R(crate::R<COUNT_DITH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_DITH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_DITH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_DITH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT_DITH4` writer"]
pub struct W(crate::W<COUNT_DITH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_DITH4_SPEC>;
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
impl From<crate::W<COUNT_DITH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_DITH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Counter Value"]
pub struct COUNT_R(crate::FieldReader<u32, u32>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Counter Value"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | ((value as u32 & 0x000f_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:23 - Counter Value"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith4](index.html) module"]
pub struct COUNT_DITH4_SPEC;
impl crate::RegisterSpec for COUNT_DITH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_dith4::R](R) reader structure"]
impl crate::Readable for COUNT_DITH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count_dith4::W](W) writer structure"]
impl crate::Writable for COUNT_DITH4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT_DITH4 to value 0"]
impl crate::Resettable for COUNT_DITH4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
