#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreGraphics::CGPoint;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::CoreServices::AE::AEDesc;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
use crate::NSObject::_NSZone;
use crate::Security::OpaqueSecIdentityRef;
use crate::Security::SSLProtocol;
use crate::Security::__SecTrust;
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
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSComparisonResult {
  NSOrderedAscending = -1,
  NSOrderedSame = 0,
  NSOrderedDescending = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSEnumerationOptions : usize { const NSEnumerationConcurrent = 1 ; const NSEnumerationReverse = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSSortOptions : usize { const NSSortConcurrent = 1 ; const NSSortStable = 16 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSQualityOfService {
  NSQualityOfServiceUserInteractive = 33,
  NSQualityOfServiceUserInitiated = 25,
  NSQualityOfServiceUtility = 17,
  NSQualityOfServiceBackground = 9,
  NSQualityOfServiceDefault = -1,
}
pub type NSZone = _NSZone;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSCopyingProto {
  #[objrs(selector = "copyWithZone:")]
  #[cfg(feature = "RK_Foundation")]
  fn copyWithZone_(&self, zone: Option<&NSZone>) -> Arc<Object>;
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSMutableCopyingProto {
  #[objrs(selector = "mutableCopyWithZone:")]
  #[cfg(feature = "RK_Foundation")]
  fn mutableCopyWithZone_(&self, zone: Option<&NSZone>) -> Arc<Object>;
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSCodingProto {
  #[objrs(selector = "encodeWithCoder:")]
  #[cfg(feature = "RK_Foundation")]
  fn encodeWithCoder_(&self, aCoder: &NSCoder) -> ();
//   #[objrs(selector = "initWithCoder:")]
//   #[cfg(feature = "RK_Foundation")]
//   fn newWithCoder_(aDecoder: &NSCoder) -> Option<Arc<Self>>;
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSSecureCodingProto {}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSDiscardableContentProto {
  #[objrs(selector = "beginContentAccess")]
  #[cfg(feature = "RK_Foundation")]
  fn beginContentAccess(&self) -> bool;
  #[objrs(selector = "endContentAccess")]
  #[cfg(feature = "RK_Foundation")]
  fn endContentAccess(&self) -> ();
  #[objrs(selector = "discardContentIfPossible")]
  #[cfg(feature = "RK_Foundation")]
  fn discardContentIfPossible(&self) -> ();
  #[objrs(selector = "isContentDiscarded")]
  #[cfg(feature = "RK_Foundation")]
  fn isContentDiscarded(&self) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSFastEnumerationState {
  pub state: usize,
  pub itemsPtr: *mut *mut Object,
  pub mutationsPtr: *mut usize,
  pub extra: [usize; 5],
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSFastEnumerationProto {
  #[objrs(selector = "countByEnumeratingWithState:objects:count:")]
  #[cfg(feature = "RK_Foundation")]
  fn countByEnumeratingWithState_objects_count_(
    &self,
    state: &NSFastEnumerationState,
    buffer: &mut &Option<Arc<Object>>,
    len: usize,
  ) -> usize;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSEnumerator;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSValue;
# [ objrs ( class , super = NSValue ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNumber;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NSRange {
  pub location: usize,
  pub length: usize,
}
pub type NSRange = _NSRange;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSArray;
bitflags! { # [ repr ( C ) ] pub struct NSBinarySearchingOptions : usize { const NSBinarySearchingFirstEqual = 256 ; const NSBinarySearchingLastEqual = 512 ; const NSBinarySearchingInsertionIndex = 1024 ; } }
# [ objrs ( class , super = NSArray ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableArray;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSItemProviderRepresentationVisibility {
  NSItemProviderRepresentationVisibilityAll = 0,
  NSItemProviderRepresentationVisibilityTeam = 1,
  NSItemProviderRepresentationVisibilityGroup = 2,
  NSItemProviderRepresentationVisibilityOwnProcess = 3,
}
bitflags! { # [ repr ( C ) ] pub struct NSItemProviderFileOptions : isize { const NSItemProviderFileOptionOpenInPlace = 1 ; } }
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSItemProviderWritingProto {
  #[objrs(selector = "itemProviderVisibilityForRepresentationWithTypeIdentifier:")]
  #[cfg(feature = "RK_Foundation")]
  fn itemProviderVisibilityForRepresentationWithTypeIdentifier_(
    &self,
    typeIdentifier: &NSString,
  ) -> NSItemProviderRepresentationVisibility;
  #[objrs(selector = "loadDataWithTypeIdentifier:forItemProviderCompletionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn loadDataWithTypeIdentifier_forItemProviderCompletionHandler_(
    &self,
    typeIdentifier: &NSString,
    completionHandler: (),
  ) -> Option<Arc<NSProgress>>;
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSItemProviderReadingProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSItemProvider;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSItemProviderErrorCode {
  NSItemProviderUnknownError = -1,
  NSItemProviderItemUnavailableError = -1000,
  NSItemProviderUnexpectedValueClassError = -1100,
  NSItemProviderUnavailableCoercionError = -1200,
}
bitflags! { # [ repr ( C ) ] pub struct NSStringCompareOptions : usize { const NSCaseInsensitiveSearch = 1 ; const NSLiteralSearch = 2 ; const NSBackwardsSearch = 4 ; const NSAnchoredSearch = 8 ; const NSNumericSearch = 64 ; const NSDiacriticInsensitiveSearch = 128 ; const NSWidthInsensitiveSearch = 256 ; const NSForcedOrderingSearch = 512 ; const NSRegularExpressionSearch = 1024 ; } }
pub type NSStringEncoding = usize;
bitflags! { # [ repr ( C ) ] pub struct NSStringEncodingConversionOptions : usize { const NSStringEncodingConversionAllowLossy = 1 ; const NSStringEncodingConversionExternalRepresentation = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSString;
bitflags! { # [ repr ( C ) ] pub struct NSStringEnumerationOptions : usize { const NSStringEnumerationByLines = 0 ; const NSStringEnumerationByParagraphs = 1 ; const NSStringEnumerationByComposedCharacterSequences = 2 ; const NSStringEnumerationByWords = 3 ; const NSStringEnumerationBySentences = 4 ; const NSStringEnumerationReverse = 256 ; const NSStringEnumerationSubstringNotRequired = 512 ; const NSStringEnumerationLocalized = 1024 ; } }
# [ objrs ( class , super = NSString ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableString;
# [ objrs ( class , super = NSString ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSimpleCString;
# [ objrs ( class , super = NSSimpleCString ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSConstantString;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDictionary;
# [ objrs ( class , super = NSDictionary ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableDictionary;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSet;
# [ objrs ( class , super = NSSet ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableSet;
# [ objrs ( class , super = NSMutableSet ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCountedSet;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSProgress;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSProgressReportingProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNotification;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNotificationCenter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSBundle;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSSwappedFloat {
  pub v: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSSwappedDouble {
  pub v: u64,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDate;
bitflags! { # [ repr ( C ) ] pub struct NSCalendarUnit : usize { const NSCalendarUnitEra = 2 ; const NSCalendarUnitYear = 4 ; const NSCalendarUnitMonth = 8 ; const NSCalendarUnitDay = 16 ; const NSCalendarUnitHour = 32 ; const NSCalendarUnitMinute = 64 ; const NSCalendarUnitSecond = 128 ; const NSCalendarUnitWeekday = 512 ; const NSCalendarUnitWeekdayOrdinal = 1024 ; const NSCalendarUnitQuarter = 2048 ; const NSCalendarUnitWeekOfMonth = 4096 ; const NSCalendarUnitWeekOfYear = 8192 ; const NSCalendarUnitYearForWeekOfYear = 16384 ; const NSCalendarUnitNanosecond = 32768 ; const NSCalendarUnitCalendar = 1048576 ; const NSCalendarUnitTimeZone = 2097152 ; const NSWeekCalendarUnit = 256 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSCalendarOptions : usize { const NSCalendarWrapComponents = 1 ; const NSCalendarMatchStrictly = 2 ; const NSCalendarSearchBackwards = 4 ; const NSCalendarMatchPreviousTimePreservingSmallerUnits = 256 ; const NSCalendarMatchNextTimePreservingSmallerUnits = 512 ; const NSCalendarMatchNextTime = 1024 ; const NSCalendarMatchFirst = 4096 ; const NSCalendarMatchLast = 8192 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCalendar;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDateComponents;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCharacterSet;
# [ objrs ( class , super = NSCharacterSet ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableCharacterSet;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSDecodingFailurePolicy {
  NSDecodingFailurePolicyRaiseException = 0,
  NSDecodingFailurePolicySetErrorAndReturn = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCoder;
bitflags! { # [ repr ( C ) ] pub struct NSDataReadingOptions : usize { const NSDataReadingMappedIfSafe = 1 ; const NSDataReadingUncached = 2 ; const NSDataReadingMappedAlways = 8 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSDataWritingOptions : usize { const NSDataWritingAtomic = 1 ; const NSDataWritingWithoutOverwriting = 2 ; const NSDataWritingFileProtectionNone = 268435456 ; const NSDataWritingFileProtectionComplete = 536870912 ; const NSDataWritingFileProtectionCompleteUnlessOpen = 805306368 ; const NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication = 1073741824 ; const NSDataWritingFileProtectionMask = 4026531840 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSDataSearchOptions : usize { const NSDataSearchBackwards = 1 ; const NSDataSearchAnchored = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSDataBase64EncodingOptions : usize { const NSDataBase64Encoding64CharacterLineLength = 1 ; const NSDataBase64Encoding76CharacterLineLength = 2 ; const NSDataBase64EncodingEndLineWithCarriageReturn = 16 ; const NSDataBase64EncodingEndLineWithLineFeed = 32 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSDataBase64DecodingOptions : usize { const NSDataBase64DecodingIgnoreUnknownCharacters = 1 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSData;
# [ objrs ( class , super = NSData ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableData;
# [ objrs ( class , super = NSMutableData ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPurgeableData;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDateInterval;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAttributedString;
bitflags! { # [ repr ( C ) ] pub struct NSAttributedStringEnumerationOptions : usize { const NSAttributedStringEnumerationReverse = 2 ; const NSAttributedStringEnumerationLongestEffectiveRangeNotRequired = 1048576 ; } }
# [ objrs ( class , super = NSAttributedString ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableAttributedString;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSFormattingContext {
  NSFormattingContextUnknown = 0,
  NSFormattingContextDynamic = 1,
  NSFormattingContextStandalone = 2,
  NSFormattingContextListItem = 3,
  NSFormattingContextBeginningOfSentence = 4,
  NSFormattingContextMiddleOfSentence = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSFormattingUnitStyle {
  NSFormattingUnitStyleShort = 1,
  NSFormattingUnitStyleMedium = 2,
  NSFormattingUnitStyleLong = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFormatter;
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDateFormatter;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDateFormatterStyle {
  NSDateFormatterNoStyle = 0,
  NSDateFormatterShortStyle = 1,
  NSDateFormatterMediumStyle = 2,
  NSDateFormatterLongStyle = 3,
  NSDateFormatterFullStyle = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDateFormatterBehavior {
  NSDateFormatterBehaviorDefault = 0,
  NSDateFormatterBehavior10_0 = 1000,
  NSDateFormatterBehavior10_4 = 1040,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDateIntervalFormatterStyle {
  NSDateIntervalFormatterNoStyle = 0,
  NSDateIntervalFormatterShortStyle = 1,
  NSDateIntervalFormatterMediumStyle = 2,
  NSDateIntervalFormatterLongStyle = 3,
  NSDateIntervalFormatterFullStyle = 4,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDateIntervalFormatter;
bitflags! { # [ repr ( C ) ] pub struct NSISO8601DateFormatOptions : usize { const NSISO8601DateFormatWithYear = 1 ; const NSISO8601DateFormatWithMonth = 2 ; const NSISO8601DateFormatWithWeekOfYear = 4 ; const NSISO8601DateFormatWithDay = 16 ; const NSISO8601DateFormatWithTime = 32 ; const NSISO8601DateFormatWithTimeZone = 64 ; const NSISO8601DateFormatWithSpaceBetweenDateAndTime = 128 ; const NSISO8601DateFormatWithDashSeparatorInDate = 256 ; const NSISO8601DateFormatWithColonSeparatorInTime = 512 ; const NSISO8601DateFormatWithColonSeparatorInTimeZone = 1024 ; const NSISO8601DateFormatWithFractionalSeconds = 2048 ; const NSISO8601DateFormatWithFullDate = 275 ; const NSISO8601DateFormatWithFullTime = 1632 ; const NSISO8601DateFormatWithInternetDateTime = 1907 ; } }
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSISO8601DateFormatter;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSMassFormatterUnit {
  NSMassFormatterUnitGram = 11,
  NSMassFormatterUnitKilogram = 14,
  NSMassFormatterUnitOunce = 1537,
  NSMassFormatterUnitPound = 1538,
  NSMassFormatterUnitStone = 1539,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMassFormatter;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLengthFormatterUnit {
  NSLengthFormatterUnitMillimeter = 8,
  NSLengthFormatterUnitCentimeter = 9,
  NSLengthFormatterUnitMeter = 11,
  NSLengthFormatterUnitKilometer = 14,
  NSLengthFormatterUnitInch = 1281,
  NSLengthFormatterUnitFoot = 1282,
  NSLengthFormatterUnitYard = 1283,
  NSLengthFormatterUnitMile = 1284,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSLengthFormatter;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSEnergyFormatterUnit {
  NSEnergyFormatterUnitJoule = 11,
  NSEnergyFormatterUnitKilojoule = 14,
  NSEnergyFormatterUnitCalorie = 1793,
  NSEnergyFormatterUnitKilocalorie = 1794,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSEnergyFormatter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitConverter;
# [ objrs ( class , super = NSUnitConverter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitConverterLinear;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnit;
# [ objrs ( class , super = NSUnit ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDimension;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitAcceleration;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitAngle;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitArea;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitConcentrationMass;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitDispersion;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitDuration;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitElectricCharge;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitElectricCurrent;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitElectricPotentialDifference;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitElectricResistance;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitEnergy;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitFrequency;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitFuelEfficiency;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitLength;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitIlluminance;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitMass;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitPower;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitPressure;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitSpeed;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitTemperature;
# [ objrs ( class , super = NSDimension ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnitVolume;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMeasurement;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSNumberFormatterBehavior {
  NSNumberFormatterBehaviorDefault = 0,
  NSNumberFormatterBehavior10_0 = 1000,
  NSNumberFormatterBehavior10_4 = 1040,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNumberFormatter;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSNumberFormatterStyle {
  NSNumberFormatterNoStyle = 0,
  NSNumberFormatterDecimalStyle = 1,
  NSNumberFormatterCurrencyStyle = 2,
  NSNumberFormatterPercentStyle = 3,
  NSNumberFormatterScientificStyle = 4,
  NSNumberFormatterSpellOutStyle = 5,
  NSNumberFormatterOrdinalStyle = 6,
  NSNumberFormatterCurrencyISOCodeStyle = 8,
  NSNumberFormatterCurrencyPluralStyle = 9,
  NSNumberFormatterCurrencyAccountingStyle = 10,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSNumberFormatterPadPosition {
  NSNumberFormatterPadBeforePrefix = 0,
  NSNumberFormatterPadAfterPrefix = 1,
  NSNumberFormatterPadBeforeSuffix = 2,
  NSNumberFormatterPadAfterSuffix = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSNumberFormatterRoundingMode {
  NSNumberFormatterRoundCeiling = 0,
  NSNumberFormatterRoundFloor = 1,
  NSNumberFormatterRoundDown = 2,
  NSNumberFormatterRoundUp = 3,
  NSNumberFormatterRoundHalfEven = 4,
  NSNumberFormatterRoundHalfDown = 5,
  NSNumberFormatterRoundHalfUp = 6,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSLocale;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLocaleLanguageDirection {
  NSLocaleLanguageDirectionUnknown = 0,
  NSLocaleLanguageDirectionLeftToRight = 1,
  NSLocaleLanguageDirectionRightToLeft = 2,
  NSLocaleLanguageDirectionTopToBottom = 3,
  NSLocaleLanguageDirectionBottomToTop = 4,
}
bitflags! { # [ repr ( C ) ] pub struct NSMeasurementFormatterUnitOptions : usize { const NSMeasurementFormatterUnitOptionsProvidedUnit = 1 ; const NSMeasurementFormatterUnitOptionsNaturalScale = 2 ; const NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit = 4 ; } }
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMeasurementFormatter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPersonNameComponents;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPersonNameComponentsFormatterStyle {
  NSPersonNameComponentsFormatterStyleDefault = 0,
  NSPersonNameComponentsFormatterStyleShort = 1,
  NSPersonNameComponentsFormatterStyleMedium = 2,
  NSPersonNameComponentsFormatterStyleLong = 3,
  NSPersonNameComponentsFormatterStyleAbbreviated = 4,
}
bitflags! { # [ repr ( C ) ] pub struct NSPersonNameComponentsFormatterOptions : usize { const NSPersonNameComponentsFormatterPhonetic = 2 ; } }
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPersonNameComponentsFormatter;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRoundingMode {
  NSRoundPlain = 0,
  NSRoundDown = 1,
  NSRoundUp = 2,
  NSRoundBankers = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCalculationError {
  NSCalculationNoError = 0,
  NSCalculationLossOfPrecision = 1,
  NSCalculationUnderflow = 2,
  NSCalculationOverflow = 3,
  NSCalculationDivideByZero = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSDecimal {
  pub _exponent: i32,
  pub _length: u32,
  pub _isNegative: u32,
  pub _isCompact: u32,
  pub _reserved: u32,
  pub _mantissa: [u16; 8],
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScanner;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSException;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAssertionHandler;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSDecimalNumberBehaviorsProto {
  #[objrs(selector = "roundingMode")]
  #[cfg(feature = "RK_Foundation")]
  fn roundingMode(&self) -> NSRoundingMode;
  #[objrs(selector = "scale")]
  #[cfg(feature = "RK_Foundation")]
  fn scale(&self) -> i16;
  #[objrs(selector = "exceptionDuringOperation:error:leftOperand:rightOperand:")]
  #[cfg(feature = "RK_Foundation")]
  fn exceptionDuringOperation_error_leftOperand_rightOperand_(
    &self,
    operation: SelectorRef,
    error: NSCalculationError,
    leftOperand: &NSDecimalNumber,
    rightOperand: Option<&NSDecimalNumber>,
  ) -> Option<Arc<NSDecimalNumber>>;
}
# [ objrs ( class , super = NSNumber ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDecimalNumber;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDecimalNumberHandler;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSError;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRunLoop;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileHandle;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPipe;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSearchPathDirectory {
  NSApplicationDirectory = 1,
  NSDemoApplicationDirectory = 2,
  NSDeveloperApplicationDirectory = 3,
  NSAdminApplicationDirectory = 4,
  NSLibraryDirectory = 5,
  NSDeveloperDirectory = 6,
  NSUserDirectory = 7,
  NSDocumentationDirectory = 8,
  NSDocumentDirectory = 9,
  NSCoreServiceDirectory = 10,
  NSAutosavedInformationDirectory = 11,
  NSDesktopDirectory = 12,
  NSCachesDirectory = 13,
  NSApplicationSupportDirectory = 14,
  NSDownloadsDirectory = 15,
  NSInputMethodsDirectory = 16,
  NSMoviesDirectory = 17,
  NSMusicDirectory = 18,
  NSPicturesDirectory = 19,
  NSPrinterDescriptionDirectory = 20,
  NSSharedPublicDirectory = 21,
  NSPreferencePanesDirectory = 22,
  NSApplicationScriptsDirectory = 23,
  NSItemReplacementDirectory = 99,
  NSAllApplicationsDirectory = 100,
  NSAllLibrariesDirectory = 101,
  NSTrashDirectory = 102,
}
bitflags! { # [ repr ( C ) ] pub struct NSSearchPathDomainMask : usize { const NSUserDomainMask = 1 ; const NSLocalDomainMask = 2 ; const NSNetworkDomainMask = 4 ; const NSSystemDomainMask = 8 ; const NSAllDomainsMask = 65535 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSURLHandleStatus {
  NSURLHandleNotLoaded = 0,
  NSURLHandleLoadSucceeded = 1,
  NSURLHandleLoadInProgress = 2,
  NSURLHandleLoadFailed = 3,
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLHandleClientProto {
  #[objrs(selector = "URLHandle:resourceDataDidBecomeAvailable:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLHandle_resourceDataDidBecomeAvailable_(
    &self,
    sender: Option<&NSURLHandle>,
    newBytes: Option<&NSData>,
  ) -> ();
  #[objrs(selector = "URLHandleResourceDidBeginLoading:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLHandleResourceDidBeginLoading_(&self, sender: Option<&NSURLHandle>) -> ();
  #[objrs(selector = "URLHandleResourceDidFinishLoading:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLHandleResourceDidFinishLoading_(&self, sender: Option<&NSURLHandle>) -> ();
  #[objrs(selector = "URLHandleResourceDidCancelLoading:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLHandleResourceDidCancelLoading_(&self, sender: Option<&NSURLHandle>) -> ();
  #[objrs(selector = "URLHandle:resourceDidFailLoadingWithReason:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLHandle_resourceDidFailLoadingWithReason_(
    &self,
    sender: Option<&NSURLHandle>,
    reason: Option<&NSString>,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLHandle;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURL;
bitflags! { # [ repr ( C ) ] pub struct NSURLBookmarkCreationOptions : usize { const NSURLBookmarkCreationPreferFileIDResolution = 256 ; const NSURLBookmarkCreationMinimalBookmark = 512 ; const NSURLBookmarkCreationSuitableForBookmarkFile = 1024 ; const NSURLBookmarkCreationWithSecurityScope = 2048 ; const NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess = 4096 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSURLBookmarkResolutionOptions : usize { const NSURLBookmarkResolutionWithoutUI = 256 ; const NSURLBookmarkResolutionWithoutMounting = 512 ; const NSURLBookmarkResolutionWithSecurityScope = 1024 ; } }
pub type NSURLBookmarkFileCreationOptions = usize;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLQueryItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLComponents;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileSecurity;
bitflags! { # [ repr ( C ) ] pub struct NSVolumeEnumerationOptions : usize { const NSVolumeEnumerationSkipHiddenVolumes = 2 ; const NSVolumeEnumerationProduceFileReferenceURLs = 4 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSDirectoryEnumerationOptions : usize { const NSDirectoryEnumerationSkipsSubdirectoryDescendants = 1 ; const NSDirectoryEnumerationSkipsPackageDescendants = 2 ; const NSDirectoryEnumerationSkipsHiddenFiles = 4 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSFileManagerItemReplacementOptions : usize { const NSFileManagerItemReplacementUsingNewMetadataOnly = 1 ; const NSFileManagerItemReplacementWithoutDeletingBackupItem = 2 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLRelationship {
  NSURLRelationshipContains = 0,
  NSURLRelationshipSame = 1,
  NSURLRelationshipOther = 2,
}
bitflags! { # [ repr ( C ) ] pub struct NSFileManagerUnmountOptions : usize { const NSFileManagerUnmountAllPartitionsAndEjectDisk = 1 ; const NSFileManagerUnmountWithoutUI = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileManager;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSFileManagerDelegateProto {
  #[objrs(selector = "fileManager:shouldCopyItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldCopyItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldCopyItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldCopyItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:copyingItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_copyingItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:copyingItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_copyingItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldMoveItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldMoveItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldMoveItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldMoveItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:movingItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_movingItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:movingItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_movingItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldLinkItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldLinkItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldLinkItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldLinkItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:linkingItemAtPath:toPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_linkingItemAtPath_toPath_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcPath: &NSString,
    dstPath: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:linkingItemAtURL:toURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_linkingItemAtURL_toURL_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    srcURL: &NSURL,
    dstURL: &NSURL,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldRemoveItemAtPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldRemoveItemAtPath_(
    &self,
    fileManager: &NSFileManager,
    path: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldRemoveItemAtURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldRemoveItemAtURL_(&self, fileManager: &NSFileManager, URL: &NSURL) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:removingItemAtPath:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_removingItemAtPath_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    path: &NSString,
  ) -> bool;
  #[objrs(selector = "fileManager:shouldProceedAfterError:removingItemAtURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn fileManager_shouldProceedAfterError_removingItemAtURL_(
    &self,
    fileManager: &NSFileManager,
    error: &NSError,
    URL: &NSURL,
  ) -> bool;
}
# [ objrs ( class , super = NSEnumerator ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDirectoryEnumerator;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileProviderService;
bitflags! { # [ repr ( C ) ] pub struct NSPointerFunctionsOptions : usize { const NSPointerFunctionsStrongMemory = 0 ; const NSPointerFunctionsZeroingWeakMemory = 1 ; const NSPointerFunctionsOpaqueMemory = 2 ; const NSPointerFunctionsMallocMemory = 3 ; const NSPointerFunctionsMachVirtualMemory = 4 ; const NSPointerFunctionsWeakMemory = 5 ; const NSPointerFunctionsOpaquePersonality = 256 ; const NSPointerFunctionsObjectPointerPersonality = 512 ; const NSPointerFunctionsCStringPersonality = 768 ; const NSPointerFunctionsStructPersonality = 1024 ; const NSPointerFunctionsIntegerPersonality = 1280 ; const NSPointerFunctionsCopyIn = 65536 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPointerFunctions;
pub type NSHashTableOptions = usize;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSHashTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSHashEnumerator {
  pub _pi: usize,
  pub _si: usize,
  pub _bs: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSHashTableCallBacks {
  pub hash: Option<extern "C" fn(*mut NSHashTable, *mut c_void) -> usize>,
  pub isEqual: Option<extern "C" fn(*mut NSHashTable, *mut c_void, *mut c_void) -> bool>,
  pub retain: Option<extern "C" fn(*mut NSHashTable, *mut c_void) -> ()>,
  pub release: Option<extern "C" fn(*mut NSHashTable, *mut c_void) -> ()>,
  pub describe: Option<extern "C" fn(*mut NSHashTable, *mut c_void) -> *mut NSString>,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSHTTPCookie;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSHTTPCookieAcceptPolicy {
  NSHTTPCookieAcceptPolicyAlways = 0,
  NSHTTPCookieAcceptPolicyNever = 1,
  NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSHTTPCookieStorage;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSIndexPath;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSIndexSet;
# [ objrs ( class , super = NSIndexSet ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableIndexSet;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSInvocation;
bitflags! { # [ repr ( C ) ] pub struct NSJSONReadingOptions : usize { const NSJSONReadingMutableContainers = 1 ; const NSJSONReadingMutableLeaves = 2 ; const NSJSONReadingAllowFragments = 4 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSJSONWritingOptions : usize { const NSJSONWritingPrettyPrinted = 1 ; const NSJSONWritingSortedKeys = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSJSONSerialization;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSOrderedSet;
# [ objrs ( class , super = NSOrderedSet ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableOrderedSet;
bitflags! { # [ repr ( C ) ] pub struct NSKeyValueObservingOptions : usize { const NSKeyValueObservingOptionNew = 1 ; const NSKeyValueObservingOptionOld = 2 ; const NSKeyValueObservingOptionInitial = 4 ; const NSKeyValueObservingOptionPrior = 8 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSKeyValueChange {
  NSKeyValueChangeSetting = 1,
  NSKeyValueChangeInsertion = 2,
  NSKeyValueChangeRemoval = 3,
  NSKeyValueChangeReplacement = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSKeyValueSetMutationKind {
  NSKeyValueUnionSetMutation = 1,
  NSKeyValueMinusSetMutation = 2,
  NSKeyValueIntersectSetMutation = 3,
  NSKeyValueSetSetMutation = 4,
}
bitflags! { # [ repr ( C ) ] pub struct NSPropertyListMutabilityOptions : usize { const NSPropertyListImmutable = 0 ; const NSPropertyListMutableContainers = 1 ; const NSPropertyListMutableContainersAndLeaves = 2 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPropertyListFormat {
  NSPropertyListOpenStepFormat = 1,
  NSPropertyListXMLFormat_v1_0 = 100,
  NSPropertyListBinaryFormat_v1_0 = 200,
}
pub type NSPropertyListReadOptions = NSPropertyListMutabilityOptions;
pub type NSPropertyListWriteOptions = usize;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPropertyListSerialization;
pub type NSPoint = CGPoint;
pub type NSSize = CGSize;
pub type NSRect = CGRect;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRectEdge {
  NSRectEdgeMinX = 0,
  NSRectEdgeMinY = 1,
  NSRectEdgeMaxX = 2,
  NSRectEdgeMaxY = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSEdgeInsets {
  pub top: f64,
  pub left: f64,
  pub bottom: f64,
  pub right: f64,
}
bitflags! { # [ repr ( C ) ] pub struct NSAlignmentOptions : u64 { const NSAlignMinXInward = 1 ; const NSAlignMinYInward = 2 ; const NSAlignMaxXInward = 4 ; const NSAlignMaxYInward = 8 ; const NSAlignWidthInward = 16 ; const NSAlignHeightInward = 32 ; const NSAlignMinXOutward = 256 ; const NSAlignMinYOutward = 512 ; const NSAlignMaxXOutward = 1024 ; const NSAlignMaxYOutward = 2048 ; const NSAlignWidthOutward = 4096 ; const NSAlignHeightOutward = 8192 ; const NSAlignMinXNearest = 65536 ; const NSAlignMinYNearest = 131072 ; const NSAlignMaxXNearest = 262144 ; const NSAlignMaxYNearest = 524288 ; const NSAlignWidthNearest = 1048576 ; const NSAlignHeightNearest = 2097152 ; const NSAlignRectFlipped = 9223372036854775808 ; const NSAlignAllEdgesInward = 15 ; const NSAlignAllEdgesOutward = 3840 ; const NSAlignAllEdgesNearest = 983040 ; } }
# [ objrs ( class , super = NSCoder ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSKeyedArchiver;
# [ objrs ( class , super = NSCoder ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSKeyedUnarchiver;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSKeyedArchiverDelegateProto {
  #[objrs(selector = "archiver:willEncodeObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn archiver_willEncodeObject_(
    &self,
    archiver: &NSKeyedArchiver,
    object: &Object,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "archiver:didEncodeObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn archiver_didEncodeObject_(&self, archiver: &NSKeyedArchiver, object: Option<&Object>) -> ();
  #[objrs(selector = "archiver:willReplaceObject:withObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn archiver_willReplaceObject_withObject_(
    &self,
    archiver: &NSKeyedArchiver,
    object: Option<&Object>,
    newObject: Option<&Object>,
  ) -> ();
  #[objrs(selector = "archiverWillFinish:")]
  #[cfg(feature = "RK_Foundation")]
  fn archiverWillFinish_(&self, archiver: &NSKeyedArchiver) -> ();
  #[objrs(selector = "archiverDidFinish:")]
  #[cfg(feature = "RK_Foundation")]
  fn archiverDidFinish_(&self, archiver: &NSKeyedArchiver) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSKeyedUnarchiverDelegateProto {
  #[objrs(selector = "unarchiver:cannotDecodeObjectOfClassName:originalClasses:")]
  #[cfg(feature = "RK_Foundation")]
  fn unarchiver_cannotDecodeObjectOfClassName_originalClasses_(
    &self,
    unarchiver: &NSKeyedUnarchiver,
    name: &NSString,
    classNames: &NSArray,
  ) -> Option<Arc<Class>>;
  #[objrs(selector = "unarchiver:didDecodeObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn unarchiver_didDecodeObject_(
    &self,
    unarchiver: &NSKeyedUnarchiver,
    object: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "unarchiver:willReplaceObject:withObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn unarchiver_willReplaceObject_withObject_(
    &self,
    unarchiver: &NSKeyedUnarchiver,
    object: &Object,
    newObject: &Object,
  ) -> ();
  #[objrs(selector = "unarchiverWillFinish:")]
  #[cfg(feature = "RK_Foundation")]
  fn unarchiverWillFinish_(&self, unarchiver: &NSKeyedUnarchiver) -> ();
  #[objrs(selector = "unarchiverDidFinish:")]
  #[cfg(feature = "RK_Foundation")]
  fn unarchiverDidFinish_(&self, unarchiver: &NSKeyedUnarchiver) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSLockingProto {
  #[objrs(selector = "lock")]
  #[cfg(feature = "RK_Foundation")]
  fn lock(&self) -> ();
  #[objrs(selector = "unlock")]
  #[cfg(feature = "RK_Foundation")]
  fn unlock(&self) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSLock;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSConditionLock;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRecursiveLock;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCondition;
pub type NSMapTableOptions = usize;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMapTable;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSMapEnumerator {
  pub _pi: usize,
  pub _si: usize,
  pub _bs: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSMapTableKeyCallBacks {
  pub hash: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> usize>,
  pub isEqual: Option<extern "C" fn(*mut NSMapTable, *mut c_void, *mut c_void) -> bool>,
  pub retain: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> ()>,
  pub release: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> ()>,
  pub describe: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> *mut NSString>,
  pub notAKeyMarker: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSMapTableValueCallBacks {
  pub retain: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> ()>,
  pub release: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> ()>,
  pub describe: Option<extern "C" fn(*mut NSMapTable, *mut c_void) -> *mut NSString>,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMethodSignature;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPostingStyle {
  NSPostWhenIdle = 1,
  NSPostASAP = 2,
  NSPostNow = 3,
}
bitflags! { # [ repr ( C ) ] pub struct NSNotificationCoalescing : usize { const NSNotificationNoCoalescing = 0 ; const NSNotificationCoalescingOnName = 1 ; const NSNotificationCoalescingOnSender = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNotificationQueue;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNull;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSOperation;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSOperationQueuePriority {
  NSOperationQueuePriorityVeryLow = -8,
  NSOperationQueuePriorityLow = -4,
  NSOperationQueuePriorityNormal = 0,
  NSOperationQueuePriorityHigh = 4,
  NSOperationQueuePriorityVeryHigh = 8,
}
# [ objrs ( class , super = NSOperation ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSBlockOperation;
# [ objrs ( class , super = NSOperation ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSInvocationOperation;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSOperationQueue;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSOrthography;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPointerArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPort;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSPortDelegateProto {
  #[objrs(selector = "handlePortMessage:")]
  #[cfg(feature = "RK_Foundation")]
  fn handlePortMessage_(&self, message: &NSPortMessage) -> ();
}
# [ objrs ( class , super = NSPort ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMachPort;
bitflags! { # [ repr ( C ) ] pub struct NSMachPortOptions : usize { const NSMachPortDeallocateNone = 0 ; const NSMachPortDeallocateSendRight = 1 ; const NSMachPortDeallocateReceiveRight = 2 ; } }
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSMachPortDelegateProto {
  #[objrs(selector = "handleMachMessage:")]
  #[cfg(feature = "RK_Foundation")]
  fn handleMachMessage_(&self, msg: &c_void) -> ();
}
# [ objrs ( class , super = NSPort ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMessagePort;
# [ objrs ( class , super = NSPort ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSocketPort;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSOperatingSystemVersion {
  pub majorVersion: isize,
  pub minorVersion: isize,
  pub patchVersion: isize,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSProcessInfo;
bitflags! { # [ repr ( C ) ] pub struct NSActivityOptions : u64 { const NSActivityIdleDisplaySleepDisabled = 1099511627776 ; const NSActivityIdleSystemSleepDisabled = 1048576 ; const NSActivitySuddenTerminationDisabled = 16384 ; const NSActivityAutomaticTerminationDisabled = 32768 ; const NSActivityUserInitiated = 16777215 ; const NSActivityUserInitiatedAllowingIdleSystemSleep = 15728639 ; const NSActivityBackground = 255 ; const NSActivityLatencyCritical = 1095216660480 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSProcessInfoThermalState {
  NSProcessInfoThermalStateNominal = 0,
  NSProcessInfoThermalStateFair = 1,
  NSProcessInfoThermalStateSerious = 2,
  NSProcessInfoThermalStateCritical = 3,
}
#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
pub struct NSProxy;
bitflags! { # [ repr ( C ) ] pub struct NSTextCheckingType : u64 { const NSTextCheckingTypeOrthography = 1 ; const NSTextCheckingTypeSpelling = 2 ; const NSTextCheckingTypeGrammar = 4 ; const NSTextCheckingTypeDate = 8 ; const NSTextCheckingTypeAddress = 16 ; const NSTextCheckingTypeLink = 32 ; const NSTextCheckingTypeQuote = 64 ; const NSTextCheckingTypeDash = 128 ; const NSTextCheckingTypeReplacement = 256 ; const NSTextCheckingTypeCorrection = 512 ; const NSTextCheckingTypeRegularExpression = 1024 ; const NSTextCheckingTypePhoneNumber = 2048 ; const NSTextCheckingTypeTransitInformation = 4096 ; } }
pub type NSTextCheckingTypes = u64;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSTextCheckingResult;
bitflags! { # [ repr ( C ) ] pub struct NSRegularExpressionOptions : usize { const NSRegularExpressionCaseInsensitive = 1 ; const NSRegularExpressionAllowCommentsAndWhitespace = 2 ; const NSRegularExpressionIgnoreMetacharacters = 4 ; const NSRegularExpressionDotMatchesLineSeparators = 8 ; const NSRegularExpressionAnchorsMatchLines = 16 ; const NSRegularExpressionUseUnixLineSeparators = 32 ; const NSRegularExpressionUseUnicodeWordBoundaries = 64 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRegularExpression;
bitflags! { # [ repr ( C ) ] pub struct NSMatchingOptions : usize { const NSMatchingReportProgress = 1 ; const NSMatchingReportCompletion = 2 ; const NSMatchingAnchored = 4 ; const NSMatchingWithTransparentBounds = 8 ; const NSMatchingWithoutAnchoringBounds = 16 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSMatchingFlags : usize { const NSMatchingProgress = 1 ; const NSMatchingCompleted = 2 ; const NSMatchingHitEnd = 4 ; const NSMatchingRequiredEnd = 8 ; const NSMatchingInternalError = 16 ; } }
# [ objrs ( class , super = NSRegularExpression ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDataDetector;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSortDescriptor;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSStreamStatus {
  NSStreamStatusNotOpen = 0,
  NSStreamStatusOpening = 1,
  NSStreamStatusOpen = 2,
  NSStreamStatusReading = 3,
  NSStreamStatusWriting = 4,
  NSStreamStatusAtEnd = 5,
  NSStreamStatusClosed = 6,
  NSStreamStatusError = 7,
}
bitflags! { # [ repr ( C ) ] pub struct NSStreamEvent : usize { const NSStreamEventNone = 0 ; const NSStreamEventOpenCompleted = 1 ; const NSStreamEventHasBytesAvailable = 2 ; const NSStreamEventHasSpaceAvailable = 4 ; const NSStreamEventErrorOccurred = 8 ; const NSStreamEventEndEncountered = 16 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSStream;
# [ objrs ( class , super = NSStream ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSInputStream;
# [ objrs ( class , super = NSStream ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSOutputStream;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSStreamDelegateProto {
  #[objrs(selector = "stream:handleEvent:")]
  #[cfg(feature = "RK_Foundation")]
  fn stream_handleEvent_(&self, aStream: &NSStream, eventCode: NSStreamEvent) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSThread;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSTimeZone;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTimeZoneNameStyle {
  NSTimeZoneNameStyleStandard = 0,
  NSTimeZoneNameStyleShortStandard = 1,
  NSTimeZoneNameStyleDaylightSaving = 2,
  NSTimeZoneNameStyleShortDaylightSaving = 3,
  NSTimeZoneNameStyleGeneric = 4,
  NSTimeZoneNameStyleShortGeneric = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSTimer;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLAuthenticationChallengeSenderProto {
  #[objrs(selector = "useCredential:forAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn useCredential_forAuthenticationChallenge_(
    &self,
    credential: &NSURLCredential,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "continueWithoutCredentialForAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn continueWithoutCredentialForAuthenticationChallenge_(
    &self,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "cancelAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn cancelAuthenticationChallenge_(&self, challenge: &NSURLAuthenticationChallenge) -> ();
  #[objrs(selector = "performDefaultHandlingForAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn performDefaultHandlingForAuthenticationChallenge_(
    &self,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "rejectProtectionSpaceAndContinueWithChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn rejectProtectionSpaceAndContinueWithChallenge_(
    &self,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLAuthenticationChallenge;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSURLCacheStoragePolicy {
  NSURLCacheStorageAllowed = 0,
  NSURLCacheStorageAllowedInMemoryOnly = 1,
  NSURLCacheStorageNotAllowed = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCachedURLResponse;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLCache;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLConnection;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLConnectionDelegateProto {
  #[objrs(selector = "connection:didFailWithError:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didFailWithError_(&self, connection: &NSURLConnection, error: &NSError) -> ();
  #[objrs(selector = "connectionShouldUseCredentialStorage:")]
  #[cfg(feature = "RK_Foundation")]
  fn connectionShouldUseCredentialStorage_(&self, connection: &NSURLConnection) -> bool;
  #[objrs(selector = "connection:willSendRequestForAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_willSendRequestForAuthenticationChallenge_(
    &self,
    connection: &NSURLConnection,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "connection:canAuthenticateAgainstProtectionSpace:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_canAuthenticateAgainstProtectionSpace_(
    &self,
    connection: &NSURLConnection,
    protectionSpace: &NSURLProtectionSpace,
  ) -> bool;
  #[objrs(selector = "connection:didReceiveAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didReceiveAuthenticationChallenge_(
    &self,
    connection: &NSURLConnection,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "connection:didCancelAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didCancelAuthenticationChallenge_(
    &self,
    connection: &NSURLConnection,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLConnectionDataDelegateProto {
  #[objrs(selector = "connection:willSendRequest:redirectResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_willSendRequest_redirectResponse_(
    &self,
    connection: &NSURLConnection,
    request: &NSURLRequest,
    response: Option<&NSURLResponse>,
  ) -> Option<Arc<NSURLRequest>>;
  #[objrs(selector = "connection:didReceiveResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didReceiveResponse_(
    &self,
    connection: &NSURLConnection,
    response: &NSURLResponse,
  ) -> ();
  #[objrs(selector = "connection:didReceiveData:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didReceiveData_(&self, connection: &NSURLConnection, data: &NSData) -> ();
  #[objrs(selector = "connection:needNewBodyStream:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_needNewBodyStream_(
    &self,
    connection: &NSURLConnection,
    request: &NSURLRequest,
  ) -> Option<Arc<NSInputStream>>;
  #[objrs(selector = "connection:didSendBodyData:totalBytesWritten:totalBytesExpectedToWrite:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didSendBodyData_totalBytesWritten_totalBytesExpectedToWrite_(
    &self,
    connection: &NSURLConnection,
    bytesWritten: isize,
    totalBytesWritten: isize,
    totalBytesExpectedToWrite: isize,
  ) -> ();
  #[objrs(selector = "connection:willCacheResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_willCacheResponse_(
    &self,
    connection: &NSURLConnection,
    cachedResponse: &NSCachedURLResponse,
  ) -> Option<Arc<NSCachedURLResponse>>;
  #[objrs(selector = "connectionDidFinishLoading:")]
  #[cfg(feature = "RK_Foundation")]
  fn connectionDidFinishLoading_(&self, connection: &NSURLConnection) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLConnectionDownloadDelegateProto {
  #[objrs(selector = "connection:didWriteData:totalBytesWritten:expectedTotalBytes:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_didWriteData_totalBytesWritten_expectedTotalBytes_(
    &self,
    connection: &NSURLConnection,
    bytesWritten: i64,
    totalBytesWritten: i64,
    expectedTotalBytes: i64,
  ) -> ();
  #[objrs(selector = "connectionDidResumeDownloading:totalBytesWritten:expectedTotalBytes:")]
  #[cfg(feature = "RK_Foundation")]
  fn connectionDidResumeDownloading_totalBytesWritten_expectedTotalBytes_(
    &self,
    connection: &NSURLConnection,
    totalBytesWritten: i64,
    expectedTotalBytes: i64,
  ) -> ();
  #[objrs(selector = "connectionDidFinishDownloading:destinationURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn connectionDidFinishDownloading_destinationURL_(
    &self,
    connection: &NSURLConnection,
    destinationURL: &NSURL,
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSURLCredentialPersistence {
  NSURLCredentialPersistenceNone = 0,
  NSURLCredentialPersistenceForSession = 1,
  NSURLCredentialPersistencePermanent = 2,
  NSURLCredentialPersistenceSynchronizable = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLCredential;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLProtectionSpace;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLCredentialStorage;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLProtocolClientProto {
  #[objrs(selector = "URLProtocol:wasRedirectedToRequest:redirectResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_wasRedirectedToRequest_redirectResponse_(
    &self,
    protocol: &NSURLProtocol,
    request: &NSURLRequest,
    redirectResponse: &NSURLResponse,
  ) -> ();
  #[objrs(selector = "URLProtocol:cachedResponseIsValid:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_cachedResponseIsValid_(
    &self,
    protocol: &NSURLProtocol,
    cachedResponse: &NSCachedURLResponse,
  ) -> ();
  #[objrs(selector = "URLProtocol:didReceiveResponse:cacheStoragePolicy:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_didReceiveResponse_cacheStoragePolicy_(
    &self,
    protocol: &NSURLProtocol,
    response: &NSURLResponse,
    policy: NSURLCacheStoragePolicy,
  ) -> ();
  #[objrs(selector = "URLProtocol:didLoadData:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_didLoadData_(&self, protocol: &NSURLProtocol, data: &NSData) -> ();
  #[objrs(selector = "URLProtocolDidFinishLoading:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocolDidFinishLoading_(&self, protocol: &NSURLProtocol) -> ();
  #[objrs(selector = "URLProtocol:didFailWithError:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_didFailWithError_(&self, protocol: &NSURLProtocol, error: &NSError) -> ();
  #[objrs(selector = "URLProtocol:didReceiveAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_didReceiveAuthenticationChallenge_(
    &self,
    protocol: &NSURLProtocol,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "URLProtocol:didCancelAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLProtocol_didCancelAuthenticationChallenge_(
    &self,
    protocol: &NSURLProtocol,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLProtocol;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSURLRequestCachePolicy {
  NSURLRequestUseProtocolCachePolicy = 0,
  NSURLRequestReloadIgnoringLocalCacheData = 1,
  NSURLRequestReloadIgnoringLocalAndRemoteCacheData = 4,
  NSURLRequestReturnCacheDataElseLoad = 2,
  NSURLRequestReturnCacheDataDontLoad = 3,
  NSURLRequestReloadRevalidatingCacheData = 5,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSURLRequestNetworkServiceType {
  NSURLNetworkServiceTypeDefault = 0,
  NSURLNetworkServiceTypeVoIP = 1,
  NSURLNetworkServiceTypeVideo = 2,
  NSURLNetworkServiceTypeBackground = 3,
  NSURLNetworkServiceTypeVoice = 4,
  NSURLNetworkServiceTypeResponsiveData = 6,
  NSURLNetworkServiceTypeCallSignaling = 11,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLRequest;
# [ objrs ( class , super = NSURLRequest ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMutableURLRequest;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLResponse;
# [ objrs ( class , super = NSURLResponse ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSHTTPURLResponse;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserDefaults;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSValueTransformer;
# [ objrs ( class , super = NSValueTransformer ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSecureUnarchiveFromDataTransformer;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSXMLParserExternalEntityResolvingPolicy {
  NSXMLParserResolveExternalEntitiesNever = 0,
  NSXMLParserResolveExternalEntitiesNoNetwork = 1,
  NSXMLParserResolveExternalEntitiesSameOriginOnly = 2,
  NSXMLParserResolveExternalEntitiesAlways = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLParser;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSXMLParserDelegateProto {
  #[objrs(selector = "parserDidStartDocument:")]
  #[cfg(feature = "RK_Foundation")]
  fn parserDidStartDocument_(&self, parser: &NSXMLParser) -> ();
  #[objrs(selector = "parserDidEndDocument:")]
  #[cfg(feature = "RK_Foundation")]
  fn parserDidEndDocument_(&self, parser: &NSXMLParser) -> ();
  #[objrs(selector = "parser:foundNotationDeclarationWithName:publicID:systemID:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundNotationDeclarationWithName_publicID_systemID_(
    &self,
    parser: &NSXMLParser,
    name: &NSString,
    publicID: Option<&NSString>,
    systemID: Option<&NSString>,
  ) -> ();
  #[objrs(
    selector = "parser:foundUnparsedEntityDeclarationWithName:publicID:systemID:notationName:"
  )]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundUnparsedEntityDeclarationWithName_publicID_systemID_notationName_(
    &self,
    parser: &NSXMLParser,
    name: &NSString,
    publicID: Option<&NSString>,
    systemID: Option<&NSString>,
    notationName: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:foundAttributeDeclarationWithName:forElement:type:defaultValue:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundAttributeDeclarationWithName_forElement_type_defaultValue_(
    &self,
    parser: &NSXMLParser,
    attributeName: &NSString,
    elementName: &NSString,
    type_: Option<&NSString>,
    defaultValue: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:foundElementDeclarationWithName:model:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundElementDeclarationWithName_model_(
    &self,
    parser: &NSXMLParser,
    elementName: &NSString,
    model: &NSString,
  ) -> ();
  #[objrs(selector = "parser:foundInternalEntityDeclarationWithName:value:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundInternalEntityDeclarationWithName_value_(
    &self,
    parser: &NSXMLParser,
    name: &NSString,
    value: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:foundExternalEntityDeclarationWithName:publicID:systemID:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundExternalEntityDeclarationWithName_publicID_systemID_(
    &self,
    parser: &NSXMLParser,
    name: &NSString,
    publicID: Option<&NSString>,
    systemID: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:didStartElement:namespaceURI:qualifiedName:attributes:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_didStartElement_namespaceURI_qualifiedName_attributes_(
    &self,
    parser: &NSXMLParser,
    elementName: &NSString,
    namespaceURI: Option<&NSString>,
    qName: Option<&NSString>,
    attributeDict: &NSDictionary,
  ) -> ();
  #[objrs(selector = "parser:didEndElement:namespaceURI:qualifiedName:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_didEndElement_namespaceURI_qualifiedName_(
    &self,
    parser: &NSXMLParser,
    elementName: &NSString,
    namespaceURI: Option<&NSString>,
    qName: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:didStartMappingPrefix:toURI:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_didStartMappingPrefix_toURI_(
    &self,
    parser: &NSXMLParser,
    prefix: &NSString,
    namespaceURI: &NSString,
  ) -> ();
  #[objrs(selector = "parser:didEndMappingPrefix:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_didEndMappingPrefix_(&self, parser: &NSXMLParser, prefix: &NSString) -> ();
  #[objrs(selector = "parser:foundCharacters:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundCharacters_(&self, parser: &NSXMLParser, string: &NSString) -> ();
  #[objrs(selector = "parser:foundIgnorableWhitespace:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundIgnorableWhitespace_(
    &self,
    parser: &NSXMLParser,
    whitespaceString: &NSString,
  ) -> ();
  #[objrs(selector = "parser:foundProcessingInstructionWithTarget:data:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundProcessingInstructionWithTarget_data_(
    &self,
    parser: &NSXMLParser,
    target: &NSString,
    data: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "parser:foundComment:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundComment_(&self, parser: &NSXMLParser, comment: &NSString) -> ();
  #[objrs(selector = "parser:foundCDATA:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_foundCDATA_(&self, parser: &NSXMLParser, CDATABlock: &NSData) -> ();
  #[objrs(selector = "parser:resolveExternalEntityName:systemID:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_resolveExternalEntityName_systemID_(
    &self,
    parser: &NSXMLParser,
    name: &NSString,
    systemID: Option<&NSString>,
  ) -> Option<Arc<NSData>>;
  #[objrs(selector = "parser:parseErrorOccurred:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_parseErrorOccurred_(&self, parser: &NSXMLParser, parseError: &NSError) -> ();
  #[objrs(selector = "parser:validationErrorOccurred:")]
  #[cfg(feature = "RK_Foundation")]
  fn parser_validationErrorOccurred_(&self, parser: &NSXMLParser, validationError: &NSError) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSXMLParserError {
  NSXMLParserInternalError = 1,
  NSXMLParserOutOfMemoryError = 2,
  NSXMLParserDocumentStartError = 3,
  NSXMLParserEmptyDocumentError = 4,
  NSXMLParserPrematureDocumentEndError = 5,
  NSXMLParserInvalidHexCharacterRefError = 6,
  NSXMLParserInvalidDecimalCharacterRefError = 7,
  NSXMLParserInvalidCharacterRefError = 8,
  NSXMLParserInvalidCharacterError = 9,
  NSXMLParserCharacterRefAtEOFError = 10,
  NSXMLParserCharacterRefInPrologError = 11,
  NSXMLParserCharacterRefInEpilogError = 12,
  NSXMLParserCharacterRefInDTDError = 13,
  NSXMLParserEntityRefAtEOFError = 14,
  NSXMLParserEntityRefInPrologError = 15,
  NSXMLParserEntityRefInEpilogError = 16,
  NSXMLParserEntityRefInDTDError = 17,
  NSXMLParserParsedEntityRefAtEOFError = 18,
  NSXMLParserParsedEntityRefInPrologError = 19,
  NSXMLParserParsedEntityRefInEpilogError = 20,
  NSXMLParserParsedEntityRefInInternalSubsetError = 21,
  NSXMLParserEntityReferenceWithoutNameError = 22,
  NSXMLParserEntityReferenceMissingSemiError = 23,
  NSXMLParserParsedEntityRefNoNameError = 24,
  NSXMLParserParsedEntityRefMissingSemiError = 25,
  NSXMLParserUndeclaredEntityError = 26,
  NSXMLParserUnparsedEntityError = 28,
  NSXMLParserEntityIsExternalError = 29,
  NSXMLParserEntityIsParameterError = 30,
  NSXMLParserUnknownEncodingError = 31,
  NSXMLParserEncodingNotSupportedError = 32,
  NSXMLParserStringNotStartedError = 33,
  NSXMLParserStringNotClosedError = 34,
  NSXMLParserNamespaceDeclarationError = 35,
  NSXMLParserEntityNotStartedError = 36,
  NSXMLParserEntityNotFinishedError = 37,
  NSXMLParserLessThanSymbolInAttributeError = 38,
  NSXMLParserAttributeNotStartedError = 39,
  NSXMLParserAttributeNotFinishedError = 40,
  NSXMLParserAttributeHasNoValueError = 41,
  NSXMLParserAttributeRedefinedError = 42,
  NSXMLParserLiteralNotStartedError = 43,
  NSXMLParserLiteralNotFinishedError = 44,
  NSXMLParserCommentNotFinishedError = 45,
  NSXMLParserProcessingInstructionNotStartedError = 46,
  NSXMLParserProcessingInstructionNotFinishedError = 47,
  NSXMLParserNotationNotStartedError = 48,
  NSXMLParserNotationNotFinishedError = 49,
  NSXMLParserAttributeListNotStartedError = 50,
  NSXMLParserAttributeListNotFinishedError = 51,
  NSXMLParserMixedContentDeclNotStartedError = 52,
  NSXMLParserMixedContentDeclNotFinishedError = 53,
  NSXMLParserElementContentDeclNotStartedError = 54,
  NSXMLParserElementContentDeclNotFinishedError = 55,
  NSXMLParserXMLDeclNotStartedError = 56,
  NSXMLParserXMLDeclNotFinishedError = 57,
  NSXMLParserConditionalSectionNotStartedError = 58,
  NSXMLParserConditionalSectionNotFinishedError = 59,
  NSXMLParserExternalSubsetNotFinishedError = 60,
  NSXMLParserDOCTYPEDeclNotFinishedError = 61,
  NSXMLParserMisplacedCDATAEndStringError = 62,
  NSXMLParserCDATANotFinishedError = 63,
  NSXMLParserMisplacedXMLDeclarationError = 64,
  NSXMLParserSpaceRequiredError = 65,
  NSXMLParserSeparatorRequiredError = 66,
  NSXMLParserNMTOKENRequiredError = 67,
  NSXMLParserNAMERequiredError = 68,
  NSXMLParserPCDATARequiredError = 69,
  NSXMLParserURIRequiredError = 70,
  NSXMLParserPublicIdentifierRequiredError = 71,
  NSXMLParserLTRequiredError = 72,
  NSXMLParserGTRequiredError = 73,
  NSXMLParserLTSlashRequiredError = 74,
  NSXMLParserEqualExpectedError = 75,
  NSXMLParserTagNameMismatchError = 76,
  NSXMLParserUnfinishedTagError = 77,
  NSXMLParserStandaloneValueError = 78,
  NSXMLParserInvalidEncodingNameError = 79,
  NSXMLParserCommentContainsDoubleHyphenError = 80,
  NSXMLParserInvalidEncodingError = 81,
  NSXMLParserExternalStandaloneEntityError = 82,
  NSXMLParserInvalidConditionalSectionError = 83,
  NSXMLParserEntityValueRequiredError = 84,
  NSXMLParserNotWellBalancedError = 85,
  NSXMLParserExtraContentError = 86,
  NSXMLParserInvalidCharacterInEntityError = 87,
  NSXMLParserParsedEntityRefInInternalError = 88,
  NSXMLParserEntityRefLoopError = 89,
  NSXMLParserEntityBoundaryError = 90,
  NSXMLParserInvalidURIError = 91,
  NSXMLParserURIFragmentError = 92,
  NSXMLParserNoDTDError = 94,
  NSXMLParserDelegateAbortedParseError = 512,
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSXPCProxyCreatingProto {
  #[objrs(selector = "remoteObjectProxy")]
  #[cfg(feature = "RK_Foundation")]
  fn remoteObjectProxy(&self) -> Arc<Object>;
  #[objrs(selector = "remoteObjectProxyWithErrorHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn remoteObjectProxyWithErrorHandler_(&self, handler: ()) -> Arc<Object>;
  #[objrs(selector = "synchronousRemoteObjectProxyWithErrorHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn synchronousRemoteObjectProxyWithErrorHandler_(&self, handler: ()) -> Arc<Object>;
}
bitflags! { # [ repr ( C ) ] pub struct NSXPCConnectionOptions : usize { const NSXPCConnectionPrivileged = 4096 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXPCConnection;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXPCListener;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSXPCListenerDelegateProto {
  #[objrs(selector = "listener:shouldAcceptNewConnection:")]
  #[cfg(feature = "RK_Foundation")]
  fn listener_shouldAcceptNewConnection_(
    &self,
    listener: &NSXPCListener,
    newConnection: &NSXPCConnection,
  ) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXPCInterface;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXPCListenerEndpoint;
bitflags! { # [ repr ( C ) ] pub struct NSByteCountFormatterUnits : usize { const NSByteCountFormatterUseDefault = 0 ; const NSByteCountFormatterUseBytes = 1 ; const NSByteCountFormatterUseKB = 2 ; const NSByteCountFormatterUseMB = 4 ; const NSByteCountFormatterUseGB = 8 ; const NSByteCountFormatterUseTB = 16 ; const NSByteCountFormatterUsePB = 32 ; const NSByteCountFormatterUseEB = 64 ; const NSByteCountFormatterUseZB = 128 ; const NSByteCountFormatterUseYBOrHigher = 65280 ; const NSByteCountFormatterUseAll = 65535 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSByteCountFormatterCountStyle {
  NSByteCountFormatterCountStyleFile = 0,
  NSByteCountFormatterCountStyleMemory = 1,
  NSByteCountFormatterCountStyleDecimal = 2,
  NSByteCountFormatterCountStyleBinary = 3,
}
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSByteCountFormatter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCache;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSCacheDelegateProto {
  #[objrs(selector = "cache:willEvictObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn cache_willEvictObject_(&self, cache: &NSCache, obj: &Object) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPredicate;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _predicateFlags {
  pub _evaluationBlocked: u32,
  pub _reservedPredicateFlags: u32,
}
bitflags! { # [ repr ( C ) ] pub struct NSComparisonPredicateOptions : usize { const NSCaseInsensitivePredicateOption = 1 ; const NSDiacriticInsensitivePredicateOption = 2 ; const NSNormalizedPredicateOption = 4 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSComparisonPredicateModifier {
  NSDirectPredicateModifier = 0,
  NSAllPredicateModifier = 1,
  NSAnyPredicateModifier = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPredicateOperatorType {
  NSLessThanPredicateOperatorType = 0,
  NSLessThanOrEqualToPredicateOperatorType = 1,
  NSGreaterThanPredicateOperatorType = 2,
  NSGreaterThanOrEqualToPredicateOperatorType = 3,
  NSEqualToPredicateOperatorType = 4,
  NSNotEqualToPredicateOperatorType = 5,
  NSMatchesPredicateOperatorType = 6,
  NSLikePredicateOperatorType = 7,
  NSBeginsWithPredicateOperatorType = 8,
  NSEndsWithPredicateOperatorType = 9,
  NSInPredicateOperatorType = 10,
  NSCustomSelectorPredicateOperatorType = 11,
  NSContainsPredicateOperatorType = 99,
  NSBetweenPredicateOperatorType = 100,
}
# [ objrs ( class , super = NSPredicate ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSComparisonPredicate;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCompoundPredicateType {
  NSNotPredicateType = 0,
  NSAndPredicateType = 1,
  NSOrPredicateType = 2,
}
# [ objrs ( class , super = NSPredicate ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCompoundPredicate;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSDateComponentsFormatterUnitsStyle {
  NSDateComponentsFormatterUnitsStylePositional = 0,
  NSDateComponentsFormatterUnitsStyleAbbreviated = 1,
  NSDateComponentsFormatterUnitsStyleShort = 2,
  NSDateComponentsFormatterUnitsStyleFull = 3,
  NSDateComponentsFormatterUnitsStyleSpellOut = 4,
  NSDateComponentsFormatterUnitsStyleBrief = 5,
}
bitflags! { # [ repr ( C ) ] pub struct NSDateComponentsFormatterZeroFormattingBehavior : usize { const NSDateComponentsFormatterZeroFormattingBehaviorNone = 0 ; const NSDateComponentsFormatterZeroFormattingBehaviorDefault = 1 ; const NSDateComponentsFormatterZeroFormattingBehaviorDropLeading = 2 ; const NSDateComponentsFormatterZeroFormattingBehaviorDropMiddle = 4 ; const NSDateComponentsFormatterZeroFormattingBehaviorDropTrailing = 8 ; const NSDateComponentsFormatterZeroFormattingBehaviorDropAll = 14 ; const NSDateComponentsFormatterZeroFormattingBehaviorPad = 65536 ; } }
# [ objrs ( class , super = NSFormatter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDateComponentsFormatter;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSExpressionType {
  NSConstantValueExpressionType = 0,
  NSEvaluatedObjectExpressionType = 1,
  NSVariableExpressionType = 2,
  NSKeyPathExpressionType = 3,
  NSFunctionExpressionType = 4,
  NSUnionSetExpressionType = 5,
  NSIntersectSetExpressionType = 6,
  NSMinusSetExpressionType = 7,
  NSSubqueryExpressionType = 13,
  NSAggregateExpressionType = 14,
  NSAnyKeyExpressionType = 15,
  NSBlockExpressionType = 19,
  NSConditionalExpressionType = 20,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSExpression;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _expressionFlags {
  pub _evaluationBlocked: u32,
  pub _reservedExpressionFlags: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSExtensionContext;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSExtensionItem;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSExtensionRequestHandlingProto {
  #[objrs(selector = "beginRequestWithExtensionContext:")]
  #[cfg(feature = "RK_Foundation")]
  fn beginRequestWithExtensionContext_(&self, context: &NSExtensionContext) -> ();
}
bitflags! { # [ repr ( C ) ] pub struct NSFileCoordinatorReadingOptions : usize { const NSFileCoordinatorReadingWithoutChanges = 1 ; const NSFileCoordinatorReadingResolvesSymbolicLink = 2 ; const NSFileCoordinatorReadingImmediatelyAvailableMetadataOnly = 4 ; const NSFileCoordinatorReadingForUploading = 8 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSFileCoordinatorWritingOptions : usize { const NSFileCoordinatorWritingForDeleting = 1 ; const NSFileCoordinatorWritingForMoving = 2 ; const NSFileCoordinatorWritingForMerging = 4 ; const NSFileCoordinatorWritingForReplacing = 8 ; const NSFileCoordinatorWritingContentIndependentMetadataOnly = 16 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileAccessIntent;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileCoordinator;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSFilePresenterProto {
  #[objrs(selector = "relinquishPresentedItemToReader:")]
  #[cfg(feature = "RK_Foundation")]
  fn relinquishPresentedItemToReader_(&self, reader: ()) -> ();
  #[objrs(selector = "relinquishPresentedItemToWriter:")]
  #[cfg(feature = "RK_Foundation")]
  fn relinquishPresentedItemToWriter_(&self, writer: ()) -> ();
  #[objrs(selector = "savePresentedItemChangesWithCompletionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn savePresentedItemChangesWithCompletionHandler_(&self, completionHandler: ()) -> ();
  #[objrs(selector = "accommodatePresentedItemDeletionWithCompletionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn accommodatePresentedItemDeletionWithCompletionHandler_(&self, completionHandler: ()) -> ();
  #[objrs(selector = "presentedItemDidMoveToURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidMoveToURL_(&self, newURL: &NSURL) -> ();
  #[objrs(selector = "presentedItemDidChange")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidChange(&self) -> ();
  #[objrs(selector = "presentedItemDidChangeUbiquityAttributes:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidChangeUbiquityAttributes_(&self, attributes: &NSSet) -> ();
  #[objrs(selector = "presentedItemDidGainVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidGainVersion_(&self, version: &NSFileVersion) -> ();
  #[objrs(selector = "presentedItemDidLoseVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidLoseVersion_(&self, version: &NSFileVersion) -> ();
  #[objrs(selector = "presentedItemDidResolveConflictVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedItemDidResolveConflictVersion_(&self, version: &NSFileVersion) -> ();
  #[objrs(selector = "accommodatePresentedSubitemDeletionAtURL:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn accommodatePresentedSubitemDeletionAtURL_completionHandler_(
    &self,
    url: &NSURL,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "presentedSubitemDidAppearAtURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemDidAppearAtURL_(&self, url: &NSURL) -> ();
  #[objrs(selector = "presentedSubitemAtURL:didMoveToURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemAtURL_didMoveToURL_(&self, oldURL: &NSURL, newURL: &NSURL) -> ();
  #[objrs(selector = "presentedSubitemDidChangeAtURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemDidChangeAtURL_(&self, url: &NSURL) -> ();
  #[objrs(selector = "presentedSubitemAtURL:didGainVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemAtURL_didGainVersion_(&self, url: &NSURL, version: &NSFileVersion) -> ();
  #[objrs(selector = "presentedSubitemAtURL:didLoseVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemAtURL_didLoseVersion_(&self, url: &NSURL, version: &NSFileVersion) -> ();
  #[objrs(selector = "presentedSubitemAtURL:didResolveConflictVersion:")]
  #[cfg(feature = "RK_Foundation")]
  fn presentedSubitemAtURL_didResolveConflictVersion_(
    &self,
    url: &NSURL,
    version: &NSFileVersion,
  ) -> ();
}
bitflags! { # [ repr ( C ) ] pub struct NSFileVersionAddingOptions : usize { const NSFileVersionAddingByMoving = 1 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSFileVersionReplacingOptions : usize { const NSFileVersionReplacingByMoving = 1 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileVersion;
bitflags! { # [ repr ( C ) ] pub struct NSFileWrapperReadingOptions : usize { const NSFileWrapperReadingImmediate = 1 ; const NSFileWrapperReadingWithoutMapping = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSFileWrapperWritingOptions : usize { const NSFileWrapperWritingAtomic = 1 ; const NSFileWrapperWritingWithNameUpdating = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSFileWrapper;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLinguisticTaggerUnit {
  NSLinguisticTaggerUnitWord = 0,
  NSLinguisticTaggerUnitSentence = 1,
  NSLinguisticTaggerUnitParagraph = 2,
  NSLinguisticTaggerUnitDocument = 3,
}
bitflags! { # [ repr ( C ) ] pub struct NSLinguisticTaggerOptions : usize { const NSLinguisticTaggerOmitWords = 1 ; const NSLinguisticTaggerOmitPunctuation = 2 ; const NSLinguisticTaggerOmitWhitespace = 4 ; const NSLinguisticTaggerOmitOther = 8 ; const NSLinguisticTaggerJoinNames = 16 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSLinguisticTagger;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMetadataQuery;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSMetadataQueryDelegateProto {
  #[objrs(selector = "metadataQuery:replacementObjectForResultObject:")]
  #[cfg(feature = "RK_Foundation")]
  fn metadataQuery_replacementObjectForResultObject_(
    &self,
    query: &NSMetadataQuery,
    result: &NSMetadataItem,
  ) -> Arc<Object>;
  #[objrs(selector = "metadataQuery:replacementValueForAttribute:value:")]
  #[cfg(feature = "RK_Foundation")]
  fn metadataQuery_replacementValueForAttribute_value_(
    &self,
    query: &NSMetadataQuery,
    attrName: &NSString,
    attrValue: &Object,
  ) -> Arc<Object>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMetadataItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMetadataQueryAttributeValueTuple;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMetadataQueryResultGroup;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSNetServicesError {
  NSNetServicesUnknownError = -72000,
  NSNetServicesCollisionError = -72001,
  NSNetServicesNotFoundError = -72002,
  NSNetServicesActivityInProgress = -72003,
  NSNetServicesBadArgumentError = -72004,
  NSNetServicesCancelledError = -72005,
  NSNetServicesInvalidError = -72006,
  NSNetServicesTimeoutError = -72007,
}
bitflags! { # [ repr ( C ) ] pub struct NSNetServiceOptions : usize { const NSNetServiceNoAutoRename = 1 ; const NSNetServiceListenForConnections = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNetService;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNetServiceBrowser;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSNetServiceDelegateProto {
  #[objrs(selector = "netServiceWillPublish:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceWillPublish_(&self, sender: &NSNetService) -> ();
  #[objrs(selector = "netServiceDidPublish:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceDidPublish_(&self, sender: &NSNetService) -> ();
  #[objrs(selector = "netService:didNotPublish:")]
  #[cfg(feature = "RK_Foundation")]
  fn netService_didNotPublish_(&self, sender: &NSNetService, errorDict: &NSDictionary) -> ();
  #[objrs(selector = "netServiceWillResolve:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceWillResolve_(&self, sender: &NSNetService) -> ();
  #[objrs(selector = "netServiceDidResolveAddress:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceDidResolveAddress_(&self, sender: &NSNetService) -> ();
  #[objrs(selector = "netService:didNotResolve:")]
  #[cfg(feature = "RK_Foundation")]
  fn netService_didNotResolve_(&self, sender: &NSNetService, errorDict: &NSDictionary) -> ();
  #[objrs(selector = "netServiceDidStop:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceDidStop_(&self, sender: &NSNetService) -> ();
  #[objrs(selector = "netService:didUpdateTXTRecordData:")]
  #[cfg(feature = "RK_Foundation")]
  fn netService_didUpdateTXTRecordData_(&self, sender: &NSNetService, data: &NSData) -> ();
  #[objrs(selector = "netService:didAcceptConnectionWithInputStream:outputStream:")]
  #[cfg(feature = "RK_Foundation")]
  fn netService_didAcceptConnectionWithInputStream_outputStream_(
    &self,
    sender: &NSNetService,
    inputStream: &NSInputStream,
    outputStream: &NSOutputStream,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSNetServiceBrowserDelegateProto {
  #[objrs(selector = "netServiceBrowserWillSearch:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowserWillSearch_(&self, browser: &NSNetServiceBrowser) -> ();
  #[objrs(selector = "netServiceBrowserDidStopSearch:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowserDidStopSearch_(&self, browser: &NSNetServiceBrowser) -> ();
  #[objrs(selector = "netServiceBrowser:didNotSearch:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowser_didNotSearch_(
    &self,
    browser: &NSNetServiceBrowser,
    errorDict: &NSDictionary,
  ) -> ();
  #[objrs(selector = "netServiceBrowser:didFindDomain:moreComing:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowser_didFindDomain_moreComing_(
    &self,
    browser: &NSNetServiceBrowser,
    domainString: &NSString,
    moreComing: bool,
  ) -> ();
  #[objrs(selector = "netServiceBrowser:didFindService:moreComing:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowser_didFindService_moreComing_(
    &self,
    browser: &NSNetServiceBrowser,
    service: &NSNetService,
    moreComing: bool,
  ) -> ();
  #[objrs(selector = "netServiceBrowser:didRemoveDomain:moreComing:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowser_didRemoveDomain_moreComing_(
    &self,
    browser: &NSNetServiceBrowser,
    domainString: &NSString,
    moreComing: bool,
  ) -> ();
  #[objrs(selector = "netServiceBrowser:didRemoveService:moreComing:")]
  #[cfg(feature = "RK_Foundation")]
  fn netServiceBrowser_didRemoveService_moreComing_(
    &self,
    browser: &NSNetServiceBrowser,
    service: &NSNetService,
    moreComing: bool,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUbiquitousKeyValueStore;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUndoManager;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSession;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLSessionTaskState {
  NSURLSessionTaskStateRunning = 0,
  NSURLSessionTaskStateSuspended = 1,
  NSURLSessionTaskStateCanceling = 2,
  NSURLSessionTaskStateCompleted = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionTask;
# [ objrs ( class , super = NSURLSessionTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionDataTask;
# [ objrs ( class , super = NSURLSessionDataTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionUploadTask;
# [ objrs ( class , super = NSURLSessionTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionDownloadTask;
# [ objrs ( class , super = NSURLSessionTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionStreamTask;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionConfiguration;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLSessionDelayedRequestDisposition {
  NSURLSessionDelayedRequestContinueLoading = 0,
  NSURLSessionDelayedRequestUseNewRequest = 1,
  NSURLSessionDelayedRequestCancel = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLSessionAuthChallengeDisposition {
  NSURLSessionAuthChallengeUseCredential = 0,
  NSURLSessionAuthChallengePerformDefaultHandling = 1,
  NSURLSessionAuthChallengeCancelAuthenticationChallenge = 2,
  NSURLSessionAuthChallengeRejectProtectionSpace = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLSessionResponseDisposition {
  NSURLSessionResponseCancel = 0,
  NSURLSessionResponseAllow = 1,
  NSURLSessionResponseBecomeDownload = 2,
  NSURLSessionResponseBecomeStream = 3,
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLSessionDelegateProto {
  #[objrs(selector = "URLSession:didBecomeInvalidWithError:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_didBecomeInvalidWithError_(
    &self,
    session: &NSURLSession,
    error: Option<&NSError>,
  ) -> ();
  #[objrs(selector = "URLSession:didReceiveChallenge:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_didReceiveChallenge_completionHandler_(
    &self,
    session: &NSURLSession,
    challenge: &NSURLAuthenticationChallenge,
    completionHandler: (),
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLSessionTaskDelegateProto {
  #[objrs(selector = "URLSession:task:willBeginDelayedRequest:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_willBeginDelayedRequest_completionHandler_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    request: &NSURLRequest,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "URLSession:taskIsWaitingForConnectivity:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_taskIsWaitingForConnectivity_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
  ) -> ();
  #[objrs(selector = "URLSession:task:willPerformHTTPRedirection:newRequest:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_willPerformHTTPRedirection_newRequest_completionHandler_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    response: &NSHTTPURLResponse,
    request: &NSURLRequest,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "URLSession:task:didReceiveChallenge:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_didReceiveChallenge_completionHandler_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    challenge: &NSURLAuthenticationChallenge,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "URLSession:task:needNewBodyStream:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_needNewBodyStream_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "URLSession:task:didSendBodyData:totalBytesSent:totalBytesExpectedToSend:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_didSendBodyData_totalBytesSent_totalBytesExpectedToSend_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    bytesSent: i64,
    totalBytesSent: i64,
    totalBytesExpectedToSend: i64,
  ) -> ();
  #[objrs(selector = "URLSession:task:didFinishCollectingMetrics:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_didFinishCollectingMetrics_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    metrics: &NSURLSessionTaskMetrics,
  ) -> ();
  #[objrs(selector = "URLSession:task:didCompleteWithError:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_task_didCompleteWithError_(
    &self,
    session: &NSURLSession,
    task: &NSURLSessionTask,
    error: Option<&NSError>,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLSessionDataDelegateProto {
  #[objrs(selector = "URLSession:dataTask:didReceiveResponse:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_dataTask_didReceiveResponse_completionHandler_(
    &self,
    session: &NSURLSession,
    dataTask: &NSURLSessionDataTask,
    response: &NSURLResponse,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "URLSession:dataTask:didBecomeDownloadTask:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_dataTask_didBecomeDownloadTask_(
    &self,
    session: &NSURLSession,
    dataTask: &NSURLSessionDataTask,
    downloadTask: &NSURLSessionDownloadTask,
  ) -> ();
  #[objrs(selector = "URLSession:dataTask:didBecomeStreamTask:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_dataTask_didBecomeStreamTask_(
    &self,
    session: &NSURLSession,
    dataTask: &NSURLSessionDataTask,
    streamTask: &NSURLSessionStreamTask,
  ) -> ();
  #[objrs(selector = "URLSession:dataTask:didReceiveData:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_dataTask_didReceiveData_(
    &self,
    session: &NSURLSession,
    dataTask: &NSURLSessionDataTask,
    data: &NSData,
  ) -> ();
  #[objrs(selector = "URLSession:dataTask:willCacheResponse:completionHandler:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_dataTask_willCacheResponse_completionHandler_(
    &self,
    session: &NSURLSession,
    dataTask: &NSURLSessionDataTask,
    proposedResponse: &NSCachedURLResponse,
    completionHandler: (),
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLSessionDownloadDelegateProto {
  #[objrs(selector = "URLSession:downloadTask:didFinishDownloadingToURL:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_downloadTask_didFinishDownloadingToURL_(
    &self,
    session: &NSURLSession,
    downloadTask: &NSURLSessionDownloadTask,
    location: &NSURL,
  ) -> ();
  #[objrs(
    selector = "URLSession:downloadTask:didWriteData:totalBytesWritten:totalBytesExpectedToWrite:"
  )]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_downloadTask_didWriteData_totalBytesWritten_totalBytesExpectedToWrite_(
    &self,
    session: &NSURLSession,
    downloadTask: &NSURLSessionDownloadTask,
    bytesWritten: i64,
    totalBytesWritten: i64,
    totalBytesExpectedToWrite: i64,
  ) -> ();
  #[objrs(selector = "URLSession:downloadTask:didResumeAtOffset:expectedTotalBytes:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_downloadTask_didResumeAtOffset_expectedTotalBytes_(
    &self,
    session: &NSURLSession,
    downloadTask: &NSURLSessionDownloadTask,
    fileOffset: i64,
    expectedTotalBytes: i64,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLSessionStreamDelegateProto {
  #[objrs(selector = "URLSession:readClosedForStreamTask:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_readClosedForStreamTask_(
    &self,
    session: &NSURLSession,
    streamTask: &NSURLSessionStreamTask,
  ) -> ();
  #[objrs(selector = "URLSession:writeClosedForStreamTask:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_writeClosedForStreamTask_(
    &self,
    session: &NSURLSession,
    streamTask: &NSURLSessionStreamTask,
  ) -> ();
  #[objrs(selector = "URLSession:betterRouteDiscoveredForStreamTask:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_betterRouteDiscoveredForStreamTask_(
    &self,
    session: &NSURLSession,
    streamTask: &NSURLSessionStreamTask,
  ) -> ();
  #[objrs(selector = "URLSession:streamTask:didBecomeInputStream:outputStream:")]
  #[cfg(feature = "RK_Foundation")]
  fn URLSession_streamTask_didBecomeInputStream_outputStream_(
    &self,
    session: &NSURLSession,
    streamTask: &NSURLSessionStreamTask,
    inputStream: &NSInputStream,
    outputStream: &NSOutputStream,
  ) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSURLSessionTaskMetricsResourceFetchType {
  NSURLSessionTaskMetricsResourceFetchTypeUnknown = 0,
  NSURLSessionTaskMetricsResourceFetchTypeNetworkLoad = 1,
  NSURLSessionTaskMetricsResourceFetchTypeServerPush = 2,
  NSURLSessionTaskMetricsResourceFetchTypeLocalCache = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionTaskTransactionMetrics;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLSessionTaskMetrics;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserActivity;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSUserActivityDelegateProto {
  #[objrs(selector = "userActivityWillSave:")]
  #[cfg(feature = "RK_Foundation")]
  fn userActivityWillSave_(&self, userActivity: &NSUserActivity) -> ();
  #[objrs(selector = "userActivityWasContinued:")]
  #[cfg(feature = "RK_Foundation")]
  fn userActivityWasContinued_(&self, userActivity: &NSUserActivity) -> ();
  #[objrs(selector = "userActivity:didReceiveInputStream:outputStream:")]
  #[cfg(feature = "RK_Foundation")]
  fn userActivity_didReceiveInputStream_outputStream_(
    &self,
    userActivity: &NSUserActivity,
    inputStream: &NSInputStream,
    outputStream: &NSOutputStream,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUUID;
# [ objrs ( class , super = NSCoder ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSArchiver;
# [ objrs ( class , super = NSCoder ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUnarchiver;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSBackgroundActivityResult {
  NSBackgroundActivityResultFinished = 1,
  NSBackgroundActivityResultDeferred = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSBackgroundActivityScheduler;
# [ objrs ( class , super = NSDate ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCalendarDate;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSConnection;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSConnectionDelegateProto {
  #[objrs(selector = "makeNewConnection:sender:")]
  #[cfg(feature = "RK_Foundation")]
  fn makeNewConnection_sender_(&self, conn: &NSConnection, ancestor: &NSConnection) -> bool;
  #[objrs(selector = "connection:shouldMakeNewConnection:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_shouldMakeNewConnection_(
    &self,
    ancestor: &NSConnection,
    conn: &NSConnection,
  ) -> bool;
  #[objrs(selector = "authenticationDataForComponents:")]
  #[cfg(feature = "RK_Foundation")]
  fn authenticationDataForComponents_(&self, components: &NSArray) -> Arc<NSData>;
  #[objrs(selector = "authenticateComponents:withData:")]
  #[cfg(feature = "RK_Foundation")]
  fn authenticateComponents_withData_(&self, components: &NSArray, signature: &NSData) -> bool;
  #[objrs(selector = "createConversationForConnection:")]
  #[cfg(feature = "RK_Foundation")]
  fn createConversationForConnection_(&self, conn: &NSConnection) -> Arc<Object>;
  #[objrs(selector = "connection:handleRequest:")]
  #[cfg(feature = "RK_Foundation")]
  fn connection_handleRequest_(
    &self,
    connection: &NSConnection,
    doreq: &NSDistantObjectRequest,
  ) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDistantObjectRequest;
# [ objrs ( class , super = NSProxy ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDistantObject;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSNotificationSuspensionBehavior {
  NSNotificationSuspensionBehaviorDrop = 1,
  NSNotificationSuspensionBehaviorCoalesce = 2,
  NSNotificationSuspensionBehaviorHold = 3,
  NSNotificationSuspensionBehaviorDeliverImmediately = 4,
}
bitflags! { # [ repr ( C ) ] pub struct NSDistributedNotificationOptions : usize { const NSDistributedNotificationDeliverImmediately = 1 ; const NSDistributedNotificationPostToAllSessions = 2 ; } }
# [ objrs ( class , super = NSNotificationCenter ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDistributedNotificationCenter;
# [ objrs ( class , super = NSCoder ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPortCoder;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPortMessage;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPortNameServer;
# [ objrs ( class , super = NSPortNameServer ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMachBootstrapServer;
# [ objrs ( class , super = NSPortNameServer ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMessagePortNameServer;
# [ objrs ( class , super = NSPortNameServer ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSocketPortNameServer;
# [ objrs ( class , super = NSProxy ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSProtocolChecker;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTaskTerminationReason {
  NSTaskTerminationReasonExit = 1,
  NSTaskTerminationReasonUncaughtSignal = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSTask;
bitflags! { # [ repr ( C ) ] pub struct NSXMLNodeOptions : usize { const NSXMLNodeOptionsNone = 0 ; const NSXMLNodeIsCDATA = 1 ; const NSXMLNodeExpandEmptyElement = 2 ; const NSXMLNodeCompactEmptyElement = 4 ; const NSXMLNodeUseSingleQuotes = 8 ; const NSXMLNodeUseDoubleQuotes = 16 ; const NSXMLNodeNeverEscapeContents = 32 ; const NSXMLDocumentTidyHTML = 512 ; const NSXMLDocumentTidyXML = 1024 ; const NSXMLDocumentValidate = 8192 ; const NSXMLNodeLoadExternalEntitiesAlways = 16384 ; const NSXMLNodeLoadExternalEntitiesSameOriginOnly = 32768 ; const NSXMLNodeLoadExternalEntitiesNever = 524288 ; const NSXMLDocumentXInclude = 65536 ; const NSXMLNodePrettyPrint = 131072 ; const NSXMLDocumentIncludeContentTypeDeclaration = 262144 ; const NSXMLNodePreserveNamespaceOrder = 1048576 ; const NSXMLNodePreserveAttributeOrder = 2097152 ; const NSXMLNodePreserveEntities = 4194304 ; const NSXMLNodePreservePrefixes = 8388608 ; const NSXMLNodePreserveCDATA = 16777216 ; const NSXMLNodePreserveWhitespace = 33554432 ; const NSXMLNodePreserveDTD = 67108864 ; const NSXMLNodePreserveCharacterReferences = 134217728 ; const NSXMLNodePromoteSignificantWhitespace = 268435456 ; const NSXMLNodePreserveEmptyElements = 6 ; const NSXMLNodePreserveQuotes = 24 ; const NSXMLNodePreserveAll = 4293918750 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSXMLNodeKind {
  NSXMLInvalidKind = 0,
  NSXMLDocumentKind = 1,
  NSXMLElementKind = 2,
  NSXMLAttributeKind = 3,
  NSXMLNamespaceKind = 4,
  NSXMLProcessingInstructionKind = 5,
  NSXMLCommentKind = 6,
  NSXMLTextKind = 7,
  NSXMLDTDKind = 8,
  NSXMLEntityDeclarationKind = 9,
  NSXMLAttributeDeclarationKind = 10,
  NSXMLElementDeclarationKind = 11,
  NSXMLNotationDeclarationKind = 12,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLNode;
# [ objrs ( class , super = NSXMLNode ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLDTD;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSXMLDTDNodeKind {
  NSXMLEntityGeneralKind = 1,
  NSXMLEntityParsedKind = 2,
  NSXMLEntityUnparsedKind = 3,
  NSXMLEntityParameterKind = 4,
  NSXMLEntityPredefined = 5,
  NSXMLAttributeCDATAKind = 6,
  NSXMLAttributeIDKind = 7,
  NSXMLAttributeIDRefKind = 8,
  NSXMLAttributeIDRefsKind = 9,
  NSXMLAttributeEntityKind = 10,
  NSXMLAttributeEntitiesKind = 11,
  NSXMLAttributeNMTokenKind = 12,
  NSXMLAttributeNMTokensKind = 13,
  NSXMLAttributeEnumerationKind = 14,
  NSXMLAttributeNotationKind = 15,
  NSXMLElementDeclarationUndefinedKind = 16,
  NSXMLElementDeclarationEmptyKind = 17,
  NSXMLElementDeclarationAnyKind = 18,
  NSXMLElementDeclarationMixedKind = 19,
  NSXMLElementDeclarationElementKind = 20,
}
# [ objrs ( class , super = NSXMLNode ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLDTDNode;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSXMLDocumentContentKind {
  NSXMLDocumentXMLKind = 0,
  NSXMLDocumentXHTMLKind = 1,
  NSXMLDocumentHTMLKind = 2,
  NSXMLDocumentTextKind = 3,
}
# [ objrs ( class , super = NSXMLNode ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLDocument;
# [ objrs ( class , super = NSXMLNode ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSXMLElement;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSURLDownload;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSURLDownloadDelegateProto {
  #[objrs(selector = "downloadDidBegin:")]
  #[cfg(feature = "RK_Foundation")]
  fn downloadDidBegin_(&self, download: &NSURLDownload) -> ();
  #[objrs(selector = "download:willSendRequest:redirectResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_willSendRequest_redirectResponse_(
    &self,
    download: &NSURLDownload,
    request: &NSURLRequest,
    redirectResponse: Option<&NSURLResponse>,
  ) -> Option<Arc<NSURLRequest>>;
  #[objrs(selector = "download:canAuthenticateAgainstProtectionSpace:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_canAuthenticateAgainstProtectionSpace_(
    &self,
    connection: &NSURLDownload,
    protectionSpace: &NSURLProtectionSpace,
  ) -> bool;
  #[objrs(selector = "download:didReceiveAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didReceiveAuthenticationChallenge_(
    &self,
    download: &NSURLDownload,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "download:didCancelAuthenticationChallenge:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didCancelAuthenticationChallenge_(
    &self,
    download: &NSURLDownload,
    challenge: &NSURLAuthenticationChallenge,
  ) -> ();
  #[objrs(selector = "downloadShouldUseCredentialStorage:")]
  #[cfg(feature = "RK_Foundation")]
  fn downloadShouldUseCredentialStorage_(&self, download: &NSURLDownload) -> bool;
  #[objrs(selector = "download:didReceiveResponse:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didReceiveResponse_(&self, download: &NSURLDownload, response: &NSURLResponse) -> ();
  #[objrs(selector = "download:willResumeWithResponse:fromByte:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_willResumeWithResponse_fromByte_(
    &self,
    download: &NSURLDownload,
    response: &NSURLResponse,
    startingByte: i64,
  ) -> ();
  #[objrs(selector = "download:didReceiveDataOfLength:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didReceiveDataOfLength_(&self, download: &NSURLDownload, length: usize) -> ();
  #[objrs(selector = "download:shouldDecodeSourceDataOfMIMEType:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_shouldDecodeSourceDataOfMIMEType_(
    &self,
    download: &NSURLDownload,
    encodingType: &NSString,
  ) -> bool;
  #[objrs(selector = "download:decideDestinationWithSuggestedFilename:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_decideDestinationWithSuggestedFilename_(
    &self,
    download: &NSURLDownload,
    filename: &NSString,
  ) -> ();
  #[objrs(selector = "download:didCreateDestination:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didCreateDestination_(&self, download: &NSURLDownload, path: &NSString) -> ();
  #[objrs(selector = "downloadDidFinish:")]
  #[cfg(feature = "RK_Foundation")]
  fn downloadDidFinish_(&self, download: &NSURLDownload) -> ();
  #[objrs(selector = "download:didFailWithError:")]
  #[cfg(feature = "RK_Foundation")]
  fn download_didFailWithError_(&self, download: &NSURLDownload, error: &NSError) -> ();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSAffineTransformStruct {
  pub m11: f64,
  pub m12: f64,
  pub m21: f64,
  pub m22: f64,
  pub tX: f64,
  pub tY: f64,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAffineTransform;
bitflags! { # [ repr ( C ) ] pub struct NSAppleEventSendOptions : usize { const NSAppleEventSendNoReply = 1 ; const NSAppleEventSendQueueReply = 2 ; const NSAppleEventSendWaitForReply = 3 ; const NSAppleEventSendNeverInteract = 16 ; const NSAppleEventSendCanInteract = 32 ; const NSAppleEventSendAlwaysInteract = 48 ; const NSAppleEventSendCanSwitchLayer = 64 ; const NSAppleEventSendDontRecord = 4096 ; const NSAppleEventSendDontExecute = 8192 ; const NSAppleEventSendDontAnnotate = 65536 ; const NSAppleEventSendDefaultOptions = 35 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAppleEventDescriptor;
#[repr(C)]
pub struct __NSAppleEventManagerSuspension {
  opaque: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAppleEventManager;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSAppleScript;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSClassDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDistributedLock;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSHost;
# [ objrs ( class , super = NSClassDescription ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptClassDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptCoercionHandler;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptCommand;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptCommandDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptExecutionContext;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSInsertionPosition {
  NSPositionAfter = 0,
  NSPositionBefore = 1,
  NSPositionBeginning = 2,
  NSPositionEnd = 3,
  NSPositionReplace = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRelativePosition {
  NSRelativeAfter = 0,
  NSRelativeBefore = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSWhoseSubelementIdentifier {
  NSIndexSubelement = 0,
  NSEverySubelement = 1,
  NSMiddleSubelement = 2,
  NSRandomSubelement = 3,
  NSNoSubelement = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptObjectSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSIndexSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMiddleSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSNameSpecifier;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPositionalSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSPropertySpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRandomSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRangeSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSRelativeSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUniqueIDSpecifier;
# [ objrs ( class , super = NSScriptObjectSpecifier ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSWhoseSpecifier;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSaveOptions {
  NSSaveOptionsYes = 0,
  NSSaveOptionsNo = 1,
  NSSaveOptionsAsk = 2,
}
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCloneCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCloseCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCountCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSCreateCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSDeleteCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSExistsCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSGetCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSMoveCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSQuitCommand;
# [ objrs ( class , super = NSScriptCommand ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSetCommand;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptSuiteRegistry;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTestComparisonOperation {
  NSEqualToComparison = 0,
  NSLessThanOrEqualToComparison = 1,
  NSLessThanComparison = 2,
  NSGreaterThanOrEqualToComparison = 3,
  NSGreaterThanComparison = 4,
  NSBeginsWithComparison = 5,
  NSEndsWithComparison = 6,
  NSContainsComparison = 7,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSScriptWhoseTest;
# [ objrs ( class , super = NSScriptWhoseTest ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSLogicalTest;
# [ objrs ( class , super = NSScriptWhoseTest ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSpecifierTest;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSSpellServer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ssFlags {
  pub delegateLearnsWords: u32,
  pub delegateForgetsWords: u32,
  pub busy: u32,
  pub _reserved: u32,
}
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSSpellServerDelegateProto {
  #[objrs(selector = "spellServer:findMisspelledWordInString:language:wordCount:countOnly:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_findMisspelledWordInString_language_wordCount_countOnly_(
    &self,
    sender: &NSSpellServer,
    stringToCheck: &NSString,
    language: &NSString,
    wordCount: &isize,
    countOnly: bool,
  ) -> NSRange;
  #[objrs(selector = "spellServer:suggestGuessesForWord:inLanguage:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_suggestGuessesForWord_inLanguage_(
    &self,
    sender: &NSSpellServer,
    word: &NSString,
    language: &NSString,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "spellServer:didLearnWord:inLanguage:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_didLearnWord_inLanguage_(
    &self,
    sender: &NSSpellServer,
    word: &NSString,
    language: &NSString,
  ) -> ();
  #[objrs(selector = "spellServer:didForgetWord:inLanguage:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_didForgetWord_inLanguage_(
    &self,
    sender: &NSSpellServer,
    word: &NSString,
    language: &NSString,
  ) -> ();
  #[objrs(selector = "spellServer:suggestCompletionsForPartialWordRange:inString:language:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_suggestCompletionsForPartialWordRange_inString_language_(
    &self,
    sender: &NSSpellServer,
    range: NSRange,
    string: &NSString,
    language: &NSString,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "spellServer:checkGrammarInString:language:details:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_checkGrammarInString_language_details_(
    &self,
    sender: &NSSpellServer,
    stringToCheck: &NSString,
    language: Option<&NSString>,
    details: Option<&mut &Option<Arc<NSArray>>>,
  ) -> NSRange;
  #[objrs(selector = "spellServer:checkString:offset:types:options:orthography:wordCount:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_checkString_offset_types_options_orthography_wordCount_(
    &self,
    sender: &NSSpellServer,
    stringToCheck: &NSString,
    offset: usize,
    checkingTypes: u64,
    options: Option<&NSDictionary>,
    orthography: Option<&NSOrthography>,
    wordCount: &isize,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "spellServer:recordResponse:toCorrection:forWord:language:")]
  #[cfg(feature = "RK_Foundation")]
  fn spellServer_recordResponse_toCorrection_forWord_language_(
    &self,
    sender: &NSSpellServer,
    response: usize,
    correction: &NSString,
    word: &NSString,
    language: &NSString,
  ) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSUserNotificationActivationType {
  NSUserNotificationActivationTypeNone = 0,
  NSUserNotificationActivationTypeContentsClicked = 1,
  NSUserNotificationActivationTypeActionButtonClicked = 2,
  NSUserNotificationActivationTypeReplied = 3,
  NSUserNotificationActivationTypeAdditionalActionClicked = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserNotification;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserNotificationAction;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserNotificationCenter;
#[objrs(protocol)]
#[link(name = "Foundation", kind = "framework")]
pub trait NSUserNotificationCenterDelegateProto {
  #[objrs(selector = "userNotificationCenter:didDeliverNotification:")]
  #[cfg(feature = "RK_Foundation")]
  fn userNotificationCenter_didDeliverNotification_(
    &self,
    center: &NSUserNotificationCenter,
    notification: &NSUserNotification,
  ) -> ();
  #[objrs(selector = "userNotificationCenter:didActivateNotification:")]
  #[cfg(feature = "RK_Foundation")]
  fn userNotificationCenter_didActivateNotification_(
    &self,
    center: &NSUserNotificationCenter,
    notification: &NSUserNotification,
  ) -> ();
  #[objrs(selector = "userNotificationCenter:shouldPresentNotification:")]
  #[cfg(feature = "RK_Foundation")]
  fn userNotificationCenter_shouldPresentNotification_(
    &self,
    center: &NSUserNotificationCenter,
    notification: &NSUserNotification,
  ) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserScriptTask;
# [ objrs ( class , super = NSUserScriptTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserUnixTask;
# [ objrs ( class , super = NSUserScriptTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserAppleScriptTask;
# [ objrs ( class , super = NSUserScriptTask ) ]
#[link(name = "Foundation", kind = "framework")]
pub struct NSUserAutomatorTask;
#[cfg(feature = "RK_Foundation")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {
  pub fn NSStringFromSelector(aSelector: SelectorRef) -> *mut NSString;
  pub fn NSSelectorFromString(aSelectorName: *mut NSString) -> SelectorRef;
  pub fn NSStringFromClass(aClass: *mut Class) -> *mut NSString;
  pub fn NSClassFromString(aClassName: *mut NSString) -> *mut Class;
  pub fn NSStringFromProtocol(proto: *mut Protocol) -> *mut NSString;
  pub fn NSProtocolFromString(namestr: *mut NSString) -> *mut Protocol;
  pub fn NSGetSizeAndAlignment(typePtr: *mut i8, sizep: *mut usize, alignp: *mut usize) -> *mut i8;
  pub fn NSLog(format: *mut NSString, ...) -> ();
  pub fn NSPageSize() -> usize;
  pub fn NSLogPageSize() -> usize;
  pub fn NSRoundUpToMultipleOfPageSize(bytes: usize) -> usize;
  pub fn NSRoundDownToMultipleOfPageSize(bytes: usize) -> usize;
  pub fn NSAllocateMemoryPages(bytes: usize) -> *mut c_void;
  pub fn NSDeallocateMemoryPages(ptr: *mut c_void, bytes: usize) -> ();
  pub fn NSCopyMemoryPages(source: *mut c_void, dest: *mut c_void, bytes: usize) -> ();
  pub fn NSRealMemoryAvailable() -> usize;
  pub fn CFBridgingRetain(X: *mut Object) -> *mut c_void;
  pub fn CFBridgingRelease(X: *mut c_void) -> *mut Object;
  pub fn NSMakeRange(loc: usize, len: usize) -> NSRange;
  pub fn NSMaxRange(range: NSRange) -> usize;
  pub fn NSLocationInRange(loc: usize, range: NSRange) -> bool;
  pub fn NSEqualRanges(range1: NSRange, range2: NSRange) -> bool;
  pub fn NSUnionRange(range1: NSRange, range2: NSRange) -> NSRange;
  pub fn NSIntersectionRange(range1: NSRange, range2: NSRange) -> NSRange;
  pub fn NSStringFromRange(range: NSRange) -> *mut NSString;
  pub fn NSRangeFromString(aString: *mut NSString) -> NSRange;
  pub fn NSHostByteOrder() -> isize;
  pub fn NSSwapShort(inv: u16) -> u16;
  pub fn NSSwapInt(inv: u32) -> u32;
  pub fn NSSwapLong(inv: usize) -> usize;
  pub fn NSSwapLongLong(inv: u64) -> u64;
  pub fn NSSwapBigShortToHost(x: u16) -> u16;
  pub fn NSSwapBigIntToHost(x: u32) -> u32;
  pub fn NSSwapBigLongToHost(x: usize) -> usize;
  pub fn NSSwapBigLongLongToHost(x: u64) -> u64;
  pub fn NSSwapHostShortToBig(x: u16) -> u16;
  pub fn NSSwapHostIntToBig(x: u32) -> u32;
  pub fn NSSwapHostLongToBig(x: usize) -> usize;
  pub fn NSSwapHostLongLongToBig(x: u64) -> u64;
  pub fn NSSwapLittleShortToHost(x: u16) -> u16;
  pub fn NSSwapLittleIntToHost(x: u32) -> u32;
  pub fn NSSwapLittleLongToHost(x: usize) -> usize;
  pub fn NSSwapLittleLongLongToHost(x: u64) -> u64;
  pub fn NSSwapHostShortToLittle(x: u16) -> u16;
  pub fn NSSwapHostIntToLittle(x: u32) -> u32;
  pub fn NSSwapHostLongToLittle(x: usize) -> usize;
  pub fn NSSwapHostLongLongToLittle(x: u64) -> u64;
  pub fn NSConvertHostFloatToSwapped(x: f32) -> NSSwappedFloat;
  pub fn NSConvertSwappedFloatToHost(x: NSSwappedFloat) -> f32;
  pub fn NSConvertHostDoubleToSwapped(x: f64) -> NSSwappedDouble;
  pub fn NSConvertSwappedDoubleToHost(x: NSSwappedDouble) -> f64;
  pub fn NSSwapFloat(x: NSSwappedFloat) -> NSSwappedFloat;
  pub fn NSSwapDouble(x: NSSwappedDouble) -> NSSwappedDouble;
  pub fn NSSwapBigDoubleToHost(x: NSSwappedDouble) -> f64;
  pub fn NSSwapBigFloatToHost(x: NSSwappedFloat) -> f32;
  pub fn NSSwapHostDoubleToBig(x: f64) -> NSSwappedDouble;
  pub fn NSSwapHostFloatToBig(x: f32) -> NSSwappedFloat;
  pub fn NSSwapLittleDoubleToHost(x: NSSwappedDouble) -> f64;
  pub fn NSSwapLittleFloatToHost(x: NSSwappedFloat) -> f32;
  pub fn NSSwapHostDoubleToLittle(x: f64) -> NSSwappedDouble;
  pub fn NSSwapHostFloatToLittle(x: f32) -> NSSwappedFloat;
  pub fn NXReadNSObjectFromCoder(decoder: *mut NSCoder) -> *mut NSObject;
  pub fn NSDecimalIsNotANumber(dcm: *mut NSDecimal) -> bool;
  pub fn NSDecimalCopy(destination: *mut NSDecimal, source: *mut NSDecimal) -> ();
  pub fn NSDecimalCompact(number: *mut NSDecimal) -> ();
  pub fn NSDecimalCompare(
    leftOperand: *mut NSDecimal,
    rightOperand: *mut NSDecimal,
  ) -> NSComparisonResult;
  pub fn NSDecimalRound(
    result: *mut NSDecimal,
    number: *mut NSDecimal,
    scale: isize,
    roundingMode: NSRoundingMode,
  ) -> ();
  pub fn NSDecimalNormalize(
    number1: *mut NSDecimal,
    number2: *mut NSDecimal,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalAdd(
    result: *mut NSDecimal,
    leftOperand: *mut NSDecimal,
    rightOperand: *mut NSDecimal,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalSubtract(
    result: *mut NSDecimal,
    leftOperand: *mut NSDecimal,
    rightOperand: *mut NSDecimal,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalMultiply(
    result: *mut NSDecimal,
    leftOperand: *mut NSDecimal,
    rightOperand: *mut NSDecimal,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalDivide(
    result: *mut NSDecimal,
    leftOperand: *mut NSDecimal,
    rightOperand: *mut NSDecimal,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalPower(
    result: *mut NSDecimal,
    number: *mut NSDecimal,
    power: usize,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalMultiplyByPowerOf10(
    result: *mut NSDecimal,
    number: *mut NSDecimal,
    power: i16,
    roundingMode: NSRoundingMode,
  ) -> NSCalculationError;
  pub fn NSDecimalString(dcm: *mut NSDecimal, locale: *mut Object) -> *mut NSString;
  pub fn NSGetUncaughtExceptionHandler() -> Option<extern "C" fn(*mut NSException) -> ()>;
  pub fn NSSetUncaughtExceptionHandler(_: Option<extern "C" fn(*mut NSException) -> ()>) -> ();
  pub fn NSUserName() -> *mut NSString;
  pub fn NSFullUserName() -> *mut NSString;
  pub fn NSHomeDirectory() -> *mut NSString;
  pub fn NSHomeDirectoryForUser(userName: *mut NSString) -> *mut NSString;
  pub fn NSTemporaryDirectory() -> *mut NSString;
  pub fn NSOpenStepRootDirectory() -> *mut NSString;
  pub fn NSSearchPathForDirectoriesInDomains(
    directory: NSSearchPathDirectory,
    domainMask: NSSearchPathDomainMask,
    expandTilde: bool,
  ) -> *mut NSArray;
  pub fn NSFreeHashTable(table: *mut NSHashTable) -> ();
  pub fn NSResetHashTable(table: *mut NSHashTable) -> ();
  pub fn NSCompareHashTables(table1: *mut NSHashTable, table2: *mut NSHashTable) -> bool;
  pub fn NSCopyHashTableWithZone(table: *mut NSHashTable, zone: *mut NSZone) -> *mut NSHashTable;
  pub fn NSHashGet(table: *mut NSHashTable, pointer: *mut c_void) -> *mut c_void;
  pub fn NSHashInsert(table: *mut NSHashTable, pointer: *mut c_void) -> ();
  pub fn NSHashInsertKnownAbsent(table: *mut NSHashTable, pointer: *mut c_void) -> ();
  pub fn NSHashInsertIfAbsent(table: *mut NSHashTable, pointer: *mut c_void) -> *mut c_void;
  pub fn NSHashRemove(table: *mut NSHashTable, pointer: *mut c_void) -> ();
  pub fn NSEnumerateHashTable(table: *mut NSHashTable) -> NSHashEnumerator;
  pub fn NSNextHashEnumeratorItem(enumerator: *mut NSHashEnumerator) -> *mut c_void;
  pub fn NSEndHashTableEnumeration(enumerator: *mut NSHashEnumerator) -> ();
  pub fn NSCountHashTable(table: *mut NSHashTable) -> usize;
  pub fn NSStringFromHashTable(table: *mut NSHashTable) -> *mut NSString;
  pub fn NSAllHashTableObjects(table: *mut NSHashTable) -> *mut NSArray;
  pub fn NSCreateHashTableWithZone(
    callBacks: NSHashTableCallBacks,
    capacity: usize,
    zone: *mut NSZone,
  ) -> *mut NSHashTable;
  pub fn NSCreateHashTable(callBacks: NSHashTableCallBacks, capacity: usize) -> *mut NSHashTable;
  pub fn NSMakePoint(x: f64, y: f64) -> CGPoint;
  pub fn NSMakeSize(w: f64, h: f64) -> CGSize;
  pub fn NSMakeRect(x: f64, y: f64, w: f64, h: f64) -> CGRect;
  pub fn NSMaxX(aRect: CGRect) -> f64;
  pub fn NSMaxY(aRect: CGRect) -> f64;
  pub fn NSMidX(aRect: CGRect) -> f64;
  pub fn NSMidY(aRect: CGRect) -> f64;
  pub fn NSMinX(aRect: CGRect) -> f64;
  pub fn NSMinY(aRect: CGRect) -> f64;
  pub fn NSWidth(aRect: CGRect) -> f64;
  pub fn NSHeight(aRect: CGRect) -> f64;
  pub fn NSRectFromCGRect(cgrect: CGRect) -> CGRect;
  pub fn NSRectToCGRect(nsrect: CGRect) -> CGRect;
  pub fn NSPointFromCGPoint(cgpoint: CGPoint) -> CGPoint;
  pub fn NSPointToCGPoint(nspoint: CGPoint) -> CGPoint;
  pub fn NSSizeFromCGSize(cgsize: CGSize) -> CGSize;
  pub fn NSSizeToCGSize(nssize: CGSize) -> CGSize;
  pub fn NSEdgeInsetsMake(top: f64, left: f64, bottom: f64, right: f64) -> NSEdgeInsets;
  pub fn NSEqualPoints(aPoint: CGPoint, bPoint: CGPoint) -> bool;
  pub fn NSEqualSizes(aSize: CGSize, bSize: CGSize) -> bool;
  pub fn NSEqualRects(aRect: CGRect, bRect: CGRect) -> bool;
  pub fn NSIsEmptyRect(aRect: CGRect) -> bool;
  pub fn NSEdgeInsetsEqual(aInsets: NSEdgeInsets, bInsets: NSEdgeInsets) -> bool;
  pub fn NSInsetRect(aRect: CGRect, dX: f64, dY: f64) -> CGRect;
  pub fn NSIntegralRect(aRect: CGRect) -> CGRect;
  pub fn NSIntegralRectWithOptions(aRect: CGRect, opts: NSAlignmentOptions) -> CGRect;
  pub fn NSUnionRect(aRect: CGRect, bRect: CGRect) -> CGRect;
  pub fn NSIntersectionRect(aRect: CGRect, bRect: CGRect) -> CGRect;
  pub fn NSOffsetRect(aRect: CGRect, dX: f64, dY: f64) -> CGRect;
  pub fn NSDivideRect(
    inRect: CGRect,
    slice: *mut CGRect,
    rem: *mut CGRect,
    amount: f64,
    edge: NSRectEdge,
  ) -> ();
  pub fn NSPointInRect(aPoint: CGPoint, aRect: CGRect) -> bool;
  pub fn NSMouseInRect(aPoint: CGPoint, aRect: CGRect, flipped: bool) -> bool;
  pub fn NSContainsRect(aRect: CGRect, bRect: CGRect) -> bool;
  pub fn NSIntersectsRect(aRect: CGRect, bRect: CGRect) -> bool;
  pub fn NSStringFromPoint(aPoint: CGPoint) -> *mut NSString;
  pub fn NSStringFromSize(aSize: CGSize) -> *mut NSString;
  pub fn NSStringFromRect(aRect: CGRect) -> *mut NSString;
  pub fn NSPointFromString(aString: *mut NSString) -> CGPoint;
  pub fn NSSizeFromString(aString: *mut NSString) -> CGSize;
  pub fn NSRectFromString(aString: *mut NSString) -> CGRect;
  pub fn NSFreeMapTable(table: *mut NSMapTable) -> ();
  pub fn NSResetMapTable(table: *mut NSMapTable) -> ();
  pub fn NSCompareMapTables(table1: *mut NSMapTable, table2: *mut NSMapTable) -> bool;
  pub fn NSCopyMapTableWithZone(table: *mut NSMapTable, zone: *mut NSZone) -> *mut NSMapTable;
  pub fn NSMapMember(
    table: *mut NSMapTable,
    key: *mut c_void,
    originalKey: *mut *mut c_void,
    value: *mut *mut c_void,
  ) -> bool;
  pub fn NSMapGet(table: *mut NSMapTable, key: *mut c_void) -> *mut c_void;
  pub fn NSMapInsert(table: *mut NSMapTable, key: *mut c_void, value: *mut c_void) -> ();
  pub fn NSMapInsertKnownAbsent(table: *mut NSMapTable, key: *mut c_void, value: *mut c_void)
    -> ();
  pub fn NSMapInsertIfAbsent(
    table: *mut NSMapTable,
    key: *mut c_void,
    value: *mut c_void,
  ) -> *mut c_void;
  pub fn NSMapRemove(table: *mut NSMapTable, key: *mut c_void) -> ();
  pub fn NSEnumerateMapTable(table: *mut NSMapTable) -> NSMapEnumerator;
  pub fn NSNextMapEnumeratorPair(
    enumerator: *mut NSMapEnumerator,
    key: *mut *mut c_void,
    value: *mut *mut c_void,
  ) -> bool;
  pub fn NSEndMapTableEnumeration(enumerator: *mut NSMapEnumerator) -> ();
  pub fn NSCountMapTable(table: *mut NSMapTable) -> usize;
  pub fn NSStringFromMapTable(table: *mut NSMapTable) -> *mut NSString;
  pub fn NSAllMapTableKeys(table: *mut NSMapTable) -> *mut NSArray;
  pub fn NSAllMapTableValues(table: *mut NSMapTable) -> *mut NSArray;
  pub fn NSCreateMapTableWithZone(
    keyCallBacks: NSMapTableKeyCallBacks,
    valueCallBacks: NSMapTableValueCallBacks,
    capacity: usize,
    zone: *mut NSZone,
  ) -> *mut NSMapTable;
  pub fn NSCreateMapTable(
    keyCallBacks: NSMapTableKeyCallBacks,
    valueCallBacks: NSMapTableValueCallBacks,
    capacity: usize,
  ) -> *mut NSMapTable;
  pub fn NSFileTypeForHFSTypeCode(hfsFileTypeCode: u32) -> *mut NSString;
  pub fn NSHFSTypeCodeFromFileType(fileTypeString: *mut NSString) -> u32;
  pub fn NSHFSTypeOfFile(fullFilePath: *mut NSString) -> *mut NSString;
}
