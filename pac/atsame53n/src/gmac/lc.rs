#[doc = "Register `LC` reader"]
pub struct R(crate::R<LC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCOL` reader - Late Collisions"]
pub struct LCOL_R(crate::FieldReader<u16, u16>);
impl LCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCOL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc](index.html) module"]
pub struct LC_SPEC;
impl crate::RegisterSpec for LC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc::R](R) reader structure"]
impl crate::Readable for LC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC to value 0"]
impl crate::Resettable for LC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
