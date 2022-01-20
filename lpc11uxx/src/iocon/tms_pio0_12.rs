#[doc = "Register `TMS_PIO0_12` reader"]
pub struct R(crate::R<TMS_PIO0_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMS_PIO0_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMS_PIO0_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMS_PIO0_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMS_PIO0_12` writer"]
pub struct W(crate::W<TMS_PIO0_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMS_PIO0_12_SPEC>;
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
impl From<crate::W<TMS_PIO0_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMS_PIO0_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function. Values 0x4 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: TMS."]
    TMS = 0,
    #[doc = "1: PIO0_12."]
    PIO0_12 = 1,
    #[doc = "2: AD1."]
    AD1 = 2,
    #[doc = "3: CT32B1_CAP0."]
    CT32B1_CAP0 = 3,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function. Values 0x4 to 0x7 are reserved."]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::TMS),
            1 => Some(FUNC_A::PIO0_12),
            2 => Some(FUNC_A::AD1),
            3 => Some(FUNC_A::CT32B1_CAP0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMS`"]
    #[inline(always)]
    pub fn is_tms(&self) -> bool {
        **self == FUNC_A::TMS
    }
    #[doc = "Checks if the value of the field is `PIO0_12`"]
    #[inline(always)]
    pub fn is_pio0_12(&self) -> bool {
        **self == FUNC_A::PIO0_12
    }
    #[doc = "Checks if the value of the field is `AD1`"]
    #[inline(always)]
    pub fn is_ad1(&self) -> bool {
        **self == FUNC_A::AD1
    }
    #[doc = "Checks if the value of the field is `CT32B1_CAP0`"]
    #[inline(always)]
    pub fn is_ct32b1_cap0(&self) -> bool {
        **self == FUNC_A::CT32B1_CAP0
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function. Values 0x4 to 0x7 are reserved."]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TMS."]
    #[inline(always)]
    pub fn tms(self) -> &'a mut W {
        self.variant(FUNC_A::TMS)
    }
    #[doc = "PIO0_12."]
    #[inline(always)]
    pub fn pio0_12(self) -> &'a mut W {
        self.variant(FUNC_A::PIO0_12)
    }
    #[doc = "AD1."]
    #[inline(always)]
    pub fn ad1(self) -> &'a mut W {
        self.variant(FUNC_A::AD1)
    }
    #[doc = "CT32B1_CAP0."]
    #[inline(always)]
    pub fn ct32b1_cap0(self) -> &'a mut W {
        self.variant(FUNC_A::CT32B1_CAP0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    FLOATING = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater mode."]
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::FLOATING,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLOATING`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        **self == MODE_A::FLOATING
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        **self == MODE_A::REPEATER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(MODE_A::FLOATING)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Enable."]
    ENABLED = 1,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYS` reader - Hysteresis."]
pub struct HYS_R(crate::FieldReader<bool, HYS_A>);
impl HYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::DISABLED,
            true => HYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HYS_A::ENABLED
    }
}
impl core::ops::Deref for HYS_R {
    type Target = crate::FieldReader<bool, HYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYS` writer - Hysteresis."]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HYS_A::DISABLED)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HYS_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_A {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    NOT_INVERTED = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    INVERTED = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert input"]
pub struct INV_R(crate::FieldReader<bool, INV_A>);
impl INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::NOT_INVERTED,
            true => INV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == INV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == INV_A::INVERTED
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - Invert input"]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(INV_A::NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(INV_A::INVERTED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Selects Analog/Digital mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMODE_A {
    #[doc = "0: Analog input mode."]
    ANALOG = 0,
    #[doc = "1: Digital functional mode."]
    DIGITAL = 1,
}
impl From<ADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMODE` reader - Selects Analog/Digital mode."]
pub struct ADMODE_R(crate::FieldReader<bool, ADMODE_A>);
impl ADMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMODE_A {
        match self.bits {
            false => ADMODE_A::ANALOG,
            true => ADMODE_A::DIGITAL,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == ADMODE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        **self == ADMODE_A::DIGITAL
    }
}
impl core::ops::Deref for ADMODE_R {
    type Target = crate::FieldReader<bool, ADMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMODE` writer - Selects Analog/Digital mode."]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Analog input mode."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(ADMODE_A::ANALOG)
    }
    #[doc = "Digital functional mode."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(ADMODE_A::DIGITAL)
    }
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
#[doc = "Selects 10 ns input glitch filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTR_A {
    #[doc = "0: Filter enabled."]
    ENABLED = 0,
    #[doc = "1: Filter disabled."]
    DISABLED = 1,
}
impl From<FILTR_A> for bool {
    #[inline(always)]
    fn from(variant: FILTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTR` reader - Selects 10 ns input glitch filter."]
pub struct FILTR_R(crate::FieldReader<bool, FILTR_A>);
impl FILTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTR_A {
        match self.bits {
            false => FILTR_A::ENABLED,
            true => FILTR_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FILTR_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FILTR_A::DISABLED
    }
}
impl core::ops::Deref for FILTR_R {
    type Target = crate::FieldReader<bool, FILTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTR` writer - Selects 10 ns input glitch filter."]
pub struct FILTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTR_A::ENABLED)
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTR_A::DISABLED)
    }
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
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "0: Disable."]
    DISABLED = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode."]
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Open-drain mode."]
pub struct OD_R(crate::FieldReader<bool, OD_A>);
impl OD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::DISABLED,
            true => OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == OD_A::OPEN_DRAIN
    }
}
impl core::ops::Deref for OD_R {
    type Target = crate::FieldReader<bool, OD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD` writer - Open-drain mode."]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OD_A::DISABLED)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&self) -> FILTR_R {
        FILTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function. Values 0x4 to 0x7 are reserved."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&mut self) -> FILTR_W {
        FILTR_W { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tms_pio0_12](index.html) module"]
pub struct TMS_PIO0_12_SPEC;
impl crate::RegisterSpec for TMS_PIO0_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tms_pio0_12::R](R) reader structure"]
impl crate::Readable for TMS_PIO0_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tms_pio0_12::W](W) writer structure"]
impl crate::Writable for TMS_PIO0_12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMS_PIO0_12 to value 0x90"]
impl crate::Resettable for TMS_PIO0_12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
