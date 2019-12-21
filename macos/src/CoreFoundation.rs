use crate::acl::_acl;
#[allow(unused_imports)]
use crate::objc::*;
use crate::NSObject::NSObject;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use objrs::objrs;
#[allow(unused_imports)]
use std::mem;
#[allow(unused_imports)]
use std::ptr;
#[repr(C)]
pub struct __CFString {
  opaque: u32,
}
pub type CFPropertyListRef = *mut c_void;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFComparisonResult {
  kCFCompareLessThan = -1,
  kCFCompareEqualTo = 0,
  kCFCompareGreaterThan = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFRange {
  pub location: isize,
  pub length: isize,
}
#[repr(C)]
pub struct __CFNull {
  opaque: u32,
}
#[repr(C)]
pub struct __CFAllocator {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFAllocatorContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub allocate: Option<extern "C" fn(isize, usize, *mut c_void) -> *mut c_void>,
  pub reallocate: Option<extern "C" fn(*mut c_void, isize, usize, *mut c_void) -> *mut c_void>,
  pub deallocate: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
  pub preferredSize: Option<extern "C" fn(isize, usize, *mut c_void) -> isize>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFArrayCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
}
#[repr(C)]
pub struct __CFArray {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFBagCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
  pub hash: Option<extern "C" fn(*mut c_void) -> usize>,
}
#[repr(C)]
pub struct __CFBag {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFBinaryHeapCompareContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFBinaryHeapCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub compare: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> CFComparisonResult>,
}
#[repr(C)]
pub struct __CFBinaryHeap {
  opaque: u32,
}
pub type CFBit = u32;
#[repr(C)]
pub struct __CFBitVector {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum __CFByteOrder {
  CFByteOrderUnknown = 0,
  CFByteOrderLittleEndian = 1,
  CFByteOrderBigEndian = 2,
}
pub type CFByteOrder = isize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFSwappedFloat32 {
  pub v: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFSwappedFloat64 {
  pub v: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFDictionaryKeyCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
  pub hash: Option<extern "C" fn(*mut c_void) -> usize>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFDictionaryValueCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
}
#[repr(C)]
pub struct __CFDictionary {
  opaque: u32,
}
pub type CFNotificationName = *mut __CFString;
#[repr(C)]
pub struct __CFNotificationCenter {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFNotificationSuspensionBehavior {
  CFNotificationSuspensionBehaviorDrop = 1,
  CFNotificationSuspensionBehaviorCoalesce = 2,
  CFNotificationSuspensionBehaviorHold = 3,
  CFNotificationSuspensionBehaviorDeliverImmediately = 4,
}
pub type CFLocaleIdentifier = *mut __CFString;
pub type CFLocaleKey = *mut __CFString;
#[repr(C)]
pub struct __CFLocale {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFLocaleLanguageDirection {
  kCFLocaleLanguageDirectionUnknown = 0,
  kCFLocaleLanguageDirectionLeftToRight = 1,
  kCFLocaleLanguageDirectionRightToLeft = 2,
  kCFLocaleLanguageDirectionTopToBottom = 3,
  kCFLocaleLanguageDirectionBottomToTop = 4,
}
pub type CFCalendarIdentifier = *mut __CFString;
pub type CFAbsoluteTime = f64;
#[repr(C)]
pub struct __CFDate {
  opaque: u32,
}
#[repr(C)]
pub struct __CFTimeZone {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFGregorianDate {
  pub year: i32,
  pub month: i8,
  pub day: i8,
  pub hour: i8,
  pub minute: i8,
  pub second: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFGregorianUnits {
  pub years: i32,
  pub months: i32,
  pub days: i32,
  pub hours: i32,
  pub minutes: i32,
  pub seconds: f64,
}
bitflags! { # [ repr ( C ) ] pub struct CFGregorianUnitFlags : usize { const kCFGregorianUnitsYears = 1 ; const kCFGregorianUnitsMonths = 2 ; const kCFGregorianUnitsDays = 4 ; const kCFGregorianUnitsHours = 8 ; const kCFGregorianUnitsMinutes = 16 ; const kCFGregorianUnitsSeconds = 32 ; const kCFGregorianAllUnits = 16777215 ; } }
#[repr(C)]
pub struct __CFData {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFDataSearchFlags : usize { const kCFDataSearchBackwards = 1 ; const kCFDataSearchAnchored = 2 ; } }
#[repr(C)]
pub struct __CFCharacterSet {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFCharacterSetPredefinedSet {
  kCFCharacterSetControl = 1,
  kCFCharacterSetWhitespace = 2,
  kCFCharacterSetWhitespaceAndNewline = 3,
  kCFCharacterSetDecimalDigit = 4,
  kCFCharacterSetLetter = 5,
  kCFCharacterSetLowercaseLetter = 6,
  kCFCharacterSetUppercaseLetter = 7,
  kCFCharacterSetNonBase = 8,
  kCFCharacterSetDecomposable = 9,
  kCFCharacterSetAlphaNumeric = 10,
  kCFCharacterSetPunctuation = 11,
  kCFCharacterSetCapitalizedLetter = 13,
  kCFCharacterSetSymbol = 14,
  kCFCharacterSetNewline = 15,
  kCFCharacterSetIllegal = 12,
}
pub type CFStringEncoding = u32;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CFStringBuiltInEncodings {
  kCFStringEncodingMacRoman = 0,
  kCFStringEncodingWindowsLatin1 = 1280,
  kCFStringEncodingISOLatin1 = 513,
  kCFStringEncodingNextStepLatin = 2817,
  kCFStringEncodingASCII = 1536,
  kCFStringEncodingUnicode = 256,
  kCFStringEncodingUTF8 = 134217984,
  kCFStringEncodingNonLossyASCII = 3071,
  kCFStringEncodingUTF16BE = 268435712,
  kCFStringEncodingUTF16LE = 335544576,
  kCFStringEncodingUTF32 = 201326848,
  kCFStringEncodingUTF32BE = 402653440,
  kCFStringEncodingUTF32LE = 469762304,
}
bitflags! { # [ repr ( C ) ] pub struct CFStringCompareFlags : usize { const kCFCompareCaseInsensitive = 1 ; const kCFCompareBackwards = 4 ; const kCFCompareAnchored = 8 ; const kCFCompareNonliteral = 16 ; const kCFCompareLocalized = 32 ; const kCFCompareNumerically = 64 ; const kCFCompareDiacriticInsensitive = 128 ; const kCFCompareWidthInsensitive = 256 ; const kCFCompareForcedOrdering = 512 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFStringNormalizationForm {
  kCFStringNormalizationFormD = 0,
  kCFStringNormalizationFormKD = 1,
  kCFStringNormalizationFormC = 2,
  kCFStringNormalizationFormKC = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFStringInlineBuffer {
  pub buffer: [u16; 64],
  pub theString: *mut __CFString,
  pub directUniCharBuffer: *mut u16,
  pub directCStringBuffer: *mut i8,
  pub rangeToBuffer: CFRange,
  pub bufferedRangeStart: isize,
  pub bufferedRangeEnd: isize,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFTimeZoneNameStyle {
  kCFTimeZoneNameStyleStandard = 0,
  kCFTimeZoneNameStyleShortStandard = 1,
  kCFTimeZoneNameStyleDaylightSaving = 2,
  kCFTimeZoneNameStyleShortDaylightSaving = 3,
  kCFTimeZoneNameStyleGeneric = 4,
  kCFTimeZoneNameStyleShortGeneric = 5,
}
#[repr(C)]
pub struct __CFCalendar {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFCalendarUnit : usize { const kCFCalendarUnitEra = 2 ; const kCFCalendarUnitYear = 4 ; const kCFCalendarUnitMonth = 8 ; const kCFCalendarUnitDay = 16 ; const kCFCalendarUnitHour = 32 ; const kCFCalendarUnitMinute = 64 ; const kCFCalendarUnitSecond = 128 ; const kCFCalendarUnitWeek = 256 ; const kCFCalendarUnitWeekday = 512 ; const kCFCalendarUnitWeekdayOrdinal = 1024 ; const kCFCalendarUnitQuarter = 2048 ; const kCFCalendarUnitWeekOfMonth = 4096 ; const kCFCalendarUnitWeekOfYear = 8192 ; const kCFCalendarUnitYearForWeekOfYear = 16384 ; } }
pub type CFDateFormatterKey = *mut __CFString;
#[repr(C)]
pub struct __CFDateFormatter {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFDateFormatterStyle {
  kCFDateFormatterNoStyle = 0,
  kCFDateFormatterShortStyle = 1,
  kCFDateFormatterMediumStyle = 2,
  kCFDateFormatterLongStyle = 3,
  kCFDateFormatterFullStyle = 4,
}
bitflags! { # [ repr ( C ) ] pub struct CFISO8601DateFormatOptions : usize { const kCFISO8601DateFormatWithYear = 1 ; const kCFISO8601DateFormatWithMonth = 2 ; const kCFISO8601DateFormatWithWeekOfYear = 4 ; const kCFISO8601DateFormatWithDay = 16 ; const kCFISO8601DateFormatWithTime = 32 ; const kCFISO8601DateFormatWithTimeZone = 64 ; const kCFISO8601DateFormatWithSpaceBetweenDateAndTime = 128 ; const kCFISO8601DateFormatWithDashSeparatorInDate = 256 ; const kCFISO8601DateFormatWithColonSeparatorInTime = 512 ; const kCFISO8601DateFormatWithColonSeparatorInTimeZone = 1024 ; const kCFISO8601DateFormatWithFractionalSeconds = 2048 ; const kCFISO8601DateFormatWithFullDate = 275 ; const kCFISO8601DateFormatWithFullTime = 1632 ; const kCFISO8601DateFormatWithInternetDateTime = 1907 ; } }
pub type CFErrorDomain = *mut __CFString;
#[repr(C)]
pub struct __CFError {
  opaque: u32,
}
#[repr(C)]
pub struct __CFBoolean {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFNumberType {
  kCFNumberSInt8Type = 1,
  kCFNumberSInt16Type = 2,
  kCFNumberSInt32Type = 3,
  kCFNumberSInt64Type = 4,
  kCFNumberFloat32Type = 5,
  kCFNumberFloat64Type = 6,
  kCFNumberCharType = 7,
  kCFNumberShortType = 8,
  kCFNumberIntType = 9,
  kCFNumberLongType = 10,
  kCFNumberLongLongType = 11,
  kCFNumberFloatType = 12,
  kCFNumberDoubleType = 13,
  kCFNumberCFIndexType = 14,
  kCFNumberNSIntegerType = 15,
  kCFNumberCGFloatType = 16,
}
#[repr(C)]
pub struct __CFNumber {
  opaque: u32,
}
pub type CFNumberFormatterKey = *mut __CFString;
#[repr(C)]
pub struct __CFNumberFormatter {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFNumberFormatterStyle {
  kCFNumberFormatterNoStyle = 0,
  kCFNumberFormatterDecimalStyle = 1,
  kCFNumberFormatterCurrencyStyle = 2,
  kCFNumberFormatterPercentStyle = 3,
  kCFNumberFormatterScientificStyle = 4,
  kCFNumberFormatterSpellOutStyle = 5,
  kCFNumberFormatterOrdinalStyle = 6,
  kCFNumberFormatterCurrencyISOCodeStyle = 8,
  kCFNumberFormatterCurrencyPluralStyle = 9,
  kCFNumberFormatterCurrencyAccountingStyle = 10,
}
bitflags! { # [ repr ( C ) ] pub struct CFNumberFormatterOptionFlags : usize { const kCFNumberFormatterParseIntegersOnly = 1 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFNumberFormatterRoundingMode {
  kCFNumberFormatterRoundCeiling = 0,
  kCFNumberFormatterRoundFloor = 1,
  kCFNumberFormatterRoundDown = 2,
  kCFNumberFormatterRoundUp = 3,
  kCFNumberFormatterRoundHalfEven = 4,
  kCFNumberFormatterRoundHalfDown = 5,
  kCFNumberFormatterRoundHalfUp = 6,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFNumberFormatterPadPosition {
  kCFNumberFormatterPadBeforePrefix = 0,
  kCFNumberFormatterPadAfterPrefix = 1,
  kCFNumberFormatterPadBeforeSuffix = 2,
  kCFNumberFormatterPadAfterSuffix = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFURLPathStyle {
  kCFURLPOSIXPathStyle = 0,
  kCFURLHFSPathStyle = 1,
  kCFURLWindowsPathStyle = 2,
}
#[repr(C)]
pub struct __CFURL {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFURLComponentType {
  kCFURLComponentScheme = 1,
  kCFURLComponentNetLocation = 2,
  kCFURLComponentPath = 3,
  kCFURLComponentResourceSpecifier = 4,
  kCFURLComponentUser = 5,
  kCFURLComponentPassword = 6,
  kCFURLComponentUserInfo = 7,
  kCFURLComponentHost = 8,
  kCFURLComponentPort = 9,
  kCFURLComponentParameterString = 10,
  kCFURLComponentQuery = 11,
  kCFURLComponentFragment = 12,
}
#[repr(C)]
pub struct FSRef {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFURLBookmarkCreationOptions : usize { const kCFURLBookmarkCreationMinimalBookmarkMask = 512 ; const kCFURLBookmarkCreationSuitableForBookmarkFile = 1024 ; const kCFURLBookmarkCreationWithSecurityScope = 2048 ; const kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess = 4096 ; const kCFURLBookmarkCreationPreferFileIDResolutionMask = 256 ; } }
bitflags! { # [ repr ( C ) ] pub struct CFURLBookmarkResolutionOptions : usize { const kCFURLBookmarkResolutionWithoutUIMask = 256 ; const kCFURLBookmarkResolutionWithoutMountingMask = 512 ; const kCFURLBookmarkResolutionWithSecurityScope = 1024 ; } }
pub type CFURLBookmarkFileCreationOptions = usize;
pub type CFRunLoopMode = *mut __CFString;
#[repr(C)]
pub struct __CFRunLoop {
  opaque: u32,
}
#[repr(C)]
pub struct __CFRunLoopSource {
  opaque: u32,
}
#[repr(C)]
pub struct __CFRunLoopObserver {
  opaque: u32,
}
#[repr(C)]
pub struct __CFRunLoopTimer {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CFRunLoopRunResult {
  kCFRunLoopRunFinished = 1,
  kCFRunLoopRunStopped = 2,
  kCFRunLoopRunTimedOut = 3,
  kCFRunLoopRunHandledSource = 4,
}
bitflags! { # [ repr ( C ) ] pub struct CFRunLoopActivity : usize { const kCFRunLoopEntry = 1 ; const kCFRunLoopBeforeTimers = 2 ; const kCFRunLoopBeforeSources = 4 ; const kCFRunLoopBeforeWaiting = 32 ; const kCFRunLoopAfterWaiting = 64 ; const kCFRunLoopExit = 128 ; const kCFRunLoopAllActivities = 268435455 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFRunLoopSourceContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
  pub hash: Option<extern "C" fn(*mut c_void) -> usize>,
  pub schedule: Option<extern "C" fn(*mut c_void, *mut __CFRunLoop, *mut __CFString) -> ()>,
  pub cancel: Option<extern "C" fn(*mut c_void, *mut __CFRunLoop, *mut __CFString) -> ()>,
  pub perform: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFRunLoopSourceContext1 {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
  pub hash: Option<extern "C" fn(*mut c_void) -> usize>,
  pub getPort: Option<extern "C" fn(*mut c_void) -> u32>,
  pub perform:
    Option<extern "C" fn(*mut c_void, isize, *mut __CFAllocator, *mut c_void) -> *mut c_void>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFRunLoopObserverContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFRunLoopTimerContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFSocket {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFSocketError {
  kCFSocketSuccess = 0,
  kCFSocketError = -1,
  kCFSocketTimeout = -2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFSocketSignature {
  pub protocolFamily: i32,
  pub socketType: i32,
  pub protocol: i32,
  pub address: *mut __CFData,
}
bitflags! { # [ repr ( C ) ] pub struct CFSocketCallBackType : usize { const kCFSocketNoCallBack = 0 ; const kCFSocketReadCallBack = 1 ; const kCFSocketAcceptCallBack = 2 ; const kCFSocketDataCallBack = 3 ; const kCFSocketConnectCallBack = 4 ; const kCFSocketWriteCallBack = 8 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFSocketContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFStreamError {
  pub domain: isize,
  pub error: i32,
}
pub type CFStreamPropertyKey = *mut __CFString;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFStreamStatus {
  kCFStreamStatusNotOpen = 0,
  kCFStreamStatusOpening = 1,
  kCFStreamStatusOpen = 2,
  kCFStreamStatusReading = 3,
  kCFStreamStatusWriting = 4,
  kCFStreamStatusAtEnd = 5,
  kCFStreamStatusClosed = 6,
  kCFStreamStatusError = 7,
}
bitflags! { # [ repr ( C ) ] pub struct CFStreamEventType : usize { const kCFStreamEventNone = 0 ; const kCFStreamEventOpenCompleted = 1 ; const kCFStreamEventHasBytesAvailable = 2 ; const kCFStreamEventCanAcceptBytes = 4 ; const kCFStreamEventErrorOccurred = 8 ; const kCFStreamEventEndEncountered = 16 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFStreamClientContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFReadStream {
  opaque: u32,
}
#[repr(C)]
pub struct __CFWriteStream {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFStreamErrorDomain {
  kCFStreamErrorDomainCustom = -1,
  kCFStreamErrorDomainPOSIX = 1,
  kCFStreamErrorDomainMacOSStatus = 2,
}
bitflags! { # [ repr ( C ) ] pub struct CFPropertyListMutabilityOptions : usize { const kCFPropertyListImmutable = 0 ; const kCFPropertyListMutableContainers = 1 ; const kCFPropertyListMutableContainersAndLeaves = 2 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFPropertyListFormat {
  kCFPropertyListOpenStepFormat = 1,
  kCFPropertyListXMLFormat_v1_0 = 100,
  kCFPropertyListBinaryFormat_v1_0 = 200,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFSetCallBacks {
  pub version: isize,
  pub retain: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut __CFAllocator, *mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub equal: Option<extern "C" fn(*mut c_void, *mut c_void) -> u8>,
  pub hash: Option<extern "C" fn(*mut c_void) -> usize>,
}
#[repr(C)]
pub struct __CFSet {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFStringEncodings {
  kCFStringEncodingMacJapanese = 1,
  kCFStringEncodingMacChineseTrad = 2,
  kCFStringEncodingMacKorean = 3,
  kCFStringEncodingMacArabic = 4,
  kCFStringEncodingMacHebrew = 5,
  kCFStringEncodingMacGreek = 6,
  kCFStringEncodingMacCyrillic = 7,
  kCFStringEncodingMacDevanagari = 9,
  kCFStringEncodingMacGurmukhi = 10,
  kCFStringEncodingMacGujarati = 11,
  kCFStringEncodingMacOriya = 12,
  kCFStringEncodingMacBengali = 13,
  kCFStringEncodingMacTamil = 14,
  kCFStringEncodingMacTelugu = 15,
  kCFStringEncodingMacKannada = 16,
  kCFStringEncodingMacMalayalam = 17,
  kCFStringEncodingMacSinhalese = 18,
  kCFStringEncodingMacBurmese = 19,
  kCFStringEncodingMacKhmer = 20,
  kCFStringEncodingMacThai = 21,
  kCFStringEncodingMacLaotian = 22,
  kCFStringEncodingMacGeorgian = 23,
  kCFStringEncodingMacArmenian = 24,
  kCFStringEncodingMacChineseSimp = 25,
  kCFStringEncodingMacTibetan = 26,
  kCFStringEncodingMacMongolian = 27,
  kCFStringEncodingMacEthiopic = 28,
  kCFStringEncodingMacCentralEurRoman = 29,
  kCFStringEncodingMacVietnamese = 30,
  kCFStringEncodingMacExtArabic = 31,
  kCFStringEncodingMacSymbol = 33,
  kCFStringEncodingMacDingbats = 34,
  kCFStringEncodingMacTurkish = 35,
  kCFStringEncodingMacCroatian = 36,
  kCFStringEncodingMacIcelandic = 37,
  kCFStringEncodingMacRomanian = 38,
  kCFStringEncodingMacCeltic = 39,
  kCFStringEncodingMacGaelic = 40,
  kCFStringEncodingMacFarsi = 140,
  kCFStringEncodingMacUkrainian = 152,
  kCFStringEncodingMacInuit = 236,
  kCFStringEncodingMacVT100 = 252,
  kCFStringEncodingMacHFS = 255,
  kCFStringEncodingISOLatin2 = 514,
  kCFStringEncodingISOLatin3 = 515,
  kCFStringEncodingISOLatin4 = 516,
  kCFStringEncodingISOLatinCyrillic = 517,
  kCFStringEncodingISOLatinArabic = 518,
  kCFStringEncodingISOLatinGreek = 519,
  kCFStringEncodingISOLatinHebrew = 520,
  kCFStringEncodingISOLatin5 = 521,
  kCFStringEncodingISOLatin6 = 522,
  kCFStringEncodingISOLatinThai = 523,
  kCFStringEncodingISOLatin7 = 525,
  kCFStringEncodingISOLatin8 = 526,
  kCFStringEncodingISOLatin9 = 527,
  kCFStringEncodingISOLatin10 = 528,
  kCFStringEncodingDOSLatinUS = 1024,
  kCFStringEncodingDOSGreek = 1029,
  kCFStringEncodingDOSBalticRim = 1030,
  kCFStringEncodingDOSLatin1 = 1040,
  kCFStringEncodingDOSGreek1 = 1041,
  kCFStringEncodingDOSLatin2 = 1042,
  kCFStringEncodingDOSCyrillic = 1043,
  kCFStringEncodingDOSTurkish = 1044,
  kCFStringEncodingDOSPortuguese = 1045,
  kCFStringEncodingDOSIcelandic = 1046,
  kCFStringEncodingDOSHebrew = 1047,
  kCFStringEncodingDOSCanadianFrench = 1048,
  kCFStringEncodingDOSArabic = 1049,
  kCFStringEncodingDOSNordic = 1050,
  kCFStringEncodingDOSRussian = 1051,
  kCFStringEncodingDOSGreek2 = 1052,
  kCFStringEncodingDOSThai = 1053,
  kCFStringEncodingDOSJapanese = 1056,
  kCFStringEncodingDOSChineseSimplif = 1057,
  kCFStringEncodingDOSKorean = 1058,
  kCFStringEncodingDOSChineseTrad = 1059,
  kCFStringEncodingWindowsLatin2 = 1281,
  kCFStringEncodingWindowsCyrillic = 1282,
  kCFStringEncodingWindowsGreek = 1283,
  kCFStringEncodingWindowsLatin5 = 1284,
  kCFStringEncodingWindowsHebrew = 1285,
  kCFStringEncodingWindowsArabic = 1286,
  kCFStringEncodingWindowsBalticRim = 1287,
  kCFStringEncodingWindowsVietnamese = 1288,
  kCFStringEncodingWindowsKoreanJohab = 1296,
  kCFStringEncodingANSEL = 1537,
  kCFStringEncodingJIS_X0201_76 = 1568,
  kCFStringEncodingJIS_X0208_83 = 1569,
  kCFStringEncodingJIS_X0208_90 = 1570,
  kCFStringEncodingJIS_X0212_90 = 1571,
  kCFStringEncodingJIS_C6226_78 = 1572,
  kCFStringEncodingShiftJIS_X0213 = 1576,
  kCFStringEncodingShiftJIS_X0213_MenKuTen = 1577,
  kCFStringEncodingGB_2312_80 = 1584,
  kCFStringEncodingGBK_95 = 1585,
  kCFStringEncodingGB_18030_2000 = 1586,
  kCFStringEncodingKSC_5601_87 = 1600,
  kCFStringEncodingKSC_5601_92_Johab = 1601,
  kCFStringEncodingCNS_11643_92_P1 = 1617,
  kCFStringEncodingCNS_11643_92_P2 = 1618,
  kCFStringEncodingCNS_11643_92_P3 = 1619,
  kCFStringEncodingISO_2022_JP = 2080,
  kCFStringEncodingISO_2022_JP_2 = 2081,
  kCFStringEncodingISO_2022_JP_1 = 2082,
  kCFStringEncodingISO_2022_JP_3 = 2083,
  kCFStringEncodingISO_2022_CN = 2096,
  kCFStringEncodingISO_2022_CN_EXT = 2097,
  kCFStringEncodingISO_2022_KR = 2112,
  kCFStringEncodingEUC_JP = 2336,
  kCFStringEncodingEUC_CN = 2352,
  kCFStringEncodingEUC_TW = 2353,
  kCFStringEncodingEUC_KR = 2368,
  kCFStringEncodingShiftJIS = 2561,
  kCFStringEncodingKOI8_R = 2562,
  kCFStringEncodingBig5 = 2563,
  kCFStringEncodingMacRomanLatin1 = 2564,
  kCFStringEncodingHZ_GB_2312 = 2565,
  kCFStringEncodingBig5_HKSCS_1999 = 2566,
  kCFStringEncodingVISCII = 2567,
  kCFStringEncodingKOI8_U = 2568,
  kCFStringEncodingBig5_E = 2569,
  kCFStringEncodingNextStepJapanese = 2818,
  kCFStringEncodingEBCDIC_US = 3073,
  kCFStringEncodingEBCDIC_CP037 = 3074,
  kCFStringEncodingUTF7 = 67109120,
  kCFStringEncodingUTF7_IMAP = 2576,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFTreeContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFTree {
  opaque: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFURLError {
  kCFURLUnknownError = -10,
  kCFURLUnknownSchemeError = -11,
  kCFURLResourceNotFoundError = -12,
  kCFURLResourceAccessViolationError = -13,
  kCFURLRemoteHostUnavailableError = -14,
  kCFURLImproperArgumentsError = -15,
  kCFURLUnknownPropertyKeyError = -16,
  kCFURLPropertyKeyUnavailableError = -17,
  kCFURLTimeoutError = -18,
}
#[repr(C)]
pub struct __CFUUID {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFUUIDBytes {
  pub byte0: u8,
  pub byte1: u8,
  pub byte2: u8,
  pub byte3: u8,
  pub byte4: u8,
  pub byte5: u8,
  pub byte6: u8,
  pub byte7: u8,
  pub byte8: u8,
  pub byte9: u8,
  pub byte10: u8,
  pub byte11: u8,
  pub byte12: u8,
  pub byte13: u8,
  pub byte14: u8,
  pub byte15: u8,
}
#[repr(C)]
pub struct __CFBundle {
  opaque: u32,
}
#[repr(C)]
pub struct __CFMessagePort {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFMessagePortContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFPlugInInstance {
  opaque: u32,
}
#[repr(C)]
pub struct __CFMachPort {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFMachPortContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFAttributedString {
  opaque: u32,
}
#[repr(C)]
pub struct __CFURLEnumerator {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFURLEnumeratorOptions : usize { const kCFURLEnumeratorDefaultBehavior = 0 ; const kCFURLEnumeratorDescendRecursively = 1 ; const kCFURLEnumeratorSkipInvisibles = 2 ; const kCFURLEnumeratorGenerateFileReferenceURLs = 4 ; const kCFURLEnumeratorSkipPackageContents = 8 ; const kCFURLEnumeratorIncludeDirectoriesPreOrder = 16 ; const kCFURLEnumeratorIncludeDirectoriesPostOrder = 32 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFURLEnumeratorResult {
  kCFURLEnumeratorSuccess = 1,
  kCFURLEnumeratorEnd = 2,
  kCFURLEnumeratorError = 3,
  kCFURLEnumeratorDirectoryPostOrderSuccess = 4,
}
#[repr(C)]
pub struct __CFFileSecurity {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFFileSecurityClearOptions : usize { const kCFFileSecurityClearOwner = 1 ; const kCFFileSecurityClearGroup = 2 ; const kCFFileSecurityClearMode = 4 ; const kCFFileSecurityClearOwnerUUID = 8 ; const kCFFileSecurityClearGroupUUID = 16 ; const kCFFileSecurityClearAccessControlList = 32 ; } }
#[repr(C)]
pub struct __CFStringTokenizer {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFStringTokenizerTokenType : usize { const kCFStringTokenizerTokenNone = 0 ; const kCFStringTokenizerTokenNormal = 1 ; const kCFStringTokenizerTokenHasSubTokensMask = 2 ; const kCFStringTokenizerTokenHasDerivedSubTokensMask = 4 ; const kCFStringTokenizerTokenHasHasNumbersMask = 8 ; const kCFStringTokenizerTokenHasNonLettersMask = 16 ; const kCFStringTokenizerTokenIsCJWordMask = 32 ; } }
#[repr(C)]
pub struct __CFFileDescriptor {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFFileDescriptorContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct __CFUserNotification {
  opaque: u32,
}
#[repr(C)]
pub struct __CFXMLNode {
  opaque: u32,
}
pub type CFXMLTreeRef = *mut __CFTree;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFXMLNodeTypeCode {
  kCFXMLNodeTypeDocument = 1,
  kCFXMLNodeTypeElement = 2,
  kCFXMLNodeTypeAttribute = 3,
  kCFXMLNodeTypeProcessingInstruction = 4,
  kCFXMLNodeTypeComment = 5,
  kCFXMLNodeTypeText = 6,
  kCFXMLNodeTypeCDATASection = 7,
  kCFXMLNodeTypeDocumentFragment = 8,
  kCFXMLNodeTypeEntity = 9,
  kCFXMLNodeTypeEntityReference = 10,
  kCFXMLNodeTypeDocumentType = 11,
  kCFXMLNodeTypeWhitespace = 12,
  kCFXMLNodeTypeNotation = 13,
  kCFXMLNodeTypeElementTypeDeclaration = 14,
  kCFXMLNodeTypeAttributeListDeclaration = 15,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLElementInfo {
  pub attributes: *mut __CFDictionary,
  pub attributeOrder: *mut __CFArray,
  pub isEmpty: u8,
  pub _reserved: [i8; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLProcessingInstructionInfo {
  pub dataString: *mut __CFString,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLDocumentInfo {
  pub sourceURL: *mut __CFURL,
  pub encoding: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLExternalID {
  pub systemID: *mut __CFURL,
  pub publicID: *mut __CFString,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLDocumentTypeInfo {
  pub externalID: CFXMLExternalID,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLNotationInfo {
  pub externalID: CFXMLExternalID,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLElementTypeDeclarationInfo {
  pub contentDescription: *mut __CFString,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLAttributeDeclarationInfo {
  pub attributeName: *mut __CFString,
  pub typeString: *mut __CFString,
  pub defaultString: *mut __CFString,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLAttributeListDeclarationInfo {
  pub numberOfAttributes: isize,
  pub attributes: *mut CFXMLAttributeDeclarationInfo,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CFXMLEntityTypeCode {
  kCFXMLEntityTypeParameter = 0,
  kCFXMLEntityTypeParsedInternal = 1,
  kCFXMLEntityTypeParsedExternal = 2,
  kCFXMLEntityTypeUnparsed = 3,
  kCFXMLEntityTypeCharacter = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLEntityInfo {
  pub entityType: CFXMLEntityTypeCode,
  pub replacementText: *mut __CFString,
  pub entityID: CFXMLExternalID,
  pub notationName: *mut __CFString,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLEntityReferenceInfo {
  pub entityType: CFXMLEntityTypeCode,
}
#[repr(C)]
pub struct __CFXMLParser {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CFXMLParserOptions : usize { const kCFXMLParserValidateDocument = 1 ; const kCFXMLParserSkipMetaData = 2 ; const kCFXMLParserReplacePhysicalEntities = 4 ; const kCFXMLParserSkipWhitespace = 8 ; const kCFXMLParserResolveExternalEntities = 16 ; const kCFXMLParserAddImpliedAttributes = 32 ; const kCFXMLParserAllOptions = 16777215 ; const kCFXMLParserNoOptions = 0 ; } }
bitflags! { # [ repr ( C ) ] pub struct CFXMLParserStatusCode : isize { const kCFXMLStatusParseNotBegun = - 2 ; const kCFXMLStatusParseInProgress = - 1 ; const kCFXMLStatusParseSuccessful = 0 ; const kCFXMLErrorUnexpectedEOF = 1 ; const kCFXMLErrorUnknownEncoding = 2 ; const kCFXMLErrorEncodingConversionFailure = 3 ; const kCFXMLErrorMalformedProcessingInstruction = 4 ; const kCFXMLErrorMalformedDTD = 5 ; const kCFXMLErrorMalformedName = 6 ; const kCFXMLErrorMalformedCDSect = 7 ; const kCFXMLErrorMalformedCloseTag = 8 ; const kCFXMLErrorMalformedStartTag = 9 ; const kCFXMLErrorMalformedDocument = 10 ; const kCFXMLErrorElementlessDocument = 11 ; const kCFXMLErrorMalformedComment = 12 ; const kCFXMLErrorMalformedCharacterReference = 13 ; const kCFXMLErrorMalformedParsedCharacterData = 14 ; const kCFXMLErrorNoData = 15 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLParserCallBacks {
  pub version: isize,
  pub createXMLStructure:
    Option<extern "C" fn(*mut __CFXMLParser, *mut __CFXMLNode, *mut c_void) -> *mut c_void>,
  pub addChild:
    Option<extern "C" fn(*mut __CFXMLParser, *mut c_void, *mut c_void, *mut c_void) -> ()>,
  pub endXMLStructure: Option<extern "C" fn(*mut __CFXMLParser, *mut c_void, *mut c_void) -> ()>,
  pub resolveExternalEntity:
    Option<extern "C" fn(*mut __CFXMLParser, *mut CFXMLExternalID, *mut c_void) -> *mut __CFData>,
  pub handleError:
    Option<extern "C" fn(*mut __CFXMLParser, CFXMLParserStatusCode, *mut c_void) -> u8>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFXMLParserContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[cfg(feature = "RK_CoreFoundation")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
  pub fn CFRangeMake(loc: isize, len: isize) -> CFRange;
  pub fn __CFRangeMake(loc: isize, len: isize) -> CFRange;
  pub fn CFNullGetTypeID() -> usize;
  pub fn CFAllocatorGetTypeID() -> usize;
  pub fn CFAllocatorSetDefault(allocator: *mut __CFAllocator) -> ();
  pub fn CFAllocatorGetDefault() -> *mut __CFAllocator;
  pub fn CFAllocatorCreate(
    allocator: *mut __CFAllocator,
    context: *mut CFAllocatorContext,
  ) -> *mut __CFAllocator;
  pub fn CFAllocatorAllocate(
    allocator: *mut __CFAllocator,
    size: isize,
    hint: usize,
  ) -> *mut c_void;
  pub fn CFAllocatorReallocate(
    allocator: *mut __CFAllocator,
    ptr: *mut c_void,
    newsize: isize,
    hint: usize,
  ) -> *mut c_void;
  pub fn CFAllocatorDeallocate(allocator: *mut __CFAllocator, ptr: *mut c_void) -> ();
  pub fn CFAllocatorGetPreferredSizeForSize(
    allocator: *mut __CFAllocator,
    size: isize,
    hint: usize,
  ) -> isize;
  pub fn CFAllocatorGetContext(
    allocator: *mut __CFAllocator,
    context: *mut CFAllocatorContext,
  ) -> ();
  pub fn CFGetTypeID(cf: *mut c_void) -> usize;
  pub fn CFCopyTypeIDDescription(type_id: usize) -> *mut __CFString;
  pub fn CFRetain(cf: *mut c_void) -> *mut c_void;
  pub fn CFRelease(cf: *mut c_void) -> ();
  pub fn CFAutorelease(arg: *mut c_void) -> *mut c_void;
  pub fn CFGetRetainCount(cf: *mut c_void) -> isize;
  pub fn CFEqual(cf1: *mut c_void, cf2: *mut c_void) -> u8;
  pub fn CFHash(cf: *mut c_void) -> usize;
  pub fn CFCopyDescription(cf: *mut c_void) -> *mut __CFString;
  pub fn CFGetAllocator(cf: *mut c_void) -> *mut __CFAllocator;
  pub fn CFArrayGetTypeID() -> usize;
  pub fn CFArrayCreate(
    allocator: *mut __CFAllocator,
    values: *mut *mut c_void,
    numValues: isize,
    callBacks: *mut CFArrayCallBacks,
  ) -> *mut __CFArray;
  pub fn CFArrayCreateCopy(
    allocator: *mut __CFAllocator,
    theArray: *mut __CFArray,
  ) -> *mut __CFArray;
  pub fn CFArrayCreateMutable(
    allocator: *mut __CFAllocator,
    capacity: isize,
    callBacks: *mut CFArrayCallBacks,
  ) -> *mut __CFArray;
  pub fn CFArrayCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    theArray: *mut __CFArray,
  ) -> *mut __CFArray;
  pub fn CFArrayGetCount(theArray: *mut __CFArray) -> isize;
  pub fn CFArrayGetCountOfValue(
    theArray: *mut __CFArray,
    range: CFRange,
    value: *mut c_void,
  ) -> isize;
  pub fn CFArrayContainsValue(theArray: *mut __CFArray, range: CFRange, value: *mut c_void) -> u8;
  pub fn CFArrayGetValueAtIndex(theArray: *mut __CFArray, idx: isize) -> *mut c_void;
  pub fn CFArrayGetValues(theArray: *mut __CFArray, range: CFRange, values: *mut *mut c_void)
    -> ();
  pub fn CFArrayApplyFunction(
    theArray: *mut __CFArray,
    range: CFRange,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFArrayGetFirstIndexOfValue(
    theArray: *mut __CFArray,
    range: CFRange,
    value: *mut c_void,
  ) -> isize;
  pub fn CFArrayGetLastIndexOfValue(
    theArray: *mut __CFArray,
    range: CFRange,
    value: *mut c_void,
  ) -> isize;
  pub fn CFArrayBSearchValues(
    theArray: *mut __CFArray,
    range: CFRange,
    value: *mut c_void,
    comparator: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> CFComparisonResult>,
    context: *mut c_void,
  ) -> isize;
  pub fn CFArrayAppendValue(theArray: *mut __CFArray, value: *mut c_void) -> ();
  pub fn CFArrayInsertValueAtIndex(theArray: *mut __CFArray, idx: isize, value: *mut c_void) -> ();
  pub fn CFArraySetValueAtIndex(theArray: *mut __CFArray, idx: isize, value: *mut c_void) -> ();
  pub fn CFArrayRemoveValueAtIndex(theArray: *mut __CFArray, idx: isize) -> ();
  pub fn CFArrayRemoveAllValues(theArray: *mut __CFArray) -> ();
  pub fn CFArrayReplaceValues(
    theArray: *mut __CFArray,
    range: CFRange,
    newValues: *mut *mut c_void,
    newCount: isize,
  ) -> ();
  pub fn CFArrayExchangeValuesAtIndices(theArray: *mut __CFArray, idx1: isize, idx2: isize) -> ();
  pub fn CFArraySortValues(
    theArray: *mut __CFArray,
    range: CFRange,
    comparator: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> CFComparisonResult>,
    context: *mut c_void,
  ) -> ();
  pub fn CFArrayAppendArray(
    theArray: *mut __CFArray,
    otherArray: *mut __CFArray,
    otherRange: CFRange,
  ) -> ();
  pub fn CFBagGetTypeID() -> usize;
  pub fn CFBagCreate(
    allocator: *mut __CFAllocator,
    values: *mut *mut c_void,
    numValues: isize,
    callBacks: *mut CFBagCallBacks,
  ) -> *mut __CFBag;
  pub fn CFBagCreateCopy(allocator: *mut __CFAllocator, theBag: *mut __CFBag) -> *mut __CFBag;
  pub fn CFBagCreateMutable(
    allocator: *mut __CFAllocator,
    capacity: isize,
    callBacks: *mut CFBagCallBacks,
  ) -> *mut __CFBag;
  pub fn CFBagCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    theBag: *mut __CFBag,
  ) -> *mut __CFBag;
  pub fn CFBagGetCount(theBag: *mut __CFBag) -> isize;
  pub fn CFBagGetCountOfValue(theBag: *mut __CFBag, value: *mut c_void) -> isize;
  pub fn CFBagContainsValue(theBag: *mut __CFBag, value: *mut c_void) -> u8;
  pub fn CFBagGetValue(theBag: *mut __CFBag, value: *mut c_void) -> *mut c_void;
  pub fn CFBagGetValueIfPresent(
    theBag: *mut __CFBag,
    candidate: *mut c_void,
    value: *mut *mut c_void,
  ) -> u8;
  pub fn CFBagGetValues(theBag: *mut __CFBag, values: *mut *mut c_void) -> ();
  pub fn CFBagApplyFunction(
    theBag: *mut __CFBag,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFBagAddValue(theBag: *mut __CFBag, value: *mut c_void) -> ();
  pub fn CFBagReplaceValue(theBag: *mut __CFBag, value: *mut c_void) -> ();
  pub fn CFBagSetValue(theBag: *mut __CFBag, value: *mut c_void) -> ();
  pub fn CFBagRemoveValue(theBag: *mut __CFBag, value: *mut c_void) -> ();
  pub fn CFBagRemoveAllValues(theBag: *mut __CFBag) -> ();
  pub fn CFBinaryHeapGetTypeID() -> usize;
  pub fn CFBinaryHeapCreate(
    allocator: *mut __CFAllocator,
    capacity: isize,
    callBacks: *mut CFBinaryHeapCallBacks,
    compareContext: *mut CFBinaryHeapCompareContext,
  ) -> *mut __CFBinaryHeap;
  pub fn CFBinaryHeapCreateCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    heap: *mut __CFBinaryHeap,
  ) -> *mut __CFBinaryHeap;
  pub fn CFBinaryHeapGetCount(heap: *mut __CFBinaryHeap) -> isize;
  pub fn CFBinaryHeapGetCountOfValue(heap: *mut __CFBinaryHeap, value: *mut c_void) -> isize;
  pub fn CFBinaryHeapContainsValue(heap: *mut __CFBinaryHeap, value: *mut c_void) -> u8;
  pub fn CFBinaryHeapGetMinimum(heap: *mut __CFBinaryHeap) -> *mut c_void;
  pub fn CFBinaryHeapGetMinimumIfPresent(heap: *mut __CFBinaryHeap, value: *mut *mut c_void) -> u8;
  pub fn CFBinaryHeapGetValues(heap: *mut __CFBinaryHeap, values: *mut *mut c_void) -> ();
  pub fn CFBinaryHeapApplyFunction(
    heap: *mut __CFBinaryHeap,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFBinaryHeapAddValue(heap: *mut __CFBinaryHeap, value: *mut c_void) -> ();
  pub fn CFBinaryHeapRemoveMinimumValue(heap: *mut __CFBinaryHeap) -> ();
  pub fn CFBinaryHeapRemoveAllValues(heap: *mut __CFBinaryHeap) -> ();
  pub fn CFBitVectorGetTypeID() -> usize;
  pub fn CFBitVectorCreate(
    allocator: *mut __CFAllocator,
    bytes: *mut u8,
    numBits: isize,
  ) -> *mut __CFBitVector;
  pub fn CFBitVectorCreateCopy(
    allocator: *mut __CFAllocator,
    bv: *mut __CFBitVector,
  ) -> *mut __CFBitVector;
  pub fn CFBitVectorCreateMutable(
    allocator: *mut __CFAllocator,
    capacity: isize,
  ) -> *mut __CFBitVector;
  pub fn CFBitVectorCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    bv: *mut __CFBitVector,
  ) -> *mut __CFBitVector;
  pub fn CFBitVectorGetCount(bv: *mut __CFBitVector) -> isize;
  pub fn CFBitVectorGetCountOfBit(bv: *mut __CFBitVector, range: CFRange, value: u32) -> isize;
  pub fn CFBitVectorContainsBit(bv: *mut __CFBitVector, range: CFRange, value: u32) -> u8;
  pub fn CFBitVectorGetBitAtIndex(bv: *mut __CFBitVector, idx: isize) -> u32;
  pub fn CFBitVectorGetBits(bv: *mut __CFBitVector, range: CFRange, bytes: *mut u8) -> ();
  pub fn CFBitVectorGetFirstIndexOfBit(bv: *mut __CFBitVector, range: CFRange, value: u32)
    -> isize;
  pub fn CFBitVectorGetLastIndexOfBit(bv: *mut __CFBitVector, range: CFRange, value: u32) -> isize;
  pub fn CFBitVectorSetCount(bv: *mut __CFBitVector, count: isize) -> ();
  pub fn CFBitVectorFlipBitAtIndex(bv: *mut __CFBitVector, idx: isize) -> ();
  pub fn CFBitVectorFlipBits(bv: *mut __CFBitVector, range: CFRange) -> ();
  pub fn CFBitVectorSetBitAtIndex(bv: *mut __CFBitVector, idx: isize, value: u32) -> ();
  pub fn CFBitVectorSetBits(bv: *mut __CFBitVector, range: CFRange, value: u32) -> ();
  pub fn CFBitVectorSetAllBits(bv: *mut __CFBitVector, value: u32) -> ();
  pub fn CFByteOrderGetCurrent() -> isize;
  pub fn CFSwapInt16(arg: u16) -> u16;
  pub fn CFSwapInt32(arg: u32) -> u32;
  pub fn CFSwapInt64(arg: u64) -> u64;
  pub fn CFSwapInt16BigToHost(arg: u16) -> u16;
  pub fn CFSwapInt32BigToHost(arg: u32) -> u32;
  pub fn CFSwapInt64BigToHost(arg: u64) -> u64;
  pub fn CFSwapInt16HostToBig(arg: u16) -> u16;
  pub fn CFSwapInt32HostToBig(arg: u32) -> u32;
  pub fn CFSwapInt64HostToBig(arg: u64) -> u64;
  pub fn CFSwapInt16LittleToHost(arg: u16) -> u16;
  pub fn CFSwapInt32LittleToHost(arg: u32) -> u32;
  pub fn CFSwapInt64LittleToHost(arg: u64) -> u64;
  pub fn CFSwapInt16HostToLittle(arg: u16) -> u16;
  pub fn CFSwapInt32HostToLittle(arg: u32) -> u32;
  pub fn CFSwapInt64HostToLittle(arg: u64) -> u64;
  pub fn CFConvertFloat32HostToSwapped(arg: f32) -> CFSwappedFloat32;
  pub fn CFConvertFloat32SwappedToHost(arg: CFSwappedFloat32) -> f32;
  pub fn CFConvertFloat64HostToSwapped(arg: f64) -> CFSwappedFloat64;
  pub fn CFConvertFloat64SwappedToHost(arg: CFSwappedFloat64) -> f64;
  pub fn CFConvertFloatHostToSwapped(arg: f32) -> CFSwappedFloat32;
  pub fn CFConvertFloatSwappedToHost(arg: CFSwappedFloat32) -> f32;
  pub fn CFConvertDoubleHostToSwapped(arg: f64) -> CFSwappedFloat64;
  pub fn CFConvertDoubleSwappedToHost(arg: CFSwappedFloat64) -> f64;
  pub fn CFDictionaryGetTypeID() -> usize;
  pub fn CFDictionaryCreate(
    allocator: *mut __CFAllocator,
    keys: *mut *mut c_void,
    values: *mut *mut c_void,
    numValues: isize,
    keyCallBacks: *mut CFDictionaryKeyCallBacks,
    valueCallBacks: *mut CFDictionaryValueCallBacks,
  ) -> *mut __CFDictionary;
  pub fn CFDictionaryCreateCopy(
    allocator: *mut __CFAllocator,
    theDict: *mut __CFDictionary,
  ) -> *mut __CFDictionary;
  pub fn CFDictionaryCreateMutable(
    allocator: *mut __CFAllocator,
    capacity: isize,
    keyCallBacks: *mut CFDictionaryKeyCallBacks,
    valueCallBacks: *mut CFDictionaryValueCallBacks,
  ) -> *mut __CFDictionary;
  pub fn CFDictionaryCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    theDict: *mut __CFDictionary,
  ) -> *mut __CFDictionary;
  pub fn CFDictionaryGetCount(theDict: *mut __CFDictionary) -> isize;
  pub fn CFDictionaryGetCountOfKey(theDict: *mut __CFDictionary, key: *mut c_void) -> isize;
  pub fn CFDictionaryGetCountOfValue(theDict: *mut __CFDictionary, value: *mut c_void) -> isize;
  pub fn CFDictionaryContainsKey(theDict: *mut __CFDictionary, key: *mut c_void) -> u8;
  pub fn CFDictionaryContainsValue(theDict: *mut __CFDictionary, value: *mut c_void) -> u8;
  pub fn CFDictionaryGetValue(theDict: *mut __CFDictionary, key: *mut c_void) -> *mut c_void;
  pub fn CFDictionaryGetValueIfPresent(
    theDict: *mut __CFDictionary,
    key: *mut c_void,
    value: *mut *mut c_void,
  ) -> u8;
  pub fn CFDictionaryGetKeysAndValues(
    theDict: *mut __CFDictionary,
    keys: *mut *mut c_void,
    values: *mut *mut c_void,
  ) -> ();
  pub fn CFDictionaryApplyFunction(
    theDict: *mut __CFDictionary,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFDictionaryAddValue(
    theDict: *mut __CFDictionary,
    key: *mut c_void,
    value: *mut c_void,
  ) -> ();
  pub fn CFDictionarySetValue(
    theDict: *mut __CFDictionary,
    key: *mut c_void,
    value: *mut c_void,
  ) -> ();
  pub fn CFDictionaryReplaceValue(
    theDict: *mut __CFDictionary,
    key: *mut c_void,
    value: *mut c_void,
  ) -> ();
  pub fn CFDictionaryRemoveValue(theDict: *mut __CFDictionary, key: *mut c_void) -> ();
  pub fn CFDictionaryRemoveAllValues(theDict: *mut __CFDictionary) -> ();
  pub fn CFNotificationCenterGetTypeID() -> usize;
  pub fn CFNotificationCenterGetLocalCenter() -> *mut __CFNotificationCenter;
  pub fn CFNotificationCenterGetDistributedCenter() -> *mut __CFNotificationCenter;
  pub fn CFNotificationCenterGetDarwinNotifyCenter() -> *mut __CFNotificationCenter;
  pub fn CFNotificationCenterAddObserver(
    center: *mut __CFNotificationCenter,
    observer: *mut c_void,
    callBack: Option<
      extern "C" fn(
        *mut __CFNotificationCenter,
        *mut c_void,
        *mut __CFString,
        *mut c_void,
        *mut __CFDictionary,
      ) -> (),
    >,
    name: *mut __CFString,
    object: *mut c_void,
    suspensionBehavior: CFNotificationSuspensionBehavior,
  ) -> ();
  pub fn CFNotificationCenterRemoveObserver(
    center: *mut __CFNotificationCenter,
    observer: *mut c_void,
    name: *mut __CFString,
    object: *mut c_void,
  ) -> ();
  pub fn CFNotificationCenterRemoveEveryObserver(
    center: *mut __CFNotificationCenter,
    observer: *mut c_void,
  ) -> ();
  pub fn CFNotificationCenterPostNotification(
    center: *mut __CFNotificationCenter,
    name: *mut __CFString,
    object: *mut c_void,
    userInfo: *mut __CFDictionary,
    deliverImmediately: u8,
  ) -> ();
  pub fn CFNotificationCenterPostNotificationWithOptions(
    center: *mut __CFNotificationCenter,
    name: *mut __CFString,
    object: *mut c_void,
    userInfo: *mut __CFDictionary,
    options: usize,
  ) -> ();
  pub fn CFLocaleGetTypeID() -> usize;
  pub fn CFLocaleGetSystem() -> *mut __CFLocale;
  pub fn CFLocaleCopyCurrent() -> *mut __CFLocale;
  pub fn CFLocaleCopyAvailableLocaleIdentifiers() -> *mut __CFArray;
  pub fn CFLocaleCopyISOLanguageCodes() -> *mut __CFArray;
  pub fn CFLocaleCopyISOCountryCodes() -> *mut __CFArray;
  pub fn CFLocaleCopyISOCurrencyCodes() -> *mut __CFArray;
  pub fn CFLocaleCopyCommonISOCurrencyCodes() -> *mut __CFArray;
  pub fn CFLocaleCopyPreferredLanguages() -> *mut __CFArray;
  pub fn CFLocaleCreateCanonicalLanguageIdentifierFromString(
    allocator: *mut __CFAllocator,
    localeIdentifier: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFLocaleCreateCanonicalLocaleIdentifierFromString(
    allocator: *mut __CFAllocator,
    localeIdentifier: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes(
    allocator: *mut __CFAllocator,
    lcode: i16,
    rcode: i16,
  ) -> *mut __CFString;
  pub fn CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode(
    allocator: *mut __CFAllocator,
    lcid: u32,
  ) -> *mut __CFString;
  pub fn CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier(localeIdentifier: *mut __CFString)
    -> u32;
  pub fn CFLocaleGetLanguageCharacterDirection(
    isoLangCode: *mut __CFString,
  ) -> CFLocaleLanguageDirection;
  pub fn CFLocaleGetLanguageLineDirection(
    isoLangCode: *mut __CFString,
  ) -> CFLocaleLanguageDirection;
  pub fn CFLocaleCreateComponentsFromLocaleIdentifier(
    allocator: *mut __CFAllocator,
    localeID: *mut __CFString,
  ) -> *mut __CFDictionary;
  pub fn CFLocaleCreateLocaleIdentifierFromComponents(
    allocator: *mut __CFAllocator,
    dictionary: *mut __CFDictionary,
  ) -> *mut __CFString;
  pub fn CFLocaleCreate(
    allocator: *mut __CFAllocator,
    localeIdentifier: *mut __CFString,
  ) -> *mut __CFLocale;
  pub fn CFLocaleCreateCopy(
    allocator: *mut __CFAllocator,
    locale: *mut __CFLocale,
  ) -> *mut __CFLocale;
  pub fn CFLocaleGetIdentifier(locale: *mut __CFLocale) -> *mut __CFString;
  pub fn CFLocaleGetValue(locale: *mut __CFLocale, key: *mut __CFString) -> *mut c_void;
  pub fn CFLocaleCopyDisplayNameForPropertyValue(
    displayLocale: *mut __CFLocale,
    key: *mut __CFString,
    value: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFAbsoluteTimeGetCurrent() -> f64;
  pub fn CFDateGetTypeID() -> usize;
  pub fn CFDateCreate(allocator: *mut __CFAllocator, at: f64) -> *mut __CFDate;
  pub fn CFDateGetAbsoluteTime(theDate: *mut __CFDate) -> f64;
  pub fn CFDateGetTimeIntervalSinceDate(theDate: *mut __CFDate, otherDate: *mut __CFDate) -> f64;
  pub fn CFDateCompare(
    theDate: *mut __CFDate,
    otherDate: *mut __CFDate,
    context: *mut c_void,
  ) -> CFComparisonResult;
  pub fn CFGregorianDateIsValid(gdate: CFGregorianDate, unitFlags: usize) -> u8;
  pub fn CFGregorianDateGetAbsoluteTime(gdate: CFGregorianDate, tz: *mut __CFTimeZone) -> f64;
  pub fn CFAbsoluteTimeGetGregorianDate(at: f64, tz: *mut __CFTimeZone) -> CFGregorianDate;
  pub fn CFAbsoluteTimeAddGregorianUnits(
    at: f64,
    tz: *mut __CFTimeZone,
    units: CFGregorianUnits,
  ) -> f64;
  pub fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(
    at1: f64,
    at2: f64,
    tz: *mut __CFTimeZone,
    unitFlags: usize,
  ) -> CFGregorianUnits;
  pub fn CFAbsoluteTimeGetDayOfWeek(at: f64, tz: *mut __CFTimeZone) -> i32;
  pub fn CFAbsoluteTimeGetDayOfYear(at: f64, tz: *mut __CFTimeZone) -> i32;
  pub fn CFAbsoluteTimeGetWeekOfYear(at: f64, tz: *mut __CFTimeZone) -> i32;
  pub fn CFDataGetTypeID() -> usize;
  pub fn CFDataCreate(
    allocator: *mut __CFAllocator,
    bytes: *mut u8,
    length: isize,
  ) -> *mut __CFData;
  pub fn CFDataCreateWithBytesNoCopy(
    allocator: *mut __CFAllocator,
    bytes: *mut u8,
    length: isize,
    bytesDeallocator: *mut __CFAllocator,
  ) -> *mut __CFData;
  pub fn CFDataCreateCopy(allocator: *mut __CFAllocator, theData: *mut __CFData) -> *mut __CFData;
  pub fn CFDataCreateMutable(allocator: *mut __CFAllocator, capacity: isize) -> *mut __CFData;
  pub fn CFDataCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    theData: *mut __CFData,
  ) -> *mut __CFData;
  pub fn CFDataGetLength(theData: *mut __CFData) -> isize;
  pub fn CFDataGetBytePtr(theData: *mut __CFData) -> *mut u8;
  pub fn CFDataGetMutableBytePtr(theData: *mut __CFData) -> *mut u8;
  pub fn CFDataGetBytes(theData: *mut __CFData, range: CFRange, buffer: *mut u8) -> ();
  pub fn CFDataSetLength(theData: *mut __CFData, length: isize) -> ();
  pub fn CFDataIncreaseLength(theData: *mut __CFData, extraLength: isize) -> ();
  pub fn CFDataAppendBytes(theData: *mut __CFData, bytes: *mut u8, length: isize) -> ();
  pub fn CFDataReplaceBytes(
    theData: *mut __CFData,
    range: CFRange,
    newBytes: *mut u8,
    newLength: isize,
  ) -> ();
  pub fn CFDataDeleteBytes(theData: *mut __CFData, range: CFRange) -> ();
  pub fn CFDataFind(
    theData: *mut __CFData,
    dataToFind: *mut __CFData,
    searchRange: CFRange,
    compareOptions: CFDataSearchFlags,
  ) -> CFRange;
  pub fn CFCharacterSetGetTypeID() -> usize;
  pub fn CFCharacterSetGetPredefined(
    theSetIdentifier: CFCharacterSetPredefinedSet,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateWithCharactersInRange(
    alloc: *mut __CFAllocator,
    theRange: CFRange,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateWithCharactersInString(
    alloc: *mut __CFAllocator,
    theString: *mut __CFString,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateWithBitmapRepresentation(
    alloc: *mut __CFAllocator,
    theData: *mut __CFData,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateInvertedSet(
    alloc: *mut __CFAllocator,
    theSet: *mut __CFCharacterSet,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetIsSupersetOfSet(
    theSet: *mut __CFCharacterSet,
    theOtherset: *mut __CFCharacterSet,
  ) -> u8;
  pub fn CFCharacterSetHasMemberInPlane(theSet: *mut __CFCharacterSet, thePlane: isize) -> u8;
  pub fn CFCharacterSetCreateMutable(alloc: *mut __CFAllocator) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateCopy(
    alloc: *mut __CFAllocator,
    theSet: *mut __CFCharacterSet,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetCreateMutableCopy(
    alloc: *mut __CFAllocator,
    theSet: *mut __CFCharacterSet,
  ) -> *mut __CFCharacterSet;
  pub fn CFCharacterSetIsCharacterMember(theSet: *mut __CFCharacterSet, theChar: u16) -> u8;
  pub fn CFCharacterSetIsLongCharacterMember(theSet: *mut __CFCharacterSet, theChar: u32) -> u8;
  pub fn CFCharacterSetCreateBitmapRepresentation(
    alloc: *mut __CFAllocator,
    theSet: *mut __CFCharacterSet,
  ) -> *mut __CFData;
  pub fn CFCharacterSetAddCharactersInRange(theSet: *mut __CFCharacterSet, theRange: CFRange)
    -> ();
  pub fn CFCharacterSetRemoveCharactersInRange(
    theSet: *mut __CFCharacterSet,
    theRange: CFRange,
  ) -> ();
  pub fn CFCharacterSetAddCharactersInString(
    theSet: *mut __CFCharacterSet,
    theString: *mut __CFString,
  ) -> ();
  pub fn CFCharacterSetRemoveCharactersInString(
    theSet: *mut __CFCharacterSet,
    theString: *mut __CFString,
  ) -> ();
  pub fn CFCharacterSetUnion(
    theSet: *mut __CFCharacterSet,
    theOtherSet: *mut __CFCharacterSet,
  ) -> ();
  pub fn CFCharacterSetIntersect(
    theSet: *mut __CFCharacterSet,
    theOtherSet: *mut __CFCharacterSet,
  ) -> ();
  pub fn CFCharacterSetInvert(theSet: *mut __CFCharacterSet) -> ();
  pub fn CFStringGetTypeID() -> usize;
  pub fn CFStringCreateWithPascalString(
    alloc: *mut __CFAllocator,
    pStr: *mut u8,
    encoding: u32,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithCString(
    alloc: *mut __CFAllocator,
    cStr: *mut i8,
    encoding: u32,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithBytes(
    alloc: *mut __CFAllocator,
    bytes: *mut u8,
    numBytes: isize,
    encoding: u32,
    isExternalRepresentation: u8,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithCharacters(
    alloc: *mut __CFAllocator,
    chars: *mut u16,
    numChars: isize,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithPascalStringNoCopy(
    alloc: *mut __CFAllocator,
    pStr: *mut u8,
    encoding: u32,
    contentsDeallocator: *mut __CFAllocator,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithCStringNoCopy(
    alloc: *mut __CFAllocator,
    cStr: *mut i8,
    encoding: u32,
    contentsDeallocator: *mut __CFAllocator,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithBytesNoCopy(
    alloc: *mut __CFAllocator,
    bytes: *mut u8,
    numBytes: isize,
    encoding: u32,
    isExternalRepresentation: u8,
    contentsDeallocator: *mut __CFAllocator,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithCharactersNoCopy(
    alloc: *mut __CFAllocator,
    chars: *mut u16,
    numChars: isize,
    contentsDeallocator: *mut __CFAllocator,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithSubstring(
    alloc: *mut __CFAllocator,
    str: *mut __CFString,
    range: CFRange,
  ) -> *mut __CFString;
  pub fn CFStringCreateCopy(
    alloc: *mut __CFAllocator,
    theString: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFStringCreateWithFormat(
    alloc: *mut __CFAllocator,
    formatOptions: *mut __CFDictionary,
    format: *mut __CFString,
    ...
  ) -> *mut __CFString;
  pub fn CFStringCreateMutable(alloc: *mut __CFAllocator, maxLength: isize) -> *mut __CFString;
  pub fn CFStringCreateMutableCopy(
    alloc: *mut __CFAllocator,
    maxLength: isize,
    theString: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFStringCreateMutableWithExternalCharactersNoCopy(
    alloc: *mut __CFAllocator,
    chars: *mut u16,
    numChars: isize,
    capacity: isize,
    externalCharactersAllocator: *mut __CFAllocator,
  ) -> *mut __CFString;
  pub fn CFStringGetLength(theString: *mut __CFString) -> isize;
  pub fn CFStringGetCharacterAtIndex(theString: *mut __CFString, idx: isize) -> u16;
  pub fn CFStringGetCharacters(theString: *mut __CFString, range: CFRange, buffer: *mut u16) -> ();
  pub fn CFStringGetPascalString(
    theString: *mut __CFString,
    buffer: *mut u8,
    bufferSize: isize,
    encoding: u32,
  ) -> u8;
  pub fn CFStringGetCString(
    theString: *mut __CFString,
    buffer: *mut i8,
    bufferSize: isize,
    encoding: u32,
  ) -> u8;
  pub fn CFStringGetPascalStringPtr(theString: *mut __CFString, encoding: u32) -> *mut u8;
  pub fn CFStringGetCStringPtr(theString: *mut __CFString, encoding: u32) -> *mut i8;
  pub fn CFStringGetCharactersPtr(theString: *mut __CFString) -> *mut u16;
  pub fn CFStringGetBytes(
    theString: *mut __CFString,
    range: CFRange,
    encoding: u32,
    lossByte: u8,
    isExternalRepresentation: u8,
    buffer: *mut u8,
    maxBufLen: isize,
    usedBufLen: *mut isize,
  ) -> isize;
  pub fn CFStringCreateFromExternalRepresentation(
    alloc: *mut __CFAllocator,
    data: *mut __CFData,
    encoding: u32,
  ) -> *mut __CFString;
  pub fn CFStringCreateExternalRepresentation(
    alloc: *mut __CFAllocator,
    theString: *mut __CFString,
    encoding: u32,
    lossByte: u8,
  ) -> *mut __CFData;
  pub fn CFStringGetSmallestEncoding(theString: *mut __CFString) -> u32;
  pub fn CFStringGetFastestEncoding(theString: *mut __CFString) -> u32;
  pub fn CFStringGetSystemEncoding() -> u32;
  pub fn CFStringGetMaximumSizeForEncoding(length: isize, encoding: u32) -> isize;
  pub fn CFStringGetFileSystemRepresentation(
    string: *mut __CFString,
    buffer: *mut i8,
    maxBufLen: isize,
  ) -> u8;
  pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: *mut __CFString) -> isize;
  pub fn CFStringCreateWithFileSystemRepresentation(
    alloc: *mut __CFAllocator,
    buffer: *mut i8,
  ) -> *mut __CFString;
  pub fn CFStringCompareWithOptionsAndLocale(
    theString1: *mut __CFString,
    theString2: *mut __CFString,
    rangeToCompare: CFRange,
    compareOptions: CFStringCompareFlags,
    locale: *mut __CFLocale,
  ) -> CFComparisonResult;
  pub fn CFStringCompareWithOptions(
    theString1: *mut __CFString,
    theString2: *mut __CFString,
    rangeToCompare: CFRange,
    compareOptions: CFStringCompareFlags,
  ) -> CFComparisonResult;
  pub fn CFStringCompare(
    theString1: *mut __CFString,
    theString2: *mut __CFString,
    compareOptions: CFStringCompareFlags,
  ) -> CFComparisonResult;
  pub fn CFStringFindWithOptionsAndLocale(
    theString: *mut __CFString,
    stringToFind: *mut __CFString,
    rangeToSearch: CFRange,
    searchOptions: CFStringCompareFlags,
    locale: *mut __CFLocale,
    result: *mut CFRange,
  ) -> u8;
  pub fn CFStringFindWithOptions(
    theString: *mut __CFString,
    stringToFind: *mut __CFString,
    rangeToSearch: CFRange,
    searchOptions: CFStringCompareFlags,
    result: *mut CFRange,
  ) -> u8;
  pub fn CFStringCreateArrayWithFindResults(
    alloc: *mut __CFAllocator,
    theString: *mut __CFString,
    stringToFind: *mut __CFString,
    rangeToSearch: CFRange,
    compareOptions: CFStringCompareFlags,
  ) -> *mut __CFArray;
  pub fn CFStringFind(
    theString: *mut __CFString,
    stringToFind: *mut __CFString,
    compareOptions: CFStringCompareFlags,
  ) -> CFRange;
  pub fn CFStringHasPrefix(theString: *mut __CFString, prefix: *mut __CFString) -> u8;
  pub fn CFStringHasSuffix(theString: *mut __CFString, suffix: *mut __CFString) -> u8;
  pub fn CFStringGetRangeOfComposedCharactersAtIndex(
    theString: *mut __CFString,
    theIndex: isize,
  ) -> CFRange;
  pub fn CFStringFindCharacterFromSet(
    theString: *mut __CFString,
    theSet: *mut __CFCharacterSet,
    rangeToSearch: CFRange,
    searchOptions: CFStringCompareFlags,
    result: *mut CFRange,
  ) -> u8;
  pub fn CFStringGetLineBounds(
    theString: *mut __CFString,
    range: CFRange,
    lineBeginIndex: *mut isize,
    lineEndIndex: *mut isize,
    contentsEndIndex: *mut isize,
  ) -> ();
  pub fn CFStringGetParagraphBounds(
    string: *mut __CFString,
    range: CFRange,
    parBeginIndex: *mut isize,
    parEndIndex: *mut isize,
    contentsEndIndex: *mut isize,
  ) -> ();
  pub fn CFStringGetHyphenationLocationBeforeIndex(
    string: *mut __CFString,
    location: isize,
    limitRange: CFRange,
    options: usize,
    locale: *mut __CFLocale,
    character: *mut u32,
  ) -> isize;
  pub fn CFStringIsHyphenationAvailableForLocale(locale: *mut __CFLocale) -> u8;
  pub fn CFStringCreateByCombiningStrings(
    alloc: *mut __CFAllocator,
    theArray: *mut __CFArray,
    separatorString: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFStringCreateArrayBySeparatingStrings(
    alloc: *mut __CFAllocator,
    theString: *mut __CFString,
    separatorString: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFStringGetIntValue(str: *mut __CFString) -> i32;
  pub fn CFStringGetDoubleValue(str: *mut __CFString) -> f64;
  pub fn CFStringAppend(theString: *mut __CFString, appendedString: *mut __CFString) -> ();
  pub fn CFStringAppendCharacters(
    theString: *mut __CFString,
    chars: *mut u16,
    numChars: isize,
  ) -> ();
  pub fn CFStringAppendPascalString(theString: *mut __CFString, pStr: *mut u8, encoding: u32)
    -> ();
  pub fn CFStringAppendCString(theString: *mut __CFString, cStr: *mut i8, encoding: u32) -> ();
  pub fn CFStringAppendFormat(
    theString: *mut __CFString,
    formatOptions: *mut __CFDictionary,
    format: *mut __CFString,
    ...
  ) -> ();
  pub fn CFStringInsert(str: *mut __CFString, idx: isize, insertedStr: *mut __CFString) -> ();
  pub fn CFStringDelete(theString: *mut __CFString, range: CFRange) -> ();
  pub fn CFStringReplace(
    theString: *mut __CFString,
    range: CFRange,
    replacement: *mut __CFString,
  ) -> ();
  pub fn CFStringReplaceAll(theString: *mut __CFString, replacement: *mut __CFString) -> ();
  pub fn CFStringFindAndReplace(
    theString: *mut __CFString,
    stringToFind: *mut __CFString,
    replacementString: *mut __CFString,
    rangeToSearch: CFRange,
    compareOptions: CFStringCompareFlags,
  ) -> isize;
  pub fn CFStringSetExternalCharactersNoCopy(
    theString: *mut __CFString,
    chars: *mut u16,
    length: isize,
    capacity: isize,
  ) -> ();
  pub fn CFStringPad(
    theString: *mut __CFString,
    padString: *mut __CFString,
    length: isize,
    indexIntoPad: isize,
  ) -> ();
  pub fn CFStringTrim(theString: *mut __CFString, trimString: *mut __CFString) -> ();
  pub fn CFStringTrimWhitespace(theString: *mut __CFString) -> ();
  pub fn CFStringLowercase(theString: *mut __CFString, locale: *mut __CFLocale) -> ();
  pub fn CFStringUppercase(theString: *mut __CFString, locale: *mut __CFLocale) -> ();
  pub fn CFStringCapitalize(theString: *mut __CFString, locale: *mut __CFLocale) -> ();
  pub fn CFStringNormalize(theString: *mut __CFString, theForm: CFStringNormalizationForm) -> ();
  pub fn CFStringFold(
    theString: *mut __CFString,
    theFlags: CFStringCompareFlags,
    theLocale: *mut __CFLocale,
  ) -> ();
  pub fn CFStringTransform(
    string: *mut __CFString,
    range: *mut CFRange,
    transform: *mut __CFString,
    reverse: u8,
  ) -> u8;
  pub fn CFStringIsEncodingAvailable(encoding: u32) -> u8;
  pub fn CFStringGetListOfAvailableEncodings() -> *mut u32;
  pub fn CFStringGetNameOfEncoding(encoding: u32) -> *mut __CFString;
  pub fn CFStringConvertEncodingToNSStringEncoding(encoding: u32) -> usize;
  pub fn CFStringConvertNSStringEncodingToEncoding(encoding: usize) -> u32;
  pub fn CFStringConvertEncodingToWindowsCodepage(encoding: u32) -> u32;
  pub fn CFStringConvertWindowsCodepageToEncoding(codepage: u32) -> u32;
  pub fn CFStringConvertIANACharSetNameToEncoding(theString: *mut __CFString) -> u32;
  pub fn CFStringConvertEncodingToIANACharSetName(encoding: u32) -> *mut __CFString;
  pub fn CFStringGetMostCompatibleMacStringEncoding(encoding: u32) -> u32;
  pub fn CFStringInitInlineBuffer(
    str: *mut __CFString,
    buf: *mut CFStringInlineBuffer,
    range: CFRange,
  ) -> ();
  pub fn CFStringGetCharacterFromInlineBuffer(buf: *mut CFStringInlineBuffer, idx: isize) -> u16;
  pub fn CFStringIsSurrogateHighCharacter(character: u16) -> u8;
  pub fn CFStringIsSurrogateLowCharacter(character: u16) -> u8;
  pub fn CFStringGetLongCharacterForSurrogatePair(surrogateHigh: u16, surrogateLow: u16) -> u32;
  pub fn CFStringGetSurrogatePairForLongCharacter(character: u32, surrogates: *mut u16) -> u8;
  pub fn CFShow(obj: *mut c_void) -> ();
  pub fn CFShowStr(str: *mut __CFString) -> ();
  pub fn __CFStringMakeConstantString(cStr: *mut i8) -> *mut __CFString;
  pub fn CFTimeZoneGetTypeID() -> usize;
  pub fn CFTimeZoneCopySystem() -> *mut __CFTimeZone;
  pub fn CFTimeZoneResetSystem() -> ();
  pub fn CFTimeZoneCopyDefault() -> *mut __CFTimeZone;
  pub fn CFTimeZoneSetDefault(tz: *mut __CFTimeZone) -> ();
  pub fn CFTimeZoneCopyKnownNames() -> *mut __CFArray;
  pub fn CFTimeZoneCopyAbbreviationDictionary() -> *mut __CFDictionary;
  pub fn CFTimeZoneSetAbbreviationDictionary(dict: *mut __CFDictionary) -> ();
  pub fn CFTimeZoneCreate(
    allocator: *mut __CFAllocator,
    name: *mut __CFString,
    data: *mut __CFData,
  ) -> *mut __CFTimeZone;
  pub fn CFTimeZoneCreateWithTimeIntervalFromGMT(
    allocator: *mut __CFAllocator,
    ti: f64,
  ) -> *mut __CFTimeZone;
  pub fn CFTimeZoneCreateWithName(
    allocator: *mut __CFAllocator,
    name: *mut __CFString,
    tryAbbrev: u8,
  ) -> *mut __CFTimeZone;
  pub fn CFTimeZoneGetName(tz: *mut __CFTimeZone) -> *mut __CFString;
  pub fn CFTimeZoneGetData(tz: *mut __CFTimeZone) -> *mut __CFData;
  pub fn CFTimeZoneGetSecondsFromGMT(tz: *mut __CFTimeZone, at: f64) -> f64;
  pub fn CFTimeZoneCopyAbbreviation(tz: *mut __CFTimeZone, at: f64) -> *mut __CFString;
  pub fn CFTimeZoneIsDaylightSavingTime(tz: *mut __CFTimeZone, at: f64) -> u8;
  pub fn CFTimeZoneGetDaylightSavingTimeOffset(tz: *mut __CFTimeZone, at: f64) -> f64;
  pub fn CFTimeZoneGetNextDaylightSavingTimeTransition(tz: *mut __CFTimeZone, at: f64) -> f64;
  pub fn CFTimeZoneCopyLocalizedName(
    tz: *mut __CFTimeZone,
    style: CFTimeZoneNameStyle,
    locale: *mut __CFLocale,
  ) -> *mut __CFString;
  pub fn CFCalendarGetTypeID() -> usize;
  pub fn CFCalendarCopyCurrent() -> *mut __CFCalendar;
  pub fn CFCalendarCreateWithIdentifier(
    allocator: *mut __CFAllocator,
    identifier: *mut __CFString,
  ) -> *mut __CFCalendar;
  pub fn CFCalendarGetIdentifier(calendar: *mut __CFCalendar) -> *mut __CFString;
  pub fn CFCalendarCopyLocale(calendar: *mut __CFCalendar) -> *mut __CFLocale;
  pub fn CFCalendarSetLocale(calendar: *mut __CFCalendar, locale: *mut __CFLocale) -> ();
  pub fn CFCalendarCopyTimeZone(calendar: *mut __CFCalendar) -> *mut __CFTimeZone;
  pub fn CFCalendarSetTimeZone(calendar: *mut __CFCalendar, tz: *mut __CFTimeZone) -> ();
  pub fn CFCalendarGetFirstWeekday(calendar: *mut __CFCalendar) -> isize;
  pub fn CFCalendarSetFirstWeekday(calendar: *mut __CFCalendar, wkdy: isize) -> ();
  pub fn CFCalendarGetMinimumDaysInFirstWeek(calendar: *mut __CFCalendar) -> isize;
  pub fn CFCalendarSetMinimumDaysInFirstWeek(calendar: *mut __CFCalendar, mwd: isize) -> ();
  pub fn CFCalendarGetMinimumRangeOfUnit(
    calendar: *mut __CFCalendar,
    unit: CFCalendarUnit,
  ) -> CFRange;
  pub fn CFCalendarGetMaximumRangeOfUnit(
    calendar: *mut __CFCalendar,
    unit: CFCalendarUnit,
  ) -> CFRange;
  pub fn CFCalendarGetRangeOfUnit(
    calendar: *mut __CFCalendar,
    smallerUnit: CFCalendarUnit,
    biggerUnit: CFCalendarUnit,
    at: f64,
  ) -> CFRange;
  pub fn CFCalendarGetOrdinalityOfUnit(
    calendar: *mut __CFCalendar,
    smallerUnit: CFCalendarUnit,
    biggerUnit: CFCalendarUnit,
    at: f64,
  ) -> isize;
  pub fn CFCalendarGetTimeRangeOfUnit(
    calendar: *mut __CFCalendar,
    unit: CFCalendarUnit,
    at: f64,
    startp: *mut f64,
    tip: *mut f64,
  ) -> u8;
  pub fn CFCalendarComposeAbsoluteTime(
    calendar: *mut __CFCalendar,
    at: *mut f64,
    componentDesc: *mut i8,
    ...
  ) -> u8;
  pub fn CFCalendarDecomposeAbsoluteTime(
    calendar: *mut __CFCalendar,
    at: f64,
    componentDesc: *mut i8,
    ...
  ) -> u8;
  pub fn CFCalendarAddComponents(
    calendar: *mut __CFCalendar,
    at: *mut f64,
    options: usize,
    componentDesc: *mut i8,
    ...
  ) -> u8;
  pub fn CFCalendarGetComponentDifference(
    calendar: *mut __CFCalendar,
    startingAT: f64,
    resultAT: f64,
    options: usize,
    componentDesc: *mut i8,
    ...
  ) -> u8;
  pub fn CFDateFormatterCreateDateFormatFromTemplate(
    allocator: *mut __CFAllocator,
    tmplate: *mut __CFString,
    options: usize,
    locale: *mut __CFLocale,
  ) -> *mut __CFString;
  pub fn CFDateFormatterGetTypeID() -> usize;
  pub fn CFDateFormatterCreateISO8601Formatter(
    allocator: *mut __CFAllocator,
    formatOptions: CFISO8601DateFormatOptions,
  ) -> *mut __CFDateFormatter;
  pub fn CFDateFormatterCreate(
    allocator: *mut __CFAllocator,
    locale: *mut __CFLocale,
    dateStyle: CFDateFormatterStyle,
    timeStyle: CFDateFormatterStyle,
  ) -> *mut __CFDateFormatter;
  pub fn CFDateFormatterGetLocale(formatter: *mut __CFDateFormatter) -> *mut __CFLocale;
  pub fn CFDateFormatterGetDateStyle(formatter: *mut __CFDateFormatter) -> CFDateFormatterStyle;
  pub fn CFDateFormatterGetTimeStyle(formatter: *mut __CFDateFormatter) -> CFDateFormatterStyle;
  pub fn CFDateFormatterGetFormat(formatter: *mut __CFDateFormatter) -> *mut __CFString;
  pub fn CFDateFormatterSetFormat(
    formatter: *mut __CFDateFormatter,
    formatString: *mut __CFString,
  ) -> ();
  pub fn CFDateFormatterCreateStringWithDate(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFDateFormatter,
    date: *mut __CFDate,
  ) -> *mut __CFString;
  pub fn CFDateFormatterCreateStringWithAbsoluteTime(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFDateFormatter,
    at: f64,
  ) -> *mut __CFString;
  pub fn CFDateFormatterCreateDateFromString(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFDateFormatter,
    string: *mut __CFString,
    rangep: *mut CFRange,
  ) -> *mut __CFDate;
  pub fn CFDateFormatterGetAbsoluteTimeFromString(
    formatter: *mut __CFDateFormatter,
    string: *mut __CFString,
    rangep: *mut CFRange,
    atp: *mut f64,
  ) -> u8;
  pub fn CFDateFormatterSetProperty(
    formatter: *mut __CFDateFormatter,
    key: *mut __CFString,
    value: *mut c_void,
  ) -> ();
  pub fn CFDateFormatterCopyProperty(
    formatter: *mut __CFDateFormatter,
    key: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFErrorGetTypeID() -> usize;
  pub fn CFErrorCreate(
    allocator: *mut __CFAllocator,
    domain: *mut __CFString,
    code: isize,
    userInfo: *mut __CFDictionary,
  ) -> *mut __CFError;
  pub fn CFErrorCreateWithUserInfoKeysAndValues(
    allocator: *mut __CFAllocator,
    domain: *mut __CFString,
    code: isize,
    userInfoKeys: *mut *const c_void,
    userInfoValues: *mut *const c_void,
    numUserInfoValues: isize,
  ) -> *mut __CFError;
  pub fn CFErrorGetDomain(err: *mut __CFError) -> *mut __CFString;
  pub fn CFErrorGetCode(err: *mut __CFError) -> isize;
  pub fn CFErrorCopyUserInfo(err: *mut __CFError) -> *mut __CFDictionary;
  pub fn CFErrorCopyDescription(err: *mut __CFError) -> *mut __CFString;
  pub fn CFErrorCopyFailureReason(err: *mut __CFError) -> *mut __CFString;
  pub fn CFErrorCopyRecoverySuggestion(err: *mut __CFError) -> *mut __CFString;
  pub fn CFBooleanGetTypeID() -> usize;
  pub fn CFBooleanGetValue(boolean: *mut __CFBoolean) -> u8;
  pub fn CFNumberGetTypeID() -> usize;
  pub fn CFNumberCreate(
    allocator: *mut __CFAllocator,
    theType: CFNumberType,
    valuePtr: *mut c_void,
  ) -> *mut __CFNumber;
  pub fn CFNumberGetType(number: *mut __CFNumber) -> CFNumberType;
  pub fn CFNumberGetByteSize(number: *mut __CFNumber) -> isize;
  pub fn CFNumberIsFloatType(number: *mut __CFNumber) -> u8;
  pub fn CFNumberGetValue(
    number: *mut __CFNumber,
    theType: CFNumberType,
    valuePtr: *mut c_void,
  ) -> u8;
  pub fn CFNumberCompare(
    number: *mut __CFNumber,
    otherNumber: *mut __CFNumber,
    context: *mut c_void,
  ) -> CFComparisonResult;
  pub fn CFNumberFormatterGetTypeID() -> usize;
  pub fn CFNumberFormatterCreate(
    allocator: *mut __CFAllocator,
    locale: *mut __CFLocale,
    style: CFNumberFormatterStyle,
  ) -> *mut __CFNumberFormatter;
  pub fn CFNumberFormatterGetLocale(formatter: *mut __CFNumberFormatter) -> *mut __CFLocale;
  pub fn CFNumberFormatterGetStyle(formatter: *mut __CFNumberFormatter) -> CFNumberFormatterStyle;
  pub fn CFNumberFormatterGetFormat(formatter: *mut __CFNumberFormatter) -> *mut __CFString;
  pub fn CFNumberFormatterSetFormat(
    formatter: *mut __CFNumberFormatter,
    formatString: *mut __CFString,
  ) -> ();
  pub fn CFNumberFormatterCreateStringWithNumber(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFNumberFormatter,
    number: *mut __CFNumber,
  ) -> *mut __CFString;
  pub fn CFNumberFormatterCreateStringWithValue(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFNumberFormatter,
    numberType: CFNumberType,
    valuePtr: *mut c_void,
  ) -> *mut __CFString;
  pub fn CFNumberFormatterCreateNumberFromString(
    allocator: *mut __CFAllocator,
    formatter: *mut __CFNumberFormatter,
    string: *mut __CFString,
    rangep: *mut CFRange,
    options: usize,
  ) -> *mut __CFNumber;
  pub fn CFNumberFormatterGetValueFromString(
    formatter: *mut __CFNumberFormatter,
    string: *mut __CFString,
    rangep: *mut CFRange,
    numberType: CFNumberType,
    valuePtr: *mut c_void,
  ) -> u8;
  pub fn CFNumberFormatterSetProperty(
    formatter: *mut __CFNumberFormatter,
    key: *mut __CFString,
    value: *mut c_void,
  ) -> ();
  pub fn CFNumberFormatterCopyProperty(
    formatter: *mut __CFNumberFormatter,
    key: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
    currencyCode: *mut __CFString,
    defaultFractionDigits: *mut i32,
    roundingIncrement: *mut f64,
  ) -> u8;
  pub fn CFPreferencesCopyAppValue(
    key: *mut __CFString,
    applicationID: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFPreferencesGetAppBooleanValue(
    key: *mut __CFString,
    applicationID: *mut __CFString,
    keyExistsAndHasValidFormat: *mut u8,
  ) -> u8;
  pub fn CFPreferencesGetAppIntegerValue(
    key: *mut __CFString,
    applicationID: *mut __CFString,
    keyExistsAndHasValidFormat: *mut u8,
  ) -> isize;
  pub fn CFPreferencesSetAppValue(
    key: *mut __CFString,
    value: *mut c_void,
    applicationID: *mut __CFString,
  ) -> ();
  pub fn CFPreferencesAddSuitePreferencesToApp(
    applicationID: *mut __CFString,
    suiteID: *mut __CFString,
  ) -> ();
  pub fn CFPreferencesRemoveSuitePreferencesFromApp(
    applicationID: *mut __CFString,
    suiteID: *mut __CFString,
  ) -> ();
  pub fn CFPreferencesAppSynchronize(applicationID: *mut __CFString) -> u8;
  pub fn CFPreferencesCopyValue(
    key: *mut __CFString,
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFPreferencesCopyMultiple(
    keysToFetch: *mut __CFArray,
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> *mut __CFDictionary;
  pub fn CFPreferencesSetValue(
    key: *mut __CFString,
    value: *mut c_void,
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> ();
  pub fn CFPreferencesSetMultiple(
    keysToSet: *mut __CFDictionary,
    keysToRemove: *mut __CFArray,
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> ();
  pub fn CFPreferencesSynchronize(
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> u8;
  pub fn CFPreferencesCopyApplicationList(
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFPreferencesCopyKeyList(
    applicationID: *mut __CFString,
    userName: *mut __CFString,
    hostName: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFPreferencesAppValueIsForced(key: *mut __CFString, applicationID: *mut __CFString) -> u8;
  pub fn CFURLGetTypeID() -> usize;
  pub fn CFURLCreateWithBytes(
    allocator: *mut __CFAllocator,
    URLBytes: *mut u8,
    length: isize,
    encoding: u32,
    baseURL: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLCreateData(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    encoding: u32,
    escapeWhitespace: u8,
  ) -> *mut __CFData;
  pub fn CFURLCreateWithString(
    allocator: *mut __CFAllocator,
    URLString: *mut __CFString,
    baseURL: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLCreateAbsoluteURLWithBytes(
    alloc: *mut __CFAllocator,
    relativeURLBytes: *mut u8,
    length: isize,
    encoding: u32,
    baseURL: *mut __CFURL,
    useCompatibilityMode: u8,
  ) -> *mut __CFURL;
  pub fn CFURLCreateWithFileSystemPath(
    allocator: *mut __CFAllocator,
    filePath: *mut __CFString,
    pathStyle: CFURLPathStyle,
    isDirectory: u8,
  ) -> *mut __CFURL;
  pub fn CFURLCreateFromFileSystemRepresentation(
    allocator: *mut __CFAllocator,
    buffer: *mut u8,
    bufLen: isize,
    isDirectory: u8,
  ) -> *mut __CFURL;
  pub fn CFURLCreateWithFileSystemPathRelativeToBase(
    allocator: *mut __CFAllocator,
    filePath: *mut __CFString,
    pathStyle: CFURLPathStyle,
    isDirectory: u8,
    baseURL: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLCreateFromFileSystemRepresentationRelativeToBase(
    allocator: *mut __CFAllocator,
    buffer: *mut u8,
    bufLen: isize,
    isDirectory: u8,
    baseURL: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLGetFileSystemRepresentation(
    url: *mut __CFURL,
    resolveAgainstBase: u8,
    buffer: *mut u8,
    maxBufLen: isize,
  ) -> u8;
  pub fn CFURLCopyAbsoluteURL(relativeURL: *mut __CFURL) -> *mut __CFURL;
  pub fn CFURLGetString(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLGetBaseURL(anURL: *mut __CFURL) -> *mut __CFURL;
  pub fn CFURLCanBeDecomposed(anURL: *mut __CFURL) -> u8;
  pub fn CFURLCopyScheme(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyNetLocation(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyPath(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyStrictPath(anURL: *mut __CFURL, isAbsolute: *mut u8) -> *mut __CFString;
  pub fn CFURLCopyFileSystemPath(anURL: *mut __CFURL, pathStyle: CFURLPathStyle)
    -> *mut __CFString;
  pub fn CFURLHasDirectoryPath(anURL: *mut __CFURL) -> u8;
  pub fn CFURLCopyResourceSpecifier(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyHostName(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLGetPortNumber(anURL: *mut __CFURL) -> i32;
  pub fn CFURLCopyUserName(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyPassword(anURL: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyParameterString(
    anURL: *mut __CFURL,
    charactersToLeaveEscaped: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFURLCopyQueryString(
    anURL: *mut __CFURL,
    charactersToLeaveEscaped: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFURLCopyFragment(
    anURL: *mut __CFURL,
    charactersToLeaveEscaped: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFURLCopyLastPathComponent(url: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCopyPathExtension(url: *mut __CFURL) -> *mut __CFString;
  pub fn CFURLCreateCopyAppendingPathComponent(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    pathComponent: *mut __CFString,
    isDirectory: u8,
  ) -> *mut __CFURL;
  pub fn CFURLCreateCopyDeletingLastPathComponent(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLCreateCopyAppendingPathExtension(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    extension: *mut __CFString,
  ) -> *mut __CFURL;
  pub fn CFURLCreateCopyDeletingPathExtension(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
  ) -> *mut __CFURL;
  pub fn CFURLGetBytes(url: *mut __CFURL, buffer: *mut u8, bufferLength: isize) -> isize;
  pub fn CFURLGetByteRangeForComponent(
    url: *mut __CFURL,
    component: CFURLComponentType,
    rangeIncludingSeparators: *mut CFRange,
  ) -> CFRange;
  pub fn CFURLCreateStringByReplacingPercentEscapes(
    allocator: *mut __CFAllocator,
    originalString: *mut __CFString,
    charactersToLeaveEscaped: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding(
    allocator: *mut __CFAllocator,
    origString: *mut __CFString,
    charsToLeaveEscaped: *mut __CFString,
    encoding: u32,
  ) -> *mut __CFString;
  pub fn CFURLCreateStringByAddingPercentEscapes(
    allocator: *mut __CFAllocator,
    originalString: *mut __CFString,
    charactersToLeaveUnescaped: *mut __CFString,
    legalURLCharactersToBeEscaped: *mut __CFString,
    encoding: u32,
  ) -> *mut __CFString;
  pub fn CFURLIsFileReferenceURL(url: *mut __CFURL) -> u8;
  pub fn CFURLCreateFileReferenceURL(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    error: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn CFURLCreateFilePathURL(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    error: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn CFURLCreateFromFSRef(allocator: *mut __CFAllocator, fsRef: *mut FSRef) -> *mut __CFURL;
  pub fn CFURLGetFSRef(url: *mut __CFURL, fsRef: *mut FSRef) -> u8;
  pub fn CFURLCopyResourcePropertyForKey(
    url: *mut __CFURL,
    key: *mut __CFString,
    propertyValueTypeRefPtr: *mut c_void,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CFURLCopyResourcePropertiesForKeys(
    url: *mut __CFURL,
    keys: *mut __CFArray,
    error: *mut *mut __CFError,
  ) -> *mut __CFDictionary;
  pub fn CFURLSetResourcePropertyForKey(
    url: *mut __CFURL,
    key: *mut __CFString,
    propertyValue: *mut c_void,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CFURLSetResourcePropertiesForKeys(
    url: *mut __CFURL,
    keyedPropertyValues: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CFURLClearResourcePropertyCacheForKey(url: *mut __CFURL, key: *mut __CFString) -> ();
  pub fn CFURLClearResourcePropertyCache(url: *mut __CFURL) -> ();
  pub fn CFURLSetTemporaryResourcePropertyForKey(
    url: *mut __CFURL,
    key: *mut __CFString,
    propertyValue: *mut c_void,
  ) -> ();
  pub fn CFURLResourceIsReachable(url: *mut __CFURL, error: *mut *mut __CFError) -> u8;
  pub fn CFURLCreateBookmarkData(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
    options: CFURLBookmarkCreationOptions,
    resourcePropertiesToInclude: *mut __CFArray,
    relativeToURL: *mut __CFURL,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn CFURLCreateByResolvingBookmarkData(
    allocator: *mut __CFAllocator,
    bookmark: *mut __CFData,
    options: CFURLBookmarkResolutionOptions,
    relativeToURL: *mut __CFURL,
    resourcePropertiesToInclude: *mut __CFArray,
    isStale: *mut u8,
    error: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn CFURLCreateResourcePropertiesForKeysFromBookmarkData(
    allocator: *mut __CFAllocator,
    resourcePropertiesToReturn: *mut __CFArray,
    bookmark: *mut __CFData,
  ) -> *mut __CFDictionary;
  pub fn CFURLCreateResourcePropertyForKeyFromBookmarkData(
    allocator: *mut __CFAllocator,
    resourcePropertyKey: *mut __CFString,
    bookmark: *mut __CFData,
  ) -> *mut c_void;
  pub fn CFURLCreateBookmarkDataFromFile(
    allocator: *mut __CFAllocator,
    fileURL: *mut __CFURL,
    errorRef: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn CFURLWriteBookmarkDataToFile(
    bookmarkRef: *mut __CFData,
    fileURL: *mut __CFURL,
    options: usize,
    errorRef: *mut *mut __CFError,
  ) -> u8;
  pub fn CFURLCreateBookmarkDataFromAliasRecord(
    allocatorRef: *mut __CFAllocator,
    aliasRecordDataRef: *mut __CFData,
  ) -> *mut __CFData;
  pub fn CFURLStartAccessingSecurityScopedResource(url: *mut __CFURL) -> u8;
  pub fn CFURLStopAccessingSecurityScopedResource(url: *mut __CFURL) -> ();
  pub fn CFRunLoopGetTypeID() -> usize;
  pub fn CFRunLoopGetCurrent() -> *mut __CFRunLoop;
  pub fn CFRunLoopGetMain() -> *mut __CFRunLoop;
  pub fn CFRunLoopCopyCurrentMode(rl: *mut __CFRunLoop) -> *mut __CFString;
  pub fn CFRunLoopCopyAllModes(rl: *mut __CFRunLoop) -> *mut __CFArray;
  pub fn CFRunLoopAddCommonMode(rl: *mut __CFRunLoop, mode: *mut __CFString) -> ();
  pub fn CFRunLoopGetNextTimerFireDate(rl: *mut __CFRunLoop, mode: *mut __CFString) -> f64;
  pub fn CFRunLoopRun() -> ();
  pub fn CFRunLoopRunInMode(
    mode: *mut __CFString,
    seconds: f64,
    returnAfterSourceHandled: u8,
  ) -> CFRunLoopRunResult;
  pub fn CFRunLoopIsWaiting(rl: *mut __CFRunLoop) -> u8;
  pub fn CFRunLoopWakeUp(rl: *mut __CFRunLoop) -> ();
  pub fn CFRunLoopStop(rl: *mut __CFRunLoop) -> ();
  pub fn CFRunLoopPerformBlock(rl: *mut __CFRunLoop, mode: *mut c_void, block: ()) -> ();
  pub fn CFRunLoopContainsSource(
    rl: *mut __CFRunLoop,
    source: *mut __CFRunLoopSource,
    mode: *mut __CFString,
  ) -> u8;
  pub fn CFRunLoopAddSource(
    rl: *mut __CFRunLoop,
    source: *mut __CFRunLoopSource,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopRemoveSource(
    rl: *mut __CFRunLoop,
    source: *mut __CFRunLoopSource,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopContainsObserver(
    rl: *mut __CFRunLoop,
    observer: *mut __CFRunLoopObserver,
    mode: *mut __CFString,
  ) -> u8;
  pub fn CFRunLoopAddObserver(
    rl: *mut __CFRunLoop,
    observer: *mut __CFRunLoopObserver,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopRemoveObserver(
    rl: *mut __CFRunLoop,
    observer: *mut __CFRunLoopObserver,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopContainsTimer(
    rl: *mut __CFRunLoop,
    timer: *mut __CFRunLoopTimer,
    mode: *mut __CFString,
  ) -> u8;
  pub fn CFRunLoopAddTimer(
    rl: *mut __CFRunLoop,
    timer: *mut __CFRunLoopTimer,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopRemoveTimer(
    rl: *mut __CFRunLoop,
    timer: *mut __CFRunLoopTimer,
    mode: *mut __CFString,
  ) -> ();
  pub fn CFRunLoopSourceGetTypeID() -> usize;
  pub fn CFRunLoopSourceCreate(
    allocator: *mut __CFAllocator,
    order: isize,
    context: *mut CFRunLoopSourceContext,
  ) -> *mut __CFRunLoopSource;
  pub fn CFRunLoopSourceGetOrder(source: *mut __CFRunLoopSource) -> isize;
  pub fn CFRunLoopSourceInvalidate(source: *mut __CFRunLoopSource) -> ();
  pub fn CFRunLoopSourceIsValid(source: *mut __CFRunLoopSource) -> u8;
  pub fn CFRunLoopSourceGetContext(
    source: *mut __CFRunLoopSource,
    context: *mut CFRunLoopSourceContext,
  ) -> ();
  pub fn CFRunLoopSourceSignal(source: *mut __CFRunLoopSource) -> ();
  pub fn CFRunLoopObserverGetTypeID() -> usize;
  pub fn CFRunLoopObserverCreate(
    allocator: *mut __CFAllocator,
    activities: usize,
    repeats: u8,
    order: isize,
    callout: Option<extern "C" fn(*mut __CFRunLoopObserver, CFRunLoopActivity, *mut c_void) -> ()>,
    context: *mut CFRunLoopObserverContext,
  ) -> *mut __CFRunLoopObserver;
  pub fn CFRunLoopObserverCreateWithHandler(
    allocator: *mut __CFAllocator,
    activities: usize,
    repeats: u8,
    order: isize,
    block: (),
  ) -> *mut __CFRunLoopObserver;
  pub fn CFRunLoopObserverGetActivities(observer: *mut __CFRunLoopObserver) -> usize;
  pub fn CFRunLoopObserverDoesRepeat(observer: *mut __CFRunLoopObserver) -> u8;
  pub fn CFRunLoopObserverGetOrder(observer: *mut __CFRunLoopObserver) -> isize;
  pub fn CFRunLoopObserverInvalidate(observer: *mut __CFRunLoopObserver) -> ();
  pub fn CFRunLoopObserverIsValid(observer: *mut __CFRunLoopObserver) -> u8;
  pub fn CFRunLoopObserverGetContext(
    observer: *mut __CFRunLoopObserver,
    context: *mut CFRunLoopObserverContext,
  ) -> ();
  pub fn CFRunLoopTimerGetTypeID() -> usize;
  pub fn CFRunLoopTimerCreate(
    allocator: *mut __CFAllocator,
    fireDate: f64,
    interval: f64,
    flags: usize,
    order: isize,
    callout: Option<extern "C" fn(*mut __CFRunLoopTimer, *mut c_void) -> ()>,
    context: *mut CFRunLoopTimerContext,
  ) -> *mut __CFRunLoopTimer;
  pub fn CFRunLoopTimerCreateWithHandler(
    allocator: *mut __CFAllocator,
    fireDate: f64,
    interval: f64,
    flags: usize,
    order: isize,
    block: (),
  ) -> *mut __CFRunLoopTimer;
  pub fn CFRunLoopTimerGetNextFireDate(timer: *mut __CFRunLoopTimer) -> f64;
  pub fn CFRunLoopTimerSetNextFireDate(timer: *mut __CFRunLoopTimer, fireDate: f64) -> ();
  pub fn CFRunLoopTimerGetInterval(timer: *mut __CFRunLoopTimer) -> f64;
  pub fn CFRunLoopTimerDoesRepeat(timer: *mut __CFRunLoopTimer) -> u8;
  pub fn CFRunLoopTimerGetOrder(timer: *mut __CFRunLoopTimer) -> isize;
  pub fn CFRunLoopTimerInvalidate(timer: *mut __CFRunLoopTimer) -> ();
  pub fn CFRunLoopTimerIsValid(timer: *mut __CFRunLoopTimer) -> u8;
  pub fn CFRunLoopTimerGetContext(
    timer: *mut __CFRunLoopTimer,
    context: *mut CFRunLoopTimerContext,
  ) -> ();
  pub fn CFRunLoopTimerGetTolerance(timer: *mut __CFRunLoopTimer) -> f64;
  pub fn CFRunLoopTimerSetTolerance(timer: *mut __CFRunLoopTimer, tolerance: f64) -> ();
  pub fn CFSocketGetTypeID() -> usize;
  pub fn CFSocketCreate(
    allocator: *mut __CFAllocator,
    protocolFamily: i32,
    socketType: i32,
    protocol: i32,
    callBackTypes: usize,
    callout: Option<
      extern "C" fn(
        *mut __CFSocket,
        CFSocketCallBackType,
        *mut __CFData,
        *mut c_void,
        *mut c_void,
      ) -> (),
    >,
    context: *mut CFSocketContext,
  ) -> *mut __CFSocket;
  pub fn CFSocketCreateWithNative(
    allocator: *mut __CFAllocator,
    sock: i32,
    callBackTypes: usize,
    callout: Option<
      extern "C" fn(
        *mut __CFSocket,
        CFSocketCallBackType,
        *mut __CFData,
        *mut c_void,
        *mut c_void,
      ) -> (),
    >,
    context: *mut CFSocketContext,
  ) -> *mut __CFSocket;
  pub fn CFSocketCreateWithSocketSignature(
    allocator: *mut __CFAllocator,
    signature: *mut CFSocketSignature,
    callBackTypes: usize,
    callout: Option<
      extern "C" fn(
        *mut __CFSocket,
        CFSocketCallBackType,
        *mut __CFData,
        *mut c_void,
        *mut c_void,
      ) -> (),
    >,
    context: *mut CFSocketContext,
  ) -> *mut __CFSocket;
  pub fn CFSocketCreateConnectedToSocketSignature(
    allocator: *mut __CFAllocator,
    signature: *mut CFSocketSignature,
    callBackTypes: usize,
    callout: Option<
      extern "C" fn(
        *mut __CFSocket,
        CFSocketCallBackType,
        *mut __CFData,
        *mut c_void,
        *mut c_void,
      ) -> (),
    >,
    context: *mut CFSocketContext,
    timeout: f64,
  ) -> *mut __CFSocket;
  pub fn CFSocketSetAddress(s: *mut __CFSocket, address: *mut __CFData) -> CFSocketError;
  pub fn CFSocketConnectToAddress(
    s: *mut __CFSocket,
    address: *mut __CFData,
    timeout: f64,
  ) -> CFSocketError;
  pub fn CFSocketInvalidate(s: *mut __CFSocket) -> ();
  pub fn CFSocketIsValid(s: *mut __CFSocket) -> u8;
  pub fn CFSocketCopyAddress(s: *mut __CFSocket) -> *mut __CFData;
  pub fn CFSocketCopyPeerAddress(s: *mut __CFSocket) -> *mut __CFData;
  pub fn CFSocketGetContext(s: *mut __CFSocket, context: *mut CFSocketContext) -> ();
  pub fn CFSocketGetNative(s: *mut __CFSocket) -> i32;
  pub fn CFSocketCreateRunLoopSource(
    allocator: *mut __CFAllocator,
    s: *mut __CFSocket,
    order: isize,
  ) -> *mut __CFRunLoopSource;
  pub fn CFSocketGetSocketFlags(s: *mut __CFSocket) -> usize;
  pub fn CFSocketSetSocketFlags(s: *mut __CFSocket, flags: usize) -> ();
  pub fn CFSocketDisableCallBacks(s: *mut __CFSocket, callBackTypes: usize) -> ();
  pub fn CFSocketEnableCallBacks(s: *mut __CFSocket, callBackTypes: usize) -> ();
  pub fn CFSocketSendData(
    s: *mut __CFSocket,
    address: *mut __CFData,
    data: *mut __CFData,
    timeout: f64,
  ) -> CFSocketError;
  pub fn CFSocketRegisterValue(
    nameServerSignature: *mut CFSocketSignature,
    timeout: f64,
    name: *mut __CFString,
    value: *mut c_void,
  ) -> CFSocketError;
  pub fn CFSocketCopyRegisteredValue(
    nameServerSignature: *mut CFSocketSignature,
    timeout: f64,
    name: *mut __CFString,
    value: *mut *mut c_void,
    nameServerAddress: *mut *mut __CFData,
  ) -> CFSocketError;
  pub fn CFSocketRegisterSocketSignature(
    nameServerSignature: *mut CFSocketSignature,
    timeout: f64,
    name: *mut __CFString,
    signature: *mut CFSocketSignature,
  ) -> CFSocketError;
  pub fn CFSocketCopyRegisteredSocketSignature(
    nameServerSignature: *mut CFSocketSignature,
    timeout: f64,
    name: *mut __CFString,
    signature: *mut CFSocketSignature,
    nameServerAddress: *mut *mut __CFData,
  ) -> CFSocketError;
  pub fn CFSocketUnregister(
    nameServerSignature: *mut CFSocketSignature,
    timeout: f64,
    name: *mut __CFString,
  ) -> CFSocketError;
  pub fn CFSocketSetDefaultNameRegistryPortNumber(port: u16) -> ();
  pub fn CFSocketGetDefaultNameRegistryPortNumber() -> u16;
  pub fn CFReadStreamGetTypeID() -> usize;
  pub fn CFWriteStreamGetTypeID() -> usize;
  pub fn CFReadStreamCreateWithBytesNoCopy(
    alloc: *mut __CFAllocator,
    bytes: *mut u8,
    length: isize,
    bytesDeallocator: *mut __CFAllocator,
  ) -> *mut __CFReadStream;
  pub fn CFWriteStreamCreateWithBuffer(
    alloc: *mut __CFAllocator,
    buffer: *mut u8,
    bufferCapacity: isize,
  ) -> *mut __CFWriteStream;
  pub fn CFWriteStreamCreateWithAllocatedBuffers(
    alloc: *mut __CFAllocator,
    bufferAllocator: *mut __CFAllocator,
  ) -> *mut __CFWriteStream;
  pub fn CFReadStreamCreateWithFile(
    alloc: *mut __CFAllocator,
    fileURL: *mut __CFURL,
  ) -> *mut __CFReadStream;
  pub fn CFWriteStreamCreateWithFile(
    alloc: *mut __CFAllocator,
    fileURL: *mut __CFURL,
  ) -> *mut __CFWriteStream;
  pub fn CFStreamCreateBoundPair(
    alloc: *mut __CFAllocator,
    readStream: *mut *mut __CFReadStream,
    writeStream: *mut *mut __CFWriteStream,
    transferBufferSize: isize,
  ) -> ();
  pub fn CFStreamCreatePairWithSocket(
    alloc: *mut __CFAllocator,
    sock: i32,
    readStream: *mut *mut __CFReadStream,
    writeStream: *mut *mut __CFWriteStream,
  ) -> ();
  pub fn CFStreamCreatePairWithSocketToHost(
    alloc: *mut __CFAllocator,
    host: *mut __CFString,
    port: u32,
    readStream: *mut *mut __CFReadStream,
    writeStream: *mut *mut __CFWriteStream,
  ) -> ();
  pub fn CFStreamCreatePairWithPeerSocketSignature(
    alloc: *mut __CFAllocator,
    signature: *mut CFSocketSignature,
    readStream: *mut *mut __CFReadStream,
    writeStream: *mut *mut __CFWriteStream,
  ) -> ();
  pub fn CFReadStreamGetStatus(stream: *mut __CFReadStream) -> CFStreamStatus;
  pub fn CFWriteStreamGetStatus(stream: *mut __CFWriteStream) -> CFStreamStatus;
  pub fn CFReadStreamCopyError(stream: *mut __CFReadStream) -> *mut __CFError;
  pub fn CFWriteStreamCopyError(stream: *mut __CFWriteStream) -> *mut __CFError;
  pub fn CFReadStreamOpen(stream: *mut __CFReadStream) -> u8;
  pub fn CFWriteStreamOpen(stream: *mut __CFWriteStream) -> u8;
  pub fn CFReadStreamClose(stream: *mut __CFReadStream) -> ();
  pub fn CFWriteStreamClose(stream: *mut __CFWriteStream) -> ();
  pub fn CFReadStreamHasBytesAvailable(stream: *mut __CFReadStream) -> u8;
  pub fn CFReadStreamRead(
    stream: *mut __CFReadStream,
    buffer: *mut u8,
    bufferLength: isize,
  ) -> isize;
  pub fn CFReadStreamGetBuffer(
    stream: *mut __CFReadStream,
    maxBytesToRead: isize,
    numBytesRead: *mut isize,
  ) -> *mut u8;
  pub fn CFWriteStreamCanAcceptBytes(stream: *mut __CFWriteStream) -> u8;
  pub fn CFWriteStreamWrite(
    stream: *mut __CFWriteStream,
    buffer: *mut u8,
    bufferLength: isize,
  ) -> isize;
  pub fn CFReadStreamCopyProperty(
    stream: *mut __CFReadStream,
    propertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFWriteStreamCopyProperty(
    stream: *mut __CFWriteStream,
    propertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFReadStreamSetProperty(
    stream: *mut __CFReadStream,
    propertyName: *mut __CFString,
    propertyValue: *mut c_void,
  ) -> u8;
  pub fn CFWriteStreamSetProperty(
    stream: *mut __CFWriteStream,
    propertyName: *mut __CFString,
    propertyValue: *mut c_void,
  ) -> u8;
  pub fn CFReadStreamSetClient(
    stream: *mut __CFReadStream,
    streamEvents: usize,
    clientCB: Option<extern "C" fn(*mut __CFReadStream, CFStreamEventType, *mut c_void) -> ()>,
    clientContext: *mut CFStreamClientContext,
  ) -> u8;
  pub fn CFWriteStreamSetClient(
    stream: *mut __CFWriteStream,
    streamEvents: usize,
    clientCB: Option<extern "C" fn(*mut __CFWriteStream, CFStreamEventType, *mut c_void) -> ()>,
    clientContext: *mut CFStreamClientContext,
  ) -> u8;
  pub fn CFReadStreamScheduleWithRunLoop(
    stream: *mut __CFReadStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn CFWriteStreamScheduleWithRunLoop(
    stream: *mut __CFWriteStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn CFReadStreamUnscheduleFromRunLoop(
    stream: *mut __CFReadStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn CFWriteStreamUnscheduleFromRunLoop(
    stream: *mut __CFWriteStream,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn CFReadStreamSetDispatchQueue(stream: *mut __CFReadStream, q: *mut NSObject) -> ();
  pub fn CFWriteStreamSetDispatchQueue(stream: *mut __CFWriteStream, q: *mut NSObject) -> ();
  pub fn CFReadStreamCopyDispatchQueue(stream: *mut __CFReadStream) -> *mut NSObject;
  pub fn CFWriteStreamCopyDispatchQueue(stream: *mut __CFWriteStream) -> *mut NSObject;
  pub fn CFReadStreamGetError(stream: *mut __CFReadStream) -> CFStreamError;
  pub fn CFWriteStreamGetError(stream: *mut __CFWriteStream) -> CFStreamError;
  pub fn CFPropertyListCreateFromXMLData(
    allocator: *mut __CFAllocator,
    xmlData: *mut __CFData,
    mutabilityOption: usize,
    errorString: *mut *mut __CFString,
  ) -> *mut c_void;
  pub fn CFPropertyListCreateXMLData(
    allocator: *mut __CFAllocator,
    propertyList: *mut c_void,
  ) -> *mut __CFData;
  pub fn CFPropertyListCreateDeepCopy(
    allocator: *mut __CFAllocator,
    propertyList: *mut c_void,
    mutabilityOption: usize,
  ) -> *mut c_void;
  pub fn CFPropertyListIsValid(plist: *mut c_void, format: CFPropertyListFormat) -> u8;
  pub fn CFPropertyListWriteToStream(
    propertyList: *mut c_void,
    stream: *mut __CFWriteStream,
    format: CFPropertyListFormat,
    errorString: *mut *mut __CFString,
  ) -> isize;
  pub fn CFPropertyListCreateFromStream(
    allocator: *mut __CFAllocator,
    stream: *mut __CFReadStream,
    streamLength: isize,
    mutabilityOption: usize,
    format: *mut CFPropertyListFormat,
    errorString: *mut *mut __CFString,
  ) -> *mut c_void;
  pub fn CFPropertyListCreateWithData(
    allocator: *mut __CFAllocator,
    data: *mut __CFData,
    options: usize,
    format: *mut CFPropertyListFormat,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn CFPropertyListCreateWithStream(
    allocator: *mut __CFAllocator,
    stream: *mut __CFReadStream,
    streamLength: isize,
    options: usize,
    format: *mut CFPropertyListFormat,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn CFPropertyListWrite(
    propertyList: *mut c_void,
    stream: *mut __CFWriteStream,
    format: CFPropertyListFormat,
    options: usize,
    error: *mut *mut __CFError,
  ) -> isize;
  pub fn CFPropertyListCreateData(
    allocator: *mut __CFAllocator,
    propertyList: *mut c_void,
    format: CFPropertyListFormat,
    options: usize,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn CFSetGetTypeID() -> usize;
  pub fn CFSetCreate(
    allocator: *mut __CFAllocator,
    values: *mut *mut c_void,
    numValues: isize,
    callBacks: *mut CFSetCallBacks,
  ) -> *mut __CFSet;
  pub fn CFSetCreateCopy(allocator: *mut __CFAllocator, theSet: *mut __CFSet) -> *mut __CFSet;
  pub fn CFSetCreateMutable(
    allocator: *mut __CFAllocator,
    capacity: isize,
    callBacks: *mut CFSetCallBacks,
  ) -> *mut __CFSet;
  pub fn CFSetCreateMutableCopy(
    allocator: *mut __CFAllocator,
    capacity: isize,
    theSet: *mut __CFSet,
  ) -> *mut __CFSet;
  pub fn CFSetGetCount(theSet: *mut __CFSet) -> isize;
  pub fn CFSetGetCountOfValue(theSet: *mut __CFSet, value: *mut c_void) -> isize;
  pub fn CFSetContainsValue(theSet: *mut __CFSet, value: *mut c_void) -> u8;
  pub fn CFSetGetValue(theSet: *mut __CFSet, value: *mut c_void) -> *mut c_void;
  pub fn CFSetGetValueIfPresent(
    theSet: *mut __CFSet,
    candidate: *mut c_void,
    value: *mut *mut c_void,
  ) -> u8;
  pub fn CFSetGetValues(theSet: *mut __CFSet, values: *mut *mut c_void) -> ();
  pub fn CFSetApplyFunction(
    theSet: *mut __CFSet,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFSetAddValue(theSet: *mut __CFSet, value: *mut c_void) -> ();
  pub fn CFSetReplaceValue(theSet: *mut __CFSet, value: *mut c_void) -> ();
  pub fn CFSetSetValue(theSet: *mut __CFSet, value: *mut c_void) -> ();
  pub fn CFSetRemoveValue(theSet: *mut __CFSet, value: *mut c_void) -> ();
  pub fn CFSetRemoveAllValues(theSet: *mut __CFSet) -> ();
  pub fn CFTreeGetTypeID() -> usize;
  pub fn CFTreeCreate(allocator: *mut __CFAllocator, context: *mut CFTreeContext) -> *mut __CFTree;
  pub fn CFTreeGetParent(tree: *mut __CFTree) -> *mut __CFTree;
  pub fn CFTreeGetNextSibling(tree: *mut __CFTree) -> *mut __CFTree;
  pub fn CFTreeGetFirstChild(tree: *mut __CFTree) -> *mut __CFTree;
  pub fn CFTreeGetContext(tree: *mut __CFTree, context: *mut CFTreeContext) -> ();
  pub fn CFTreeGetChildCount(tree: *mut __CFTree) -> isize;
  pub fn CFTreeGetChildAtIndex(tree: *mut __CFTree, idx: isize) -> *mut __CFTree;
  pub fn CFTreeGetChildren(tree: *mut __CFTree, children: *mut *mut __CFTree) -> ();
  pub fn CFTreeApplyFunctionToChildren(
    tree: *mut __CFTree,
    applier: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    context: *mut c_void,
  ) -> ();
  pub fn CFTreeFindRoot(tree: *mut __CFTree) -> *mut __CFTree;
  pub fn CFTreeSetContext(tree: *mut __CFTree, context: *mut CFTreeContext) -> ();
  pub fn CFTreePrependChild(tree: *mut __CFTree, newChild: *mut __CFTree) -> ();
  pub fn CFTreeAppendChild(tree: *mut __CFTree, newChild: *mut __CFTree) -> ();
  pub fn CFTreeInsertSibling(tree: *mut __CFTree, newSibling: *mut __CFTree) -> ();
  pub fn CFTreeRemove(tree: *mut __CFTree) -> ();
  pub fn CFTreeRemoveAllChildren(tree: *mut __CFTree) -> ();
  pub fn CFTreeSortChildren(
    tree: *mut __CFTree,
    comparator: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> CFComparisonResult>,
    context: *mut c_void,
  ) -> ();
  pub fn CFURLCreateDataAndPropertiesFromResource(
    alloc: *mut __CFAllocator,
    url: *mut __CFURL,
    resourceData: *mut *mut __CFData,
    properties: *mut *mut __CFDictionary,
    desiredProperties: *mut __CFArray,
    errorCode: *mut i32,
  ) -> u8;
  pub fn CFURLWriteDataAndPropertiesToResource(
    url: *mut __CFURL,
    dataToWrite: *mut __CFData,
    propertiesToWrite: *mut __CFDictionary,
    errorCode: *mut i32,
  ) -> u8;
  pub fn CFURLDestroyResource(url: *mut __CFURL, errorCode: *mut i32) -> u8;
  pub fn CFURLCreatePropertyFromResource(
    alloc: *mut __CFAllocator,
    url: *mut __CFURL,
    property: *mut __CFString,
    errorCode: *mut i32,
  ) -> *mut c_void;
  pub fn CFUUIDGetTypeID() -> usize;
  pub fn CFUUIDCreate(alloc: *mut __CFAllocator) -> *mut __CFUUID;
  pub fn CFUUIDCreateWithBytes(
    alloc: *mut __CFAllocator,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    byte8: u8,
    byte9: u8,
    byte10: u8,
    byte11: u8,
    byte12: u8,
    byte13: u8,
    byte14: u8,
    byte15: u8,
  ) -> *mut __CFUUID;
  pub fn CFUUIDCreateFromString(
    alloc: *mut __CFAllocator,
    uuidStr: *mut __CFString,
  ) -> *mut __CFUUID;
  pub fn CFUUIDCreateString(alloc: *mut __CFAllocator, uuid: *mut __CFUUID) -> *mut __CFString;
  pub fn CFUUIDGetConstantUUIDWithBytes(
    alloc: *mut __CFAllocator,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    byte8: u8,
    byte9: u8,
    byte10: u8,
    byte11: u8,
    byte12: u8,
    byte13: u8,
    byte14: u8,
    byte15: u8,
  ) -> *mut __CFUUID;
  pub fn CFUUIDGetUUIDBytes(uuid: *mut __CFUUID) -> CFUUIDBytes;
  pub fn CFUUIDCreateFromUUIDBytes(alloc: *mut __CFAllocator, bytes: CFUUIDBytes) -> *mut __CFUUID;
  pub fn CFBundleGetMainBundle() -> *mut __CFBundle;
  pub fn CFBundleGetBundleWithIdentifier(bundleID: *mut __CFString) -> *mut __CFBundle;
  pub fn CFBundleGetAllBundles() -> *mut __CFArray;
  pub fn CFBundleGetTypeID() -> usize;
  pub fn CFBundleCreate(allocator: *mut __CFAllocator, bundleURL: *mut __CFURL) -> *mut __CFBundle;
  pub fn CFBundleCreateBundlesFromDirectory(
    allocator: *mut __CFAllocator,
    directoryURL: *mut __CFURL,
    bundleType: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFBundleCopyBundleURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleGetValueForInfoDictionaryKey(
    bundle: *mut __CFBundle,
    key: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFBundleGetInfoDictionary(bundle: *mut __CFBundle) -> *mut __CFDictionary;
  pub fn CFBundleGetLocalInfoDictionary(bundle: *mut __CFBundle) -> *mut __CFDictionary;
  pub fn CFBundleGetPackageInfo(
    bundle: *mut __CFBundle,
    packageType: *mut u32,
    packageCreator: *mut u32,
  ) -> ();
  pub fn CFBundleGetIdentifier(bundle: *mut __CFBundle) -> *mut __CFString;
  pub fn CFBundleGetVersionNumber(bundle: *mut __CFBundle) -> u32;
  pub fn CFBundleGetDevelopmentRegion(bundle: *mut __CFBundle) -> *mut __CFString;
  pub fn CFBundleCopySupportFilesDirectoryURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopyResourcesDirectoryURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopyPrivateFrameworksURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopySharedFrameworksURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopySharedSupportURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopyBuiltInPlugInsURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopyInfoDictionaryInDirectory(bundleURL: *mut __CFURL) -> *mut __CFDictionary;
  pub fn CFBundleGetPackageInfoInDirectory(
    url: *mut __CFURL,
    packageType: *mut u32,
    packageCreator: *mut u32,
  ) -> u8;
  pub fn CFBundleCopyResourceURL(
    bundle: *mut __CFBundle,
    resourceName: *mut __CFString,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
  ) -> *mut __CFURL;
  pub fn CFBundleCopyResourceURLsOfType(
    bundle: *mut __CFBundle,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFBundleCopyLocalizedString(
    bundle: *mut __CFBundle,
    key: *mut __CFString,
    value: *mut __CFString,
    tableName: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CFBundleCopyResourceURLInDirectory(
    bundleURL: *mut __CFURL,
    resourceName: *mut __CFString,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
  ) -> *mut __CFURL;
  pub fn CFBundleCopyResourceURLsOfTypeInDirectory(
    bundleURL: *mut __CFURL,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFBundleCopyBundleLocalizations(bundle: *mut __CFBundle) -> *mut __CFArray;
  pub fn CFBundleCopyPreferredLocalizationsFromArray(locArray: *mut __CFArray) -> *mut __CFArray;
  pub fn CFBundleCopyLocalizationsForPreferences(
    locArray: *mut __CFArray,
    prefArray: *mut __CFArray,
  ) -> *mut __CFArray;
  pub fn CFBundleCopyResourceURLForLocalization(
    bundle: *mut __CFBundle,
    resourceName: *mut __CFString,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
    localizationName: *mut __CFString,
  ) -> *mut __CFURL;
  pub fn CFBundleCopyResourceURLsOfTypeForLocalization(
    bundle: *mut __CFBundle,
    resourceType: *mut __CFString,
    subDirName: *mut __CFString,
    localizationName: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn CFBundleCopyInfoDictionaryForURL(url: *mut __CFURL) -> *mut __CFDictionary;
  pub fn CFBundleCopyLocalizationsForURL(url: *mut __CFURL) -> *mut __CFArray;
  pub fn CFBundleCopyExecutableArchitecturesForURL(url: *mut __CFURL) -> *mut __CFArray;
  pub fn CFBundleCopyExecutableURL(bundle: *mut __CFBundle) -> *mut __CFURL;
  pub fn CFBundleCopyExecutableArchitectures(bundle: *mut __CFBundle) -> *mut __CFArray;
  pub fn CFBundlePreflightExecutable(bundle: *mut __CFBundle, error: *mut *mut __CFError) -> u8;
  pub fn CFBundleLoadExecutableAndReturnError(
    bundle: *mut __CFBundle,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CFBundleLoadExecutable(bundle: *mut __CFBundle) -> u8;
  pub fn CFBundleIsExecutableLoaded(bundle: *mut __CFBundle) -> u8;
  pub fn CFBundleUnloadExecutable(bundle: *mut __CFBundle) -> ();
  pub fn CFBundleGetFunctionPointerForName(
    bundle: *mut __CFBundle,
    functionName: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFBundleGetFunctionPointersForNames(
    bundle: *mut __CFBundle,
    functionNames: *mut __CFArray,
    ftbl: *mut *mut c_void,
  ) -> ();
  pub fn CFBundleGetDataPointerForName(
    bundle: *mut __CFBundle,
    symbolName: *mut __CFString,
  ) -> *mut c_void;
  pub fn CFBundleGetDataPointersForNames(
    bundle: *mut __CFBundle,
    symbolNames: *mut __CFArray,
    stbl: *mut *mut c_void,
  ) -> ();
  pub fn CFBundleCopyAuxiliaryExecutableURL(
    bundle: *mut __CFBundle,
    executableName: *mut __CFString,
  ) -> *mut __CFURL;
  pub fn CFBundleGetPlugIn(bundle: *mut __CFBundle) -> *mut __CFBundle;
  pub fn CFBundleOpenBundleResourceMap(bundle: *mut __CFBundle) -> i32;
  pub fn CFBundleOpenBundleResourceFiles(
    bundle: *mut __CFBundle,
    refNum: *mut i32,
    localizedRefNum: *mut i32,
  ) -> i32;
  pub fn CFBundleCloseBundleResourceMap(bundle: *mut __CFBundle, refNum: i32) -> ();
  pub fn CFMessagePortGetTypeID() -> usize;
  pub fn CFMessagePortCreateLocal(
    allocator: *mut __CFAllocator,
    name: *mut __CFString,
    callout: Option<
      extern "C" fn(*mut __CFMessagePort, i32, *mut __CFData, *mut c_void) -> *mut __CFData,
    >,
    context: *mut CFMessagePortContext,
    shouldFreeInfo: *mut u8,
  ) -> *mut __CFMessagePort;
  pub fn CFMessagePortCreateRemote(
    allocator: *mut __CFAllocator,
    name: *mut __CFString,
  ) -> *mut __CFMessagePort;
  pub fn CFMessagePortIsRemote(ms: *mut __CFMessagePort) -> u8;
  pub fn CFMessagePortGetName(ms: *mut __CFMessagePort) -> *mut __CFString;
  pub fn CFMessagePortSetName(ms: *mut __CFMessagePort, newName: *mut __CFString) -> u8;
  pub fn CFMessagePortGetContext(
    ms: *mut __CFMessagePort,
    context: *mut CFMessagePortContext,
  ) -> ();
  pub fn CFMessagePortInvalidate(ms: *mut __CFMessagePort) -> ();
  pub fn CFMessagePortIsValid(ms: *mut __CFMessagePort) -> u8;
  pub fn CFMessagePortGetInvalidationCallBack(
    ms: *mut __CFMessagePort,
  ) -> Option<extern "C" fn(*mut __CFMessagePort, *mut c_void) -> ()>;
  pub fn CFMessagePortSetInvalidationCallBack(
    ms: *mut __CFMessagePort,
    callout: Option<extern "C" fn(*mut __CFMessagePort, *mut c_void) -> ()>,
  ) -> ();
  pub fn CFMessagePortSendRequest(
    remote: *mut __CFMessagePort,
    msgid: i32,
    data: *mut __CFData,
    sendTimeout: f64,
    rcvTimeout: f64,
    replyMode: *mut __CFString,
    returnData: *mut *mut __CFData,
  ) -> i32;
  pub fn CFMessagePortCreateRunLoopSource(
    allocator: *mut __CFAllocator,
    local: *mut __CFMessagePort,
    order: isize,
  ) -> *mut __CFRunLoopSource;
  pub fn CFMessagePortSetDispatchQueue(ms: *mut __CFMessagePort, queue: *mut NSObject) -> ();
  pub fn CFPlugInGetTypeID() -> usize;
  pub fn CFPlugInCreate(allocator: *mut __CFAllocator, plugInURL: *mut __CFURL) -> *mut __CFBundle;
  pub fn CFPlugInGetBundle(plugIn: *mut __CFBundle) -> *mut __CFBundle;
  pub fn CFPlugInSetLoadOnDemand(plugIn: *mut __CFBundle, flag: u8) -> ();
  pub fn CFPlugInIsLoadOnDemand(plugIn: *mut __CFBundle) -> u8;
  pub fn CFPlugInFindFactoriesForPlugInType(typeUUID: *mut __CFUUID) -> *mut __CFArray;
  pub fn CFPlugInFindFactoriesForPlugInTypeInPlugIn(
    typeUUID: *mut __CFUUID,
    plugIn: *mut __CFBundle,
  ) -> *mut __CFArray;
  pub fn CFPlugInInstanceCreate(
    allocator: *mut __CFAllocator,
    factoryUUID: *mut __CFUUID,
    typeUUID: *mut __CFUUID,
  ) -> *mut c_void;
  pub fn CFPlugInRegisterFactoryFunction(
    factoryUUID: *mut __CFUUID,
    func: Option<extern "C" fn(*mut __CFAllocator, *mut __CFUUID) -> *mut c_void>,
  ) -> u8;
  pub fn CFPlugInRegisterFactoryFunctionByName(
    factoryUUID: *mut __CFUUID,
    plugIn: *mut __CFBundle,
    functionName: *mut __CFString,
  ) -> u8;
  pub fn CFPlugInUnregisterFactory(factoryUUID: *mut __CFUUID) -> u8;
  pub fn CFPlugInRegisterPlugInType(factoryUUID: *mut __CFUUID, typeUUID: *mut __CFUUID) -> u8;
  pub fn CFPlugInUnregisterPlugInType(factoryUUID: *mut __CFUUID, typeUUID: *mut __CFUUID) -> u8;
  pub fn CFPlugInAddInstanceForFactory(factoryID: *mut __CFUUID) -> ();
  pub fn CFPlugInRemoveInstanceForFactory(factoryID: *mut __CFUUID) -> ();
  pub fn CFPlugInInstanceGetInterfaceFunctionTable(
    instance: *mut __CFPlugInInstance,
    interfaceName: *mut __CFString,
    ftbl: *mut *mut c_void,
  ) -> u8;
  pub fn CFPlugInInstanceGetFactoryName(instance: *mut __CFPlugInInstance) -> *mut __CFString;
  pub fn CFPlugInInstanceGetInstanceData(instance: *mut __CFPlugInInstance) -> *mut c_void;
  pub fn CFPlugInInstanceGetTypeID() -> usize;
  pub fn CFPlugInInstanceCreateWithInstanceDataSize(
    allocator: *mut __CFAllocator,
    instanceDataSize: isize,
    deallocateInstanceFunction: Option<extern "C" fn(*mut c_void) -> ()>,
    factoryName: *mut __CFString,
    getInterfaceFunction: Option<
      extern "C" fn(*mut __CFPlugInInstance, *mut __CFString, *mut *mut c_void) -> u8,
    >,
  ) -> *mut __CFPlugInInstance;
  pub fn CFMachPortGetTypeID() -> usize;
  pub fn CFMachPortCreate(
    allocator: *mut __CFAllocator,
    callout: Option<extern "C" fn(*mut __CFMachPort, *mut c_void, isize, *mut c_void) -> ()>,
    context: *mut CFMachPortContext,
    shouldFreeInfo: *mut u8,
  ) -> *mut __CFMachPort;
  pub fn CFMachPortCreateWithPort(
    allocator: *mut __CFAllocator,
    portNum: u32,
    callout: Option<extern "C" fn(*mut __CFMachPort, *mut c_void, isize, *mut c_void) -> ()>,
    context: *mut CFMachPortContext,
    shouldFreeInfo: *mut u8,
  ) -> *mut __CFMachPort;
  pub fn CFMachPortGetPort(port: *mut __CFMachPort) -> u32;
  pub fn CFMachPortGetContext(port: *mut __CFMachPort, context: *mut CFMachPortContext) -> ();
  pub fn CFMachPortInvalidate(port: *mut __CFMachPort) -> ();
  pub fn CFMachPortIsValid(port: *mut __CFMachPort) -> u8;
  pub fn CFMachPortGetInvalidationCallBack(
    port: *mut __CFMachPort,
  ) -> Option<extern "C" fn(*mut __CFMachPort, *mut c_void) -> ()>;
  pub fn CFMachPortSetInvalidationCallBack(
    port: *mut __CFMachPort,
    callout: Option<extern "C" fn(*mut __CFMachPort, *mut c_void) -> ()>,
  ) -> ();
  pub fn CFMachPortCreateRunLoopSource(
    allocator: *mut __CFAllocator,
    port: *mut __CFMachPort,
    order: isize,
  ) -> *mut __CFRunLoopSource;
  pub fn CFAttributedStringGetTypeID() -> usize;
  pub fn CFAttributedStringCreate(
    alloc: *mut __CFAllocator,
    str: *mut __CFString,
    attributes: *mut __CFDictionary,
  ) -> *mut __CFAttributedString;
  pub fn CFAttributedStringCreateWithSubstring(
    alloc: *mut __CFAllocator,
    aStr: *mut __CFAttributedString,
    range: CFRange,
  ) -> *mut __CFAttributedString;
  pub fn CFAttributedStringCreateCopy(
    alloc: *mut __CFAllocator,
    aStr: *mut __CFAttributedString,
  ) -> *mut __CFAttributedString;
  pub fn CFAttributedStringGetString(aStr: *mut __CFAttributedString) -> *mut __CFString;
  pub fn CFAttributedStringGetLength(aStr: *mut __CFAttributedString) -> isize;
  pub fn CFAttributedStringGetAttributes(
    aStr: *mut __CFAttributedString,
    loc: isize,
    effectiveRange: *mut CFRange,
  ) -> *mut __CFDictionary;
  pub fn CFAttributedStringGetAttribute(
    aStr: *mut __CFAttributedString,
    loc: isize,
    attrName: *mut __CFString,
    effectiveRange: *mut CFRange,
  ) -> *mut c_void;
  pub fn CFAttributedStringGetAttributesAndLongestEffectiveRange(
    aStr: *mut __CFAttributedString,
    loc: isize,
    inRange: CFRange,
    longestEffectiveRange: *mut CFRange,
  ) -> *mut __CFDictionary;
  pub fn CFAttributedStringGetAttributeAndLongestEffectiveRange(
    aStr: *mut __CFAttributedString,
    loc: isize,
    attrName: *mut __CFString,
    inRange: CFRange,
    longestEffectiveRange: *mut CFRange,
  ) -> *mut c_void;
  pub fn CFAttributedStringCreateMutableCopy(
    alloc: *mut __CFAllocator,
    maxLength: isize,
    aStr: *mut __CFAttributedString,
  ) -> *mut __CFAttributedString;
  pub fn CFAttributedStringCreateMutable(
    alloc: *mut __CFAllocator,
    maxLength: isize,
  ) -> *mut __CFAttributedString;
  pub fn CFAttributedStringReplaceString(
    aStr: *mut __CFAttributedString,
    range: CFRange,
    replacement: *mut __CFString,
  ) -> ();
  pub fn CFAttributedStringGetMutableString(aStr: *mut __CFAttributedString) -> *mut __CFString;
  pub fn CFAttributedStringSetAttributes(
    aStr: *mut __CFAttributedString,
    range: CFRange,
    replacement: *mut __CFDictionary,
    clearOtherAttributes: u8,
  ) -> ();
  pub fn CFAttributedStringSetAttribute(
    aStr: *mut __CFAttributedString,
    range: CFRange,
    attrName: *mut __CFString,
    value: *mut c_void,
  ) -> ();
  pub fn CFAttributedStringRemoveAttribute(
    aStr: *mut __CFAttributedString,
    range: CFRange,
    attrName: *mut __CFString,
  ) -> ();
  pub fn CFAttributedStringReplaceAttributedString(
    aStr: *mut __CFAttributedString,
    range: CFRange,
    replacement: *mut __CFAttributedString,
  ) -> ();
  pub fn CFAttributedStringBeginEditing(aStr: *mut __CFAttributedString) -> ();
  pub fn CFAttributedStringEndEditing(aStr: *mut __CFAttributedString) -> ();
  pub fn CFURLEnumeratorGetTypeID() -> usize;
  pub fn CFURLEnumeratorCreateForDirectoryURL(
    alloc: *mut __CFAllocator,
    directoryURL: *mut __CFURL,
    option: CFURLEnumeratorOptions,
    propertyKeys: *mut __CFArray,
  ) -> *mut __CFURLEnumerator;
  pub fn CFURLEnumeratorCreateForMountedVolumes(
    alloc: *mut __CFAllocator,
    option: CFURLEnumeratorOptions,
    propertyKeys: *mut __CFArray,
  ) -> *mut __CFURLEnumerator;
  pub fn CFURLEnumeratorGetNextURL(
    enumerator: *mut __CFURLEnumerator,
    url: *mut *mut __CFURL,
    error: *mut *mut __CFError,
  ) -> CFURLEnumeratorResult;
  pub fn CFURLEnumeratorSkipDescendents(enumerator: *mut __CFURLEnumerator) -> ();
  pub fn CFURLEnumeratorGetDescendentLevel(enumerator: *mut __CFURLEnumerator) -> isize;
  pub fn CFURLEnumeratorGetSourceDidChange(enumerator: *mut __CFURLEnumerator) -> u8;
  pub fn CFFileSecurityGetTypeID() -> usize;
  pub fn CFFileSecurityCreate(allocator: *mut __CFAllocator) -> *mut __CFFileSecurity;
  pub fn CFFileSecurityCreateCopy(
    allocator: *mut __CFAllocator,
    fileSec: *mut __CFFileSecurity,
  ) -> *mut __CFFileSecurity;
  pub fn CFFileSecurityCopyOwnerUUID(
    fileSec: *mut __CFFileSecurity,
    ownerUUID: *mut *mut __CFUUID,
  ) -> u8;
  pub fn CFFileSecuritySetOwnerUUID(fileSec: *mut __CFFileSecurity, ownerUUID: *mut __CFUUID)
    -> u8;
  pub fn CFFileSecurityCopyGroupUUID(
    fileSec: *mut __CFFileSecurity,
    groupUUID: *mut *mut __CFUUID,
  ) -> u8;
  pub fn CFFileSecuritySetGroupUUID(fileSec: *mut __CFFileSecurity, groupUUID: *mut __CFUUID)
    -> u8;
  pub fn CFFileSecurityCopyAccessControlList(
    fileSec: *mut __CFFileSecurity,
    accessControlList: *mut *mut _acl,
  ) -> u8;
  pub fn CFFileSecuritySetAccessControlList(
    fileSec: *mut __CFFileSecurity,
    accessControlList: *mut _acl,
  ) -> u8;
  pub fn CFFileSecurityGetOwner(fileSec: *mut __CFFileSecurity, owner: *mut u32) -> u8;
  pub fn CFFileSecuritySetOwner(fileSec: *mut __CFFileSecurity, owner: u32) -> u8;
  pub fn CFFileSecurityGetGroup(fileSec: *mut __CFFileSecurity, group: *mut u32) -> u8;
  pub fn CFFileSecuritySetGroup(fileSec: *mut __CFFileSecurity, group: u32) -> u8;
  pub fn CFFileSecurityGetMode(fileSec: *mut __CFFileSecurity, mode: *mut u16) -> u8;
  pub fn CFFileSecuritySetMode(fileSec: *mut __CFFileSecurity, mode: u16) -> u8;
  pub fn CFFileSecurityClearProperties(
    fileSec: *mut __CFFileSecurity,
    clearPropertyMask: CFFileSecurityClearOptions,
  ) -> u8;
  pub fn CFStringTokenizerCopyBestStringLanguage(
    string: *mut __CFString,
    range: CFRange,
  ) -> *mut __CFString;
  pub fn CFStringTokenizerGetTypeID() -> usize;
  pub fn CFStringTokenizerCreate(
    alloc: *mut __CFAllocator,
    string: *mut __CFString,
    range: CFRange,
    options: usize,
    locale: *mut __CFLocale,
  ) -> *mut __CFStringTokenizer;
  pub fn CFStringTokenizerSetString(
    tokenizer: *mut __CFStringTokenizer,
    string: *mut __CFString,
    range: CFRange,
  ) -> ();
  pub fn CFStringTokenizerGoToTokenAtIndex(
    tokenizer: *mut __CFStringTokenizer,
    index: isize,
  ) -> CFStringTokenizerTokenType;
  pub fn CFStringTokenizerAdvanceToNextToken(
    tokenizer: *mut __CFStringTokenizer,
  ) -> CFStringTokenizerTokenType;
  pub fn CFStringTokenizerGetCurrentTokenRange(tokenizer: *mut __CFStringTokenizer) -> CFRange;
  pub fn CFStringTokenizerCopyCurrentTokenAttribute(
    tokenizer: *mut __CFStringTokenizer,
    attribute: usize,
  ) -> *mut c_void;
  pub fn CFStringTokenizerGetCurrentSubTokens(
    tokenizer: *mut __CFStringTokenizer,
    ranges: *mut CFRange,
    maxRangeLength: isize,
    derivedSubTokens: *mut __CFArray,
  ) -> isize;
  pub fn CFFileDescriptorGetTypeID() -> usize;
  pub fn CFFileDescriptorCreate(
    allocator: *mut __CFAllocator,
    fd: i32,
    closeOnInvalidate: u8,
    callout: Option<extern "C" fn(*mut __CFFileDescriptor, usize, *mut c_void) -> ()>,
    context: *mut CFFileDescriptorContext,
  ) -> *mut __CFFileDescriptor;
  pub fn CFFileDescriptorGetNativeDescriptor(f: *mut __CFFileDescriptor) -> i32;
  pub fn CFFileDescriptorGetContext(
    f: *mut __CFFileDescriptor,
    context: *mut CFFileDescriptorContext,
  ) -> ();
  pub fn CFFileDescriptorEnableCallBacks(f: *mut __CFFileDescriptor, callBackTypes: usize) -> ();
  pub fn CFFileDescriptorDisableCallBacks(f: *mut __CFFileDescriptor, callBackTypes: usize) -> ();
  pub fn CFFileDescriptorInvalidate(f: *mut __CFFileDescriptor) -> ();
  pub fn CFFileDescriptorIsValid(f: *mut __CFFileDescriptor) -> u8;
  pub fn CFFileDescriptorCreateRunLoopSource(
    allocator: *mut __CFAllocator,
    f: *mut __CFFileDescriptor,
    order: isize,
  ) -> *mut __CFRunLoopSource;
  pub fn CFUserNotificationGetTypeID() -> usize;
  pub fn CFUserNotificationCreate(
    allocator: *mut __CFAllocator,
    timeout: f64,
    flags: usize,
    error: *mut i32,
    dictionary: *mut __CFDictionary,
  ) -> *mut __CFUserNotification;
  pub fn CFUserNotificationReceiveResponse(
    userNotification: *mut __CFUserNotification,
    timeout: f64,
    responseFlags: *mut usize,
  ) -> i32;
  pub fn CFUserNotificationGetResponseValue(
    userNotification: *mut __CFUserNotification,
    key: *mut __CFString,
    idx: isize,
  ) -> *mut __CFString;
  pub fn CFUserNotificationGetResponseDictionary(
    userNotification: *mut __CFUserNotification,
  ) -> *mut __CFDictionary;
  pub fn CFUserNotificationUpdate(
    userNotification: *mut __CFUserNotification,
    timeout: f64,
    flags: usize,
    dictionary: *mut __CFDictionary,
  ) -> i32;
  pub fn CFUserNotificationCancel(userNotification: *mut __CFUserNotification) -> i32;
  pub fn CFUserNotificationCreateRunLoopSource(
    allocator: *mut __CFAllocator,
    userNotification: *mut __CFUserNotification,
    callout: Option<extern "C" fn(*mut __CFUserNotification, usize) -> ()>,
    order: isize,
  ) -> *mut __CFRunLoopSource;
  pub fn CFUserNotificationDisplayNotice(
    timeout: f64,
    flags: usize,
    iconURL: *mut __CFURL,
    soundURL: *mut __CFURL,
    localizationURL: *mut __CFURL,
    alertHeader: *mut __CFString,
    alertMessage: *mut __CFString,
    defaultButtonTitle: *mut __CFString,
  ) -> i32;
  pub fn CFUserNotificationDisplayAlert(
    timeout: f64,
    flags: usize,
    iconURL: *mut __CFURL,
    soundURL: *mut __CFURL,
    localizationURL: *mut __CFURL,
    alertHeader: *mut __CFString,
    alertMessage: *mut __CFString,
    defaultButtonTitle: *mut __CFString,
    alternateButtonTitle: *mut __CFString,
    otherButtonTitle: *mut __CFString,
    responseFlags: *mut usize,
  ) -> i32;
  pub fn CFUserNotificationCheckBoxChecked(i: isize) -> usize;
  pub fn CFUserNotificationSecureTextField(i: isize) -> usize;
  pub fn CFUserNotificationPopUpSelection(n: isize) -> usize;
  pub fn CFXMLNodeGetTypeID() -> usize;
  pub fn CFXMLNodeCreate(
    alloc: *mut __CFAllocator,
    xmlType: CFXMLNodeTypeCode,
    dataString: *mut __CFString,
    additionalInfoPtr: *mut c_void,
    version: isize,
  ) -> *mut __CFXMLNode;
  pub fn CFXMLNodeCreateCopy(
    alloc: *mut __CFAllocator,
    origNode: *mut __CFXMLNode,
  ) -> *mut __CFXMLNode;
  pub fn CFXMLNodeGetTypeCode(node: *mut __CFXMLNode) -> CFXMLNodeTypeCode;
  pub fn CFXMLNodeGetString(node: *mut __CFXMLNode) -> *mut __CFString;
  pub fn CFXMLNodeGetInfoPtr(node: *mut __CFXMLNode) -> *mut c_void;
  pub fn CFXMLNodeGetVersion(node: *mut __CFXMLNode) -> isize;
  pub fn CFXMLTreeCreateWithNode(
    allocator: *mut __CFAllocator,
    node: *mut __CFXMLNode,
  ) -> *mut __CFTree;
  pub fn CFXMLTreeGetNode(xmlTree: *mut __CFTree) -> *mut __CFXMLNode;
  pub fn CFXMLParserGetTypeID() -> usize;
  pub fn CFXMLParserCreate(
    allocator: *mut __CFAllocator,
    xmlData: *mut __CFData,
    dataSource: *mut __CFURL,
    parseOptions: usize,
    versionOfNodes: isize,
    callBacks: *mut CFXMLParserCallBacks,
    context: *mut CFXMLParserContext,
  ) -> *mut __CFXMLParser;
  pub fn CFXMLParserCreateWithDataFromURL(
    allocator: *mut __CFAllocator,
    dataSource: *mut __CFURL,
    parseOptions: usize,
    versionOfNodes: isize,
    callBacks: *mut CFXMLParserCallBacks,
    context: *mut CFXMLParserContext,
  ) -> *mut __CFXMLParser;
  pub fn CFXMLParserGetContext(parser: *mut __CFXMLParser, context: *mut CFXMLParserContext) -> ();
  pub fn CFXMLParserGetCallBacks(
    parser: *mut __CFXMLParser,
    callBacks: *mut CFXMLParserCallBacks,
  ) -> ();
  pub fn CFXMLParserGetSourceURL(parser: *mut __CFXMLParser) -> *mut __CFURL;
  pub fn CFXMLParserGetLocation(parser: *mut __CFXMLParser) -> isize;
  pub fn CFXMLParserGetLineNumber(parser: *mut __CFXMLParser) -> isize;
  pub fn CFXMLParserGetDocument(parser: *mut __CFXMLParser) -> *mut c_void;
  pub fn CFXMLParserGetStatusCode(parser: *mut __CFXMLParser) -> CFXMLParserStatusCode;
  pub fn CFXMLParserCopyErrorDescription(parser: *mut __CFXMLParser) -> *mut __CFString;
  pub fn CFXMLParserAbort(
    parser: *mut __CFXMLParser,
    errorCode: CFXMLParserStatusCode,
    errorDescription: *mut __CFString,
  ) -> ();
  pub fn CFXMLParserParse(parser: *mut __CFXMLParser) -> u8;
  pub fn CFXMLTreeCreateFromData(
    allocator: *mut __CFAllocator,
    xmlData: *mut __CFData,
    dataSource: *mut __CFURL,
    parseOptions: usize,
    versionOfNodes: isize,
  ) -> *mut __CFTree;
  pub fn CFXMLTreeCreateFromDataWithError(
    allocator: *mut __CFAllocator,
    xmlData: *mut __CFData,
    dataSource: *mut __CFURL,
    parseOptions: usize,
    versionOfNodes: isize,
    errorDict: *mut *mut __CFDictionary,
  ) -> *mut __CFTree;
  pub fn CFXMLTreeCreateWithDataFromURL(
    allocator: *mut __CFAllocator,
    dataSource: *mut __CFURL,
    parseOptions: usize,
    versionOfNodes: isize,
  ) -> *mut __CFTree;
  pub fn CFXMLTreeCreateXMLData(
    allocator: *mut __CFAllocator,
    xmlTree: *mut __CFTree,
  ) -> *mut __CFData;
  pub fn CFXMLCreateStringByEscapingEntities(
    allocator: *mut __CFAllocator,
    string: *mut __CFString,
    entitiesDictionary: *mut __CFDictionary,
  ) -> *mut __CFString;
  pub fn CFXMLCreateStringByUnescapingEntities(
    allocator: *mut __CFAllocator,
    string: *mut __CFString,
    entitiesDictionary: *mut __CFDictionary,
  ) -> *mut __CFString;
}
