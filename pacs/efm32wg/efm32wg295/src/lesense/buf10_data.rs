#[doc = "Register `BUF10_DATA` reader"]
pub struct R(crate::R<BUF10_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF10_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF10_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF10_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF10_DATA` writer"]
pub struct W(crate::W<BUF10_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF10_DATA_SPEC>;
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
impl From<crate::W<BUF10_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF10_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Scan result buffer"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Scan result buffer"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, BUF10_DATA_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Scan result buffer"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan result buffer"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan results\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf10_data](index.html) module"]
pub struct BUF10_DATA_SPEC;
impl crate::RegisterSpec for BUF10_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf10_data::R](R) reader structure"]
impl crate::Readable for BUF10_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf10_data::W](W) writer structure"]
impl crate::Writable for BUF10_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF10_DATA to value 0"]
impl crate::Resettable for BUF10_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
