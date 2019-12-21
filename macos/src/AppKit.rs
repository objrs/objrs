#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreData::NSAttributeType;
use crate::CoreData::NSManagedObjectContext;
use crate::CoreGraphics::CGColor;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGContext;
use crate::CoreGraphics::CGImage;
use crate::CoreGraphics::CGPoint;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::CoreGraphics::__CGEvent;
use crate::CoreImage::CIColor;
use crate::CoreImage::CIImage;
use crate::CoreServices::LaunchServices::OpaqueIconRef;
use crate::CoreVideo::CVTimeStamp;
use crate::Foundation::NSAffineTransform;
use crate::Foundation::NSAlignmentOptions;
use crate::Foundation::NSAppleEventDescriptor;
use crate::Foundation::NSArray;
use crate::Foundation::NSAttributedString;
use crate::Foundation::NSBundle;
use crate::Foundation::NSCalendar;
use crate::Foundation::NSCharacterSet;
use crate::Foundation::NSCloseCommand;
use crate::Foundation::NSCoder;
use crate::Foundation::NSCodingProto;
use crate::Foundation::NSComparisonPredicateModifier;
use crate::Foundation::NSComparisonResult;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSData;
use crate::Foundation::NSDate;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSEdgeInsets;
use crate::Foundation::NSEnumerator;
use crate::Foundation::NSError;
use crate::Foundation::NSException;
use crate::Foundation::NSExpression;
use crate::Foundation::NSExtensionContext;
use crate::Foundation::NSFilePresenterProto;
use crate::Foundation::NSFileWrapper;
use crate::Foundation::NSFormatter;
use crate::Foundation::NSIndexPath;
use crate::Foundation::NSIndexSet;
use crate::Foundation::NSItemProvider;
use crate::Foundation::NSLocale;
use crate::Foundation::NSLockingProto;
use crate::Foundation::NSMutableArray;
use crate::Foundation::NSMutableAttributedString;
use crate::Foundation::NSMutableCopyingProto;
use crate::Foundation::NSMutableData;
use crate::Foundation::NSMutableDictionary;
use crate::Foundation::NSNotification;
use crate::Foundation::NSNotificationCenter;
use crate::Foundation::NSNumber;
use crate::Foundation::NSOperationQueue;
use crate::Foundation::NSOrthography;
use crate::Foundation::NSPredicate;
use crate::Foundation::NSProgress;
use crate::Foundation::NSProgressReportingProto;
use crate::Foundation::NSRange;
use crate::Foundation::NSRectEdge;
use crate::Foundation::NSScriptCommand;
use crate::Foundation::NSScriptObjectSpecifier;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSSet;
use crate::Foundation::NSSortDescriptor;
use crate::Foundation::NSString;
use crate::Foundation::NSTextCheckingResult;
use crate::Foundation::NSTimeZone;
use crate::Foundation::NSUndoManager;
use crate::Foundation::NSUserActivity;
use crate::Foundation::NSUserDefaults;
use crate::Foundation::NSValue;
use crate::Foundation::NSZone;
use crate::Foundation::NSURL;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
use crate::OpenGL::_CGLContextObject;
use crate::OpenGL::_CGLPBufferObject;
use crate::OpenGL::_CGLPixelFormatObject;
use crate::QuartzCore::CALayer;
use crate::QuartzCore::CAOpenGLLayer;
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
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCompositingOperation {
  NSCompositingOperationClear = 0,
  NSCompositingOperationCopy = 1,
  NSCompositingOperationSourceOver = 2,
  NSCompositingOperationSourceIn = 3,
  NSCompositingOperationSourceOut = 4,
  NSCompositingOperationSourceAtop = 5,
  NSCompositingOperationDestinationOver = 6,
  NSCompositingOperationDestinationIn = 7,
  NSCompositingOperationDestinationOut = 8,
  NSCompositingOperationDestinationAtop = 9,
  NSCompositingOperationXOR = 10,
  NSCompositingOperationPlusDarker = 11,
  NSCompositingOperationHighlight = 12,
  NSCompositingOperationPlusLighter = 13,
  NSCompositingOperationMultiply = 14,
  NSCompositingOperationScreen = 15,
  NSCompositingOperationOverlay = 16,
  NSCompositingOperationDarken = 17,
  NSCompositingOperationLighten = 18,
  NSCompositingOperationColorDodge = 19,
  NSCompositingOperationColorBurn = 20,
  NSCompositingOperationSoftLight = 21,
  NSCompositingOperationHardLight = 22,
  NSCompositingOperationDifference = 23,
  NSCompositingOperationExclusion = 24,
  NSCompositingOperationHue = 25,
  NSCompositingOperationSaturation = 26,
  NSCompositingOperationColor = 27,
  NSCompositingOperationLuminosity = 28,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBackingStoreType {
  NSBackingStoreRetained = 0,
  NSBackingStoreNonretained = 1,
  NSBackingStoreBuffered = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWindowOrderingMode {
  NSWindowAbove = 1,
  NSWindowBelow = -1,
  NSWindowOut = 0,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFocusRingPlacement {
  NSFocusRingOnly = 0,
  NSFocusRingBelow = 1,
  NSFocusRingAbove = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFocusRingType {
  NSFocusRingTypeDefault = 0,
  NSFocusRingTypeNone = 1,
  NSFocusRingTypeExterior = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSColorRenderingIntent {
  NSColorRenderingIntentDefault = 0,
  NSColorRenderingIntentAbsoluteColorimetric = 1,
  NSColorRenderingIntentRelativeColorimetric = 2,
  NSColorRenderingIntentPerceptual = 3,
  NSColorRenderingIntentSaturation = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum NSWindowDepth {
  NSWindowDepthTwentyfourBitRGB = 520,
  NSWindowDepthSixtyfourBitRGB = 528,
  NSWindowDepthOnehundredtwentyeightBitRGB = 544,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSDisplayGamut {
  NSDisplayGamutSRGB = 1,
  NSDisplayGamutP3 = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSAnimationEffect {
  NSAnimationEffectDisappearingItemDefault = 0,
  NSAnimationEffectPoof = 10,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageInterpolation {
  NSImageInterpolationDefault = 0,
  NSImageInterpolationNone = 1,
  NSImageInterpolationLow = 2,
  NSImageInterpolationMedium = 4,
  NSImageInterpolationHigh = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGraphicsContext;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityAnnotationPosition {
  NSAccessibilityAnnotationPositionFullRange = 0,
  NSAccessibilityAnnotationPositionStart = 1,
  NSAccessibilityAnnotationPositionEnd = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityOrientation {
  NSAccessibilityOrientationUnknown = 0,
  NSAccessibilityOrientationVertical = 1,
  NSAccessibilityOrientationHorizontal = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilitySortDirection {
  NSAccessibilitySortDirectionUnknown = 0,
  NSAccessibilitySortDirectionAscending = 1,
  NSAccessibilitySortDirectionDescending = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityRulerMarkerType {
  NSAccessibilityRulerMarkerTypeUnknown = 0,
  NSAccessibilityRulerMarkerTypeTabStopLeft = 1,
  NSAccessibilityRulerMarkerTypeTabStopRight = 2,
  NSAccessibilityRulerMarkerTypeTabStopCenter = 3,
  NSAccessibilityRulerMarkerTypeTabStopDecimal = 4,
  NSAccessibilityRulerMarkerTypeIndentHead = 5,
  NSAccessibilityRulerMarkerTypeIndentTail = 6,
  NSAccessibilityRulerMarkerTypeIndentFirstLine = 7,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityUnits {
  NSAccessibilityUnitsUnknown = 0,
  NSAccessibilityUnitsInches = 1,
  NSAccessibilityUnitsCentimeters = 2,
  NSAccessibilityUnitsPoints = 3,
  NSAccessibilityUnitsPicas = 4,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityPriorityLevel {
  NSAccessibilityPriorityLow = 10,
  NSAccessibilityPriorityMedium = 50,
  NSAccessibilityPriorityHigh = 90,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAccessibilityCustomAction;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityElementProto {
  #[objrs(selector = "accessibilityFrame")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityFrame(&self) -> CGRect;
  #[objrs(selector = "accessibilityParent")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityParent(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "isAccessibilityFocused")]
  #[cfg(feature = "RK_AppKit")]
  fn isAccessibilityFocused(&self) -> bool;
  #[objrs(selector = "accessibilityIdentifier")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityIdentifier(&self) -> Arc<NSString>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityGroupProto {}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityButtonProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityPerformPress")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformPress(&self) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilitySwitchProto {
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityPerformIncrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformIncrement(&self) -> bool;
  #[objrs(selector = "accessibilityPerformDecrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformDecrement(&self) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityRadioButtonProto {
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<NSNumber>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityCheckBoxProto {
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<NSNumber>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityStaticTextProto {
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityAttributedStringForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityAttributedStringForRange_(
    &self,
    range: NSRange,
  ) -> Option<Arc<NSAttributedString>>;
  #[objrs(selector = "accessibilityVisibleCharacterRange")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityVisibleCharacterRange(&self) -> NSRange;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityNavigableStaticTextProto {
  #[objrs(selector = "accessibilityStringForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityStringForRange_(&self, range: NSRange) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityLineForIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLineForIndex_(&self, index: isize) -> isize;
  #[objrs(selector = "accessibilityRangeForLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRangeForLine_(&self, lineNumber: isize) -> NSRange;
  #[objrs(selector = "accessibilityFrameForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityFrameForRange_(&self, range: NSRange) -> CGRect;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityProgressIndicatorProto {
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<NSNumber>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityStepperProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityPerformIncrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformIncrement(&self) -> bool;
  #[objrs(selector = "accessibilityPerformDecrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformDecrement(&self) -> bool;
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<Object>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilitySliderProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityValue")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityValue(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "accessibilityPerformIncrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformIncrement(&self) -> bool;
  #[objrs(selector = "accessibilityPerformDecrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformDecrement(&self) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityImageProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Option<Arc<NSString>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityContainsTransientUIProto {
  #[objrs(selector = "accessibilityPerformShowAlternateUI")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformShowAlternateUI(&self) -> bool;
  #[objrs(selector = "accessibilityPerformShowDefaultUI")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformShowDefaultUI(&self) -> bool;
  #[objrs(selector = "isAccessibilityAlternateUIVisible")]
  #[cfg(feature = "RK_AppKit")]
  fn isAccessibilityAlternateUIVisible(&self) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityTableProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityRows")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRows(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilitySelectedRows")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilitySelectedRows(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "setAccessibilitySelectedRows:")]
  #[cfg(feature = "RK_AppKit")]
  fn setAccessibilitySelectedRows_(&self, selectedRows: &NSArray) -> ();
  #[objrs(selector = "accessibilityVisibleRows")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityVisibleRows(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityColumns")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityColumns(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityVisibleColumns")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityVisibleColumns(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilitySelectedColumns")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilitySelectedColumns(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityHeaderGroup")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityHeaderGroup(&self) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilitySelectedCells")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilitySelectedCells(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityVisibleCells")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityVisibleCells(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityRowHeaderUIElements")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRowHeaderUIElements(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilityColumnHeaderUIElements")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityColumnHeaderUIElements(&self) -> Option<Arc<NSArray>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityOutlineProto {}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityListProto {}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityRowProto {
  #[objrs(selector = "accessibilityIndex")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityIndex(&self) -> isize;
  #[objrs(selector = "accessibilityDisclosureLevel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityDisclosureLevel(&self) -> isize;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityLayoutAreaProto {
  #[objrs(selector = "accessibilityLabel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLabel(&self) -> Arc<NSString>;
  #[objrs(selector = "accessibilityChildren")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityChildren(&self) -> Option<Arc<NSArray>>;
  #[objrs(selector = "accessibilitySelectedChildren")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilitySelectedChildren(&self) -> Option<Arc<NSArray>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityLayoutItemProto {
  #[objrs(selector = "setAccessibilityFrame:")]
  #[cfg(feature = "RK_AppKit")]
  fn setAccessibilityFrame_(&self, frame: CGRect) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityElementLoadingProto {
  #[objrs(selector = "accessibilityElementWithToken:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityElementWithToken_(&self, token: &Object) -> Option<Arc<Object>>;
  #[objrs(selector = "accessibilityRangeInTargetElementWithToken:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRangeInTargetElementWithToken_(&self, token: &Object) -> NSRange;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityProto {
  #[objrs(selector = "accessibilityLayoutPointForScreenPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLayoutPointForScreenPoint_(&self, point: CGPoint) -> CGPoint;
  #[objrs(selector = "accessibilityLayoutSizeForScreenSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLayoutSizeForScreenSize_(&self, size: CGSize) -> CGSize;
  #[objrs(selector = "accessibilityScreenPointForLayoutPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityScreenPointForLayoutPoint_(&self, point: CGPoint) -> CGPoint;
  #[objrs(selector = "accessibilityScreenSizeForLayoutSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityScreenSizeForLayoutSize_(&self, size: CGSize) -> CGSize;
  #[objrs(selector = "accessibilityCellForColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityCellForColumn_row_(&self, column: isize, row: isize) -> Option<Arc<Object>>;
  #[objrs(selector = "accessibilityAttributedStringForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityAttributedStringForRange_(
    &self,
    range: NSRange,
  ) -> Option<Arc<NSAttributedString>>;
  #[objrs(selector = "accessibilityRangeForLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRangeForLine_(&self, line: isize) -> NSRange;
  #[objrs(selector = "accessibilityStringForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityStringForRange_(&self, range: NSRange) -> Option<Arc<NSString>>;
  #[objrs(selector = "accessibilityRangeForPosition:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRangeForPosition_(&self, point: CGPoint) -> NSRange;
  #[objrs(selector = "accessibilityRangeForIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRangeForIndex_(&self, index: isize) -> NSRange;
  #[objrs(selector = "accessibilityFrameForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityFrameForRange_(&self, range: NSRange) -> CGRect;
  #[objrs(selector = "accessibilityRTFForRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityRTFForRange_(&self, range: NSRange) -> Option<Arc<NSData>>;
  #[objrs(selector = "accessibilityStyleRangeForIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityStyleRangeForIndex_(&self, index: isize) -> NSRange;
  #[objrs(selector = "accessibilityLineForIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityLineForIndex_(&self, index: isize) -> isize;
  #[objrs(selector = "accessibilityPerformCancel")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformCancel(&self) -> bool;
  #[objrs(selector = "accessibilityPerformConfirm")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformConfirm(&self) -> bool;
  #[objrs(selector = "accessibilityPerformDecrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformDecrement(&self) -> bool;
  #[objrs(selector = "accessibilityPerformDelete")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformDelete(&self) -> bool;
  #[objrs(selector = "accessibilityPerformIncrement")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformIncrement(&self) -> bool;
  #[objrs(selector = "accessibilityPerformPick")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformPick(&self) -> bool;
  #[objrs(selector = "accessibilityPerformPress")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformPress(&self) -> bool;
  #[objrs(selector = "accessibilityPerformRaise")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformRaise(&self) -> bool;
  #[objrs(selector = "accessibilityPerformShowAlternateUI")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformShowAlternateUI(&self) -> bool;
  #[objrs(selector = "accessibilityPerformShowDefaultUI")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformShowDefaultUI(&self) -> bool;
  #[objrs(selector = "accessibilityPerformShowMenu")]
  #[cfg(feature = "RK_AppKit")]
  fn accessibilityPerformShowMenu(&self) -> bool;
  #[objrs(selector = "isAccessibilitySelectorAllowed:")]
  #[cfg(feature = "RK_AppKit")]
  fn isAccessibilitySelectorAllowed_(&self, selector: SelectorRef) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAccessibilityElement;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityCustomRotorSearchDirection {
  NSAccessibilityCustomRotorSearchDirectionPrevious = 0,
  NSAccessibilityCustomRotorSearchDirectionNext = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSAccessibilityCustomRotorType {
  NSAccessibilityCustomRotorTypeCustom = 0,
  NSAccessibilityCustomRotorTypeAny = 1,
  NSAccessibilityCustomRotorTypeAnnotation = 2,
  NSAccessibilityCustomRotorTypeBoldText = 3,
  NSAccessibilityCustomRotorTypeHeading = 4,
  NSAccessibilityCustomRotorTypeHeadingLevel1 = 5,
  NSAccessibilityCustomRotorTypeHeadingLevel2 = 6,
  NSAccessibilityCustomRotorTypeHeadingLevel3 = 7,
  NSAccessibilityCustomRotorTypeHeadingLevel4 = 8,
  NSAccessibilityCustomRotorTypeHeadingLevel5 = 9,
  NSAccessibilityCustomRotorTypeHeadingLevel6 = 10,
  NSAccessibilityCustomRotorTypeImage = 11,
  NSAccessibilityCustomRotorTypeItalicText = 12,
  NSAccessibilityCustomRotorTypeLandmark = 13,
  NSAccessibilityCustomRotorTypeLink = 14,
  NSAccessibilityCustomRotorTypeList = 15,
  NSAccessibilityCustomRotorTypeMisspelledWord = 16,
  NSAccessibilityCustomRotorTypeTable = 17,
  NSAccessibilityCustomRotorTypeTextField = 18,
  NSAccessibilityCustomRotorTypeUnderlinedText = 19,
  NSAccessibilityCustomRotorTypeVisitedLink = 20,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAccessibilityCustomRotor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAccessibilityCustomRotorSearchParameters;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAccessibilityCustomRotorItemResult;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAccessibilityCustomRotorItemSearchDelegateProto {
  #[objrs(selector = "rotor:resultForSearchParameters:")]
  #[cfg(feature = "RK_AppKit")]
  fn rotor_resultForSearchParameters_(
    &self,
    rotor: &NSAccessibilityCustomRotor,
    searchParameters: &NSAccessibilityCustomRotorSearchParameters,
  ) -> Option<Arc<NSAccessibilityCustomRotorItemResult>>;
}
bitflags! { # [ repr ( C ) ] pub struct NSWorkspaceLaunchOptions : usize { const NSWorkspaceLaunchAndPrint = 2 ; const NSWorkspaceLaunchWithErrorPresentation = 64 ; const NSWorkspaceLaunchInhibitingBackgroundOnly = 128 ; const NSWorkspaceLaunchWithoutAddingToRecents = 256 ; const NSWorkspaceLaunchWithoutActivation = 512 ; const NSWorkspaceLaunchAsync = 65536 ; const NSWorkspaceLaunchNewInstance = 524288 ; const NSWorkspaceLaunchAndHide = 1048576 ; const NSWorkspaceLaunchAndHideOthers = 2097152 ; const NSWorkspaceLaunchAllowingClassicStartup = 131072 ; const NSWorkspaceLaunchPreferringClassic = 262144 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSWorkspaceIconCreationOptions : usize { const NSExcludeQuickDrawElementsIconCreationOption = 2 ; const NSExclude10_4ElementsIconCreationOption = 4 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWorkspace;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWorkspaceAuthorizationType {
  NSWorkspaceAuthorizationTypeCreateSymbolicLink = 0,
  NSWorkspaceAuthorizationTypeSetAttributes = 1,
  NSWorkspaceAuthorizationTypeReplaceFile = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWorkspaceAuthorization;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSAnimationCurve {
  NSAnimationEaseInOut = 0,
  NSAnimationEaseIn = 1,
  NSAnimationEaseOut = 2,
  NSAnimationLinear = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSAnimationBlockingMode {
  NSAnimationBlocking = 0,
  NSAnimationNonblocking = 1,
  NSAnimationNonblockingThreaded = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAnimation;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __aFlags {
  pub delegateAnimationShouldStart: u32,
  pub delegateAnimationDidStop: u32,
  pub delegateAnimationDidEnd: u32,
  pub delegateAnimationValueForProgress: u32,
  pub delegateAnimationDidReachProgressMark: u32,
  pub animating: u32,
  pub blocking: u32,
  pub sendProgressAllTheTime: u32,
  pub hasHandler: u32,
  pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __aSettings {
  pub animationCurve: u32,
  pub animationBlockingMode: u32,
  pub reserved: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAnimationDelegateProto {
  #[objrs(selector = "animationShouldStart:")]
  #[cfg(feature = "RK_AppKit")]
  fn animationShouldStart_(&self, animation: &NSAnimation) -> bool;
  #[objrs(selector = "animationDidStop:")]
  #[cfg(feature = "RK_AppKit")]
  fn animationDidStop_(&self, animation: &NSAnimation) -> ();
  #[objrs(selector = "animationDidEnd:")]
  #[cfg(feature = "RK_AppKit")]
  fn animationDidEnd_(&self, animation: &NSAnimation) -> ();
  #[objrs(selector = "animation:valueForProgress:")]
  #[cfg(feature = "RK_AppKit")]
  fn animation_valueForProgress_(&self, animation: &NSAnimation, progress: f32) -> f32;
  #[objrs(selector = "animation:didReachProgressMark:")]
  #[cfg(feature = "RK_AppKit")]
  fn animation_didReachProgressMark_(&self, animation: &NSAnimation, progress: f32) -> ();
}
# [ objrs ( class , super = NSAnimation ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSViewAnimation;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAnimatablePropertyContainerProto {
  #[objrs(selector = "animator")]
  #[cfg(feature = "RK_AppKit")]
  fn animator(&self) -> Arc<Self>;
  #[objrs(selector = "animationForKey:")]
  #[cfg(feature = "RK_AppKit")]
  fn animationForKey_(&self, key: &NSString) -> Option<Arc<Object>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAppearance;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAppearanceCustomizationProto {}
bitflags! { # [ repr ( C ) ] pub struct NSPasteboardContentsOptions : usize { const NSPasteboardContentsCurrentHostOnly = 1 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPasteboard;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPasteboardTypeOwnerProto {
  #[objrs(selector = "pasteboard:provideDataForType:")]
  #[cfg(feature = "RK_AppKit")]
  fn pasteboard_provideDataForType_(&self, sender: &NSPasteboard, type_: &NSString) -> ();
  #[objrs(selector = "pasteboardChangedOwner:")]
  #[cfg(feature = "RK_AppKit")]
  fn pasteboardChangedOwner_(&self, sender: &NSPasteboard) -> ();
}
bitflags! { # [ repr ( C ) ] pub struct NSPasteboardWritingOptions : usize { const NSPasteboardWritingPromised = 512 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPasteboardWritingProto {
  #[objrs(selector = "writableTypesForPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn writableTypesForPasteboard_(&self, pasteboard: &NSPasteboard) -> Arc<NSArray>;
  #[objrs(selector = "writingOptionsForType:pasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn writingOptionsForType_pasteboard_(
    &self,
    type_: &NSString,
    pasteboard: &NSPasteboard,
  ) -> NSPasteboardWritingOptions;
  #[objrs(selector = "pasteboardPropertyListForType:")]
  #[cfg(feature = "RK_AppKit")]
  fn pasteboardPropertyListForType_(&self, type_: &NSString) -> Option<Arc<Object>>;
}
bitflags! { # [ repr ( C ) ] pub struct NSPasteboardReadingOptions : usize { const NSPasteboardReadingAsData = 0 ; const NSPasteboardReadingAsString = 1 ; const NSPasteboardReadingAsPropertyList = 2 ; const NSPasteboardReadingAsKeyedArchive = 4 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPasteboardReadingProto {
//   #[objrs(selector = "initWithPasteboardPropertyList:ofType:")]
//   #[cfg(feature = "RK_AppKit")]
//   fn newWithPasteboardPropertyList_ofType_(
//     propertyList: &Object,
//     type_: &NSString,
//   ) -> Option<Arc<Object>>;
}
bitflags! { # [ repr ( C ) ] pub struct NSDragOperation : usize { const NSDragOperationNone = 0 ; const NSDragOperationCopy = 1 ; const NSDragOperationLink = 2 ; const NSDragOperationGeneric = 4 ; const NSDragOperationPrivate = 8 ; const NSDragOperationMove = 16 ; const NSDragOperationDelete = 32 ; const NSDragOperationEvery = 18446744073709551615 ; const NSDragOperationAll_Obsolete = 15 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSDraggingFormation {
  NSDraggingFormationDefault = 0,
  NSDraggingFormationNone = 1,
  NSDraggingFormationPile = 2,
  NSDraggingFormationList = 3,
  NSDraggingFormationStack = 4,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSDraggingContext {
  NSDraggingContextOutsideApplication = 0,
  NSDraggingContextWithinApplication = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSDraggingItemEnumerationOptions : usize { const NSDraggingItemEnumerationConcurrent = 1 ; const NSDraggingItemEnumerationClearNonenumeratedImages = 65536 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSpringLoadingHighlight {
  NSSpringLoadingHighlightNone = 0,
  NSSpringLoadingHighlightStandard = 1,
  NSSpringLoadingHighlightEmphasized = 2,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDraggingInfoProto {
  #[objrs(selector = "slideDraggedImageTo:")]
  #[cfg(feature = "RK_AppKit")]
  fn slideDraggedImageTo_(&self, screenPoint: CGPoint) -> ();
  #[objrs(selector = "namesOfPromisedFilesDroppedAtDestination:")]
  #[cfg(feature = "RK_AppKit")]
  fn namesOfPromisedFilesDroppedAtDestination_(
    &self,
    dropDestination: &NSURL,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:")]
  #[cfg(feature = "RK_AppKit")]
  fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock_(
    &self,
    enumOpts: NSDraggingItemEnumerationOptions,
    view: Option<&NSView>,
    classArray: &NSArray,
    searchOptions: &NSDictionary,
    block: (),
  ) -> ();
  #[objrs(selector = "resetSpringLoading")]
  #[cfg(feature = "RK_AppKit")]
  fn resetSpringLoading(&self) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDraggingDestinationProto {
  #[objrs(selector = "draggingEntered:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingEntered_(&self, sender: &Object) -> NSDragOperation;
  #[objrs(selector = "draggingUpdated:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingUpdated_(&self, sender: &Object) -> NSDragOperation;
  #[objrs(selector = "draggingExited:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingExited_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "prepareForDragOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn prepareForDragOperation_(&self, sender: &Object) -> bool;
  #[objrs(selector = "performDragOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn performDragOperation_(&self, sender: &Object) -> bool;
  #[objrs(selector = "concludeDragOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn concludeDragOperation_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "draggingEnded:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingEnded_(&self, sender: &Object) -> ();
  #[objrs(selector = "wantsPeriodicDraggingUpdates")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsPeriodicDraggingUpdates(&self) -> bool;
  #[objrs(selector = "updateDraggingItemsForDrag:")]
  #[cfg(feature = "RK_AppKit")]
  fn updateDraggingItemsForDrag_(&self, sender: Option<&Object>) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDraggingSourceProto {
  #[objrs(selector = "draggingSession:sourceOperationMaskForDraggingContext:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingSession_sourceOperationMaskForDraggingContext_(
    &self,
    session: &NSDraggingSession,
    context: NSDraggingContext,
  ) -> NSDragOperation;
  #[objrs(selector = "draggingSession:willBeginAtPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingSession_willBeginAtPoint_(
    &self,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
  ) -> ();
  #[objrs(selector = "draggingSession:movedToPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingSession_movedToPoint_(&self, session: &NSDraggingSession, screenPoint: CGPoint) -> ();
  #[objrs(selector = "draggingSession:endedAtPoint:operation:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingSession_endedAtPoint_operation_(
    &self,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    operation: NSDragOperation,
  ) -> ();
  #[objrs(selector = "ignoreModifierKeysForDraggingSession:")]
  #[cfg(feature = "RK_AppKit")]
  fn ignoreModifierKeysForDraggingSession_(&self, session: &NSDraggingSession) -> bool;
}
bitflags! { # [ repr ( C ) ] pub struct NSSpringLoadingOptions : usize { const NSSpringLoadingDisabled = 0 ; const NSSpringLoadingEnabled = 1 ; const NSSpringLoadingContinuousActivation = 2 ; const NSSpringLoadingNoHover = 8 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSpringLoadingDestinationProto {
  #[objrs(selector = "springLoadingActivated:draggingInfo:")]
  #[cfg(feature = "RK_AppKit")]
  fn springLoadingActivated_draggingInfo_(&self, activated: bool, draggingInfo: &Object) -> ();
  #[objrs(selector = "springLoadingHighlightChanged:")]
  #[cfg(feature = "RK_AppKit")]
  fn springLoadingHighlightChanged_(&self, draggingInfo: &Object) -> ();
  #[objrs(selector = "springLoadingEntered:")]
  #[cfg(feature = "RK_AppKit")]
  fn springLoadingEntered_(&self, draggingInfo: &Object) -> NSSpringLoadingOptions;
  #[objrs(selector = "springLoadingUpdated:")]
  #[cfg(feature = "RK_AppKit")]
  fn springLoadingUpdated_(&self, draggingInfo: &Object) -> NSSpringLoadingOptions;
  #[objrs(selector = "springLoadingExited:")]
  #[cfg(feature = "RK_AppKit")]
  fn springLoadingExited_(&self, draggingInfo: &Object) -> ();
  #[objrs(selector = "draggingEnded:")]
  #[cfg(feature = "RK_AppKit")]
  fn draggingEnded_(&self, draggingInfo: &Object) -> ();
}
bitflags! { # [ repr ( C ) ] pub struct NSTouchPhase : usize { const NSTouchPhaseBegan = 1 ; const NSTouchPhaseMoved = 2 ; const NSTouchPhaseStationary = 4 ; const NSTouchPhaseEnded = 8 ; const NSTouchPhaseCancelled = 16 ; const NSTouchPhaseTouching = 7 ; const NSTouchPhaseAny = 18446744073709551615 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTouchType {
  NSTouchTypeDirect = 0,
  NSTouchTypeIndirect = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSTouchTypeMask : usize { const NSTouchTypeMaskDirect = 1 ; const NSTouchTypeMaskIndirect = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTouch;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSEventType {
  NSEventTypeLeftMouseDown = 1,
  NSEventTypeLeftMouseUp = 2,
  NSEventTypeRightMouseDown = 3,
  NSEventTypeRightMouseUp = 4,
  NSEventTypeMouseMoved = 5,
  NSEventTypeLeftMouseDragged = 6,
  NSEventTypeRightMouseDragged = 7,
  NSEventTypeMouseEntered = 8,
  NSEventTypeMouseExited = 9,
  NSEventTypeKeyDown = 10,
  NSEventTypeKeyUp = 11,
  NSEventTypeFlagsChanged = 12,
  NSEventTypeAppKitDefined = 13,
  NSEventTypeSystemDefined = 14,
  NSEventTypeApplicationDefined = 15,
  NSEventTypePeriodic = 16,
  NSEventTypeCursorUpdate = 17,
  NSEventTypeScrollWheel = 22,
  NSEventTypeTabletPoint = 23,
  NSEventTypeTabletProximity = 24,
  NSEventTypeOtherMouseDown = 25,
  NSEventTypeOtherMouseUp = 26,
  NSEventTypeOtherMouseDragged = 27,
  NSEventTypeGesture = 29,
  NSEventTypeMagnify = 30,
  NSEventTypeSwipe = 31,
  NSEventTypeRotate = 18,
  NSEventTypeBeginGesture = 19,
  NSEventTypeEndGesture = 20,
  NSEventTypeSmartMagnify = 32,
  NSEventTypeQuickLook = 33,
  NSEventTypePressure = 34,
  NSEventTypeDirectTouch = 37,
}
bitflags! { # [ repr ( C ) ] pub struct NSEventMask : u64 { const NSEventMaskLeftMouseDown = 2 ; const NSEventMaskLeftMouseUp = 4 ; const NSEventMaskRightMouseDown = 8 ; const NSEventMaskRightMouseUp = 16 ; const NSEventMaskMouseMoved = 32 ; const NSEventMaskLeftMouseDragged = 64 ; const NSEventMaskRightMouseDragged = 128 ; const NSEventMaskMouseEntered = 256 ; const NSEventMaskMouseExited = 512 ; const NSEventMaskKeyDown = 1024 ; const NSEventMaskKeyUp = 2048 ; const NSEventMaskFlagsChanged = 4096 ; const NSEventMaskAppKitDefined = 8192 ; const NSEventMaskSystemDefined = 16384 ; const NSEventMaskApplicationDefined = 32768 ; const NSEventMaskPeriodic = 65536 ; const NSEventMaskCursorUpdate = 131072 ; const NSEventMaskScrollWheel = 4194304 ; const NSEventMaskTabletPoint = 8388608 ; const NSEventMaskTabletProximity = 16777216 ; const NSEventMaskOtherMouseDown = 33554432 ; const NSEventMaskOtherMouseUp = 67108864 ; const NSEventMaskOtherMouseDragged = 134217728 ; const NSEventMaskGesture = 536870912 ; const NSEventMaskMagnify = 1073741824 ; const NSEventMaskSwipe = 2147483648 ; const NSEventMaskRotate = 262144 ; const NSEventMaskBeginGesture = 524288 ; const NSEventMaskEndGesture = 1048576 ; const NSEventMaskSmartMagnify = 4294967296 ; const NSEventMaskPressure = 17179869184 ; const NSEventMaskDirectTouch = 137438953472 ; const NSEventMaskAny = 18446744073709551615 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSEventModifierFlags : usize { const NSEventModifierFlagCapsLock = 65536 ; const NSEventModifierFlagShift = 131072 ; const NSEventModifierFlagControl = 262144 ; const NSEventModifierFlagOption = 524288 ; const NSEventModifierFlagCommand = 1048576 ; const NSEventModifierFlagNumericPad = 2097152 ; const NSEventModifierFlagHelp = 4194304 ; const NSEventModifierFlagFunction = 8388608 ; const NSEventModifierFlagDeviceIndependentFlagsMask = 4294901760 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPointingDeviceType {
  NSPointingDeviceTypeUnknown = 0,
  NSPointingDeviceTypePen = 1,
  NSPointingDeviceTypeCursor = 2,
  NSPointingDeviceTypeEraser = 3,
}
bitflags! { # [ repr ( C ) ] pub struct NSEventButtonMask : usize { const NSEventButtonMaskPenTip = 1 ; const NSEventButtonMaskPenLowerSide = 2 ; const NSEventButtonMaskPenUpperSide = 4 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSEventPhase : usize { const NSEventPhaseNone = 0 ; const NSEventPhaseBegan = 1 ; const NSEventPhaseStationary = 2 ; const NSEventPhaseChanged = 4 ; const NSEventPhaseEnded = 8 ; const NSEventPhaseCancelled = 16 ; const NSEventPhaseMayBegin = 32 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSEventGestureAxis {
  NSEventGestureAxisNone = 0,
  NSEventGestureAxisHorizontal = 1,
  NSEventGestureAxisVertical = 2,
}
bitflags! { # [ repr ( C ) ] pub struct NSEventSwipeTrackingOptions : usize { const NSEventSwipeTrackingLockDirection = 1 ; const NSEventSwipeTrackingClampGestureAmount = 2 ; } }
#[repr(i16)]
#[derive(Copy, Clone)]
pub enum NSEventSubtype {
  NSEventSubtypeWindowExposed = 0,
  NSEventSubtypeApplicationActivated = 1,
  NSEventSubtypeApplicationDeactivated = 2,
  NSEventSubtypeWindowMoved = 4,
  NSEventSubtypeScreenChanged = 8,
  NSEventSubtypeTouch = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPressureBehavior {
  NSPressureBehaviorUnknown = -1,
  NSPressureBehaviorPrimaryDefault = 0,
  NSPressureBehaviorPrimaryClick = 1,
  NSPressureBehaviorPrimaryGeneric = 2,
  NSPressureBehaviorPrimaryAccelerator = 3,
  NSPressureBehaviorPrimaryDeepClick = 5,
  NSPressureBehaviorPrimaryDeepDrag = 6,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSEvent;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSResponder;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSStandardKeyBindingRespondingProto {
  #[objrs(selector = "insertText:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertText_(&self, insertString: &Object) -> ();
  #[objrs(selector = "doCommandBySelector:")]
  #[cfg(feature = "RK_AppKit")]
  fn doCommandBySelector_(&self, selector: SelectorRef) -> ();
  #[objrs(selector = "moveForward:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveForward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveRight:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveRight_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveBackward:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveBackward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveLeft:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveLeft_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveUp_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveDown:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveDown_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordForward:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordForward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordBackward:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordBackward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfParagraph:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfParagraph_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfParagraph:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfParagraph_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfDocument:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfDocument_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfDocument:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfDocument_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "pageDown:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageDown_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "pageUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageUp_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "centerSelectionInVisibleArea:")]
  #[cfg(feature = "RK_AppKit")]
  fn centerSelectionInVisibleArea_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveBackwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveBackwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveForwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveForwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordForwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordForwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordBackwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordBackwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveUpAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveUpAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveDownAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveDownAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfLineAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfLineAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfLineAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfLineAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfParagraphAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfParagraphAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfParagraphAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfParagraphAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToEndOfDocumentAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToEndOfDocumentAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToBeginningOfDocumentAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToBeginningOfDocumentAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "pageDownAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageDownAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "pageUpAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageUpAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveParagraphForwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveParagraphForwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveParagraphBackwardAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveParagraphBackwardAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordRight:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordRight_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordLeft:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordLeft_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveRightAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveRightAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveLeftAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveLeftAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordRightAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordRightAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveWordLeftAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveWordLeftAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToLeftEndOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToLeftEndOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToRightEndOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToRightEndOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToLeftEndOfLineAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToLeftEndOfLineAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "moveToRightEndOfLineAndModifySelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn moveToRightEndOfLineAndModifySelection_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollPageUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollPageUp_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollPageDown:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollPageDown_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollLineUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollLineUp_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollLineDown:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollLineDown_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollToBeginningOfDocument:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollToBeginningOfDocument_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "scrollToEndOfDocument:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollToEndOfDocument_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "transpose:")]
  #[cfg(feature = "RK_AppKit")]
  fn transpose_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "transposeWords:")]
  #[cfg(feature = "RK_AppKit")]
  fn transposeWords_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "selectAll:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectAll_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "selectParagraph:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectParagraph_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "selectLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "selectWord:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectWord_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "indent:")]
  #[cfg(feature = "RK_AppKit")]
  fn indent_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertTab:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertTab_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertBacktab:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertBacktab_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertNewline:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertNewline_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertParagraphSeparator:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertParagraphSeparator_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertNewlineIgnoringFieldEditor:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertNewlineIgnoringFieldEditor_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertTabIgnoringFieldEditor:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertTabIgnoringFieldEditor_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertLineBreak:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertLineBreak_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertContainerBreak:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertContainerBreak_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertSingleQuoteIgnoringSubstitution:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertSingleQuoteIgnoringSubstitution_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "insertDoubleQuoteIgnoringSubstitution:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertDoubleQuoteIgnoringSubstitution_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "changeCaseOfLetter:")]
  #[cfg(feature = "RK_AppKit")]
  fn changeCaseOfLetter_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "uppercaseWord:")]
  #[cfg(feature = "RK_AppKit")]
  fn uppercaseWord_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "lowercaseWord:")]
  #[cfg(feature = "RK_AppKit")]
  fn lowercaseWord_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "capitalizeWord:")]
  #[cfg(feature = "RK_AppKit")]
  fn capitalizeWord_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteForward:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteForward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteBackward:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteBackward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteBackwardByDecomposingPreviousCharacter:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteBackwardByDecomposingPreviousCharacter_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteWordForward:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteWordForward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteWordBackward:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteWordBackward_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteToBeginningOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteToBeginningOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteToEndOfLine:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteToEndOfLine_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteToBeginningOfParagraph:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteToBeginningOfParagraph_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteToEndOfParagraph:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteToEndOfParagraph_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "yank:")]
  #[cfg(feature = "RK_AppKit")]
  fn yank_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "complete:")]
  #[cfg(feature = "RK_AppKit")]
  fn complete_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "setMark:")]
  #[cfg(feature = "RK_AppKit")]
  fn setMark_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "deleteToMark:")]
  #[cfg(feature = "RK_AppKit")]
  fn deleteToMark_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "selectToMark:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectToMark_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "swapWithMark:")]
  #[cfg(feature = "RK_AppKit")]
  fn swapWithMark_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "cancelOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn cancelOperation_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeBaseWritingDirectionNatural:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeBaseWritingDirectionNatural_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeBaseWritingDirectionLeftToRight:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeBaseWritingDirectionLeftToRight_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeBaseWritingDirectionRightToLeft:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeBaseWritingDirectionRightToLeft_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeTextWritingDirectionNatural:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeTextWritingDirectionNatural_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeTextWritingDirectionLeftToRight:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeTextWritingDirectionLeftToRight_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "makeTextWritingDirectionRightToLeft:")]
  #[cfg(feature = "RK_AppKit")]
  fn makeTextWritingDirectionRightToLeft_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "quickLookPreviewItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn quickLookPreviewItems_(&self, sender: Option<&Object>) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSUserInterfaceItemIdentificationProto {}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSUserInterfaceLayoutDirection {
  NSUserInterfaceLayoutDirectionLeftToRight = 0,
  NSUserInterfaceLayoutDirectionRightToLeft = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSUserInterfaceLayoutOrientation {
  NSUserInterfaceLayoutOrientationHorizontal = 0,
  NSUserInterfaceLayoutOrientationVertical = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSAutoresizingMaskOptions : usize { const NSViewNotSizable = 0 ; const NSViewMinXMargin = 1 ; const NSViewWidthSizable = 2 ; const NSViewMaxXMargin = 4 ; const NSViewMinYMargin = 8 ; const NSViewHeightSizable = 16 ; const NSViewMaxYMargin = 32 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBorderType {
  NSNoBorder = 0,
  NSLineBorder = 1,
  NSBezelBorder = 2,
  NSGrooveBorder = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSViewLayerContentsRedrawPolicy {
  NSViewLayerContentsRedrawNever = 0,
  NSViewLayerContentsRedrawOnSetNeedsDisplay = 1,
  NSViewLayerContentsRedrawDuringViewResize = 2,
  NSViewLayerContentsRedrawBeforeViewResize = 3,
  NSViewLayerContentsRedrawCrossfade = 4,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSViewLayerContentsPlacement {
  NSViewLayerContentsPlacementScaleAxesIndependently = 0,
  NSViewLayerContentsPlacementScaleProportionallyToFit = 1,
  NSViewLayerContentsPlacementScaleProportionallyToFill = 2,
  NSViewLayerContentsPlacementCenter = 3,
  NSViewLayerContentsPlacementTop = 4,
  NSViewLayerContentsPlacementTopRight = 5,
  NSViewLayerContentsPlacementRight = 6,
  NSViewLayerContentsPlacementBottomRight = 7,
  NSViewLayerContentsPlacementBottom = 8,
  NSViewLayerContentsPlacementBottomLeft = 9,
  NSViewLayerContentsPlacementLeft = 10,
  NSViewLayerContentsPlacementTopLeft = 11,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __VFlags {
  pub aboutToResize: u32,
  pub isOpaque: u32,
  pub unused3: u32,
  pub hasNotMessedWithIsFlipped: u32,
  pub ignoreHitTest: u32,
  pub specialArchiving: u32,
  pub needsDisplayForBounds: u32,
  pub isFlipped: u32,
  pub removingWithoutInvalidation: u32,
  pub needsBoundsChangeNote: u32,
  pub boundsChangeNotesSuspended: u32,
  pub unused2: u32,
  pub needsFrameChangeNote: u32,
  pub frameChangeNotesSuspended: u32,
  pub canDrawSubviewsIntoLayer: u32,
  pub allowsVibrancy: u32,
  pub needsDisplay: u32,
  pub unused1: u32,
  pub autoresizeSubviews: u32,
  pub autosizing: u32,
  pub rotatedOrScaledFromBase: u32,
  pub rotatedFromBase: u32,
}
pub type _VFlags = __VFlags;
pub type NSTrackingRectTag = isize;
pub type NSToolTipTag = isize;
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSView;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __VFlags2 {
  pub nextKeyViewRefCount: u32,
  pub previousKeyViewRefCount: u32,
  pub isVisibleRect: u32,
  pub hasToolTip: u32,
  pub cachedIsFlipped: u32,
  pub menuWasSet: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSViewLayerContentScaleDelegateProto {
  #[objrs(selector = "layer:shouldInheritContentsScale:fromWindow:")]
  #[cfg(feature = "RK_AppKit")]
  fn layer_shouldInheritContentsScale_fromWindow_(
    &self,
    layer: &CALayer,
    newScale: f64,
    window: &NSWindow,
  ) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSViewToolTipOwnerProto {
  #[objrs(selector = "view:stringForToolTip:point:userData:")]
  #[cfg(feature = "RK_AppKit")]
  fn view_stringForToolTip_point_userData_(
    &self,
    view: &NSView,
    tag: isize,
    point: CGPoint,
    data: Option<&c_void>,
  ) -> Arc<NSString>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSChangeSpellingProto {
  #[objrs(selector = "changeSpelling:")]
  #[cfg(feature = "RK_AppKit")]
  fn changeSpelling_(&self, sender: Option<&Object>) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSIgnoreMisspelledWordsProto {
  #[objrs(selector = "ignoreSpelling:")]
  #[cfg(feature = "RK_AppKit")]
  fn ignoreSpelling_(&self, sender: Option<&Object>) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextAlignment {
  NSTextAlignmentLeft = 0,
  NSTextAlignmentRight = 1,
  NSTextAlignmentCenter = 2,
  NSTextAlignmentJustified = 3,
  NSTextAlignmentNatural = 4,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWritingDirection {
  NSWritingDirectionNatural = -1,
  NSWritingDirectionLeftToRight = 0,
  NSWritingDirectionRightToLeft = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTextMovement {
  NSTextMovementReturn = 16,
  NSTextMovementTab = 17,
  NSTextMovementBacktab = 18,
  NSTextMovementLeft = 19,
  NSTextMovementRight = 20,
  NSTextMovementUp = 21,
  NSTextMovementDown = 22,
  NSTextMovementCancel = 23,
  NSTextMovementOther = 0,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSText;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextDelegateProto {
  #[objrs(selector = "textShouldBeginEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn textShouldBeginEditing_(&self, textObject: &NSText) -> bool;
  #[objrs(selector = "textShouldEndEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn textShouldEndEditing_(&self, textObject: &NSText) -> bool;
  #[objrs(selector = "textDidBeginEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn textDidBeginEditing_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "textDidEndEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn textDidEndEditing_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "textDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textDidChange_(&self, notification: &NSNotification) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextTab;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLineBreakMode {
  NSLineBreakByWordWrapping = 0,
  NSLineBreakByCharWrapping = 1,
  NSLineBreakByClipping = 2,
  NSLineBreakByTruncatingHead = 3,
  NSLineBreakByTruncatingTail = 4,
  NSLineBreakByTruncatingMiddle = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSParagraphStyle;
# [ objrs ( class , super = NSParagraphStyle ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMutableParagraphStyle;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextTabType {
  NSLeftTabStopType = 0,
  NSRightTabStopType = 1,
  NSCenterTabStopType = 2,
  NSDecimalTabStopType = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCellType {
  NSNullCellType = 0,
  NSTextCellType = 1,
  NSImageCellType = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCellAttribute {
  NSCellDisabled = 0,
  NSCellState = 1,
  NSPushInCell = 2,
  NSCellEditable = 3,
  NSChangeGrayCell = 4,
  NSCellHighlighted = 5,
  NSCellLightsByContents = 6,
  NSCellLightsByGray = 7,
  NSChangeBackgroundCell = 8,
  NSCellLightsByBackground = 9,
  NSCellIsBordered = 10,
  NSCellHasOverlappingImage = 11,
  NSCellHasImageHorizontal = 12,
  NSCellHasImageOnLeftOrBottom = 13,
  NSCellChangesContents = 14,
  NSCellIsInsetButton = 15,
  NSCellAllowsMixedState = 16,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCellImagePosition {
  NSNoImage = 0,
  NSImageOnly = 1,
  NSImageLeft = 2,
  NSImageRight = 3,
  NSImageBelow = 4,
  NSImageAbove = 5,
  NSImageOverlaps = 6,
  NSImageLeading = 7,
  NSImageTrailing = 8,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageScaling {
  NSImageScaleProportionallyDown = 0,
  NSImageScaleAxesIndependently = 1,
  NSImageScaleNone = 2,
  NSImageScaleProportionallyUpOrDown = 3,
}
pub type NSControlStateValue = isize;
pub type NSCellStateValue = isize;
bitflags! { # [ repr ( C ) ] pub struct NSCellStyleMask : usize { const NSNoCellMask = 0 ; const NSContentsCellMask = 1 ; const NSPushInCellMask = 2 ; const NSChangeGrayCellMask = 4 ; const NSChangeBackgroundCellMask = 8 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSControlTint {
  NSDefaultControlTint = 0,
  NSBlueControlTint = 1,
  NSGraphiteControlTint = 6,
  NSClearControlTint = 7,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSControlSize {
  NSControlSizeRegular = 0,
  NSControlSizeSmall = 1,
  NSControlSizeMini = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CFlags {
  pub state: u32,
  pub highlighted: u32,
  pub disabled: u32,
  pub editable: u32,
  pub type_: NSCellType,
  pub vCentered: u32,
  pub hCentered: u32,
  pub bordered: u32,
  pub bezeled: u32,
  pub selectable: u32,
  pub scrollable: u32,
  pub continuous: u32,
  pub actOnMouseDown: u32,
  pub isLeaf: u32,
  pub invalidObjectValue: u32,
  pub invalidFont: u32,
  pub lineBreakMode: NSLineBreakMode,
  pub __reserved: u32,
  pub allowsAppearanceEffects: u32,
  pub singleLineMode: u32,
  pub actOnMouseDragged: u32,
  pub isLoaded: u32,
  pub truncateLastLine: u32,
  pub dontActOnMouseUp: u32,
  pub isWhite: u32,
  pub useUserKeyEquivalent: u32,
  pub showsFirstResponder: u32,
  pub focusRingType: u32,
  pub wasSelectable: u32,
  pub hasInvalidObject: u32,
  pub allowsEditingTextAttributes: u32,
  pub importsGraphics: u32,
  pub alignment: NSTextAlignment,
  pub layoutDirectionRTL: u32,
  pub backgroundStyle: u32,
  pub cellReserved2: u32,
  pub refusesFirstResponder: u32,
  pub needsHighlightedText: u32,
  pub dontAllowsUndo: u32,
  pub currentlyEditing: u32,
  pub allowsMixedState: u32,
  pub inMixedState: u32,
  pub sendsActionOnEndEditing: u32,
  pub inSendAction: u32,
  pub menuWasSet: u32,
  pub controlTint: u32,
  pub controlSize: u32,
  pub branchImageDisabled: u32,
  pub drawingInRevealover: u32,
  pub needsHighlightedTextHint: u32,
}
pub type _CFlags = __CFlags;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCell;
bitflags! { # [ repr ( C ) ] pub struct NSCellHitResult : usize { const NSCellHitNone = 0 ; const NSCellHitContentArea = 1 ; const NSCellHitEditableTextArea = 2 ; const NSCellHitTrackableArea = 4 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSBackgroundStyle {
  NSBackgroundStyleNormal = 0,
  NSBackgroundStyleEmphasized = 1,
  NSBackgroundStyleRaised = 2,
  NSBackgroundStyleLowered = 3,
}
# [ objrs ( class , super = NSCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSActionCell;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSValidatedUserInterfaceItemProto {}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSUserInterfaceValidationsProto {
  #[objrs(selector = "validateUserInterfaceItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn validateUserInterfaceItem_(&self, item: &Object) -> bool;
}
bitflags! { # [ repr ( C ) ] pub struct NSApplicationActivationOptions : usize { const NSApplicationActivateAllWindows = 1 ; const NSApplicationActivateIgnoringOtherApps = 2 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSApplicationActivationPolicy {
  NSApplicationActivationPolicyRegular = 0,
  NSApplicationActivationPolicyAccessory = 1,
  NSApplicationActivationPolicyProhibited = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSRunningApplication;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSNib;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _NSNibFlags {
  pub _isKeyed: u32,
  pub _inheritsDecodeTimeBundle: u32,
  pub _inheritsDecodeTimePath: u32,
  pub _reserved: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMenuItem;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __miFlags {
  pub keGenerationCount: u32,
  pub disabled: u32,
  pub isSeparator: u32,
  pub hidden: u32,
  pub alternate: u32,
  pub moreAlternate: u32,
  pub singleAlternate: u32,
  pub indent: u32,
  pub keShareMode: u32,
  pub state: u32,
  pub destructive: u32,
  pub RESERVED1: u32,
  pub limitedView: u32,
  pub nextItemIsAlternate: u32,
  pub blockKE: u32,
  pub ignoredForAccessibility: u32,
  pub hiddenActiveKE: u32,
  pub noRepeatKEs: u32,
  pub targetWeak: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMenu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mFlags {
  pub noAutoenable: u32,
  pub inMain: u32,
  pub internalPerformAction: u32,
  pub condenseSeparators: u32,
  pub disabled: u32,
  pub ownedByPopUp: u32,
  pub delegateNeedsUpdate: u32,
  pub delegateUpdateItem: u32,
  pub delegateHasKeyEquiv: u32,
  pub delegateHasAltKeyEquiv: u32,
  pub excludeMarkColumn: u32,
  pub isContextualMenu: u32,
  pub cmPluginMode: u32,
  pub invertedCMPluginTypes: u32,
  pub allowsDifferentSelection: u32,
  pub noTopPadding: u32,
  pub noBottomPadding: u32,
  pub hasNCStyle: u32,
  pub delegateIsUnsafeUnretained: u32,
  pub avoidUsingCache: u32,
  pub RESERVED: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSMenuItemValidationProto {
  #[objrs(selector = "validateMenuItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn validateMenuItem_(&self, menuItem: &NSMenuItem) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSMenuDelegateProto {
  #[objrs(selector = "menuNeedsUpdate:")]
  #[cfg(feature = "RK_AppKit")]
  fn menuNeedsUpdate_(&self, menu: &NSMenu) -> ();
  #[objrs(selector = "numberOfItemsInMenu:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfItemsInMenu_(&self, menu: &NSMenu) -> isize;
  #[objrs(selector = "menu:updateItem:atIndex:shouldCancel:")]
  #[cfg(feature = "RK_AppKit")]
  fn menu_updateItem_atIndex_shouldCancel_(
    &self,
    menu: &NSMenu,
    item: &NSMenuItem,
    index: isize,
    shouldCancel: bool,
  ) -> bool;
  #[objrs(selector = "menuHasKeyEquivalent:forEvent:target:action:")]
  #[cfg(feature = "RK_AppKit")]
  fn menuHasKeyEquivalent_forEvent_target_action_(
    &self,
    menu: &NSMenu,
    event: &NSEvent,
    target: &mut &Option<Arc<Object>>,
    action: &SelectorRef,
  ) -> bool;
  #[objrs(selector = "menuWillOpen:")]
  #[cfg(feature = "RK_AppKit")]
  fn menuWillOpen_(&self, menu: &NSMenu) -> ();
  #[objrs(selector = "menuDidClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn menuDidClose_(&self, menu: &NSMenu) -> ();
  #[objrs(selector = "menu:willHighlightItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn menu_willHighlightItem_(&self, menu: &NSMenu, item: Option<&NSMenuItem>) -> ();
  #[objrs(selector = "confinementRectForMenu:onScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn confinementRectForMenu_onScreen_(&self, menu: &NSMenu, screen: Option<&NSScreen>) -> CGRect;
}
bitflags! { # [ repr ( C ) ] pub struct NSMenuProperties : usize { const NSMenuPropertyItemTitle = 1 ; const NSMenuPropertyItemAttributedTitle = 2 ; const NSMenuPropertyItemKeyEquivalent = 4 ; const NSMenuPropertyItemImage = 8 ; const NSMenuPropertyItemEnabled = 16 ; const NSMenuPropertyItemAccessibilityDescription = 32 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPrinterTableStatus {
  NSPrinterTableOK = 0,
  NSPrinterTableNotFound = 1,
  NSPrinterTableError = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPrinter;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPaperOrientation {
  NSPaperOrientationPortrait = 0,
  NSPaperOrientationLandscape = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPrintingPaginationMode {
  NSPrintingPaginationModeAutomatic = 0,
  NSPrintingPaginationModeFit = 1,
  NSPrintingPaginationModeClip = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPrintInfo;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPrintingOrientation {
  NSPortraitOrientation = 0,
  NSLandscapeOrientation = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBindingSelectionMarker;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSEditorProto {
  #[objrs(selector = "discardEditing")]
  #[cfg(feature = "RK_AppKit")]
  fn discardEditing(&self) -> ();
  #[objrs(selector = "commitEditing")]
  #[cfg(feature = "RK_AppKit")]
  fn commitEditing(&self) -> bool;
  #[objrs(selector = "commitEditingWithDelegate:didCommitSelector:contextInfo:")]
  #[cfg(feature = "RK_AppKit")]
  fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
    &self,
    delegate: Option<&Object>,
    didCommitSelector: SelectorRef,
    contextInfo: Option<&c_void>,
  ) -> ();
  #[objrs(selector = "commitEditingAndReturnError:")]
  #[cfg(feature = "RK_AppKit")]
  fn commitEditingAndReturnError_(&self, error: Option<&mut &Option<Arc<NSError>>>) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSEditorRegistrationProto {
  #[objrs(selector = "objectDidBeginEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn objectDidBeginEditing_(&self, editor: &Object) -> ();
  #[objrs(selector = "objectDidEndEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn objectDidEndEditing_(&self, editor: &Object) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDocumentChangeType {
  NSChangeDone = 0,
  NSChangeUndone = 1,
  NSChangeRedone = 5,
  NSChangeCleared = 2,
  NSChangeReadOtherContents = 3,
  NSChangeAutosaved = 4,
  NSChangeDiscardable = 256,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSaveOperationType {
  NSSaveOperation = 0,
  NSSaveAsOperation = 1,
  NSSaveToOperation = 2,
  NSAutosaveInPlaceOperation = 4,
  NSAutosaveElsewhereOperation = 3,
  NSAutosaveAsOperation = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDocument;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __docFlags {
  pub inClose: u32,
  pub hasUndoManager: u32,
  pub unused: u32,
  pub reconciledToFileName: u32,
  pub checkingDisplayName: u32,
  pub hasInvalidRestorableState: u32,
  pub hasEverHadInvalidRestorableState: u32,
  pub RESERVED: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSUserActivityRestoringProto {
  #[objrs(selector = "restoreUserActivityState:")]
  #[cfg(feature = "RK_AppKit")]
  fn restoreUserActivityState_(&self, userActivity: &NSUserActivity) -> ();
}
pub type NSModalResponse = isize;
bitflags! { # [ repr ( C ) ] pub struct NSApplicationPresentationOptions : usize { const NSApplicationPresentationDefault = 0 ; const NSApplicationPresentationAutoHideDock = 1 ; const NSApplicationPresentationHideDock = 2 ; const NSApplicationPresentationAutoHideMenuBar = 4 ; const NSApplicationPresentationHideMenuBar = 8 ; const NSApplicationPresentationDisableAppleMenu = 16 ; const NSApplicationPresentationDisableProcessSwitching = 32 ; const NSApplicationPresentationDisableForceQuit = 64 ; const NSApplicationPresentationDisableSessionTermination = 128 ; const NSApplicationPresentationDisableHideApplication = 256 ; const NSApplicationPresentationDisableMenuBarTransparency = 512 ; const NSApplicationPresentationFullScreen = 1024 ; const NSApplicationPresentationAutoHideToolbar = 2048 ; const NSApplicationPresentationDisableCursorLocationAssistance = 4096 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSApplicationOcclusionState : usize { const NSApplicationOcclusionStateVisible = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSWindowListOptions : isize { const NSWindowListOrderedFrontToBack = 1 ; } }
#[repr(C)]
pub struct _NSModalSession {
  opaque: u32,
}
#[repr(C)]
pub struct NSThreadPrivate {
  opaque: u32,
}
pub type _NSThreadPrivate = NSThreadPrivate;
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSApplication;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __appFlags {
  pub _hidden: u32,
  pub _appleEventActivationInProgress: u32,
  pub _active: u32,
  pub _hasBeenRun: u32,
  pub _doingUnhide: u32,
  pub _delegateReturnsValidRequestor: u32,
  pub _deactPending: u32,
  pub _invalidState: u32,
  pub _invalidEvent: u32,
  pub _postedWindowsNeedUpdateNote: u32,
  pub _wantsToActivate: u32,
  pub _doingHide: u32,
  pub _dontSendShouldTerminate: u32,
  pub _ignoresFullScreen: u32,
  pub _finishedLaunching: u32,
  pub _hasEventDelegate: u32,
  pub _appDying: u32,
  pub _didNSOpenOrPrint: u32,
  pub _inDealloc: u32,
  pub _pendingDidFinish: u32,
  pub _hasKeyFocus: u32,
  pub _panelsNonactivating: u32,
  pub _hiddenOnLaunch: u32,
  pub _openStatus: u32,
  pub _batchOrdering: u32,
  pub _waitingForTerminationReply: u32,
  pub _windowMoveDisabled: u32,
  pub _enumeratingMemoryPressureHandlers: u32,
  pub _didTryRestoringPersistentState: u32,
  pub _reservedN: u32,
  pub _mightBeSwitching: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRequestUserAttentionType {
  NSCriticalRequest = 0,
  NSInformationalRequest = 10,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSApplicationDelegateReply {
  NSApplicationDelegateReplySuccess = 0,
  NSApplicationDelegateReplyCancel = 1,
  NSApplicationDelegateReplyFailure = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSApplicationTerminateReply {
  NSTerminateCancel = 0,
  NSTerminateNow = 1,
  NSTerminateLater = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSApplicationPrintReply {
  NSPrintingCancelled = 0,
  NSPrintingSuccess = 1,
  NSPrintingFailure = 3,
  NSPrintingReplyLater = 2,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSApplicationDelegateProto {
  #[objrs(selector = "applicationShouldTerminate:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationShouldTerminate_(&self, sender: &NSApplication) -> NSApplicationTerminateReply;
  #[objrs(selector = "application:openURLs:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_openURLs_(&self, application: &NSApplication, urls: &NSArray) -> ();
  #[objrs(selector = "application:openFile:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_openFile_(&self, sender: &NSApplication, filename: &NSString) -> bool;
  #[objrs(selector = "application:openFiles:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_openFiles_(&self, sender: &NSApplication, filenames: &NSArray) -> ();
  #[objrs(selector = "application:openTempFile:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_openTempFile_(&self, sender: &NSApplication, filename: &NSString) -> bool;
  #[objrs(selector = "applicationShouldOpenUntitledFile:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationShouldOpenUntitledFile_(&self, sender: &NSApplication) -> bool;
  #[objrs(selector = "applicationOpenUntitledFile:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationOpenUntitledFile_(&self, sender: &NSApplication) -> bool;
  #[objrs(selector = "application:openFileWithoutUI:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_openFileWithoutUI_(&self, sender: &Object, filename: &NSString) -> bool;
  #[objrs(selector = "application:printFile:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_printFile_(&self, sender: &NSApplication, filename: &NSString) -> bool;
  #[objrs(selector = "application:printFiles:withSettings:showPrintPanels:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_printFiles_withSettings_showPrintPanels_(
    &self,
    application: &NSApplication,
    fileNames: &NSArray,
    printSettings: &NSDictionary,
    showPrintPanels: bool,
  ) -> NSApplicationPrintReply;
  #[objrs(selector = "applicationShouldTerminateAfterLastWindowClosed:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationShouldTerminateAfterLastWindowClosed_(&self, sender: &NSApplication) -> bool;
  #[objrs(selector = "applicationShouldHandleReopen:hasVisibleWindows:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationShouldHandleReopen_hasVisibleWindows_(
    &self,
    sender: &NSApplication,
    flag: bool,
  ) -> bool;
  #[objrs(selector = "applicationDockMenu:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDockMenu_(&self, sender: &NSApplication) -> Option<Arc<NSMenu>>;
  #[objrs(selector = "application:willPresentError:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_willPresentError_(
    &self,
    application: &NSApplication,
    error: &NSError,
  ) -> Arc<NSError>;
  #[objrs(selector = "application:didRegisterForRemoteNotificationsWithDeviceToken:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didRegisterForRemoteNotificationsWithDeviceToken_(
    &self,
    application: &NSApplication,
    deviceToken: &NSData,
  ) -> ();
  #[objrs(selector = "application:didFailToRegisterForRemoteNotificationsWithError:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didFailToRegisterForRemoteNotificationsWithError_(
    &self,
    application: &NSApplication,
    error: &NSError,
  ) -> ();
  #[objrs(selector = "application:didReceiveRemoteNotification:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didReceiveRemoteNotification_(
    &self,
    application: &NSApplication,
    userInfo: &NSDictionary,
  ) -> ();
  #[objrs(selector = "application:willEncodeRestorableState:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_willEncodeRestorableState_(&self, app: &NSApplication, coder: &NSCoder) -> ();
  #[objrs(selector = "application:didDecodeRestorableState:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didDecodeRestorableState_(&self, app: &NSApplication, coder: &NSCoder) -> ();
  #[objrs(selector = "application:willContinueUserActivityWithType:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_willContinueUserActivityWithType_(
    &self,
    application: &NSApplication,
    userActivityType: &NSString,
  ) -> bool;
  #[objrs(selector = "application:continueUserActivity:restorationHandler:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_continueUserActivity_restorationHandler_(
    &self,
    application: &NSApplication,
    userActivity: &NSUserActivity,
    restorationHandler: (),
  ) -> bool;
  #[objrs(selector = "application:didFailToContinueUserActivityWithType:error:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didFailToContinueUserActivityWithType_error_(
    &self,
    application: &NSApplication,
    userActivityType: &NSString,
    error: &NSError,
  ) -> ();
  #[objrs(selector = "application:didUpdateUserActivity:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_didUpdateUserActivity_(
    &self,
    application: &NSApplication,
    userActivity: &NSUserActivity,
  ) -> ();
  #[objrs(selector = "application:delegateHandlesKey:")]
  #[cfg(feature = "RK_AppKit")]
  fn application_delegateHandlesKey_(&self, sender: &NSApplication, key: &NSString) -> bool;
  #[objrs(selector = "applicationWillFinishLaunching:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillFinishLaunching_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidFinishLaunching:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidFinishLaunching_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillHide:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillHide_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidHide:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidHide_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillUnhide:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillUnhide_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidUnhide:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidUnhide_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillBecomeActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillBecomeActive_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidBecomeActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidBecomeActive_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillResignActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillResignActive_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidResignActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidResignActive_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillUpdate:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillUpdate_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidUpdate:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidUpdate_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationWillTerminate:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationWillTerminate_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidChangeScreenParameters:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidChangeScreenParameters_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "applicationDidChangeOcclusionState:")]
  #[cfg(feature = "RK_AppKit")]
  fn applicationDidChangeOcclusionState_(&self, notification: &NSNotification) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSServicesMenuRequestorProto {
  #[objrs(selector = "writeSelectionToPasteboard:types:")]
  #[cfg(feature = "RK_AppKit")]
  fn writeSelectionToPasteboard_types_(&self, pboard: &NSPasteboard, types: &NSArray) -> bool;
  #[objrs(selector = "readSelectionFromPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn readSelectionFromPasteboard_(&self, pboard: &NSPasteboard) -> bool;
}
bitflags! { # [ repr ( C ) ] pub struct NSRemoteNotificationType : usize { const NSRemoteNotificationTypeNone = 0 ; const NSRemoteNotificationTypeBadge = 1 ; const NSRemoteNotificationTypeSound = 2 ; const NSRemoteNotificationTypeAlert = 4 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSHelpManager;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSAlertStyle {
  NSAlertStyleWarning = 0,
  NSAlertStyleInformational = 1,
  NSAlertStyleCritical = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAlert;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAlertDelegateProto {
  #[objrs(selector = "alertShowHelp:")]
  #[cfg(feature = "RK_AppKit")]
  fn alertShowHelp_(&self, alert: &NSAlert) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAnimationContext;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTitlePosition {
  NSNoTitle = 0,
  NSAboveTop = 1,
  NSAtTop = 2,
  NSBelowTop = 3,
  NSAboveBottom = 4,
  NSAtBottom = 5,
  NSBelowBottom = 6,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBoxType {
  NSBoxPrimary = 0,
  NSBoxSeparator = 2,
  NSBoxCustom = 4,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBox;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bFlags {
  pub borderType: NSBorderType,
  pub titlePosition: NSTitlePosition,
  pub backgroundTransparent: u32,
  pub orientation: u32,
  pub needsTile: u32,
  pub transparent: u32,
  pub colorAltInterpretation: u32,
  pub boxType: u32,
  pub useSuperAddSubview: u32,
  pub _RESERVED: u32,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSControl;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __conFlags {
  pub enabled: u32,
  pub ignoreMultiClick: u32,
  pub calcSize: u32,
  pub drawingAncestor: u32,
  pub ibReserved: u32,
  pub updateCellFocus: u32,
  pub allowsLogicalLayoutDirection: u32,
  pub asmlwidth: u32,
  pub hsmlwidth: u32,
  pub dontValidate: u32,
  pub reserved: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSControlTextEditingDelegateProto {
  #[objrs(selector = "controlTextDidBeginEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn controlTextDidBeginEditing_(&self, obj: &NSNotification) -> ();
  #[objrs(selector = "controlTextDidEndEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn controlTextDidEndEditing_(&self, obj: &NSNotification) -> ();
  #[objrs(selector = "controlTextDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn controlTextDidChange_(&self, obj: &NSNotification) -> ();
  #[objrs(selector = "control:textShouldBeginEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_textShouldBeginEditing_(&self, control: &NSControl, fieldEditor: &NSText) -> bool;
  #[objrs(selector = "control:textShouldEndEditing:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_textShouldEndEditing_(&self, control: &NSControl, fieldEditor: &NSText) -> bool;
  #[objrs(selector = "control:didFailToFormatString:errorDescription:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_didFailToFormatString_errorDescription_(
    &self,
    control: &NSControl,
    string: &NSString,
    error: Option<&NSString>,
  ) -> bool;
  #[objrs(selector = "control:didFailToValidatePartialString:errorDescription:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_didFailToValidatePartialString_errorDescription_(
    &self,
    control: &NSControl,
    string: &NSString,
    error: Option<&NSString>,
  ) -> ();
  #[objrs(selector = "control:isValidObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_isValidObject_(&self, control: &NSControl, obj: Option<&Object>) -> bool;
  #[objrs(selector = "control:textView:doCommandBySelector:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_textView_doCommandBySelector_(
    &self,
    control: &NSControl,
    textView: &NSTextView,
    commandSelector: SelectorRef,
  ) -> bool;
  #[objrs(selector = "control:textView:completions:forPartialWordRange:indexOfSelectedItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn control_textView_completions_forPartialWordRange_indexOfSelectedItem_(
    &self,
    control: &NSControl,
    textView: &NSTextView,
    words: &NSArray,
    charRange: NSRange,
    index: &isize,
  ) -> Arc<NSArray>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSButtonType {
  NSButtonTypeMomentaryLight = 0,
  NSButtonTypePushOnPushOff = 1,
  NSButtonTypeToggle = 2,
  NSButtonTypeSwitch = 3,
  NSButtonTypeRadio = 4,
  NSButtonTypeMomentaryChange = 5,
  NSButtonTypeOnOff = 6,
  NSButtonTypeMomentaryPushIn = 7,
  NSButtonTypeAccelerator = 8,
  NSButtonTypeMultiLevelAccelerator = 9,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBezelStyle {
  NSBezelStyleRounded = 1,
  NSBezelStyleRegularSquare = 2,
  NSBezelStyleDisclosure = 5,
  NSBezelStyleShadowlessSquare = 6,
  NSBezelStyleCircular = 7,
  NSBezelStyleTexturedSquare = 8,
  NSBezelStyleHelpButton = 9,
  NSBezelStyleSmallSquare = 10,
  NSBezelStyleTexturedRounded = 11,
  NSBezelStyleRoundRect = 12,
  NSBezelStyleRecessed = 13,
  NSBezelStyleRoundedDisclosure = 14,
  NSBezelStyleInline = 15,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __BCFlags {
  pub __reserved: u32,
  pub calculatingPreferredAppearance: u32,
  pub alwaysRadioExclusive: u32,
  pub leadingOrTrailing: u32,
  pub shouldNotHighlightOnPerformClick: u32,
  pub imageHugsTitle: u32,
  pub allowTitleTightening: u32,
  pub isDrawingFocus: u32,
  pub useButtonImageSource: u32,
  pub hasTitleTextField: u32,
  pub isDrawingDisclosure: u32,
  pub suppressAXValueChangeNote: u32,
  pub doesNotDimImage: u32,
  pub inset: u32,
  pub transparent: u32,
  pub inIntermediateDisclosure: u32,
  pub hasKeyEquivalentInsteadOfImage: u32,
  pub imageSizeDiff: u32,
  pub imageAndText: u32,
  pub bottomOrLeft: u32,
  pub horizontal: u32,
  pub imageOverlaps: u32,
  pub bordered: u32,
  pub drawing: u32,
  pub lightByGray: u32,
  pub lightByBackground: u32,
  pub lightByContents: u32,
  pub changeGray: u32,
  pub changeBackground: u32,
  pub changeContents: u32,
  pub pushIn: u32,
}
pub type _BCFlags = __BCFlags;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __BCFlags2 {
  pub bezelStyle: u32,
  pub showsBorderOnlyWhileMouseInside: u32,
  pub mouseInside: u32,
  pub bezelStyle2: u32,
  pub imageScaling: u32,
  pub keyEquivalentModifierMask: u32,
}
pub type _BCFlags2 = __BCFlags2;
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSButtonCell;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSGradientType {
  NSGradientNone = 0,
  NSGradientConcaveWeak = 1,
  NSGradientConcaveStrong = 2,
  NSGradientConvexWeak = 3,
  NSGradientConvexStrong = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSUserInterfaceCompressionOptions;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSUserInterfaceCompressionProto {
  #[objrs(selector = "compressWithPrioritizedCompressionOptions:")]
  #[cfg(feature = "RK_AppKit")]
  fn compressWithPrioritizedCompressionOptions_(&self, prioritizedOptions: &NSArray) -> ();
  #[objrs(selector = "minimumSizeWithPrioritizedCompressionOptions:")]
  #[cfg(feature = "RK_AppKit")]
  fn minimumSizeWithPrioritizedCompressionOptions_(&self, prioritizedOptions: &NSArray) -> CGSize;
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSButton;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTouchBarItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTouchBar;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTouchBarDelegateProto {
  #[objrs(selector = "touchBar:makeItemForIdentifier:")]
  #[cfg(feature = "RK_AppKit")]
  fn touchBar_makeItemForIdentifier_(
    &self,
    touchBar: &NSTouchBar,
    identifier: &NSString,
  ) -> Option<Arc<NSTouchBarItem>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTouchBarProviderProto {}
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCandidateListTouchBarItem;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCandidateListTouchBarItemDelegateProto {
  #[objrs(selector = "candidateListTouchBarItem:beginSelectingCandidateAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn candidateListTouchBarItem_beginSelectingCandidateAtIndex_(
    &self,
    anItem: &NSCandidateListTouchBarItem,
    index: isize,
  ) -> ();
  #[objrs(selector = "candidateListTouchBarItem:changeSelectionFromCandidateAtIndex:toIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn candidateListTouchBarItem_changeSelectionFromCandidateAtIndex_toIndex_(
    &self,
    anItem: &NSCandidateListTouchBarItem,
    previousIndex: isize,
    index: isize,
  ) -> ();
  #[objrs(selector = "candidateListTouchBarItem:endSelectingCandidateAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn candidateListTouchBarItem_endSelectingCandidateAtIndex_(
    &self,
    anItem: &NSCandidateListTouchBarItem,
    index: isize,
  ) -> ();
  #[objrs(selector = "candidateListTouchBarItem:changedCandidateListVisibility:")]
  #[cfg(feature = "RK_AppKit")]
  fn candidateListTouchBarItem_changedCandidateListVisibility_(
    &self,
    anItem: &NSCandidateListTouchBarItem,
    isVisible: bool,
  ) -> ();
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSClipView;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPopoverAppearance {
  NSPopoverAppearanceMinimal = 0,
  NSPopoverAppearanceHUD = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPopoverBehavior {
  NSPopoverBehaviorApplicationDefined = 0,
  NSPopoverBehaviorTransient = 1,
  NSPopoverBehaviorSemitransient = 2,
}
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPopover;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPopoverDelegateProto {
  #[objrs(selector = "popoverShouldClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverShouldClose_(&self, popover: &NSPopover) -> bool;
  #[objrs(selector = "popoverShouldDetach:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverShouldDetach_(&self, popover: &NSPopover) -> bool;
  #[objrs(selector = "popoverDidDetach:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverDidDetach_(&self, popover: &NSPopover) -> ();
  #[objrs(selector = "detachableWindowForPopover:")]
  #[cfg(feature = "RK_AppKit")]
  fn detachableWindowForPopover_(&self, popover: &NSPopover) -> Option<Arc<NSWindow>>;
  #[objrs(selector = "popoverWillShow:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverWillShow_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "popoverDidShow:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverDidShow_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "popoverWillClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverWillClose_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "popoverDidClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn popoverDidClose_(&self, notification: &NSNotification) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStoryboard;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _storyboardFlags {
  pub reserved: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStoryboardSegue;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSeguePerformingProto {
  #[objrs(selector = "prepareForSegue:sender:")]
  #[cfg(feature = "RK_AppKit")]
  fn prepareForSegue_sender_(&self, segue: &NSStoryboardSegue, sender: Option<&Object>) -> ();
  #[objrs(selector = "performSegueWithIdentifier:sender:")]
  #[cfg(feature = "RK_AppKit")]
  fn performSegueWithIdentifier_sender_(
    &self,
    identifier: &NSString,
    sender: Option<&Object>,
  ) -> ();
  #[objrs(selector = "shouldPerformSegueWithIdentifier:sender:")]
  #[cfg(feature = "RK_AppKit")]
  fn shouldPerformSegueWithIdentifier_sender_(
    &self,
    identifier: &NSString,
    sender: Option<&Object>,
  ) -> bool;
}
bitflags! { # [ repr ( C ) ] pub struct NSViewControllerTransitionOptions : usize { const NSViewControllerTransitionNone = 0 ; const NSViewControllerTransitionCrossfade = 1 ; const NSViewControllerTransitionSlideUp = 16 ; const NSViewControllerTransitionSlideDown = 32 ; const NSViewControllerTransitionSlideLeft = 64 ; const NSViewControllerTransitionSlideRight = 128 ; const NSViewControllerTransitionSlideForward = 320 ; const NSViewControllerTransitionSlideBackward = 384 ; const NSViewControllerTransitionAllowUserInteraction = 4096 ; } }
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSViewController;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSViewControllerPresentationAnimatorProto {
  #[objrs(selector = "animatePresentationOfViewController:fromViewController:")]
  #[cfg(feature = "RK_AppKit")]
  fn animatePresentationOfViewController_fromViewController_(
    &self,
    viewController: &NSViewController,
    fromViewController: &NSViewController,
  ) -> ();
  #[objrs(selector = "animateDismissalOfViewController:fromViewController:")]
  #[cfg(feature = "RK_AppKit")]
  fn animateDismissalOfViewController_fromViewController_(
    &self,
    viewController: &NSViewController,
    fromViewController: &NSViewController,
  ) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCollectionViewDropOperation {
  NSCollectionViewDropOn = 0,
  NSCollectionViewDropBefore = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCollectionViewItemHighlightState {
  NSCollectionViewItemHighlightNone = 0,
  NSCollectionViewItemHighlightForSelection = 1,
  NSCollectionViewItemHighlightForDeselection = 2,
  NSCollectionViewItemHighlightAsDropTarget = 3,
}
bitflags! { # [ repr ( C ) ] pub struct NSCollectionViewScrollPosition : usize { const NSCollectionViewScrollPositionNone = 0 ; const NSCollectionViewScrollPositionTop = 1 ; const NSCollectionViewScrollPositionCenteredVertically = 2 ; const NSCollectionViewScrollPositionBottom = 4 ; const NSCollectionViewScrollPositionNearestHorizontalEdge = 512 ; const NSCollectionViewScrollPositionLeft = 8 ; const NSCollectionViewScrollPositionCenteredHorizontally = 16 ; const NSCollectionViewScrollPositionRight = 32 ; const NSCollectionViewScrollPositionLeadingEdge = 64 ; const NSCollectionViewScrollPositionTrailingEdge = 128 ; const NSCollectionViewScrollPositionNearestVerticalEdge = 256 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewElementProto {
  #[objrs(selector = "prepareForReuse")]
  #[cfg(feature = "RK_AppKit")]
  fn prepareForReuse(&self) -> ();
  #[objrs(selector = "applyLayoutAttributes:")]
  #[cfg(feature = "RK_AppKit")]
  fn applyLayoutAttributes_(&self, layoutAttributes: &NSCollectionViewLayoutAttributes) -> ();
  #[objrs(selector = "willTransitionFromLayout:toLayout:")]
  #[cfg(feature = "RK_AppKit")]
  fn willTransitionFromLayout_toLayout_(
    &self,
    oldLayout: &NSCollectionViewLayout,
    newLayout: &NSCollectionViewLayout,
  ) -> ();
  #[objrs(selector = "didTransitionFromLayout:toLayout:")]
  #[cfg(feature = "RK_AppKit")]
  fn didTransitionFromLayout_toLayout_(
    &self,
    oldLayout: &NSCollectionViewLayout,
    newLayout: &NSCollectionViewLayout,
  ) -> ();
  #[objrs(selector = "preferredLayoutAttributesFittingAttributes:")]
  #[cfg(feature = "RK_AppKit")]
  fn preferredLayoutAttributesFittingAttributes_(
    &self,
    layoutAttributes: &NSCollectionViewLayoutAttributes,
  ) -> Arc<NSCollectionViewLayoutAttributes>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewSectionHeaderViewProto {}
# [ objrs ( class , super = NSViewController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewItem;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionView;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewDataSourceProto {
  #[objrs(selector = "collectionView:numberOfItemsInSection:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_numberOfItemsInSection_(
    &self,
    collectionView: &NSCollectionView,
    section: isize,
  ) -> isize;
  #[objrs(selector = "collectionView:itemForRepresentedObjectAtIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_itemForRepresentedObjectAtIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    indexPath: &NSIndexPath,
  ) -> Arc<NSCollectionViewItem>;
  #[objrs(selector = "numberOfSectionsInCollectionView:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfSectionsInCollectionView_(&self, collectionView: &NSCollectionView) -> isize;
  #[objrs(selector = "collectionView:viewForSupplementaryElementOfKind:atIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_viewForSupplementaryElementOfKind_atIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    kind: &NSString,
    indexPath: &NSIndexPath,
  ) -> Arc<NSView>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewPrefetchingProto {
  #[objrs(selector = "collectionView:prefetchItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_prefetchItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSArray,
  ) -> ();
  #[objrs(selector = "collectionView:cancelPrefetchingForItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_cancelPrefetchingForItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSArray,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewDelegateProto {
  #[objrs(selector = "collectionView:canDragItemsAtIndexPaths:withEvent:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_canDragItemsAtIndexPaths_withEvent_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
    event: &NSEvent,
  ) -> bool;
  #[objrs(selector = "collectionView:canDragItemsAtIndexes:withEvent:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_canDragItemsAtIndexes_withEvent_(
    &self,
    collectionView: &NSCollectionView,
    indexes: &NSIndexSet,
    event: &NSEvent,
  ) -> bool;
  #[objrs(selector = "collectionView:writeItemsAtIndexPaths:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_writeItemsAtIndexPaths_toPasteboard_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "collectionView:writeItemsAtIndexes:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_writeItemsAtIndexes_toPasteboard_(
    &self,
    collectionView: &NSCollectionView,
    indexes: &NSIndexSet,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(
    selector = "collectionView:namesOfPromisedFilesDroppedAtDestination:forDraggedItemsAtIndexPaths:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_namesOfPromisedFilesDroppedAtDestination_forDraggedItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    dropURL: &NSURL,
    indexPaths: &NSSet,
  ) -> Arc<NSArray>;
  #[objrs(
    selector = "collectionView:namesOfPromisedFilesDroppedAtDestination:forDraggedItemsAtIndexes:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_namesOfPromisedFilesDroppedAtDestination_forDraggedItemsAtIndexes_(
    &self,
    collectionView: &NSCollectionView,
    dropURL: &NSURL,
    indexes: &NSIndexSet,
  ) -> Arc<NSArray>;
  #[objrs(selector = "collectionView:draggingImageForItemsAtIndexPaths:withEvent:offset:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_draggingImageForItemsAtIndexPaths_withEvent_offset_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
    event: &NSEvent,
    dragImageOffset: &CGPoint,
  ) -> Arc<NSImage>;
  #[objrs(selector = "collectionView:draggingImageForItemsAtIndexes:withEvent:offset:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_draggingImageForItemsAtIndexes_withEvent_offset_(
    &self,
    collectionView: &NSCollectionView,
    indexes: &NSIndexSet,
    event: &NSEvent,
    dragImageOffset: &CGPoint,
  ) -> Arc<NSImage>;
  #[objrs(selector = "collectionView:validateDrop:proposedIndexPath:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_validateDrop_proposedIndexPath_dropOperation_(
    &self,
    collectionView: &NSCollectionView,
    draggingInfo: &Object,
    proposedDropIndexPath: &mut &Arc<NSIndexPath>,
    proposedDropOperation: &NSCollectionViewDropOperation,
  ) -> NSDragOperation;
  #[objrs(selector = "collectionView:validateDrop:proposedIndex:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_validateDrop_proposedIndex_dropOperation_(
    &self,
    collectionView: &NSCollectionView,
    draggingInfo: &Object,
    proposedDropIndex: &isize,
    proposedDropOperation: &NSCollectionViewDropOperation,
  ) -> NSDragOperation;
  #[objrs(selector = "collectionView:acceptDrop:indexPath:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_acceptDrop_indexPath_dropOperation_(
    &self,
    collectionView: &NSCollectionView,
    draggingInfo: &Object,
    indexPath: &NSIndexPath,
    dropOperation: NSCollectionViewDropOperation,
  ) -> bool;
  #[objrs(selector = "collectionView:acceptDrop:index:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_acceptDrop_index_dropOperation_(
    &self,
    collectionView: &NSCollectionView,
    draggingInfo: &Object,
    index: isize,
    dropOperation: NSCollectionViewDropOperation,
  ) -> bool;
  #[objrs(selector = "collectionView:pasteboardWriterForItemAtIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_pasteboardWriterForItemAtIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    indexPath: &NSIndexPath,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "collectionView:pasteboardWriterForItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_pasteboardWriterForItemAtIndex_(
    &self,
    collectionView: &NSCollectionView,
    index: usize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "collectionView:draggingSession:willBeginAtPoint:forItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_draggingSession_willBeginAtPoint_forItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    indexPaths: &NSSet,
  ) -> ();
  #[objrs(selector = "collectionView:draggingSession:willBeginAtPoint:forItemsAtIndexes:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_draggingSession_willBeginAtPoint_forItemsAtIndexes_(
    &self,
    collectionView: &NSCollectionView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    indexes: &NSIndexSet,
  ) -> ();
  #[objrs(selector = "collectionView:draggingSession:endedAtPoint:dragOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_draggingSession_endedAtPoint_dragOperation_(
    &self,
    collectionView: &NSCollectionView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    operation: NSDragOperation,
  ) -> ();
  #[objrs(selector = "collectionView:updateDraggingItemsForDrag:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_updateDraggingItemsForDrag_(
    &self,
    collectionView: &NSCollectionView,
    draggingInfo: &Object,
  ) -> ();
  #[objrs(selector = "collectionView:shouldChangeItemsAtIndexPaths:toHighlightState:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_shouldChangeItemsAtIndexPaths_toHighlightState_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
    highlightState: NSCollectionViewItemHighlightState,
  ) -> Arc<NSSet>;
  #[objrs(selector = "collectionView:didChangeItemsAtIndexPaths:toHighlightState:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_didChangeItemsAtIndexPaths_toHighlightState_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
    highlightState: NSCollectionViewItemHighlightState,
  ) -> ();
  #[objrs(selector = "collectionView:shouldSelectItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_shouldSelectItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
  ) -> Arc<NSSet>;
  #[objrs(selector = "collectionView:shouldDeselectItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_shouldDeselectItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
  ) -> Arc<NSSet>;
  #[objrs(selector = "collectionView:didSelectItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_didSelectItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
  ) -> ();
  #[objrs(selector = "collectionView:didDeselectItemsAtIndexPaths:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_didDeselectItemsAtIndexPaths_(
    &self,
    collectionView: &NSCollectionView,
    indexPaths: &NSSet,
  ) -> ();
  #[objrs(selector = "collectionView:willDisplayItem:forRepresentedObjectAtIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_willDisplayItem_forRepresentedObjectAtIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    item: &NSCollectionViewItem,
    indexPath: &NSIndexPath,
  ) -> ();
  #[objrs(selector = "collectionView:willDisplaySupplementaryView:forElementKind:atIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_willDisplaySupplementaryView_forElementKind_atIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    view: &NSView,
    elementKind: &NSString,
    indexPath: &NSIndexPath,
  ) -> ();
  #[objrs(selector = "collectionView:didEndDisplayingItem:forRepresentedObjectAtIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_didEndDisplayingItem_forRepresentedObjectAtIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    item: &NSCollectionViewItem,
    indexPath: &NSIndexPath,
  ) -> ();
  #[objrs(
    selector = "collectionView:didEndDisplayingSupplementaryView:forElementOfKind:atIndexPath:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_didEndDisplayingSupplementaryView_forElementOfKind_atIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    view: &NSView,
    elementKind: &NSString,
    indexPath: &NSIndexPath,
  ) -> ();
  #[objrs(selector = "collectionView:transitionLayoutForOldLayout:newLayout:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_transitionLayoutForOldLayout_newLayout_(
    &self,
    collectionView: &NSCollectionView,
    fromLayout: &NSCollectionViewLayout,
    toLayout: &NSCollectionViewLayout,
  ) -> Arc<NSCollectionViewTransitionLayout>;
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCollectionElementCategory {
  NSCollectionElementCategoryItem = 0,
  NSCollectionElementCategorySupplementaryView = 1,
  NSCollectionElementCategoryDecorationView = 2,
  NSCollectionElementCategoryInterItemGap = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewLayoutAttributes;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCollectionUpdateAction {
  NSCollectionUpdateActionInsert = 0,
  NSCollectionUpdateActionDelete = 1,
  NSCollectionUpdateActionReload = 2,
  NSCollectionUpdateActionMove = 3,
  NSCollectionUpdateActionNone = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewUpdateItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewLayoutInvalidationContext;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewLayout;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCollectionViewScrollDirection {
  NSCollectionViewScrollDirectionVertical = 0,
  NSCollectionViewScrollDirectionHorizontal = 1,
}
# [ objrs ( class , super = NSCollectionViewLayoutInvalidationContext ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewFlowLayoutInvalidationContext;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCollectionViewDelegateFlowLayoutProto {
  #[objrs(selector = "collectionView:layout:sizeForItemAtIndexPath:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_sizeForItemAtIndexPath_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    indexPath: &NSIndexPath,
  ) -> CGSize;
  #[objrs(selector = "collectionView:layout:insetForSectionAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_insetForSectionAtIndex_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    section: isize,
  ) -> NSEdgeInsets;
  #[objrs(selector = "collectionView:layout:minimumLineSpacingForSectionAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_minimumLineSpacingForSectionAtIndex_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    section: isize,
  ) -> f64;
  #[objrs(selector = "collectionView:layout:minimumInteritemSpacingForSectionAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    section: isize,
  ) -> f64;
  #[objrs(selector = "collectionView:layout:referenceSizeForHeaderInSection:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_referenceSizeForHeaderInSection_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    section: isize,
  ) -> CGSize;
  #[objrs(selector = "collectionView:layout:referenceSizeForFooterInSection:")]
  #[cfg(feature = "RK_AppKit")]
  fn collectionView_layout_referenceSizeForFooterInSection_(
    &self,
    collectionView: &NSCollectionView,
    collectionViewLayout: &NSCollectionViewLayout,
    section: isize,
  ) -> CGSize;
}
# [ objrs ( class , super = NSCollectionViewLayout ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewFlowLayout;
# [ objrs ( class , super = NSCollectionViewLayout ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewGridLayout;
# [ objrs ( class , super = NSCollectionViewLayout ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCollectionViewTransitionLayout;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDockTile;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDockTilePlugInProto {
  #[objrs(selector = "setDockTile:")]
  #[cfg(feature = "RK_AppKit")]
  fn setDockTile_(&self, dockTile: Option<&NSDockTile>) -> ();
  #[objrs(selector = "dockMenu")]
  #[cfg(feature = "RK_AppKit")]
  fn dockMenu(&self) -> Option<Arc<NSMenu>>;
}
pub type NSFontSymbolicTraits = u32;
bitflags! { # [ repr ( C ) ] pub struct NSFontDescriptorSymbolicTraits : u32 { const NSFontDescriptorTraitItalic = 1 ; const NSFontDescriptorTraitBold = 2 ; const NSFontDescriptorTraitExpanded = 32 ; const NSFontDescriptorTraitCondensed = 64 ; const NSFontDescriptorTraitMonoSpace = 1024 ; const NSFontDescriptorTraitVertical = 2048 ; const NSFontDescriptorTraitUIOptimized = 4096 ; const NSFontDescriptorTraitTightLeading = 32768 ; const NSFontDescriptorTraitLooseLeading = 65536 ; const NSFontDescriptorClassMask = 4026531840 ; const NSFontDescriptorClassUnknown = 0 ; const NSFontDescriptorClassOldStyleSerifs = 268435456 ; const NSFontDescriptorClassTransitionalSerifs = 536870912 ; const NSFontDescriptorClassModernSerifs = 805306368 ; const NSFontDescriptorClassClarendonSerifs = 1073741824 ; const NSFontDescriptorClassSlabSerifs = 1342177280 ; const NSFontDescriptorClassFreeformSerifs = 1879048192 ; const NSFontDescriptorClassSansSerif = 2147483648 ; const NSFontDescriptorClassOrnamentals = 2415919104 ; const NSFontDescriptorClassScripts = 2684354560 ; const NSFontDescriptorClassSymbolic = 3221225472 ; } }
pub type NSFontWeight = f64;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFontDescriptor;
pub type NSFontFamilyClass = u32;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFont;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fFlags {
  pub _isScreenFont: u32,
  pub _systemFontType: u32,
  pub _reserved1: u32,
  pub _matrixIsIdentity: u32,
  pub _renderingMode: u32,
  pub _inInstanceCache: u32,
  pub _appearanceSize: u32,
  pub _reserved2: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFontRenderingMode {
  NSFontDefaultRenderingMode = 0,
  NSFontAntialiasedRenderingMode = 1,
  NSFontIntegerAdvancementsRenderingMode = 2,
  NSFontAntialiasedIntegerAdvancementsRenderingMode = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSMultibyteGlyphPacking {
  NSNativeShortGlyphPacking = 5,
}
bitflags! { # [ repr ( C ) ] pub struct NSFontAssetRequestOptions : usize { const NSFontAssetRequestOptionUsesStandardUI = 1 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFontAssetRequest;
bitflags! { # [ repr ( C ) ] pub struct NSFontCollectionVisibility : usize { const NSFontCollectionVisibilityProcess = 1 ; const NSFontCollectionVisibilityUser = 2 ; const NSFontCollectionVisibilityComputer = 4 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFontCollection;
# [ objrs ( class , super = NSFontCollection ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMutableFontCollection;
bitflags! { # [ repr ( C ) ] pub struct NSFontTraitMask : usize { const NSItalicFontMask = 1 ; const NSBoldFontMask = 2 ; const NSUnboldFontMask = 4 ; const NSNonStandardCharacterSetFontMask = 8 ; const NSNarrowFontMask = 16 ; const NSExpandedFontMask = 32 ; const NSCondensedFontMask = 64 ; const NSSmallCapsFontMask = 128 ; const NSPosterFontMask = 256 ; const NSCompressedFontMask = 512 ; const NSFixedPitchFontMask = 1024 ; const NSUnitalicFontMask = 16777216 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSFontCollectionOptions : usize { const NSFontCollectionApplicationOnlyMask = 1 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFontAction {
  NSNoFontChangeAction = 0,
  NSViaPanelFontAction = 1,
  NSAddTraitFontAction = 2,
  NSSizeUpFontAction = 3,
  NSSizeDownFontAction = 4,
  NSHeavierFontAction = 5,
  NSLighterFontAction = 6,
  NSRemoveTraitFontAction = 7,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFontManager;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fmFlags {
  pub multipleFont: u32,
  pub disabled: u32,
  pub senderTagMode: u32,
  pub _RESERVED: u32,
}
bitflags! { # [ repr ( C ) ] pub struct NSWindowStyleMask : usize { const NSWindowStyleMaskBorderless = 0 ; const NSWindowStyleMaskTitled = 1 ; const NSWindowStyleMaskClosable = 2 ; const NSWindowStyleMaskMiniaturizable = 4 ; const NSWindowStyleMaskResizable = 8 ; const NSWindowStyleMaskTexturedBackground = 256 ; const NSWindowStyleMaskUnifiedTitleAndToolbar = 4096 ; const NSWindowStyleMaskFullScreen = 16384 ; const NSWindowStyleMaskFullSizeContentView = 32768 ; const NSWindowStyleMaskUtilityWindow = 16 ; const NSWindowStyleMaskDocModalWindow = 64 ; const NSWindowStyleMaskNonactivatingPanel = 128 ; const NSWindowStyleMaskHUDWindow = 8192 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSWindowSharingType {
  NSWindowSharingNone = 0,
  NSWindowSharingReadOnly = 1,
  NSWindowSharingReadWrite = 2,
}
bitflags! { # [ repr ( C ) ] pub struct NSWindowCollectionBehavior : usize { const NSWindowCollectionBehaviorDefault = 0 ; const NSWindowCollectionBehaviorCanJoinAllSpaces = 1 ; const NSWindowCollectionBehaviorMoveToActiveSpace = 2 ; const NSWindowCollectionBehaviorManaged = 4 ; const NSWindowCollectionBehaviorTransient = 8 ; const NSWindowCollectionBehaviorStationary = 16 ; const NSWindowCollectionBehaviorParticipatesInCycle = 32 ; const NSWindowCollectionBehaviorIgnoresCycle = 64 ; const NSWindowCollectionBehaviorFullScreenPrimary = 128 ; const NSWindowCollectionBehaviorFullScreenAuxiliary = 256 ; const NSWindowCollectionBehaviorFullScreenNone = 512 ; const NSWindowCollectionBehaviorFullScreenAllowsTiling = 2048 ; const NSWindowCollectionBehaviorFullScreenDisallowsTiling = 4096 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWindowAnimationBehavior {
  NSWindowAnimationBehaviorDefault = 0,
  NSWindowAnimationBehaviorNone = 2,
  NSWindowAnimationBehaviorDocumentWindow = 3,
  NSWindowAnimationBehaviorUtilityWindow = 4,
  NSWindowAnimationBehaviorAlertPanel = 5,
}
bitflags! { # [ repr ( C ) ] pub struct NSWindowNumberListOptions : usize { const NSWindowNumberListAllApplications = 1 ; const NSWindowNumberListAllSpaces = 16 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSWindowOcclusionState : usize { const NSWindowOcclusionStateVisible = 2 ; } }
pub type NSWindowLevel = isize;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSelectionDirection {
  NSDirectSelection = 0,
  NSSelectingNext = 1,
  NSSelectingPrevious = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSWindowButton {
  NSWindowCloseButton = 0,
  NSWindowMiniaturizeButton = 1,
  NSWindowZoomButton = 2,
  NSWindowToolbarButton = 3,
  NSWindowDocumentIconButton = 4,
  NSWindowDocumentVersionsButton = 6,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWindowTitleVisibility {
  NSWindowTitleVisible = 0,
  NSWindowTitleHidden = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWindowUserTabbingPreference {
  NSWindowUserTabbingPreferenceManual = 0,
  NSWindowUserTabbingPreferenceAlways = 1,
  NSWindowUserTabbingPreferenceInFullScreen = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWindowTabbingMode {
  NSWindowTabbingModeAutomatic = 0,
  NSWindowTabbingModePreferred = 1,
  NSWindowTabbingModeDisallowed = 2,
}
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWindow;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wFlags {
  pub backing: u32,
  pub visible: u32,
  pub isMainWindow: u32,
  pub isKeyWindow: u32,
  pub hidesOnDeactivate: u32,
  pub dontFreeWhenClosed: u32,
  pub oneShot: u32,
  pub deferred: u32,
  pub cursorRectsDisabled: u32,
  pub haveFreeCursorRects: u32,
  pub validCursorRects: u32,
  pub docEdited: u32,
  pub staticDepthLimit: u32,
  pub worksWhenModal: u32,
  pub limitedBecomeKey: u32,
  pub needsFlush: u32,
  pub viewsNeedDisplay: u32,
  pub ignoredFirstMouse: u32,
  pub repostedFirstMouse: u32,
  pub windowDying: u32,
  pub tempHidden: u32,
  pub floatingPanel: u32,
  pub wantsToBeOnMainScreen: u32,
  pub needsBuildLayerTree: u32,
  pub deferCanDraw: u32,
  pub titleIsRepresentedFilename: u32,
  pub excludedFromWindowsMenu: u32,
  pub depthLimit: u32,
  pub delegateReturnsValidRequestor: u32,
  pub lmouseupPending: u32,
  pub rmouseupPending: u32,
  pub hasColorSensitiveUI: u32,
  pub wantsToRegDragTypes: u32,
  pub sentInvalidateCursorRectsMsg: u32,
  pub avoidsActivation: u32,
  pub frameSavedUsingTitle: u32,
  pub didRegDragTypes: u32,
  pub delayedOneShot: u32,
  pub postedNeedsDisplayNote: u32,
  pub unused2: u32,
  pub initialFirstResponderTempSet: u32,
  pub autodisplay: u32,
  pub tossedFirstEvent: u32,
  pub isImageCache: u32,
  pub autolayoutEngagedSomewhere: u32,
  pub hasRegisteredBackdropViews: u32,
  pub unused3: u32,
  pub keyViewSelectionDirection: u32,
  pub defaultButtonCellKETemporarilyDisabled: u32,
  pub defaultButtonCellKEDisabled: u32,
  pub menuHasBeenSet: u32,
  pub wantsToBeModal: u32,
  pub showingModalFrame: u32,
  pub isTerminating: u32,
  pub makingFirstResponderForMouseDown: u32,
  pub needsZoom: u32,
  pub sentWindowNeedsDisplayMsg: u32,
  pub wasModalAtSometime: u32,
  pub windowWillBecomeFS: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSWindowDelegateProto {
  #[objrs(selector = "windowShouldClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowShouldClose_(&self, sender: &NSWindow) -> bool;
  #[objrs(selector = "windowWillReturnFieldEditor:toObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillReturnFieldEditor_toObject_(
    &self,
    sender: &NSWindow,
    client: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "windowWillResize:toSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillResize_toSize_(&self, sender: &NSWindow, frameSize: CGSize) -> CGSize;
  #[objrs(selector = "windowWillUseStandardFrame:defaultFrame:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillUseStandardFrame_defaultFrame_(&self, window: &NSWindow, newFrame: CGRect)
    -> CGRect;
  #[objrs(selector = "windowShouldZoom:toFrame:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowShouldZoom_toFrame_(&self, window: &NSWindow, newFrame: CGRect) -> bool;
  #[objrs(selector = "windowWillReturnUndoManager:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillReturnUndoManager_(&self, window: &NSWindow) -> Option<Arc<NSUndoManager>>;
  #[objrs(selector = "window:willPositionSheet:usingRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_willPositionSheet_usingRect_(
    &self,
    window: &NSWindow,
    sheet: &NSWindow,
    rect: CGRect,
  ) -> CGRect;
  #[objrs(selector = "window:shouldPopUpDocumentPathMenu:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_shouldPopUpDocumentPathMenu_(&self, window: &NSWindow, menu: &NSMenu) -> bool;
  #[objrs(selector = "window:shouldDragDocumentWithEvent:from:withPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_shouldDragDocumentWithEvent_from_withPasteboard_(
    &self,
    window: &NSWindow,
    event: &NSEvent,
    dragImageLocation: CGPoint,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "window:willUseFullScreenContentSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_willUseFullScreenContentSize_(&self, window: &NSWindow, proposedSize: CGSize)
    -> CGSize;
  #[objrs(selector = "window:willUseFullScreenPresentationOptions:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_willUseFullScreenPresentationOptions_(
    &self,
    window: &NSWindow,
    proposedOptions: NSApplicationPresentationOptions,
  ) -> NSApplicationPresentationOptions;
  #[objrs(selector = "customWindowsToEnterFullScreenForWindow:")]
  #[cfg(feature = "RK_AppKit")]
  fn customWindowsToEnterFullScreenForWindow_(&self, window: &NSWindow) -> Option<Arc<NSArray>>;
  #[objrs(selector = "window:startCustomAnimationToEnterFullScreenWithDuration:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_startCustomAnimationToEnterFullScreenWithDuration_(
    &self,
    window: &NSWindow,
    duration: f64,
  ) -> ();
  #[objrs(selector = "windowDidFailToEnterFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidFailToEnterFullScreen_(&self, window: &NSWindow) -> ();
  #[objrs(selector = "customWindowsToExitFullScreenForWindow:")]
  #[cfg(feature = "RK_AppKit")]
  fn customWindowsToExitFullScreenForWindow_(&self, window: &NSWindow) -> Option<Arc<NSArray>>;
  #[objrs(selector = "window:startCustomAnimationToExitFullScreenWithDuration:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_startCustomAnimationToExitFullScreenWithDuration_(
    &self,
    window: &NSWindow,
    duration: f64,
  ) -> ();
  #[objrs(selector = "customWindowsToEnterFullScreenForWindow:onScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn customWindowsToEnterFullScreenForWindow_onScreen_(
    &self,
    window: &NSWindow,
    screen: &NSScreen,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "window:startCustomAnimationToEnterFullScreenOnScreen:withDuration:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_startCustomAnimationToEnterFullScreenOnScreen_withDuration_(
    &self,
    window: &NSWindow,
    screen: &NSScreen,
    duration: f64,
  ) -> ();
  #[objrs(selector = "windowDidFailToExitFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidFailToExitFullScreen_(&self, window: &NSWindow) -> ();
  #[objrs(selector = "window:willResizeForVersionBrowserWithMaxPreferredSize:maxAllowedSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_willResizeForVersionBrowserWithMaxPreferredSize_maxAllowedSize_(
    &self,
    window: &NSWindow,
    maxPreferredFrameSize: CGSize,
    maxAllowedFrameSize: CGSize,
  ) -> CGSize;
  #[objrs(selector = "window:willEncodeRestorableState:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_willEncodeRestorableState_(&self, window: &NSWindow, state: &NSCoder) -> ();
  #[objrs(selector = "window:didDecodeRestorableState:")]
  #[cfg(feature = "RK_AppKit")]
  fn window_didDecodeRestorableState_(&self, window: &NSWindow, state: &NSCoder) -> ();
  #[objrs(selector = "windowDidResize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidResize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidExpose:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidExpose_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillMove:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillMove_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidMove:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidMove_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidBecomeKey:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidBecomeKey_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidResignKey:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidResignKey_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidBecomeMain:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidBecomeMain_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidResignMain:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidResignMain_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillClose_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillMiniaturize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillMiniaturize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidMiniaturize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidMiniaturize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidDeminiaturize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidDeminiaturize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidUpdate:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidUpdate_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidChangeScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidChangeScreen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidChangeScreenProfile:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidChangeScreenProfile_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidChangeBackingProperties:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidChangeBackingProperties_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillBeginSheet:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillBeginSheet_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidEndSheet:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidEndSheet_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillStartLiveResize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillStartLiveResize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidEndLiveResize:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidEndLiveResize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillEnterFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillEnterFullScreen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidEnterFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidEnterFullScreen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillExitFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillExitFullScreen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidExitFullScreen:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidExitFullScreen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillEnterVersionBrowser:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillEnterVersionBrowser_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidEnterVersionBrowser:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidEnterVersionBrowser_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowWillExitVersionBrowser:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowWillExitVersionBrowser_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidExitVersionBrowser:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidExitVersionBrowser_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "windowDidChangeOcclusionState:")]
  #[cfg(feature = "RK_AppKit")]
  fn windowDidChangeOcclusionState_(&self, notification: &NSNotification) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSWindowBackingLocation {
  NSWindowBackingLocationDefault = 0,
  NSWindowBackingLocationVideoMemory = 1,
  NSWindowBackingLocationMainMemory = 2,
}
# [ objrs ( class , super = NSWindow ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPanel;
bitflags! { # [ repr ( C ) ] pub struct NSFontPanelModeMask : usize { const NSFontPanelModeMaskFace = 1 ; const NSFontPanelModeMaskSize = 2 ; const NSFontPanelModeMaskCollection = 4 ; const NSFontPanelModeMaskUnderlineEffect = 256 ; const NSFontPanelModeMaskStrikethroughEffect = 512 ; const NSFontPanelModeMaskTextColorEffect = 1024 ; const NSFontPanelModeMaskDocumentColorEffect = 2048 ; const NSFontPanelModeMaskShadowEffect = 4096 ; const NSFontPanelModeMaskAllEffects = 1048320 ; const NSFontPanelModesMaskStandardModes = 65535 ; const NSFontPanelModesMaskAllModes = 4294967295 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSFontChangingProto {
  #[objrs(selector = "changeFont:")]
  #[cfg(feature = "RK_AppKit")]
  fn changeFont_(&self, sender: Option<&NSFontManager>) -> ();
  #[objrs(selector = "validModesForFontPanel:")]
  #[cfg(feature = "RK_AppKit")]
  fn validModesForFontPanel_(&self, fontPanel: &NSFontPanel) -> NSFontPanelModeMask;
}
# [ objrs ( class , super = NSPanel ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFontPanel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __fpFlags {
  pub setFontChange: u32,
  pub setFontAttributeChange: u32,
  pub _delRespFamily: u32,
  pub _delRespFace: u32,
  pub _delRespSize: u32,
  pub _delRespColl: u32,
  pub _collectionDisabled: u32,
  pub _sizeDisabled: u32,
  pub _faceDisabled: u32,
  pub showEffects: u32,
  pub _uiMode: u32,
  pub _miniMode: u32,
  pub _reserved: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSMatrixMode {
  NSRadioModeMatrix = 0,
  NSHighlightModeMatrix = 1,
  NSListModeMatrix = 2,
  NSTrackModeMatrix = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __MFlags {
  pub reservedMatrix: u32,
  pub cellSizeNeedsAutorecalc: u32,
  pub autorecalculatesCellSize: u32,
  pub drawingContextMenuHighlightOnAllSelectedRows: u32,
  pub drawingContextMenuHighlight: u32,
  pub browserOptimizationsEnabled: u32,
  pub reservedMatrix3: u32,
  pub tmpAllowNonVisibleCellsToBecomeFirstResponder: u32,
  pub reservedMatrix2: u32,
  pub checkForSimpleTrackingMode: u32,
  pub useSimpleTrackingMode: u32,
  pub refusesFirstResponder: u32,
  pub dontScroll: u32,
  pub changingSelectionWithKeyboard: u32,
  pub onlySetKeyCell: u32,
  pub currentlySelectingCell: u32,
  pub allowsIncrementalSearching: u32,
  pub tabKeyTraversesCellsExplicitlySet: u32,
  pub tabKeyTraversesCells: u32,
  pub drawingAncestor: u32,
  pub autosizeCells: u32,
  pub drawsBackground: u32,
  pub drawsCellBackground: u32,
  pub selectionByRect: u32,
  pub autoscroll: u32,
  pub allowEmptySel: u32,
  pub listMode: u32,
  pub radioMode: u32,
  pub highlightMode: u32,
}
pub type _MFlags = __MFlags;
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMatrix;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSMatrixDelegateProto {}
# [ objrs ( class , super = NSMatrix ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSForm;
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFormCell;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorList;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _colorListFlags {
  pub colorsLoaded: u32,
  pub editable: u32,
  pub hasDeviceSpecificLists: u32,
  pub dirty: u32,
  pub hasFrozen: u32,
  pub notificationsDisabled: u32,
  pub hasAttemptedLoadingBundleForDirectory: u32,
  pub isProfileBased: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSColorType {
  NSColorTypeComponentBased = 0,
  NSColorTypePattern = 1,
  NSColorTypeCatalog = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSColorSystemEffect {
  NSColorSystemEffectNone = 0,
  NSColorSystemEffectPressed = 1,
  NSColorSystemEffectDeepPressed = 2,
  NSColorSystemEffectDisabled = 3,
  NSColorSystemEffectRollover = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColor;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSColorSpaceModel {
  NSColorSpaceModelUnknown = -1,
  NSColorSpaceModelGray = 0,
  NSColorSpaceModelRGB = 1,
  NSColorSpaceModelCMYK = 2,
  NSColorSpaceModelLAB = 3,
  NSColorSpaceModelDeviceN = 4,
  NSColorSpaceModelIndexed = 5,
  NSColorSpaceModelPatterned = 6,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorSpace;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSImageLayoutDirection {
  NSImageLayoutDirectionUnspecified = -1,
  NSImageLayoutDirectionLeftToRight = 2,
  NSImageLayoutDirectionRightToLeft = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSImageRep;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __repFlags {
  pub hasAlpha: u32,
  pub isOpaque: u32,
  pub cacheParamsComputed: u32,
  pub cacheAlphaComputed: u32,
  pub loadState: u32,
  pub keepCacheWindow: u32,
  pub reserved: u32,
  pub bitsPerSample: u32,
  pub internalLayoutDirection: u32,
  pub gsaved: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTIFFCompression {
  NSTIFFCompressionNone = 1,
  NSTIFFCompressionCCITTFAX3 = 3,
  NSTIFFCompressionCCITTFAX4 = 4,
  NSTIFFCompressionLZW = 5,
  NSTIFFCompressionJPEG = 6,
  NSTIFFCompressionNEXT = 32766,
  NSTIFFCompressionPackBits = 32773,
  NSTIFFCompressionOldJPEG = 32865,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBitmapImageFileType {
  NSBitmapImageFileTypeTIFF = 0,
  NSBitmapImageFileTypeBMP = 1,
  NSBitmapImageFileTypeGIF = 2,
  NSBitmapImageFileTypeJPEG = 3,
  NSBitmapImageFileTypePNG = 4,
  NSBitmapImageFileTypeJPEG2000 = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSImageRepLoadStatus {
  NSImageRepLoadStatusUnknownType = -1,
  NSImageRepLoadStatusReadingHeader = -2,
  NSImageRepLoadStatusWillNeedAllData = -3,
  NSImageRepLoadStatusInvalidData = -4,
  NSImageRepLoadStatusUnexpectedEOF = -5,
  NSImageRepLoadStatusCompleted = -6,
}
bitflags! { # [ repr ( C ) ] pub struct NSBitmapFormat : usize { const NSBitmapFormatAlphaFirst = 1 ; const NSBitmapFormatAlphaNonpremultiplied = 2 ; const NSBitmapFormatFloatingPointSamples = 4 ; const NSBitmapFormatSixteenBitLittleEndian = 256 ; const NSBitmapFormatThirtyTwoBitLittleEndian = 512 ; const NSBitmapFormatSixteenBitBigEndian = 1024 ; const NSBitmapFormatThirtyTwoBitBigEndian = 2048 ; } }
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBitmapImageRep;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bitmapRepFlags {
  pub bitsPerPixel: u32,
  pub isPlanar: u32,
  pub explicitPlanes: u32,
  pub imageSourceIsIndexed: u32,
  pub dataLoaded: u32,
  pub colorModel: u32,
  pub tierTwoInfoIsLoaded: u32,
  pub respectO: u32,
  pub compressionFactor: u32,
  pub imageNumber: u32,
  pub bitmapFormat: u32,
  pub cgImageIsPrimary: u32,
  pub compression: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __Brflags {
  pub firstVisibleCalculationDisabled: u32,
  pub prefersAllColumnUserResizing: u32,
  pub usesSmallScrollers: u32,
  pub usesSmallSizeTitleFont: u32,
  pub actionNeedsToBeSent: u32,
  pub acceptsFirstMouse: u32,
  pub refusesFirstResponder: u32,
  pub disableCompositing: u32,
  pub delegateSelectsCellsByRow: u32,
  pub allowsIncrementalSearching: u32,
  pub time: u32,
  pub hasHorizontalScroller: u32,
  pub prohibitEmptySel: u32,
  pub sendActionOnArrowKeys: u32,
  pub dontDrawTitles: u32,
  pub acceptArrowKeys: u32,
  pub delegateValidatesColumns: u32,
  pub delegateDoesNotCreateRowsInMatrix: u32,
  pub delegateSelectsCellsByString: u32,
  pub delegateSetsTitles: u32,
  pub delegateImplementsWillDisplayCell: u32,
  pub separateColumns: u32,
  pub titleFromPrevious: u32,
  pub isTitled: u32,
  pub reuseColumns: u32,
  pub allowsBranchSelection: u32,
  pub allowsMultipleSelection: u32,
}
pub type _Brflags = __Brflags;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBrowserColumnResizingType {
  NSBrowserNoColumnResizing = 0,
  NSBrowserAutoColumnResizing = 1,
  NSBrowserUserColumnResizing = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBrowserDropOperation {
  NSBrowserDropOn = 0,
  NSBrowserDropAbove = 1,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBrowser;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSBrowserDelegateProto {
  #[objrs(selector = "browser:numberOfRowsInColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_numberOfRowsInColumn_(&self, sender: &NSBrowser, column: isize) -> isize;
  #[objrs(selector = "browser:createRowsForColumn:inMatrix:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_createRowsForColumn_inMatrix_(
    &self,
    sender: &NSBrowser,
    column: isize,
    matrix: &NSMatrix,
  ) -> ();
  #[objrs(selector = "browser:numberOfChildrenOfItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_numberOfChildrenOfItem_(&self, browser: &NSBrowser, item: Option<&Object>) -> isize;
  #[objrs(selector = "browser:child:ofItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_child_ofItem_(
    &self,
    browser: &NSBrowser,
    index: isize,
    item: Option<&Object>,
  ) -> Arc<Object>;
  #[objrs(selector = "browser:isLeafItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_isLeafItem_(&self, browser: &NSBrowser, item: Option<&Object>) -> bool;
  #[objrs(selector = "browser:objectValueForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_objectValueForItem_(
    &self,
    browser: &NSBrowser,
    item: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "browser:heightOfRow:inColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_heightOfRow_inColumn_(
    &self,
    browser: &NSBrowser,
    row: isize,
    columnIndex: isize,
  ) -> f64;
  #[objrs(selector = "rootItemForBrowser:")]
  #[cfg(feature = "RK_AppKit")]
  fn rootItemForBrowser_(&self, browser: &NSBrowser) -> Option<Arc<Object>>;
  #[objrs(selector = "browser:setObjectValue:forItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_setObjectValue_forItem_(
    &self,
    browser: &NSBrowser,
    object: Option<&Object>,
    item: Option<&Object>,
  ) -> ();
  #[objrs(selector = "browser:shouldEditItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_shouldEditItem_(&self, browser: &NSBrowser, item: Option<&Object>) -> bool;
  #[objrs(selector = "browser:willDisplayCell:atRow:column:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_willDisplayCell_atRow_column_(
    &self,
    sender: &NSBrowser,
    cell: &Object,
    row: isize,
    column: isize,
  ) -> ();
  #[objrs(selector = "browser:titleOfColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_titleOfColumn_(&self, sender: &NSBrowser, column: isize) -> Option<Arc<NSString>>;
  #[objrs(selector = "browser:selectCellWithString:inColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_selectCellWithString_inColumn_(
    &self,
    sender: &NSBrowser,
    title: &NSString,
    column: isize,
  ) -> bool;
  #[objrs(selector = "browser:selectRow:inColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_selectRow_inColumn_(&self, sender: &NSBrowser, row: isize, column: isize) -> bool;
  #[objrs(selector = "browser:isColumnValid:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_isColumnValid_(&self, sender: &NSBrowser, column: isize) -> bool;
  #[objrs(selector = "browserWillScroll:")]
  #[cfg(feature = "RK_AppKit")]
  fn browserWillScroll_(&self, sender: &NSBrowser) -> ();
  #[objrs(selector = "browserDidScroll:")]
  #[cfg(feature = "RK_AppKit")]
  fn browserDidScroll_(&self, sender: &NSBrowser) -> ();
  #[objrs(selector = "browser:shouldSizeColumn:forUserResize:toWidth:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_shouldSizeColumn_forUserResize_toWidth_(
    &self,
    browser: &NSBrowser,
    columnIndex: isize,
    forUserResize: bool,
    suggestedWidth: f64,
  ) -> f64;
  #[objrs(selector = "browser:sizeToFitWidthOfColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_sizeToFitWidthOfColumn_(&self, browser: &NSBrowser, columnIndex: isize) -> f64;
  #[objrs(selector = "browserColumnConfigurationDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn browserColumnConfigurationDidChange_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "browser:shouldShowCellExpansionForRow:column:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_shouldShowCellExpansionForRow_column_(
    &self,
    browser: &NSBrowser,
    row: isize,
    column: isize,
  ) -> bool;
  #[objrs(selector = "browser:writeRowsWithIndexes:inColumn:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_writeRowsWithIndexes_inColumn_toPasteboard_(
    &self,
    browser: &NSBrowser,
    rowIndexes: &NSIndexSet,
    column: isize,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(
    selector = "browser:namesOfPromisedFilesDroppedAtDestination:forDraggedRowsWithIndexes:inColumn:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn browser_namesOfPromisedFilesDroppedAtDestination_forDraggedRowsWithIndexes_inColumn_(
    &self,
    browser: &NSBrowser,
    dropDestination: &NSURL,
    rowIndexes: &NSIndexSet,
    column: isize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "browser:canDragRowsWithIndexes:inColumn:withEvent:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_canDragRowsWithIndexes_inColumn_withEvent_(
    &self,
    browser: &NSBrowser,
    rowIndexes: &NSIndexSet,
    column: isize,
    event: &NSEvent,
  ) -> bool;
  #[objrs(selector = "browser:draggingImageForRowsWithIndexes:inColumn:withEvent:offset:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_draggingImageForRowsWithIndexes_inColumn_withEvent_offset_(
    &self,
    browser: &NSBrowser,
    rowIndexes: &NSIndexSet,
    column: isize,
    event: &NSEvent,
    dragImageOffset: &CGPoint,
  ) -> Option<Arc<NSImage>>;
  #[objrs(selector = "browser:validateDrop:proposedRow:column:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_validateDrop_proposedRow_column_dropOperation_(
    &self,
    browser: &NSBrowser,
    info: &Object,
    row: &isize,
    column: &isize,
    dropOperation: &NSBrowserDropOperation,
  ) -> NSDragOperation;
  #[objrs(selector = "browser:acceptDrop:atRow:column:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_acceptDrop_atRow_column_dropOperation_(
    &self,
    browser: &NSBrowser,
    info: &Object,
    row: isize,
    column: isize,
    dropOperation: NSBrowserDropOperation,
  ) -> bool;
  #[objrs(selector = "browser:typeSelectStringForRow:inColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_typeSelectStringForRow_inColumn_(
    &self,
    browser: &NSBrowser,
    row: isize,
    column: isize,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "browser:shouldTypeSelectForEvent:withCurrentSearchString:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_shouldTypeSelectForEvent_withCurrentSearchString_(
    &self,
    browser: &NSBrowser,
    event: &NSEvent,
    searchString: Option<&NSString>,
  ) -> bool;
  #[objrs(selector = "browser:nextTypeSelectMatchFromRow:toRow:inColumn:forString:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_nextTypeSelectMatchFromRow_toRow_inColumn_forString_(
    &self,
    browser: &NSBrowser,
    startRow: isize,
    endRow: isize,
    column: isize,
    searchString: Option<&NSString>,
  ) -> isize;
  #[objrs(selector = "browser:previewViewControllerForLeafItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_previewViewControllerForLeafItem_(
    &self,
    browser: &NSBrowser,
    item: &Object,
  ) -> Option<Arc<NSViewController>>;
  #[objrs(selector = "browser:headerViewControllerForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_headerViewControllerForItem_(
    &self,
    browser: &NSBrowser,
    item: Option<&Object>,
  ) -> Option<Arc<NSViewController>>;
  #[objrs(selector = "browser:didChangeLastColumn:toColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_didChangeLastColumn_toColumn_(
    &self,
    browser: &NSBrowser,
    oldLastColumn: isize,
    column: isize,
  ) -> ();
  #[objrs(selector = "browser:selectionIndexesForProposedSelection:inColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn browser_selectionIndexesForProposedSelection_inColumn_(
    &self,
    browser: &NSBrowser,
    proposedSelectionIndexes: &NSIndexSet,
    column: isize,
  ) -> Arc<NSIndexSet>;
}
# [ objrs ( class , super = NSCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBrowserCell;
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCachedImageRep;
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCIImageRep;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSColorPanelMode {
  NSColorPanelModeNone = -1,
  NSColorPanelModeGray = 0,
  NSColorPanelModeRGB = 1,
  NSColorPanelModeCMYK = 2,
  NSColorPanelModeHSB = 3,
  NSColorPanelModeCustomPalette = 4,
  NSColorPanelModeColorList = 5,
  NSColorPanelModeWheel = 6,
  NSColorPanelModeCrayon = 7,
}
bitflags! { # [ repr ( C ) ] pub struct NSColorPanelOptions : usize { const NSColorPanelGrayModeMask = 1 ; const NSColorPanelRGBModeMask = 2 ; const NSColorPanelCMYKModeMask = 4 ; const NSColorPanelHSBModeMask = 8 ; const NSColorPanelCustomPaletteModeMask = 16 ; const NSColorPanelColorListModeMask = 32 ; const NSColorPanelWheelModeMask = 64 ; const NSColorPanelCrayonModeMask = 128 ; const NSColorPanelAllModesMask = 65535 ; } }
# [ objrs ( class , super = NSPanel ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorPanel;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSColorChangingProto {
  #[objrs(selector = "changeColor:")]
  #[cfg(feature = "RK_AppKit")]
  fn changeColor_(&self, sender: Option<&NSColorPanel>) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSColorPickingDefaultProto {
//   #[objrs(selector = "initWithPickerMask:colorPanel:")]
//   #[cfg(feature = "RK_AppKit")]
//   fn newWithPickerMask_colorPanel_(
//     mask: usize,
//     owningColorPanel: &NSColorPanel,
//   ) -> Option<Arc<Self>>;
  #[objrs(selector = "provideNewButtonImage")]
  #[cfg(feature = "RK_AppKit")]
  fn provideNewButtonImage(&self) -> Arc<NSImage>;
  #[objrs(selector = "insertNewButtonImage:in:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertNewButtonImage_in_(&self, newButtonImage: &NSImage, buttonCell: &NSButtonCell) -> ();
  #[objrs(selector = "viewSizeChanged:")]
  #[cfg(feature = "RK_AppKit")]
  fn viewSizeChanged_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "alphaControlAddedOrRemoved:")]
  #[cfg(feature = "RK_AppKit")]
  fn alphaControlAddedOrRemoved_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "attachColorList:")]
  #[cfg(feature = "RK_AppKit")]
  fn attachColorList_(&self, colorList: &NSColorList) -> ();
  #[objrs(selector = "detachColorList:")]
  #[cfg(feature = "RK_AppKit")]
  fn detachColorList_(&self, colorList: &NSColorList) -> ();
  #[objrs(selector = "setMode:")]
  #[cfg(feature = "RK_AppKit")]
  fn setMode_(&self, mode: NSColorPanelMode) -> ();
  #[objrs(selector = "buttonToolTip")]
  #[cfg(feature = "RK_AppKit")]
  fn buttonToolTip(&self) -> Arc<NSString>;
  #[objrs(selector = "minContentSize")]
  #[cfg(feature = "RK_AppKit")]
  fn minContentSize(&self) -> CGSize;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSColorPickingCustomProto {
  #[objrs(selector = "supportsMode:")]
  #[cfg(feature = "RK_AppKit")]
  fn supportsMode_(&self, mode: NSColorPanelMode) -> bool;
  #[objrs(selector = "currentMode")]
  #[cfg(feature = "RK_AppKit")]
  fn currentMode(&self) -> NSColorPanelMode;
  #[objrs(selector = "provideNewView:")]
  #[cfg(feature = "RK_AppKit")]
  fn provideNewView_(&self, initialRequest: bool) -> Arc<NSView>;
  #[objrs(selector = "setColor:")]
  #[cfg(feature = "RK_AppKit")]
  fn setColor_(&self, newColor: &NSColor) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorPicker;
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorPickerTouchBarItem;
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSColorWell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __cwFlags {
  pub isActive: u32,
  pub isBordered: u32,
  pub cantDraw: u32,
  pub isNotContinuous: u32,
  pub refusesFR: u32,
  pub reservedColorWell: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCursor;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _cursorFlags {
  pub onMouseExited: u32,
  pub onMouseEntered: u32,
  pub cursorType: u32,
}
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCustomImageRep;
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSCustomTouchBarItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDocumentController;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDraggingImageComponent;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDraggingItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDraggingSession;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFilePromiseProvider;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSFilePromiseProviderDelegateProto {
  #[objrs(selector = "filePromiseProvider:fileNameForType:")]
  #[cfg(feature = "RK_AppKit")]
  fn filePromiseProvider_fileNameForType_(
    &self,
    filePromiseProvider: &NSFilePromiseProvider,
    fileType: &NSString,
  ) -> Arc<NSString>;
  #[objrs(selector = "filePromiseProvider:writePromiseToURL:completionHandler:")]
  #[cfg(feature = "RK_AppKit")]
  fn filePromiseProvider_writePromiseToURL_completionHandler_(
    &self,
    filePromiseProvider: &NSFilePromiseProvider,
    url: &NSURL,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "operationQueueForFilePromiseProvider:")]
  #[cfg(feature = "RK_AppKit")]
  fn operationQueueForFilePromiseProvider_(
    &self,
    filePromiseProvider: &NSFilePromiseProvider,
  ) -> Arc<NSOperationQueue>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSFilePromiseReceiver;
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSEPSImageRep;
bitflags! { # [ repr ( C ) ] pub struct NSGradientDrawingOptions : usize { const NSGradientDrawsBeforeStartingLocation = 1 ; const NSGradientDrawsAfterEndingLocation = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGradient;
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGroupTouchBarItem;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __gtbiFlags {
  pub hasAutomaticLayoutDirection: u32,
  pub reserved: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSGestureRecognizerState {
  NSGestureRecognizerStatePossible = 0,
  NSGestureRecognizerStateBegan = 1,
  NSGestureRecognizerStateChanged = 2,
  NSGestureRecognizerStateEnded = 3,
  NSGestureRecognizerStateCancelled = 4,
  NSGestureRecognizerStateFailed = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGestureRecognizer;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSGestureRecognizerDelegateProto {
  #[objrs(selector = "gestureRecognizer:shouldAttemptToRecognizeWithEvent:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizer_shouldAttemptToRecognizeWithEvent_(
    &self,
    gestureRecognizer: &NSGestureRecognizer,
    event: &NSEvent,
  ) -> bool;
  #[objrs(selector = "gestureRecognizerShouldBegin:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizerShouldBegin_(&self, gestureRecognizer: &NSGestureRecognizer) -> bool;
  #[objrs(selector = "gestureRecognizer:shouldRecognizeSimultaneouslyWithGestureRecognizer:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizer_shouldRecognizeSimultaneouslyWithGestureRecognizer_(
    &self,
    gestureRecognizer: &NSGestureRecognizer,
    otherGestureRecognizer: &NSGestureRecognizer,
  ) -> bool;
  #[objrs(selector = "gestureRecognizer:shouldRequireFailureOfGestureRecognizer:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizer_shouldRequireFailureOfGestureRecognizer_(
    &self,
    gestureRecognizer: &NSGestureRecognizer,
    otherGestureRecognizer: &NSGestureRecognizer,
  ) -> bool;
  #[objrs(selector = "gestureRecognizer:shouldBeRequiredToFailByGestureRecognizer:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizer_shouldBeRequiredToFailByGestureRecognizer_(
    &self,
    gestureRecognizer: &NSGestureRecognizer,
    otherGestureRecognizer: &NSGestureRecognizer,
  ) -> bool;
  #[objrs(selector = "gestureRecognizer:shouldReceiveTouch:")]
  #[cfg(feature = "RK_AppKit")]
  fn gestureRecognizer_shouldReceiveTouch_(
    &self,
    gestureRecognizer: &NSGestureRecognizer,
    touch: &NSTouch,
  ) -> bool;
}
# [ objrs ( class , super = NSGestureRecognizer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSClickGestureRecognizer;
# [ objrs ( class , super = NSGestureRecognizer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPanGestureRecognizer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pgrFlags {
  pub reserved0: u32,
  pub reserved: u32,
}
# [ objrs ( class , super = NSGestureRecognizer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPressGestureRecognizer;
# [ objrs ( class , super = NSGestureRecognizer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMagnificationGestureRecognizer;
# [ objrs ( class , super = NSGestureRecognizer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSRotationGestureRecognizer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutAnchor;
# [ objrs ( class , super = NSLayoutAnchor ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutXAxisAnchor;
# [ objrs ( class , super = NSLayoutAnchor ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutYAxisAnchor;
# [ objrs ( class , super = NSLayoutAnchor ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutDimension;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLayoutRelation {
  NSLayoutRelationLessThanOrEqual = -1,
  NSLayoutRelationEqual = 0,
  NSLayoutRelationGreaterThanOrEqual = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLayoutAttribute {
  NSLayoutAttributeLeft = 1,
  NSLayoutAttributeRight = 2,
  NSLayoutAttributeTop = 3,
  NSLayoutAttributeBottom = 4,
  NSLayoutAttributeLeading = 5,
  NSLayoutAttributeTrailing = 6,
  NSLayoutAttributeWidth = 7,
  NSLayoutAttributeHeight = 8,
  NSLayoutAttributeCenterX = 9,
  NSLayoutAttributeCenterY = 10,
  NSLayoutAttributeLastBaseline = 11,
  NSLayoutAttributeFirstBaseline = 12,
  NSLayoutAttributeNotAnAttribute = 0,
}
bitflags! { # [ repr ( C ) ] pub struct NSLayoutFormatOptions : usize { const NSLayoutFormatAlignAllLeft = 2 ; const NSLayoutFormatAlignAllRight = 4 ; const NSLayoutFormatAlignAllTop = 8 ; const NSLayoutFormatAlignAllBottom = 16 ; const NSLayoutFormatAlignAllLeading = 32 ; const NSLayoutFormatAlignAllTrailing = 64 ; const NSLayoutFormatAlignAllCenterX = 512 ; const NSLayoutFormatAlignAllCenterY = 1024 ; const NSLayoutFormatAlignAllBaseline = 2048 ; const NSLayoutFormatAlignAllFirstBaseline = 4096 ; const NSLayoutFormatAlignmentMask = 65535 ; const NSLayoutFormatDirectionLeadingToTrailing = 0 ; const NSLayoutFormatDirectionLeftToRight = 65536 ; const NSLayoutFormatDirectionRightToLeft = 131072 ; const NSLayoutFormatDirectionMask = 196608 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLayoutConstraintOrientation {
  NSLayoutConstraintOrientationHorizontal = 0,
  NSLayoutConstraintOrientationVertical = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutConstraint;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutGuide;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageLoadStatus {
  NSImageLoadStatusCompleted = 0,
  NSImageLoadStatusCancelled = 1,
  NSImageLoadStatusInvalidData = 2,
  NSImageLoadStatusUnexpectedEOF = 3,
  NSImageLoadStatusReadError = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageCacheMode {
  NSImageCacheDefault = 0,
  NSImageCacheAlways = 1,
  NSImageCacheBySize = 2,
  NSImageCacheNever = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSImageResizingMode {
  NSImageResizingModeStretch = 0,
  NSImageResizingModeTile = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSImage;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __imageFlags {
  pub scalable: u32,
  pub dataRetained: u32,
  pub uniqueWindow: u32,
  pub sizeWasExplicitlySet: u32,
  pub builtIn: u32,
  pub needsToExpand: u32,
  pub useEPSOnResolutionMismatch: u32,
  pub matchesOnlyOnBestFittingAxis: u32,
  pub colorMatchPreferred: u32,
  pub multipleResolutionMatching: u32,
  pub focusedWhilePrinting: u32,
  pub archiveByName: u32,
  pub unboundedCacheDepth: u32,
  pub flipped: u32,
  pub aliased: u32,
  pub dirtied: u32,
  pub cacheMode: u32,
  pub sampleMode: u32,
  pub resMatchPreferred: u32,
  pub isTemplate: u32,
  pub failedToExpand: u32,
  pub reserved1: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSImageDelegateProto {
  #[objrs(selector = "imageDidNotDraw:inRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn imageDidNotDraw_inRect_(&self, sender: &NSImage, rect: CGRect) -> Option<Arc<NSImage>>;
  #[objrs(selector = "image:willLoadRepresentation:")]
  #[cfg(feature = "RK_AppKit")]
  fn image_willLoadRepresentation_(&self, image: &NSImage, rep: &NSImageRep) -> ();
  #[objrs(selector = "image:didLoadRepresentationHeader:")]
  #[cfg(feature = "RK_AppKit")]
  fn image_didLoadRepresentationHeader_(&self, image: &NSImage, rep: &NSImageRep) -> ();
  #[objrs(selector = "image:didLoadPartOfRepresentation:withValidRows:")]
  #[cfg(feature = "RK_AppKit")]
  fn image_didLoadPartOfRepresentation_withValidRows_(
    &self,
    image: &NSImage,
    rep: &NSImageRep,
    rows: isize,
  ) -> ();
  #[objrs(selector = "image:didLoadRepresentation:withStatus:")]
  #[cfg(feature = "RK_AppKit")]
  fn image_didLoadRepresentation_withStatus_(
    &self,
    image: &NSImage,
    rep: &NSImageRep,
    status: NSImageLoadStatus,
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageAlignment {
  NSImageAlignCenter = 0,
  NSImageAlignTop = 1,
  NSImageAlignTopLeft = 2,
  NSImageAlignTopRight = 3,
  NSImageAlignLeft = 4,
  NSImageAlignBottom = 5,
  NSImageAlignBottomLeft = 6,
  NSImageAlignBottomRight = 7,
  NSImageAlignRight = 8,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSImageFrameStyle {
  NSImageFrameNone = 0,
  NSImageFramePhoto = 1,
  NSImageFrameGrayBezel = 2,
  NSImageFrameGroove = 3,
  NSImageFrameButton = 4,
}
# [ objrs ( class , super = NSCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSImageCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ICFlags {
  pub _unused: u32,
  pub _animates: u32,
  pub _align: u32,
  pub _scale: u32,
  pub _style: u32,
}
#[repr(C)]
pub struct _NSImageCellAnimationState {
  opaque: u32,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSImageView;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __IVFlags {
  pub _hasImageSubview: u32,
  pub _usesSubview: u32,
  pub _hasCachedUsesSubview: u32,
  pub _unused: u32,
  pub _rejectsMultiFileDrops: u32,
  pub _compatibleScalingAndAlignment: u32,
  pub _reserved: u32,
  pub _overridesDrawing: u32,
  pub _allowsCutCopyPaste: u32,
  pub _editable: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSScrubberDataSourceProto {
  #[objrs(selector = "numberOfItemsForScrubber:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfItemsForScrubber_(&self, scrubber: &NSScrubber) -> isize;
  #[objrs(selector = "scrubber:viewForItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrubber_viewForItemAtIndex_(
    &self,
    scrubber: &NSScrubber,
    index: isize,
  ) -> Arc<NSScrubberItemView>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSScrubberDelegateProto {
  #[objrs(selector = "scrubber:didSelectItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrubber_didSelectItemAtIndex_(&self, scrubber: &NSScrubber, selectedIndex: isize) -> ();
  #[objrs(selector = "scrubber:didHighlightItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrubber_didHighlightItemAtIndex_(&self, scrubber: &NSScrubber, highlightedIndex: isize)
    -> ();
  #[objrs(selector = "scrubber:didChangeVisibleRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrubber_didChangeVisibleRange_(&self, scrubber: &NSScrubber, visibleRange: NSRange) -> ();
  #[objrs(selector = "didBeginInteractingWithScrubber:")]
  #[cfg(feature = "RK_AppKit")]
  fn didBeginInteractingWithScrubber_(&self, scrubber: &NSScrubber) -> ();
  #[objrs(selector = "didFinishInteractingWithScrubber:")]
  #[cfg(feature = "RK_AppKit")]
  fn didFinishInteractingWithScrubber_(&self, scrubber: &NSScrubber) -> ();
  #[objrs(selector = "didCancelInteractingWithScrubber:")]
  #[cfg(feature = "RK_AppKit")]
  fn didCancelInteractingWithScrubber_(&self, scrubber: &NSScrubber) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrubberMode {
  NSScrubberModeFixed = 0,
  NSScrubberModeFree = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrubberAlignment {
  NSScrubberAlignmentNone = 0,
  NSScrubberAlignmentLeading = 1,
  NSScrubberAlignmentTrailing = 2,
  NSScrubberAlignmentCenter = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberSelectionStyle;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubber;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberArrangedView;
# [ objrs ( class , super = NSScrubberArrangedView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberSelectionView;
# [ objrs ( class , super = NSScrubberArrangedView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberItemView;
# [ objrs ( class , super = NSScrubberItemView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberTextItemView;
# [ objrs ( class , super = NSScrubberItemView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberImageItemView;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberLayoutAttributes;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberLayout;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSScrubberFlowLayoutDelegateProto {
  #[objrs(selector = "scrubber:layout:sizeForItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrubber_layout_sizeForItemAtIndex_(
    &self,
    scrubber: &NSScrubber,
    layout: &NSScrubberFlowLayout,
    itemIndex: isize,
  ) -> CGSize;
}
# [ objrs ( class , super = NSScrubberLayout ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberFlowLayout;
# [ objrs ( class , super = NSScrubberLayout ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrubberProportionalLayout;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSharingService;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSharingContentScope {
  NSSharingContentScopeItem = 0,
  NSSharingContentScopePartial = 1,
  NSSharingContentScopeFull = 2,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSharingServiceDelegateProto {
  #[objrs(selector = "sharingService:willShareItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_willShareItems_(
    &self,
    sharingService: &NSSharingService,
    items: &NSArray,
  ) -> ();
  #[objrs(selector = "sharingService:didFailToShareItems:error:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_didFailToShareItems_error_(
    &self,
    sharingService: &NSSharingService,
    items: &NSArray,
    error: &NSError,
  ) -> ();
  #[objrs(selector = "sharingService:didShareItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_didShareItems_(&self, sharingService: &NSSharingService, items: &NSArray)
    -> ();
  #[objrs(selector = "sharingService:sourceFrameOnScreenForShareItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_sourceFrameOnScreenForShareItem_(
    &self,
    sharingService: &NSSharingService,
    item: &Object,
  ) -> CGRect;
  #[objrs(selector = "sharingService:transitionImageForShareItem:contentRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_transitionImageForShareItem_contentRect_(
    &self,
    sharingService: &NSSharingService,
    item: &Object,
    contentRect: &CGRect,
  ) -> Option<Arc<NSImage>>;
  #[objrs(selector = "sharingService:sourceWindowForShareItems:sharingContentScope:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_sourceWindowForShareItems_sharingContentScope_(
    &self,
    sharingService: &NSSharingService,
    items: &NSArray,
    sharingContentScope: &NSSharingContentScope,
  ) -> Option<Arc<NSWindow>>;
  #[objrs(selector = "anchoringViewForSharingService:showRelativeToRect:preferredEdge:")]
  #[cfg(feature = "RK_AppKit")]
  fn anchoringViewForSharingService_showRelativeToRect_preferredEdge_(
    &self,
    sharingService: &NSSharingService,
    positioningRect: &CGRect,
    preferredEdge: &NSRectEdge,
  ) -> Option<Arc<NSView>>;
}
bitflags! { # [ repr ( C ) ] pub struct NSCloudKitSharingServiceOptions : usize { const NSCloudKitSharingServiceStandard = 0 ; const NSCloudKitSharingServiceAllowPublic = 1 ; const NSCloudKitSharingServiceAllowPrivate = 2 ; const NSCloudKitSharingServiceAllowReadOnly = 16 ; const NSCloudKitSharingServiceAllowReadWrite = 32 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCloudSharingServiceDelegateProto {
  #[objrs(selector = "sharingService:didCompleteForItems:error:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingService_didCompleteForItems_error_(
    &self,
    sharingService: &NSSharingService,
    items: &NSArray,
    error: Option<&NSError>,
  ) -> ();
  #[objrs(selector = "optionsForSharingService:shareProvider:")]
  #[cfg(feature = "RK_AppKit")]
  fn optionsForSharingService_shareProvider_(
    &self,
    cloudKitSharingService: &NSSharingService,
    provider: &NSItemProvider,
  ) -> NSCloudKitSharingServiceOptions;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSharingServicePicker;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSharingServicePickerDelegateProto {
  #[objrs(selector = "sharingServicePicker:sharingServicesForItems:proposedSharingServices:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingServicePicker_sharingServicesForItems_proposedSharingServices_(
    &self,
    sharingServicePicker: &NSSharingServicePicker,
    items: &NSArray,
    proposedServices: &NSArray,
  ) -> Arc<NSArray>;
  #[objrs(selector = "sharingServicePicker:delegateForSharingService:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingServicePicker_delegateForSharingService_(
    &self,
    sharingServicePicker: &NSSharingServicePicker,
    sharingService: &NSSharingService,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "sharingServicePicker:didChooseSharingService:")]
  #[cfg(feature = "RK_AppKit")]
  fn sharingServicePicker_didChooseSharingService_(
    &self,
    sharingServicePicker: &NSSharingServicePicker,
    service: Option<&NSSharingService>,
  ) -> ();
}
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSharingServicePickerTouchBarItem;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSharingServicePickerTouchBarItemDelegateProto {
  #[objrs(selector = "itemsForSharingServicePickerTouchBarItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn itemsForSharingServicePickerTouchBarItem_(
    &self,
    pickerTouchBarItem: &NSSharingServicePickerTouchBarItem,
  ) -> Arc<NSArray>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSliderAccessory;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSliderAccessoryBehavior;
pub type NSSliderAccessoryWidth = f64;
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSliderTouchBarItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSpeechRecognizer;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSpeechRecognizerDelegateProto {
  #[objrs(selector = "speechRecognizer:didRecognizeCommand:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechRecognizer_didRecognizeCommand_(
    &self,
    sender: &NSSpeechRecognizer,
    command: &NSString,
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSpeechBoundary {
  NSSpeechImmediateBoundary = 0,
  NSSpeechWordBoundary = 1,
  NSSpeechSentenceBoundary = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSpeechSynthesizer;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSpeechSynthesizerDelegateProto {
  #[objrs(selector = "speechSynthesizer:didFinishSpeaking:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechSynthesizer_didFinishSpeaking_(
    &self,
    sender: &NSSpeechSynthesizer,
    finishedSpeaking: bool,
  ) -> ();
  #[objrs(selector = "speechSynthesizer:willSpeakWord:ofString:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechSynthesizer_willSpeakWord_ofString_(
    &self,
    sender: &NSSpeechSynthesizer,
    characterRange: NSRange,
    string: &NSString,
  ) -> ();
  #[objrs(selector = "speechSynthesizer:willSpeakPhoneme:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechSynthesizer_willSpeakPhoneme_(
    &self,
    sender: &NSSpeechSynthesizer,
    phonemeOpcode: i16,
  ) -> ();
  #[objrs(selector = "speechSynthesizer:didEncounterErrorAtIndex:ofString:message:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechSynthesizer_didEncounterErrorAtIndex_ofString_message_(
    &self,
    sender: &NSSpeechSynthesizer,
    characterIndex: usize,
    string: &NSString,
    message: &NSString,
  ) -> ();
  #[objrs(selector = "speechSynthesizer:didEncounterSyncMessage:")]
  #[cfg(feature = "RK_AppKit")]
  fn speechSynthesizer_didEncounterSyncMessage_(
    &self,
    sender: &NSSpeechSynthesizer,
    message: &NSString,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSpellChecker;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __scFlags {
  pub autoShowGuesses: u32,
  pub needDelayedGuess: u32,
  pub unignoreInProgress: u32,
  pub wordFieldEdited: u32,
  pub inSpelling: u32,
  pub reconnectSpelling: u32,
  pub inGrammar: u32,
  pub reconnectGrammar: u32,
  pub languageIdentification: u32,
  pub languagesHidden: u32,
  pub quotesByLanguage: u32,
  pub _reserved: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCorrectionResponse {
  NSCorrectionResponseNone = 0,
  NSCorrectionResponseAccepted = 1,
  NSCorrectionResponseRejected = 2,
  NSCorrectionResponseIgnored = 3,
  NSCorrectionResponseEdited = 4,
  NSCorrectionResponseReverted = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSCorrectionIndicatorType {
  NSCorrectionIndicatorTypeDefault = 0,
  NSCorrectionIndicatorTypeReversion = 1,
  NSCorrectionIndicatorTypeGuesses = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSplitViewDividerStyle {
  NSSplitViewDividerStyleThick = 1,
  NSSplitViewDividerStyleThin = 2,
  NSSplitViewDividerStylePaneSplitter = 3,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSplitView;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSplitViewDelegateProto {
  #[objrs(selector = "splitView:canCollapseSubview:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_canCollapseSubview_(&self, splitView: &NSSplitView, subview: &NSView) -> bool;
  #[objrs(selector = "splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex_(
    &self,
    splitView: &NSSplitView,
    subview: &NSView,
    dividerIndex: isize,
  ) -> bool;
  #[objrs(selector = "splitView:constrainMinCoordinate:ofSubviewAt:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_constrainMinCoordinate_ofSubviewAt_(
    &self,
    splitView: &NSSplitView,
    proposedMinimumPosition: f64,
    dividerIndex: isize,
  ) -> f64;
  #[objrs(selector = "splitView:constrainMaxCoordinate:ofSubviewAt:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_constrainMaxCoordinate_ofSubviewAt_(
    &self,
    splitView: &NSSplitView,
    proposedMaximumPosition: f64,
    dividerIndex: isize,
  ) -> f64;
  #[objrs(selector = "splitView:constrainSplitPosition:ofSubviewAt:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_constrainSplitPosition_ofSubviewAt_(
    &self,
    splitView: &NSSplitView,
    proposedPosition: f64,
    dividerIndex: isize,
  ) -> f64;
  #[objrs(selector = "splitView:resizeSubviewsWithOldSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_resizeSubviewsWithOldSize_(&self, splitView: &NSSplitView, oldSize: CGSize) -> ();
  #[objrs(selector = "splitView:shouldAdjustSizeOfSubview:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_shouldAdjustSizeOfSubview_(&self, splitView: &NSSplitView, view: &NSView) -> bool;
  #[objrs(selector = "splitView:shouldHideDividerAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_shouldHideDividerAtIndex_(
    &self,
    splitView: &NSSplitView,
    dividerIndex: isize,
  ) -> bool;
  #[objrs(selector = "splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex_(
    &self,
    splitView: &NSSplitView,
    proposedEffectiveRect: CGRect,
    drawnRect: CGRect,
    dividerIndex: isize,
  ) -> CGRect;
  #[objrs(selector = "splitView:additionalEffectiveRectOfDividerAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitView_additionalEffectiveRectOfDividerAtIndex_(
    &self,
    splitView: &NSSplitView,
    dividerIndex: isize,
  ) -> CGRect;
  #[objrs(selector = "splitViewWillResizeSubviews:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitViewWillResizeSubviews_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "splitViewDidResizeSubviews:")]
  #[cfg(feature = "RK_AppKit")]
  fn splitViewDidResizeSubviews_(&self, notification: &NSNotification) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSplitViewItemBehavior {
  NSSplitViewItemBehaviorDefault = 0,
  NSSplitViewItemBehaviorSidebar = 1,
  NSSplitViewItemBehaviorContentList = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSplitViewItemCollapseBehavior {
  NSSplitViewItemCollapseBehaviorDefault = 0,
  NSSplitViewItemCollapseBehaviorPreferResizingSplitViewWithFixedSiblings = 1,
  NSSplitViewItemCollapseBehaviorPreferResizingSiblingsWithFixedSplitView = 2,
  NSSplitViewItemCollapseBehaviorUseConstraints = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSplitViewItem;
# [ objrs ( class , super = NSViewController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSplitViewController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __SPFlags {
  pub saveMode: u32,
  pub isExpanded: u32,
  pub allowsOtherFileTypes: u32,
  pub canCreateDirectories: u32,
  pub reserved2: u32,
  pub delegate_shouldShowFilename: u32,
  pub delegate_compareFilename: u32,
  pub delegate_shouldEnableURL: u32,
  pub delegate_validateURL: u32,
  pub delegate_didChangeToDirectoryURL: u32,
  pub changingFrameSize: u32,
  pub movingAccessoryView: u32,
  pub userAccessoryViewFrameChange: u32,
  pub canChooseDirectories: u32,
  pub canChooseFiles: u32,
  pub delegate_selectionDidChange: u32,
  pub delegate_didChangeToDirectory: u32,
  pub calledWindowOrderedIn: u32,
  pub appCentric: u32,
  pub bottomControlsDisabled: u32,
  pub okButtonDisabled: u32,
  pub accessoryViewDisclosed: u32,
  pub delegate_isValidFilename: u32,
  pub delegate_userEnteredFilename: u32,
  pub delegate_panel_willExpand: u32,
  pub canResolveUbiquitousConflicts: u32,
  pub reserved: u32,
}
pub type _SPFlags = __SPFlags;
# [ objrs ( class , super = NSPanel ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSavePanel;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSOpenSavePanelDelegateProto {
  #[objrs(selector = "panel:shouldEnableURL:")]
  #[cfg(feature = "RK_AppKit")]
  fn panel_shouldEnableURL_(&self, sender: &Object, url: &NSURL) -> bool;
  #[objrs(selector = "panel:validateURL:error:")]
  #[cfg(feature = "RK_AppKit")]
  fn panel_validateURL_error_(
    &self,
    sender: &Object,
    url: &NSURL,
    outError: Option<&mut &Option<Arc<NSError>>>,
  ) -> bool;
  #[objrs(selector = "panel:didChangeToDirectoryURL:")]
  #[cfg(feature = "RK_AppKit")]
  fn panel_didChangeToDirectoryURL_(&self, sender: &Object, url: Option<&NSURL>) -> ();
  #[objrs(selector = "panel:userEnteredFilename:confirmed:")]
  #[cfg(feature = "RK_AppKit")]
  fn panel_userEnteredFilename_confirmed_(
    &self,
    sender: &Object,
    filename: &NSString,
    okFlag: bool,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "panel:willExpand:")]
  #[cfg(feature = "RK_AppKit")]
  fn panel_willExpand_(&self, sender: &Object, expanding: bool) -> ();
  #[objrs(selector = "panelSelectionDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn panelSelectionDidChange_(&self, sender: Option<&Object>) -> ();
}
# [ objrs ( class , super = NSSavePanel ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenPanel;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPageLayout;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPasteboardItem;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPasteboardItemDataProviderProto {
  #[objrs(selector = "pasteboard:item:provideDataForType:")]
  #[cfg(feature = "RK_AppKit")]
  fn pasteboard_item_provideDataForType_(
    &self,
    pasteboard: Option<&NSPasteboard>,
    item: &NSPasteboardItem,
    type_: &NSString,
  ) -> ();
  #[objrs(selector = "pasteboardFinishedWithDataProvider:")]
  #[cfg(feature = "RK_AppKit")]
  fn pasteboardFinishedWithDataProvider_(&self, pasteboard: &NSPasteboard) -> ();
}
# [ objrs ( class , super = NSTouchBarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPopoverTouchBarItem;
# [ objrs ( class , super = NSButtonCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMenuItemCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __micFlags {
  pub needsSizing: u32,
  pub reserved: u32,
  pub needsDisplay: u32,
  pub keyEquivGlyphWidth: u32,
  pub uniqueAgainstMain: u32,
  pub RESERVED: u32,
}
# [ objrs ( class , super = NSButton ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPopUpButton;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pbFlags {
  pub needsPullsDownFromTemplate: u32,
  pub RESERVED: u32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPrintingPageOrder {
  NSDescendingPageOrder = -1,
  NSSpecialPageOrder = 0,
  NSAscendingPageOrder = 1,
  NSUnknownPageOrder = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPrintRenderingQuality {
  NSPrintRenderingQualityBest = 0,
  NSPrintRenderingQualityResponsive = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPrintOperation;
bitflags! { # [ repr ( C ) ] pub struct NSPrintPanelOptions : usize { const NSPrintPanelShowsCopies = 1 ; const NSPrintPanelShowsPageRange = 2 ; const NSPrintPanelShowsPaperSize = 4 ; const NSPrintPanelShowsOrientation = 8 ; const NSPrintPanelShowsScaling = 16 ; const NSPrintPanelShowsPrintSelection = 32 ; const NSPrintPanelShowsPageSetupAccessory = 256 ; const NSPrintPanelShowsPreview = 131072 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPrintPanelAccessorizingProto {
  #[objrs(selector = "localizedSummaryItems")]
  #[cfg(feature = "RK_AppKit")]
  fn localizedSummaryItems(&self) -> Arc<NSArray>;
  #[objrs(selector = "keyPathsForValuesAffectingPreview")]
  #[cfg(feature = "RK_AppKit")]
  fn keyPathsForValuesAffectingPreview(&self) -> Arc<NSSet>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPrintPanel;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPDFInfo;
bitflags! { # [ repr ( C ) ] pub struct NSPDFPanelOptions : isize { const NSPDFPanelShowsPaperSize = 4 ; const NSPDFPanelShowsOrientation = 8 ; const NSPDFPanelRequestsParentDirectory = 16777216 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPDFPanel;
bitflags! { # [ repr ( C ) ] pub struct NSMediaLibrary : usize { const NSMediaLibraryAudio = 1 ; const NSMediaLibraryImage = 2 ; const NSMediaLibraryMovie = 4 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMediaLibraryBrowserController;
#[repr(C)]
pub struct NSScreenAuxiliary {
  opaque: u32,
}
pub type NSScreenAuxiliaryOpaque = NSScreenAuxiliary;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScreen;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSUsableScrollerParts {
  NSNoScrollerParts = 0,
  NSOnlyScrollerArrows = 1,
  NSAllScrollerParts = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSScrollerPart {
  NSScrollerNoPart = 0,
  NSScrollerDecrementPage = 1,
  NSScrollerKnob = 2,
  NSScrollerIncrementPage = 3,
  NSScrollerDecrementLine = 4,
  NSScrollerIncrementLine = 5,
  NSScrollerKnobSlot = 6,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrollerStyle {
  NSScrollerStyleLegacy = 0,
  NSScrollerStyleOverlay = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrollerKnobStyle {
  NSScrollerKnobStyleDefault = 0,
  NSScrollerKnobStyleDark = 1,
  NSScrollerKnobStyleLight = 2,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScroller;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sFlags2 {
  pub hitPart: u32,
  pub controlSize: u32,
  pub inMaxEnd: u32,
  pub setFloatValueOverridden: u32,
  pub setFloatValueKnobProportionOverridden: u32,
  pub style: u32,
  pub styleCompatibility: u32,
  pub overlayScrollerState: u32,
  pub knobStyle: u32,
  pub sbPaused: u32,
  pub isAnimatingKnob: u32,
  pub isTrackingMouse: u32,
  pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _sFlags {
  pub isHoriz: u32,
  pub arrowsLoc: usize,
  pub partsUsable: NSUsableScrollerParts,
  pub fine: u32,
  pub needsEnableFlush: u32,
  pub thumbing: u32,
  pub slotDrawn: u32,
  pub knobDrawn: u32,
  pub lit: u32,
  pub knobLit: u32,
  pub reserved: u32,
  pub controlTint: u32,
  pub repeatCount: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSScrollArrowPosition {
  NSScrollerArrowsMaxEnd = 0,
  NSScrollerArrowsMinEnd = 1,
  NSScrollerArrowsNone = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSScrollerArrow {
  NSScrollerIncrementArrow = 0,
  NSScrollerDecrementArrow = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTextFinderAction {
  NSTextFinderActionShowFindInterface = 1,
  NSTextFinderActionNextMatch = 2,
  NSTextFinderActionPreviousMatch = 3,
  NSTextFinderActionReplaceAll = 4,
  NSTextFinderActionReplace = 5,
  NSTextFinderActionReplaceAndFind = 6,
  NSTextFinderActionSetSearchString = 7,
  NSTextFinderActionReplaceAllInSelection = 8,
  NSTextFinderActionSelectAll = 9,
  NSTextFinderActionSelectAllInSelection = 10,
  NSTextFinderActionHideFindInterface = 11,
  NSTextFinderActionShowReplaceInterface = 12,
  NSTextFinderActionHideReplaceInterface = 13,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTextFinderMatchingType {
  NSTextFinderMatchingTypeContains = 0,
  NSTextFinderMatchingTypeStartsWith = 1,
  NSTextFinderMatchingTypeFullWord = 2,
  NSTextFinderMatchingTypeEndsWith = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextFinder;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextFinderClientProto {
  #[objrs(selector = "stringAtIndex:effectiveRange:endsWithSearchBoundary:")]
  #[cfg(feature = "RK_AppKit")]
  fn stringAtIndex_effectiveRange_endsWithSearchBoundary_(
    &self,
    characterIndex: usize,
    outRange: &NSRange,
    outFlag: &bool,
  ) -> Arc<NSString>;
  #[objrs(selector = "stringLength")]
  #[cfg(feature = "RK_AppKit")]
  fn stringLength(&self) -> usize;
  #[objrs(selector = "scrollRangeToVisible:")]
  #[cfg(feature = "RK_AppKit")]
  fn scrollRangeToVisible_(&self, range: NSRange) -> ();
  #[objrs(selector = "shouldReplaceCharactersInRanges:withStrings:")]
  #[cfg(feature = "RK_AppKit")]
  fn shouldReplaceCharactersInRanges_withStrings_(
    &self,
    ranges: &NSArray,
    strings: &NSArray,
  ) -> bool;
  #[objrs(selector = "replaceCharactersInRange:withString:")]
  #[cfg(feature = "RK_AppKit")]
  fn replaceCharactersInRange_withString_(&self, range: NSRange, string: &NSString) -> ();
  #[objrs(selector = "didReplaceCharacters")]
  #[cfg(feature = "RK_AppKit")]
  fn didReplaceCharacters(&self) -> ();
  #[objrs(selector = "contentViewAtIndex:effectiveCharacterRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn contentViewAtIndex_effectiveCharacterRange_(
    &self,
    index: usize,
    outRange: &NSRange,
  ) -> Arc<NSView>;
  #[objrs(selector = "rectsForCharacterRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn rectsForCharacterRange_(&self, range: NSRange) -> Option<Arc<NSArray>>;
  #[objrs(selector = "drawCharactersInRange:forContentView:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawCharactersInRange_forContentView_(&self, range: NSRange, view: &NSView) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextFinderBarContainerProto {
  #[objrs(selector = "findBarViewDidChangeHeight")]
  #[cfg(feature = "RK_AppKit")]
  fn findBarViewDidChangeHeight(&self) -> ();
  #[objrs(selector = "contentView")]
  #[cfg(feature = "RK_AppKit")]
  fn contentView(&self) -> Option<Arc<NSView>>;
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrollElasticity {
  NSScrollElasticityAutomatic = 0,
  NSScrollElasticityNone = 1,
  NSScrollElasticityAllowed = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SFlags {
  pub RESERVED1: u32,
  pub unarchiving: u32,
  pub registeredForWindowWillClose: u32,
  pub findBarPosition: u32,
  pub predominantAxisScrolling: u32,
  pub hContentElasticity: u32,
  pub vContentElasticity: u32,
  pub RESERVED2: u32,
  pub findBarVisible: u32,
  pub autoforwardsScrollWheelEvents: u32,
  pub autohidesScrollers: u32,
  pub RESERVED3: u32,
  pub focusRingNeedsRedisplay: u32,
  pub skipRemoveSuperviewCheck: u32,
  pub doesNotDrawBackground: u32,
  pub needsTile: u32,
  pub hasVerticalRuler: u32,
  pub hasHorizontalRuler: u32,
  pub showRulers: u32,
  pub oldRulerInstalled: u32,
  pub borderType: NSBorderType,
  pub noDynamicScrolling: u32,
  pub hScrollerStatus: u32,
  pub vScrollerStatus: u32,
  pub hScrollerRequired: u32,
  pub vScrollerRequired: u32,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSScrollView;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSScrollViewFindBarPosition {
  NSScrollViewFindBarPositionAboveHorizontalRuler = 0,
  NSScrollViewFindBarPositionAboveContent = 1,
  NSScrollViewFindBarPositionBelowContent = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSegmentSwitchTracking {
  NSSegmentSwitchTrackingSelectOne = 0,
  NSSegmentSwitchTrackingSelectAny = 1,
  NSSegmentSwitchTrackingMomentary = 2,
  NSSegmentSwitchTrackingMomentaryAccelerator = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSegmentStyle {
  NSSegmentStyleAutomatic = 0,
  NSSegmentStyleRounded = 1,
  NSSegmentStyleRoundRect = 3,
  NSSegmentStyleTexturedSquare = 4,
  NSSegmentStyleSmallSquare = 6,
  NSSegmentStyleSeparated = 8,
  NSSegmentStyleTexturedRounded = 2,
  NSSegmentStyleCapsule = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSegmentDistribution {
  NSSegmentDistributionFit = 0,
  NSSegmentDistributionFill = 1,
  NSSegmentDistributionFillEqually = 2,
  NSSegmentDistributionFillProportionally = 3,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSegmentedControl;
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSegmentedCell;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTickMarkPosition {
  NSTickMarkPositionBelow = 0,
  NSTickMarkPositionAbove = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSliderType {
  NSSliderTypeLinear = 0,
  NSSliderTypeCircular = 1,
}
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSliderCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sliderCellFlags {
  pub weAreVertical: u32,
  pub weAreVerticalSet: u32,
  pub weHaveStickyOrientation: u32,
  pub isPressed: u32,
  pub allowsTickMarkValuesOnly: u32,
  pub tickMarkPosition: u32,
  pub sliderType: u32,
  pub drawing: u32,
  pub snappedToTickMark: u32,
  pub snappedToPreviousValue: u32,
  pub snappedToDefaultValue: u32,
  pub snappingAllowed: u32,
  pub collapseOnResize: u32,
  pub startedOnAccessory: u32,
  pub reserved2: u32,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSlider;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSStackViewGravity {
  NSStackViewGravityTop = 1,
  NSStackViewGravityCenter = 2,
  NSStackViewGravityBottom = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSStackViewDistribution {
  NSStackViewDistributionGravityAreas = -1,
  NSStackViewDistributionFill = 0,
  NSStackViewDistributionFillEqually = 1,
  NSStackViewDistributionFillProportionally = 2,
  NSStackViewDistributionEqualSpacing = 3,
  NSStackViewDistributionEqualCentering = 4,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStackView;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSStackViewDelegateProto {
  #[objrs(selector = "stackView:willDetachViews:")]
  #[cfg(feature = "RK_AppKit")]
  fn stackView_willDetachViews_(&self, stackView: &NSStackView, views: &NSArray) -> ();
  #[objrs(selector = "stackView:didReattachViews:")]
  #[cfg(feature = "RK_AppKit")]
  fn stackView_didReattachViews_(&self, stackView: &NSStackView, views: &NSArray) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSGridCellPlacement {
  NSGridCellPlacementInherited = 0,
  NSGridCellPlacementNone = 1,
  NSGridCellPlacementLeading = 2,
  NSGridCellPlacementTrailing = 3,
  NSGridCellPlacementCenter = 4,
  NSGridCellPlacementFill = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSGridRowAlignment {
  NSGridRowAlignmentInherited = 0,
  NSGridRowAlignmentNone = 1,
  NSGridRowAlignmentFirstBaseline = 2,
  NSGridRowAlignmentLastBaseline = 3,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGridView;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGridRow;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGridColumn;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGridCell;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextFieldBezelStyle {
  NSTextFieldSquareBezel = 0,
  NSTextFieldRoundedBezel = 1,
}
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextFieldCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __tfFlags {
  pub drawsBackground: u32,
  pub bezelStyle: u32,
  pub thcSortDirection: u32,
  pub thcSortPriority: u32,
  pub mini: u32,
  pub textColorIgnoresNormalDisableFlag: u32,
  pub textColorDisableFlag: u32,
  pub thcForceHighlightForSort: u32,
  pub invalidTextColor: u32,
  pub notificationForMarkedText: u32,
  pub inToolbar: u32,
  pub hasTextLayer: u32,
  pub isButtonTitle: u32,
  pub allowTightening: u32,
  pub thcHighlighted: u32,
  pub shouldNotClipToBounds: u32,
  pub allowsDefaultTightening: u32,
  pub enableCP: u32,
  pub automaticCompletionDisabled: u32,
  pub determiningMenuItemTextColor: u32,
  pub reservedTextFieldCell: u32,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextField;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextFieldDelegateProto {
  #[objrs(selector = "textField:textView:candidatesForSelectedRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textField_textView_candidatesForSelectedRange_(
    &self,
    textField: &NSTextField,
    textView: &NSTextView,
    selectedRange: NSRange,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "textField:textView:candidates:forSelectedRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textField_textView_candidates_forSelectedRange_(
    &self,
    textField: &NSTextField,
    textView: &NSTextView,
    candidates: &NSArray,
    selectedRange: NSRange,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textField:textView:shouldSelectCandidateAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textField_textView_shouldSelectCandidateAtIndex_(
    &self,
    textField: &NSTextField,
    textView: &NSTextView,
    index: usize,
  ) -> bool;
}
bitflags! { # [ repr ( C ) ] pub struct NSUnderlineStyle : isize { const NSUnderlineStyleNone = 0 ; const NSUnderlineStyleSingle = 1 ; const NSUnderlineStyleThick = 2 ; const NSUnderlineStyleDouble = 9 ; const NSUnderlineStylePatternDot = 256 ; const NSUnderlineStylePatternDash = 512 ; const NSUnderlineStylePatternDashDot = 768 ; const NSUnderlineStylePatternDashDotDot = 1024 ; const NSUnderlineStyleByWord = 32768 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSWritingDirectionFormatType {
  NSWritingDirectionEmbedding = 0,
  NSWritingDirectionOverride = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSSpellingState {
  NSSpellingStateSpellingFlag = 1,
  NSSpellingStateGrammarFlag = 2,
}
bitflags! { # [ repr ( C ) ] pub struct NSTextStorageEditActions : usize { const NSTextStorageEditedAttributes = 1 ; const NSTextStorageEditedCharacters = 2 ; } }
# [ objrs ( class , super = NSMutableAttributedString ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextStorage;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextStorageDelegateProto {
  #[objrs(selector = "textStorage:willProcessEditing:range:changeInLength:")]
  #[cfg(feature = "RK_AppKit")]
  fn textStorage_willProcessEditing_range_changeInLength_(
    &self,
    textStorage: &NSTextStorage,
    editedMask: NSTextStorageEditActions,
    editedRange: NSRange,
    delta: isize,
  ) -> ();
  #[objrs(selector = "textStorage:didProcessEditing:range:changeInLength:")]
  #[cfg(feature = "RK_AppKit")]
  fn textStorage_didProcessEditing_range_changeInLength_(
    &self,
    textStorage: &NSTextStorage,
    editedMask: NSTextStorageEditActions,
    editedRange: NSRange,
    delta: isize,
  ) -> ();
}
pub type NSTextStorageEditedOptions = usize;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSGlyphStorageProto {
  #[objrs(selector = "insertGlyphs:length:forStartingGlyphAtIndex:characterIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertGlyphs_length_forStartingGlyphAtIndex_characterIndex_(
    &self,
    glyphs: &u32,
    length: usize,
    glyphIndex: usize,
    charIndex: usize,
  ) -> ();
  #[objrs(selector = "setIntAttribute:value:forGlyphAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn setIntAttribute_value_forGlyphAtIndex_(
    &self,
    attributeTag: isize,
    val: isize,
    glyphIndex: usize,
  ) -> ();
  #[objrs(selector = "attributedString")]
  #[cfg(feature = "RK_AppKit")]
  fn attributedString(&self) -> Arc<NSAttributedString>;
  #[objrs(selector = "layoutOptions")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutOptions(&self) -> usize;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGlyphGenerator;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTextLayoutOrientation {
  NSTextLayoutOrientationHorizontal = 0,
  NSTextLayoutOrientationVertical = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSGlyphProperty : isize { const NSGlyphPropertyNull = 1 ; const NSGlyphPropertyControlCharacter = 2 ; const NSGlyphPropertyElastic = 4 ; const NSGlyphPropertyNonBaseCharacter = 8 ; } }
bitflags! { # [ repr ( C ) ] pub struct NSControlCharacterAction : isize { const NSControlCharacterActionZeroAdvancement = 1 ; const NSControlCharacterActionWhitespace = 2 ; const NSControlCharacterActionHorizontalTab = 4 ; const NSControlCharacterActionLineBreak = 8 ; const NSControlCharacterActionParagraphBreak = 16 ; const NSControlCharacterActionContainerBreak = 32 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextLayoutOrientationProviderProto {}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTypesetterBehavior {
  NSTypesetterLatestBehavior = -1,
  NSTypesetterOriginalBehavior = 0,
  NSTypesetterBehavior_10_2_WithCompatibility = 1,
  NSTypesetterBehavior_10_2 = 2,
  NSTypesetterBehavior_10_3 = 3,
  NSTypesetterBehavior_10_4 = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLayoutManager;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __lmFlags {
  pub containersAreFull: u32,
  pub glyphsMightDrawOutsideLines: u32,
  pub backgroundLayoutEnabled: u32,
  pub resizingInProgress: u32,
  pub allowScreenFonts: u32,
  pub cachedRectArrayInUse: u32,
  pub displayInvalidationInProgress: u32,
  pub insertionPointNeedsUpdate: u32,
  pub layoutManagerInDirtyList: u32,
  pub originalFontOverride: u32,
  pub showInvisibleCharacters: u32,
  pub showControlCharacters: u32,
  pub delegateRespondsToDidInvalidate: u32,
  pub delegateRespondsToDidComplete: u32,
  pub glyphFormat: u32,
  pub textStorageRespondsToIsEditing: u32,
  pub notifyEditedInProgress: u32,
  pub containersChanged: u32,
  pub isGeneratingGlyphs: u32,
  pub hasNonGeneratedGlyphData: u32,
  pub inBackgroundLayout: u32,
  pub syncAlignmentToDirection: u32,
  pub defaultAttachmentScaling: u32,
  pub usesFontLeading: u32,
  pub seenRightToLeft: u32,
  pub ignoresViewTransformations: u32,
  pub needToFlushGlyph: u32,
  pub flipsIfNeeded: u32,
  pub allowNonContig: u32,
  pub useNonContig: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSLayoutManagerDelegateProto {
  #[objrs(
    selector = "layoutManager:shouldGenerateGlyphs:properties:characterIndexes:font:forGlyphRange:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldGenerateGlyphs_properties_characterIndexes_font_forGlyphRange_(
    &self,
    layoutManager: &NSLayoutManager,
    glyphs: &u16,
    props: &NSGlyphProperty,
    charIndexes: &usize,
    aFont: &NSFont,
    glyphRange: NSRange,
  ) -> usize;
  #[objrs(selector = "layoutManager:lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect_(
    &self,
    layoutManager: &NSLayoutManager,
    glyphIndex: usize,
    rect: CGRect,
  ) -> f64;
  #[objrs(
    selector = "layoutManager:paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect_(
    &self,
    layoutManager: &NSLayoutManager,
    glyphIndex: usize,
    rect: CGRect,
  ) -> f64;
  #[objrs(
    selector = "layoutManager:paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect_(
    &self,
    layoutManager: &NSLayoutManager,
    glyphIndex: usize,
    rect: CGRect,
  ) -> f64;
  #[objrs(selector = "layoutManager:shouldUseAction:forControlCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldUseAction_forControlCharacterAtIndex_(
    &self,
    layoutManager: &NSLayoutManager,
    action: NSControlCharacterAction,
    charIndex: usize,
  ) -> NSControlCharacterAction;
  #[objrs(selector = "layoutManager:shouldBreakLineByWordBeforeCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldBreakLineByWordBeforeCharacterAtIndex_(
    &self,
    layoutManager: &NSLayoutManager,
    charIndex: usize,
  ) -> bool;
  #[objrs(selector = "layoutManager:shouldBreakLineByHyphenatingBeforeCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldBreakLineByHyphenatingBeforeCharacterAtIndex_(
    &self,
    layoutManager: &NSLayoutManager,
    charIndex: usize,
  ) -> bool;
  #[objrs(
    selector = "layoutManager:boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex_(
    &self,
    layoutManager: &NSLayoutManager,
    glyphIndex: usize,
    textContainer: &NSTextContainer,
    proposedRect: CGRect,
    glyphPosition: CGPoint,
    charIndex: usize,
  ) -> CGRect;
  #[objrs(
    selector = "layoutManager:shouldSetLineFragmentRect:lineFragmentUsedRect:baselineOffset:inTextContainer:forGlyphRange:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldSetLineFragmentRect_lineFragmentUsedRect_baselineOffset_inTextContainer_forGlyphRange_(
    &self,
    layoutManager: &NSLayoutManager,
    lineFragmentRect: &CGRect,
    lineFragmentUsedRect: &CGRect,
    baselineOffset: &f64,
    textContainer: &NSTextContainer,
    glyphRange: NSRange,
  ) -> bool;
  #[objrs(selector = "layoutManagerDidInvalidateLayout:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManagerDidInvalidateLayout_(&self, sender: &NSLayoutManager) -> ();
  #[objrs(selector = "layoutManager:didCompleteLayoutForTextContainer:atEnd:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_didCompleteLayoutForTextContainer_atEnd_(
    &self,
    layoutManager: &NSLayoutManager,
    textContainer: Option<&NSTextContainer>,
    layoutFinishedFlag: bool,
  ) -> ();
  #[objrs(selector = "layoutManager:textContainer:didChangeGeometryFromSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_textContainer_didChangeGeometryFromSize_(
    &self,
    layoutManager: &NSLayoutManager,
    textContainer: &NSTextContainer,
    oldSize: CGSize,
  ) -> ();
  #[objrs(
    selector = "layoutManager:shouldUseTemporaryAttributes:forDrawingToScreen:atCharacterIndex:effectiveRange:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn layoutManager_shouldUseTemporaryAttributes_forDrawingToScreen_atCharacterIndex_effectiveRange_(
    &self,
    layoutManager: &NSLayoutManager,
    attrs: &NSDictionary,
    toScreen: bool,
    charIndex: usize,
    effectiveCharRange: Option<&NSRange>,
  ) -> Option<Arc<NSDictionary>>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSGlyphInscription {
  NSGlyphInscribeBase = 0,
  NSGlyphInscribeBelow = 1,
  NSGlyphInscribeAbove = 2,
  NSGlyphInscribeOverstrike = 3,
  NSGlyphInscribeOverBelow = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextContainer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __tcFlags {
  pub widthTracksTextView: u16,
  pub heightTracksTextView: u16,
  pub observingFrameChanges: u16,
  pub lineBreakMode: u16,
  pub oldAPI: u16,
  pub _reserved: u16,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLineSweepDirection {
  NSLineSweepLeft = 0,
  NSLineSweepRight = 1,
  NSLineSweepDown = 2,
  NSLineSweepUp = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLineMovementDirection {
  NSLineDoesntMove = 0,
  NSLineMovesLeft = 1,
  NSLineMovesRight = 2,
  NSLineMovesDown = 3,
  NSLineMovesUp = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTokenStyle {
  NSTokenStyleDefault = 0,
  NSTokenStyleNone = 1,
  NSTokenStyleRounded = 2,
  NSTokenStyleSquared = 3,
  NSTokenStylePlainSquared = 4,
}
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTokenFieldCell;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTokenFieldCellDelegateProto {
  #[objrs(selector = "tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    substring: &NSString,
    tokenIndex: isize,
    selectedIndex: &isize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "tokenFieldCell:shouldAddObjects:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_shouldAddObjects_atIndex_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    tokens: &NSArray,
    index: usize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "tokenFieldCell:displayStringForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_displayStringForRepresentedObject_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    representedObject: &Object,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "tokenFieldCell:editingStringForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_editingStringForRepresentedObject_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    representedObject: &Object,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "tokenFieldCell:representedObjectForEditingString:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_representedObjectForEditingString_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    editingString: &NSString,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "tokenFieldCell:writeRepresentedObjects:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_writeRepresentedObjects_toPasteboard_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    objects: &NSArray,
    pboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "tokenFieldCell:readFromPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_readFromPasteboard_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    pboard: &NSPasteboard,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "tokenFieldCell:menuForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_menuForRepresentedObject_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    representedObject: &Object,
  ) -> Option<Arc<NSMenu>>;
  #[objrs(selector = "tokenFieldCell:hasMenuForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_hasMenuForRepresentedObject_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    representedObject: &Object,
  ) -> bool;
  #[objrs(selector = "tokenFieldCell:styleForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenFieldCell_styleForRepresentedObject_(
    &self,
    tokenFieldCell: &NSTokenFieldCell,
    representedObject: &Object,
  ) -> NSTokenStyle;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTokenFieldDelegateProto {
  #[objrs(selector = "tokenField:completionsForSubstring:indexOfToken:indexOfSelectedItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_completionsForSubstring_indexOfToken_indexOfSelectedItem_(
    &self,
    tokenField: &NSTokenField,
    substring: &NSString,
    tokenIndex: isize,
    selectedIndex: Option<&isize>,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "tokenField:shouldAddObjects:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_shouldAddObjects_atIndex_(
    &self,
    tokenField: &NSTokenField,
    tokens: &NSArray,
    index: usize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "tokenField:displayStringForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_displayStringForRepresentedObject_(
    &self,
    tokenField: &NSTokenField,
    representedObject: &Object,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "tokenField:editingStringForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_editingStringForRepresentedObject_(
    &self,
    tokenField: &NSTokenField,
    representedObject: &Object,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "tokenField:representedObjectForEditingString:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_representedObjectForEditingString_(
    &self,
    tokenField: &NSTokenField,
    editingString: &NSString,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "tokenField:writeRepresentedObjects:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_writeRepresentedObjects_toPasteboard_(
    &self,
    tokenField: &NSTokenField,
    objects: &NSArray,
    pboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "tokenField:readFromPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_readFromPasteboard_(
    &self,
    tokenField: &NSTokenField,
    pboard: &NSPasteboard,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "tokenField:menuForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_menuForRepresentedObject_(
    &self,
    tokenField: &NSTokenField,
    representedObject: &Object,
  ) -> Option<Arc<NSMenu>>;
  #[objrs(selector = "tokenField:hasMenuForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_hasMenuForRepresentedObject_(
    &self,
    tokenField: &NSTokenField,
    representedObject: &Object,
  ) -> bool;
  #[objrs(selector = "tokenField:styleForRepresentedObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn tokenField_styleForRepresentedObject_(
    &self,
    tokenField: &NSTokenField,
    representedObject: &Object,
  ) -> NSTokenStyle;
}
# [ objrs ( class , super = NSTextField ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTokenField;
bitflags! { # [ repr ( C ) ] pub struct NSTrackingAreaOptions : usize { const NSTrackingMouseEnteredAndExited = 1 ; const NSTrackingMouseMoved = 2 ; const NSTrackingCursorUpdate = 4 ; const NSTrackingActiveWhenFirstResponder = 16 ; const NSTrackingActiveInKeyWindow = 32 ; const NSTrackingActiveInActiveApp = 64 ; const NSTrackingActiveAlways = 128 ; const NSTrackingAssumeInside = 256 ; const NSTrackingInVisibleRect = 512 ; const NSTrackingEnabledDuringMouseDrag = 1024 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTrackingArea;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWindowTab;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWindowTabGroup;
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSWindowController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wcFlags {
  pub shouldCloseDocument: u32,
  pub shouldCascade: u32,
  pub nibIsLoaded: u32,
  pub nibNameIsPath: u32,
  pub settingWindowsContentViewController: u32,
  pub didInitWithCoder: u32,
  pub nibIsMakingConnections: u32,
  pub sentWindowWillLoad: u32,
  pub RESERVED: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSComboBoxDataSourceProto {
  #[objrs(selector = "numberOfItemsInComboBox:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfItemsInComboBox_(&self, comboBox: &NSComboBox) -> isize;
  #[objrs(selector = "comboBox:objectValueForItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBox_objectValueForItemAtIndex_(
    &self,
    comboBox: &NSComboBox,
    index: isize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "comboBox:indexOfItemWithStringValue:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBox_indexOfItemWithStringValue_(&self, comboBox: &NSComboBox, string: &NSString)
    -> usize;
  #[objrs(selector = "comboBox:completedString:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBox_completedString_(
    &self,
    comboBox: &NSComboBox,
    string: &NSString,
  ) -> Option<Arc<NSString>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSComboBoxDelegateProto {
  #[objrs(selector = "comboBoxWillPopUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxWillPopUp_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "comboBoxWillDismiss:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxWillDismiss_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "comboBoxSelectionDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxSelectionDidChange_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "comboBoxSelectionIsChanging:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxSelectionIsChanging_(&self, notification: &NSNotification) -> ();
}
# [ objrs ( class , super = NSTextField ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSComboBox;
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSComboBoxCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __cbcFlags {
  pub usesDataSource: u32,
  pub completes: u32,
  pub buttonBordered: u32,
  pub popUpIsUp: u32,
  pub filteringEvents: u32,
  pub drawing: u32,
  pub synchronizingSelection: u32,
  pub reserved: u32,
  pub visibleItems: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSComboBoxCellDataSourceProto {
  #[objrs(selector = "numberOfItemsInComboBoxCell:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfItemsInComboBoxCell_(&self, comboBoxCell: &NSComboBoxCell) -> isize;
  #[objrs(selector = "comboBoxCell:objectValueForItemAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxCell_objectValueForItemAtIndex_(
    &self,
    comboBoxCell: &NSComboBoxCell,
    index: isize,
  ) -> Arc<Object>;
  #[objrs(selector = "comboBoxCell:indexOfItemWithStringValue:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxCell_indexOfItemWithStringValue_(
    &self,
    comboBoxCell: &NSComboBoxCell,
    string: &NSString,
  ) -> usize;
  #[objrs(selector = "comboBoxCell:completedString:")]
  #[cfg(feature = "RK_AppKit")]
  fn comboBoxCell_completedString_(
    &self,
    comboBoxCell: &NSComboBoxCell,
    uncompletedString: &NSString,
  ) -> Option<Arc<NSString>>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextInputProto {
  #[objrs(selector = "insertText:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertText_(&self, string: Option<&Object>) -> ();
  #[objrs(selector = "doCommandBySelector:")]
  #[cfg(feature = "RK_AppKit")]
  fn doCommandBySelector_(&self, selector: SelectorRef) -> ();
  #[objrs(selector = "setMarkedText:selectedRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn setMarkedText_selectedRange_(&self, string: Option<&Object>, selRange: NSRange) -> ();
  #[objrs(selector = "unmarkText")]
  #[cfg(feature = "RK_AppKit")]
  fn unmarkText(&self) -> ();
  #[objrs(selector = "hasMarkedText")]
  #[cfg(feature = "RK_AppKit")]
  fn hasMarkedText(&self) -> bool;
  #[objrs(selector = "conversationIdentifier")]
  #[cfg(feature = "RK_AppKit")]
  fn conversationIdentifier(&self) -> isize;
  #[objrs(selector = "attributedSubstringFromRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn attributedSubstringFromRange_(&self, range: NSRange) -> Option<Arc<NSAttributedString>>;
  #[objrs(selector = "markedRange")]
  #[cfg(feature = "RK_AppKit")]
  fn markedRange(&self) -> NSRange;
  #[objrs(selector = "selectedRange")]
  #[cfg(feature = "RK_AppKit")]
  fn selectedRange(&self) -> NSRange;
  #[objrs(selector = "firstRectForCharacterRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn firstRectForCharacterRange_(&self, range: NSRange) -> CGRect;
  #[objrs(selector = "characterIndexForPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn characterIndexForPoint_(&self, point: CGPoint) -> usize;
  #[objrs(selector = "validAttributesForMarkedText")]
  #[cfg(feature = "RK_AppKit")]
  fn validAttributesForMarkedText(&self) -> Option<Arc<NSArray>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSInputManager;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextAttachmentContainerProto {
  #[objrs(selector = "imageForBounds:textContainer:characterIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn imageForBounds_textContainer_characterIndex_(
    &self,
    imageBounds: CGRect,
    textContainer: Option<&NSTextContainer>,
    charIndex: usize,
  ) -> Option<Arc<NSImage>>;
  #[objrs(
    selector = "attachmentBoundsForTextContainer:proposedLineFragment:glyphPosition:characterIndex:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn attachmentBoundsForTextContainer_proposedLineFragment_glyphPosition_characterIndex_(
    &self,
    textContainer: Option<&NSTextContainer>,
    lineFrag: CGRect,
    position: CGPoint,
    charIndex: usize,
  ) -> CGRect;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextAttachment;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextAttachmentCellProto {
  #[objrs(selector = "drawWithFrame:inView:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawWithFrame_inView_(&self, cellFrame: CGRect, controlView: Option<&NSView>) -> ();
  #[objrs(selector = "wantsToTrackMouse")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsToTrackMouse(&self) -> bool;
  #[objrs(selector = "highlight:withFrame:inView:")]
  #[cfg(feature = "RK_AppKit")]
  fn highlight_withFrame_inView_(
    &self,
    flag: bool,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
  ) -> ();
  #[objrs(selector = "trackMouse:inRect:ofView:untilMouseUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn trackMouse_inRect_ofView_untilMouseUp_(
    &self,
    theEvent: &NSEvent,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
    flag: bool,
  ) -> bool;
  #[objrs(selector = "cellSize")]
  #[cfg(feature = "RK_AppKit")]
  fn cellSize(&self) -> CGSize;
  #[objrs(selector = "cellBaselineOffset")]
  #[cfg(feature = "RK_AppKit")]
  fn cellBaselineOffset(&self) -> CGPoint;
  #[objrs(selector = "drawWithFrame:inView:characterIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawWithFrame_inView_characterIndex_(
    &self,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
    charIndex: usize,
  ) -> ();
  #[objrs(selector = "drawWithFrame:inView:characterIndex:layoutManager:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawWithFrame_inView_characterIndex_layoutManager_(
    &self,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
    charIndex: usize,
    layoutManager: &NSLayoutManager,
  ) -> ();
  #[objrs(selector = "wantsToTrackMouseForEvent:inRect:ofView:atCharacterIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsToTrackMouseForEvent_inRect_ofView_atCharacterIndex_(
    &self,
    theEvent: &NSEvent,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
    charIndex: usize,
  ) -> bool;
  #[objrs(selector = "trackMouse:inRect:ofView:atCharacterIndex:untilMouseUp:")]
  #[cfg(feature = "RK_AppKit")]
  fn trackMouse_inRect_ofView_atCharacterIndex_untilMouseUp_(
    &self,
    theEvent: &NSEvent,
    cellFrame: CGRect,
    controlView: Option<&NSView>,
    charIndex: usize,
    flag: bool,
  ) -> bool;
  #[objrs(
    selector = "cellFrameForTextContainer:proposedLineFragment:glyphPosition:characterIndex:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn cellFrameForTextContainer_proposedLineFragment_glyphPosition_characterIndex_(
    &self,
    textContainer: &NSTextContainer,
    lineFrag: CGRect,
    position: CGPoint,
    charIndex: usize,
  ) -> CGRect;
}
# [ objrs ( class , super = NSCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextAttachmentCell;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextInputClientProto {
  #[objrs(selector = "insertText:replacementRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertText_replacementRange_(&self, string: &Object, replacementRange: NSRange) -> ();
  #[objrs(selector = "doCommandBySelector:")]
  #[cfg(feature = "RK_AppKit")]
  fn doCommandBySelector_(&self, selector: SelectorRef) -> ();
  #[objrs(selector = "setMarkedText:selectedRange:replacementRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn setMarkedText_selectedRange_replacementRange_(
    &self,
    string: &Object,
    selectedRange: NSRange,
    replacementRange: NSRange,
  ) -> ();
  #[objrs(selector = "unmarkText")]
  #[cfg(feature = "RK_AppKit")]
  fn unmarkText(&self) -> ();
  #[objrs(selector = "selectedRange")]
  #[cfg(feature = "RK_AppKit")]
  fn selectedRange(&self) -> NSRange;
  #[objrs(selector = "markedRange")]
  #[cfg(feature = "RK_AppKit")]
  fn markedRange(&self) -> NSRange;
  #[objrs(selector = "hasMarkedText")]
  #[cfg(feature = "RK_AppKit")]
  fn hasMarkedText(&self) -> bool;
  #[objrs(selector = "attributedSubstringForProposedRange:actualRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn attributedSubstringForProposedRange_actualRange_(
    &self,
    range: NSRange,
    actualRange: Option<&NSRange>,
  ) -> Option<Arc<NSAttributedString>>;
  #[objrs(selector = "validAttributesForMarkedText")]
  #[cfg(feature = "RK_AppKit")]
  fn validAttributesForMarkedText(&self) -> Arc<NSArray>;
  #[objrs(selector = "firstRectForCharacterRange:actualRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn firstRectForCharacterRange_actualRange_(
    &self,
    range: NSRange,
    actualRange: Option<&NSRange>,
  ) -> CGRect;
  #[objrs(selector = "characterIndexForPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn characterIndexForPoint_(&self, point: CGPoint) -> usize;
  #[objrs(selector = "attributedString")]
  #[cfg(feature = "RK_AppKit")]
  fn attributedString(&self) -> Arc<NSAttributedString>;
  #[objrs(selector = "fractionOfDistanceThroughGlyphForPoint:")]
  #[cfg(feature = "RK_AppKit")]
  fn fractionOfDistanceThroughGlyphForPoint_(&self, point: CGPoint) -> f64;
  #[objrs(selector = "baselineDeltaForCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn baselineDeltaForCharacterAtIndex_(&self, anIndex: usize) -> f64;
  #[objrs(selector = "windowLevel")]
  #[cfg(feature = "RK_AppKit")]
  fn windowLevel(&self) -> isize;
  #[objrs(selector = "drawsVerticallyForCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawsVerticallyForCharacterAtIndex_(&self, charIndex: usize) -> bool;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSelectionGranularity {
  NSSelectByCharacter = 0,
  NSSelectByWord = 1,
  NSSelectByParagraph = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSSelectionAffinity {
  NSSelectionAffinityUpstream = 0,
  NSSelectionAffinityDownstream = 1,
}
# [ objrs ( class , super = NSText ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextView;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTextViewDelegateProto {
  #[objrs(selector = "textView:clickedOnLink:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_clickedOnLink_atIndex_(
    &self,
    textView: &NSTextView,
    link: &Object,
    charIndex: usize,
  ) -> bool;
  #[objrs(selector = "textView:clickedOnCell:inRect:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_clickedOnCell_inRect_atIndex_(
    &self,
    textView: &NSTextView,
    cell: &Object,
    cellFrame: CGRect,
    charIndex: usize,
  ) -> ();
  #[objrs(selector = "textView:doubleClickedOnCell:inRect:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_doubleClickedOnCell_inRect_atIndex_(
    &self,
    textView: &NSTextView,
    cell: &Object,
    cellFrame: CGRect,
    charIndex: usize,
  ) -> ();
  #[objrs(selector = "textView:draggedCell:inRect:event:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_draggedCell_inRect_event_atIndex_(
    &self,
    view: &NSTextView,
    cell: &Object,
    rect: CGRect,
    event: &NSEvent,
    charIndex: usize,
  ) -> ();
  #[objrs(selector = "textView:writablePasteboardTypesForCell:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_writablePasteboardTypesForCell_atIndex_(
    &self,
    view: &NSTextView,
    cell: &Object,
    charIndex: usize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:writeCell:atIndex:toPasteboard:type:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_writeCell_atIndex_toPasteboard_type_(
    &self,
    view: &NSTextView,
    cell: &Object,
    charIndex: usize,
    pboard: &NSPasteboard,
    type_: &NSString,
  ) -> bool;
  #[objrs(selector = "textView:willChangeSelectionFromCharacterRange:toCharacterRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_willChangeSelectionFromCharacterRange_toCharacterRange_(
    &self,
    textView: &NSTextView,
    oldSelectedCharRange: NSRange,
    newSelectedCharRange: NSRange,
  ) -> NSRange;
  #[objrs(selector = "textView:willChangeSelectionFromCharacterRanges:toCharacterRanges:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_willChangeSelectionFromCharacterRanges_toCharacterRanges_(
    &self,
    textView: &NSTextView,
    oldSelectedCharRanges: &NSArray,
    newSelectedCharRanges: &NSArray,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:shouldChangeTextInRanges:replacementStrings:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldChangeTextInRanges_replacementStrings_(
    &self,
    textView: &NSTextView,
    affectedRanges: &NSArray,
    replacementStrings: Option<&NSArray>,
  ) -> bool;
  #[objrs(selector = "textView:shouldChangeTypingAttributes:toAttributes:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldChangeTypingAttributes_toAttributes_(
    &self,
    textView: &NSTextView,
    oldTypingAttributes: &NSDictionary,
    newTypingAttributes: &NSDictionary,
  ) -> Arc<NSDictionary>;
  #[objrs(selector = "textViewDidChangeSelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn textViewDidChangeSelection_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "textViewDidChangeTypingAttributes:")]
  #[cfg(feature = "RK_AppKit")]
  fn textViewDidChangeTypingAttributes_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "textView:willDisplayToolTip:forCharacterAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_willDisplayToolTip_forCharacterAtIndex_(
    &self,
    textView: &NSTextView,
    tooltip: &NSString,
    characterIndex: usize,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "textView:completions:forPartialWordRange:indexOfSelectedItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_completions_forPartialWordRange_indexOfSelectedItem_(
    &self,
    textView: &NSTextView,
    words: &NSArray,
    charRange: NSRange,
    index: Option<&isize>,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:shouldChangeTextInRange:replacementString:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldChangeTextInRange_replacementString_(
    &self,
    textView: &NSTextView,
    affectedCharRange: NSRange,
    replacementString: Option<&NSString>,
  ) -> bool;
  #[objrs(selector = "textView:doCommandBySelector:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_doCommandBySelector_(
    &self,
    textView: &NSTextView,
    commandSelector: SelectorRef,
  ) -> bool;
  #[objrs(selector = "textView:shouldSetSpellingState:range:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldSetSpellingState_range_(
    &self,
    textView: &NSTextView,
    value: isize,
    affectedCharRange: NSRange,
  ) -> isize;
  #[objrs(selector = "textView:menu:forEvent:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_menu_forEvent_atIndex_(
    &self,
    view: &NSTextView,
    menu: &NSMenu,
    event: &NSEvent,
    charIndex: usize,
  ) -> Option<Arc<NSMenu>>;
  #[objrs(selector = "textView:willCheckTextInRange:options:types:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_willCheckTextInRange_options_types_(
    &self,
    view: &NSTextView,
    range: NSRange,
    options: &NSDictionary,
    checkingTypes: &u64,
  ) -> Arc<NSDictionary>;
  #[objrs(selector = "textView:didCheckTextInRange:types:options:results:orthography:wordCount:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_didCheckTextInRange_types_options_results_orthography_wordCount_(
    &self,
    view: &NSTextView,
    range: NSRange,
    checkingTypes: u64,
    options: &NSDictionary,
    results: &NSArray,
    orthography: &NSOrthography,
    wordCount: isize,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:URLForContentsOfTextAttachment:atIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_URLForContentsOfTextAttachment_atIndex_(
    &self,
    textView: &NSTextView,
    textAttachment: &NSTextAttachment,
    charIndex: usize,
  ) -> Option<Arc<NSURL>>;
  #[objrs(selector = "textView:willShowSharingServicePicker:forItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_willShowSharingServicePicker_forItems_(
    &self,
    textView: &NSTextView,
    servicePicker: &NSSharingServicePicker,
    items: &NSArray,
  ) -> Option<Arc<NSSharingServicePicker>>;
  #[objrs(selector = "undoManagerForTextView:")]
  #[cfg(feature = "RK_AppKit")]
  fn undoManagerForTextView_(&self, view: &NSTextView) -> Option<Arc<NSUndoManager>>;
  #[objrs(selector = "textView:shouldUpdateTouchBarItemIdentifiers:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldUpdateTouchBarItemIdentifiers_(
    &self,
    textView: &NSTextView,
    identifiers: &NSArray,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:candidatesForSelectedRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_candidatesForSelectedRange_(
    &self,
    textView: &NSTextView,
    selectedRange: NSRange,
  ) -> Option<Arc<NSArray>>;
  #[objrs(selector = "textView:candidates:forSelectedRange:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_candidates_forSelectedRange_(
    &self,
    textView: &NSTextView,
    candidates: &NSArray,
    selectedRange: NSRange,
  ) -> Arc<NSArray>;
  #[objrs(selector = "textView:shouldSelectCandidateAtIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_shouldSelectCandidateAtIndex_(&self, textView: &NSTextView, index: usize) -> bool;
  #[objrs(selector = "textView:clickedOnLink:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_clickedOnLink_(&self, textView: &NSTextView, link: Option<&Object>) -> bool;
  #[objrs(selector = "textView:clickedOnCell:inRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_clickedOnCell_inRect_(
    &self,
    textView: &NSTextView,
    cell: Option<&Object>,
    cellFrame: CGRect,
  ) -> ();
  #[objrs(selector = "textView:doubleClickedOnCell:inRect:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_doubleClickedOnCell_inRect_(
    &self,
    textView: &NSTextView,
    cell: Option<&Object>,
    cellFrame: CGRect,
  ) -> ();
  #[objrs(selector = "textView:draggedCell:inRect:event:")]
  #[cfg(feature = "RK_AppKit")]
  fn textView_draggedCell_inRect_event_(
    &self,
    view: &NSTextView,
    cell: Option<&Object>,
    rect: CGRect,
    event: Option<&NSEvent>,
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFindPanelAction {
  NSFindPanelActionShowFindPanel = 1,
  NSFindPanelActionNext = 2,
  NSFindPanelActionPrevious = 3,
  NSFindPanelActionReplaceAll = 4,
  NSFindPanelActionReplace = 5,
  NSFindPanelActionReplaceAndFind = 6,
  NSFindPanelActionSetFindString = 7,
  NSFindPanelActionReplaceAllInSelection = 8,
  NSFindPanelActionSelectAll = 9,
  NSFindPanelActionSelectAllInSelection = 10,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFindPanelSubstringMatchType {
  NSFindPanelSubstringMatchTypeContains = 0,
  NSFindPanelSubstringMatchTypeStartsWith = 1,
  NSFindPanelSubstringMatchTypeFullWord = 2,
  NSFindPanelSubstringMatchTypeEndsWith = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __TvFlags {
  pub refusesFirstResponder: u32,
  pub movedPostingDisableCount: u32,
  pub selectionPostingDisableCount: u32,
  pub dataSourceSetObjectValue: u32,
  pub oldAutoresizesAllColumnsToFit: u32,
  pub delegateSelectionShouldChangeInTableView: u32,
  pub delegateShouldSelectTableColumn: u32,
  pub delegateShouldSelectRow: u32,
  pub delegateShouldEditTableColumn: u32,
  pub delegateWillDisplayCell: u32,
  pub compareWidthWithSuperview: u32,
  pub allowGapRow: u32,
  pub selectionType: u32,
  pub allowsColumnSelection: u32,
  pub allowsMultipleSelection: u32,
  pub allowsEmptySelection: u32,
  pub hasBlurBackgroundViews: u32,
  pub allowsColumnResizing: u32,
  pub allowsColumnReordering: u32,
}
pub type _TvFlags = __TvFlags;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTableViewDropOperation {
  NSTableViewDropOn = 0,
  NSTableViewDropAbove = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTableViewColumnAutoresizingStyle {
  NSTableViewNoColumnAutoresizing = 0,
  NSTableViewUniformColumnAutoresizingStyle = 1,
  NSTableViewSequentialColumnAutoresizingStyle = 2,
  NSTableViewReverseSequentialColumnAutoresizingStyle = 3,
  NSTableViewLastColumnOnlyAutoresizingStyle = 4,
  NSTableViewFirstColumnOnlyAutoresizingStyle = 5,
}
bitflags! { # [ repr ( C ) ] pub struct NSTableViewGridLineStyle : usize { const NSTableViewGridNone = 0 ; const NSTableViewSolidVerticalGridLineMask = 1 ; const NSTableViewSolidHorizontalGridLineMask = 2 ; const NSTableViewDashedHorizontalGridLineMask = 8 ; } }
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTableViewRowSizeStyle {
  NSTableViewRowSizeStyleDefault = -1,
  NSTableViewRowSizeStyleCustom = 0,
  NSTableViewRowSizeStyleSmall = 1,
  NSTableViewRowSizeStyleMedium = 2,
  NSTableViewRowSizeStyleLarge = 3,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTableViewSelectionHighlightStyle {
  NSTableViewSelectionHighlightStyleNone = -1,
  NSTableViewSelectionHighlightStyleRegular = 0,
  NSTableViewSelectionHighlightStyleSourceList = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTableViewDraggingDestinationFeedbackStyle {
  NSTableViewDraggingDestinationFeedbackStyleNone = -1,
  NSTableViewDraggingDestinationFeedbackStyleRegular = 0,
  NSTableViewDraggingDestinationFeedbackStyleSourceList = 1,
  NSTableViewDraggingDestinationFeedbackStyleGap = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTableRowActionEdge {
  NSTableRowActionEdgeLeading = 0,
  NSTableRowActionEdgeTrailing = 1,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableView;
bitflags! { # [ repr ( C ) ] pub struct NSTableViewAnimationOptions : usize { const NSTableViewAnimationEffectNone = 0 ; const NSTableViewAnimationEffectFade = 1 ; const NSTableViewAnimationEffectGap = 2 ; const NSTableViewAnimationSlideUp = 16 ; const NSTableViewAnimationSlideDown = 32 ; const NSTableViewAnimationSlideLeft = 48 ; const NSTableViewAnimationSlideRight = 64 ; } }
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTableViewDelegateProto {
  #[objrs(selector = "tableView:viewForTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_viewForTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> Option<Arc<NSView>>;
  #[objrs(selector = "tableView:rowViewForRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_rowViewForRow_(
    &self,
    tableView: &NSTableView,
    row: isize,
  ) -> Option<Arc<NSTableRowView>>;
  #[objrs(selector = "tableView:didAddRowView:forRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_didAddRowView_forRow_(
    &self,
    tableView: &NSTableView,
    rowView: &NSTableRowView,
    row: isize,
  ) -> ();
  #[objrs(selector = "tableView:didRemoveRowView:forRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_didRemoveRowView_forRow_(
    &self,
    tableView: &NSTableView,
    rowView: &NSTableRowView,
    row: isize,
  ) -> ();
  #[objrs(selector = "tableView:willDisplayCell:forTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_willDisplayCell_forTableColumn_row_(
    &self,
    tableView: &NSTableView,
    cell: &Object,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> ();
  #[objrs(selector = "tableView:shouldEditTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldEditTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> bool;
  #[objrs(selector = "tableView:toolTipForCell:rect:tableColumn:row:mouseLocation:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_toolTipForCell_rect_tableColumn_row_mouseLocation_(
    &self,
    tableView: &NSTableView,
    cell: &NSCell,
    rect: &CGRect,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
    mouseLocation: CGPoint,
  ) -> Arc<NSString>;
  #[objrs(selector = "tableView:shouldShowCellExpansionForTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldShowCellExpansionForTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> bool;
  #[objrs(selector = "tableView:shouldTrackCell:forTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldTrackCell_forTableColumn_row_(
    &self,
    tableView: &NSTableView,
    cell: &NSCell,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> bool;
  #[objrs(selector = "tableView:dataCellForTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_dataCellForTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> Option<Arc<NSCell>>;
  #[objrs(selector = "selectionShouldChangeInTableView:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectionShouldChangeInTableView_(&self, tableView: &NSTableView) -> bool;
  #[objrs(selector = "tableView:shouldSelectRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldSelectRow_(&self, tableView: &NSTableView, row: isize) -> bool;
  #[objrs(selector = "tableView:selectionIndexesForProposedSelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_selectionIndexesForProposedSelection_(
    &self,
    tableView: &NSTableView,
    proposedSelectionIndexes: &NSIndexSet,
  ) -> Arc<NSIndexSet>;
  #[objrs(selector = "tableView:shouldSelectTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldSelectTableColumn_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
  ) -> bool;
  #[objrs(selector = "tableView:mouseDownInHeaderOfTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_mouseDownInHeaderOfTableColumn_(
    &self,
    tableView: &NSTableView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "tableView:didClickTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_didClickTableColumn_(
    &self,
    tableView: &NSTableView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "tableView:didDragTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_didDragTableColumn_(
    &self,
    tableView: &NSTableView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "tableView:heightOfRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_heightOfRow_(&self, tableView: &NSTableView, row: isize) -> f64;
  #[objrs(selector = "tableView:typeSelectStringForTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_typeSelectStringForTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "tableView:nextTypeSelectMatchFromRow:toRow:forString:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_nextTypeSelectMatchFromRow_toRow_forString_(
    &self,
    tableView: &NSTableView,
    startRow: isize,
    endRow: isize,
    searchString: &NSString,
  ) -> isize;
  #[objrs(selector = "tableView:shouldTypeSelectForEvent:withCurrentSearchString:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldTypeSelectForEvent_withCurrentSearchString_(
    &self,
    tableView: &NSTableView,
    event: &NSEvent,
    searchString: Option<&NSString>,
  ) -> bool;
  #[objrs(selector = "tableView:isGroupRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_isGroupRow_(&self, tableView: &NSTableView, row: isize) -> bool;
  #[objrs(selector = "tableView:sizeToFitWidthOfColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_sizeToFitWidthOfColumn_(&self, tableView: &NSTableView, column: isize) -> f64;
  #[objrs(selector = "tableView:shouldReorderColumn:toColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_shouldReorderColumn_toColumn_(
    &self,
    tableView: &NSTableView,
    columnIndex: isize,
    newColumnIndex: isize,
  ) -> bool;
  #[objrs(selector = "tableView:rowActionsForRow:edge:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_rowActionsForRow_edge_(
    &self,
    tableView: &NSTableView,
    row: isize,
    edge: NSTableRowActionEdge,
  ) -> Arc<NSArray>;
  #[objrs(selector = "tableViewSelectionDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableViewSelectionDidChange_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "tableViewColumnDidMove:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableViewColumnDidMove_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "tableViewColumnDidResize:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableViewColumnDidResize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "tableViewSelectionIsChanging:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableViewSelectionIsChanging_(&self, notification: &NSNotification) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTableViewDataSourceProto {
  #[objrs(selector = "numberOfRowsInTableView:")]
  #[cfg(feature = "RK_AppKit")]
  fn numberOfRowsInTableView_(&self, tableView: &NSTableView) -> isize;
  #[objrs(selector = "tableView:objectValueForTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_objectValueForTableColumn_row_(
    &self,
    tableView: &NSTableView,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "tableView:setObjectValue:forTableColumn:row:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_setObjectValue_forTableColumn_row_(
    &self,
    tableView: &NSTableView,
    object: Option<&Object>,
    tableColumn: Option<&NSTableColumn>,
    row: isize,
  ) -> ();
  #[objrs(selector = "tableView:sortDescriptorsDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_sortDescriptorsDidChange_(
    &self,
    tableView: &NSTableView,
    oldDescriptors: &NSArray,
  ) -> ();
  #[objrs(selector = "tableView:pasteboardWriterForRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_pasteboardWriterForRow_(
    &self,
    tableView: &NSTableView,
    row: isize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "tableView:draggingSession:willBeginAtPoint:forRowIndexes:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_draggingSession_willBeginAtPoint_forRowIndexes_(
    &self,
    tableView: &NSTableView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    rowIndexes: &NSIndexSet,
  ) -> ();
  #[objrs(selector = "tableView:draggingSession:endedAtPoint:operation:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_draggingSession_endedAtPoint_operation_(
    &self,
    tableView: &NSTableView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    operation: NSDragOperation,
  ) -> ();
  #[objrs(selector = "tableView:updateDraggingItemsForDrag:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_updateDraggingItemsForDrag_(
    &self,
    tableView: &NSTableView,
    draggingInfo: &Object,
  ) -> ();
  #[objrs(selector = "tableView:writeRowsWithIndexes:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_writeRowsWithIndexes_toPasteboard_(
    &self,
    tableView: &NSTableView,
    rowIndexes: &NSIndexSet,
    pboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "tableView:validateDrop:proposedRow:proposedDropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_validateDrop_proposedRow_proposedDropOperation_(
    &self,
    tableView: &NSTableView,
    info: &Object,
    row: isize,
    dropOperation: NSTableViewDropOperation,
  ) -> NSDragOperation;
  #[objrs(selector = "tableView:acceptDrop:row:dropOperation:")]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_acceptDrop_row_dropOperation_(
    &self,
    tableView: &NSTableView,
    info: &Object,
    row: isize,
    dropOperation: NSTableViewDropOperation,
  ) -> bool;
  #[objrs(
    selector = "tableView:namesOfPromisedFilesDroppedAtDestination:forDraggedRowsWithIndexes:"
  )]
  #[cfg(feature = "RK_AppKit")]
  fn tableView_namesOfPromisedFilesDroppedAtDestination_forDraggedRowsWithIndexes_(
    &self,
    tableView: &NSTableView,
    dropDestination: &NSURL,
    indexSet: &NSIndexSet,
  ) -> Arc<NSArray>;
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableCellView;
bitflags! { # [ repr ( C ) ] pub struct NSTableColumnResizingOptions : usize { const NSTableColumnNoResizing = 0 ; const NSTableColumnAutoresizingMask = 1 ; const NSTableColumnUserResizingMask = 2 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableColumn;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __colFlags {
  pub oldIsResizable: u32,
  pub isEditable: u32,
  pub resizedPostingDisableCount: u32,
  pub canUseReorderResizeImageCache: u32,
  pub userResizingAllowed: u32,
  pub autoResizingAllowed: u32,
  pub hidden: u32,
  pub RESERVED: u32,
}
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableHeaderCell;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableHeaderView;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableRowView;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTableViewRowActionStyle {
  NSTableViewRowActionStyleRegular = 0,
  NSTableViewRowActionStyleDestructive = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTableViewRowAction;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __OvFlags {
  pub unused3: u32,
  pub dataSourceObjectValueByItem: u32,
  pub allowAutomaticAnimations: u32,
  pub dontRedisplayOnFrameChange: u32,
  pub _isSpringLoadingFlashing: u32,
  pub unused2: u32,
  pub delegateShouldAutoExpandItem: u32,
  pub delegateAutoCollapseItem: u32,
  pub delegateAutoExpandItem: u32,
  pub delegateShouldShowOutlineCellForItem: u32,
  pub dataSourceDraggedImageMovedTo: u32,
  pub dataSourceDraggingEndedAt: u32,
  pub reloadingData: u32,
  pub validDataSourceMethods: u32,
  pub numberOfRowsDataExpandEntered: u32,
  pub delayRowEntryFreeDisabled: u32,
  pub delegateHeightOfRowByItem: u32,
  pub animateExpandAndCollapse: u32,
  pub stronglyReferencesItems: u32,
  pub selectionAdjustmentDisabled: u32,
  pub subclassRowForItem: u32,
  pub delegateWillDisplayOutlineCell: u32,
  pub enableExpandNotifications: u32,
  pub autoSaveExpandItems: u32,
  pub autoresizesOutlineColumn: u32,
  pub delegateShouldExpandItem: u32,
  pub delegateShouldCollapseItem: u32,
  pub delegateSelectionShouldChangeInOutlineView: u32,
  pub delegateShouldSelectTableColumn: u32,
  pub delegateShouldSelectItem: u32,
  pub delegateShouldEditTableColumn: u32,
  pub delegateWillDisplayCell: u32,
}
pub type _OVFlags = __OvFlags;
# [ objrs ( class , super = NSTableView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOutlineView;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSOutlineViewDataSourceProto {
  #[objrs(selector = "outlineView:numberOfChildrenOfItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_numberOfChildrenOfItem_(
    &self,
    outlineView: &NSOutlineView,
    item: Option<&Object>,
  ) -> isize;
  #[objrs(selector = "outlineView:child:ofItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_child_ofItem_(
    &self,
    outlineView: &NSOutlineView,
    index: isize,
    item: Option<&Object>,
  ) -> Arc<Object>;
  #[objrs(selector = "outlineView:isItemExpandable:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_isItemExpandable_(&self, outlineView: &NSOutlineView, item: &Object) -> bool;
  #[objrs(selector = "outlineView:objectValueForTableColumn:byItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_objectValueForTableColumn_byItem_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "outlineView:setObjectValue:forTableColumn:byItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_setObjectValue_forTableColumn_byItem_(
    &self,
    outlineView: &NSOutlineView,
    object: Option<&Object>,
    tableColumn: Option<&NSTableColumn>,
    item: Option<&Object>,
  ) -> ();
  #[objrs(selector = "outlineView:itemForPersistentObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_itemForPersistentObject_(
    &self,
    outlineView: &NSOutlineView,
    object: &Object,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "outlineView:persistentObjectForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_persistentObjectForItem_(
    &self,
    outlineView: &NSOutlineView,
    item: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "outlineView:sortDescriptorsDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_sortDescriptorsDidChange_(
    &self,
    outlineView: &NSOutlineView,
    oldDescriptors: &NSArray,
  ) -> ();
  #[objrs(selector = "outlineView:pasteboardWriterForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_pasteboardWriterForItem_(
    &self,
    outlineView: &NSOutlineView,
    item: &Object,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "outlineView:draggingSession:willBeginAtPoint:forItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_draggingSession_willBeginAtPoint_forItems_(
    &self,
    outlineView: &NSOutlineView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    draggedItems: &NSArray,
  ) -> ();
  #[objrs(selector = "outlineView:draggingSession:endedAtPoint:operation:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_draggingSession_endedAtPoint_operation_(
    &self,
    outlineView: &NSOutlineView,
    session: &NSDraggingSession,
    screenPoint: CGPoint,
    operation: NSDragOperation,
  ) -> ();
  #[objrs(selector = "outlineView:writeItems:toPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_writeItems_toPasteboard_(
    &self,
    outlineView: &NSOutlineView,
    items: &NSArray,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "outlineView:updateDraggingItemsForDrag:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_updateDraggingItemsForDrag_(
    &self,
    outlineView: &NSOutlineView,
    draggingInfo: &Object,
  ) -> ();
  #[objrs(selector = "outlineView:validateDrop:proposedItem:proposedChildIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_validateDrop_proposedItem_proposedChildIndex_(
    &self,
    outlineView: &NSOutlineView,
    info: &Object,
    item: Option<&Object>,
    index: isize,
  ) -> NSDragOperation;
  #[objrs(selector = "outlineView:acceptDrop:item:childIndex:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_acceptDrop_item_childIndex_(
    &self,
    outlineView: &NSOutlineView,
    info: &Object,
    item: Option<&Object>,
    index: isize,
  ) -> bool;
  #[objrs(selector = "outlineView:namesOfPromisedFilesDroppedAtDestination:forDraggedItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_namesOfPromisedFilesDroppedAtDestination_forDraggedItems_(
    &self,
    outlineView: &NSOutlineView,
    dropDestination: &NSURL,
    items: &NSArray,
  ) -> Arc<NSArray>;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSOutlineViewDelegateProto {
  #[objrs(selector = "outlineView:viewForTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_viewForTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> Option<Arc<NSView>>;
  #[objrs(selector = "outlineView:rowViewForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_rowViewForItem_(
    &self,
    outlineView: &NSOutlineView,
    item: &Object,
  ) -> Option<Arc<NSTableRowView>>;
  #[objrs(selector = "outlineView:didAddRowView:forRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_didAddRowView_forRow_(
    &self,
    outlineView: &NSOutlineView,
    rowView: &NSTableRowView,
    row: isize,
  ) -> ();
  #[objrs(selector = "outlineView:didRemoveRowView:forRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_didRemoveRowView_forRow_(
    &self,
    outlineView: &NSOutlineView,
    rowView: &NSTableRowView,
    row: isize,
  ) -> ();
  #[objrs(selector = "outlineView:willDisplayCell:forTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_willDisplayCell_forTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    cell: &Object,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> ();
  #[objrs(selector = "outlineView:shouldEditTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldEditTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> bool;
  #[objrs(selector = "selectionShouldChangeInOutlineView:")]
  #[cfg(feature = "RK_AppKit")]
  fn selectionShouldChangeInOutlineView_(&self, outlineView: &NSOutlineView) -> bool;
  #[objrs(selector = "outlineView:shouldSelectItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldSelectItem_(&self, outlineView: &NSOutlineView, item: &Object) -> bool;
  #[objrs(selector = "outlineView:selectionIndexesForProposedSelection:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_selectionIndexesForProposedSelection_(
    &self,
    outlineView: &NSOutlineView,
    proposedSelectionIndexes: &NSIndexSet,
  ) -> Arc<NSIndexSet>;
  #[objrs(selector = "outlineView:shouldSelectTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldSelectTableColumn_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
  ) -> bool;
  #[objrs(selector = "outlineView:mouseDownInHeaderOfTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_mouseDownInHeaderOfTableColumn_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "outlineView:didClickTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_didClickTableColumn_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "outlineView:didDragTableColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_didDragTableColumn_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: &NSTableColumn,
  ) -> ();
  #[objrs(selector = "outlineView:toolTipForCell:rect:tableColumn:item:mouseLocation:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_toolTipForCell_rect_tableColumn_item_mouseLocation_(
    &self,
    outlineView: &NSOutlineView,
    cell: &NSCell,
    rect: &CGRect,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
    mouseLocation: CGPoint,
  ) -> Arc<NSString>;
  #[objrs(selector = "outlineView:heightOfRowByItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_heightOfRowByItem_(&self, outlineView: &NSOutlineView, item: &Object) -> f64;
  #[objrs(selector = "outlineView:typeSelectStringForTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_typeSelectStringForTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> Option<Arc<NSString>>;
  #[objrs(selector = "outlineView:nextTypeSelectMatchFromItem:toItem:forString:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_nextTypeSelectMatchFromItem_toItem_forString_(
    &self,
    outlineView: &NSOutlineView,
    startItem: &Object,
    endItem: &Object,
    searchString: &NSString,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "outlineView:shouldTypeSelectForEvent:withCurrentSearchString:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldTypeSelectForEvent_withCurrentSearchString_(
    &self,
    outlineView: &NSOutlineView,
    event: &NSEvent,
    searchString: Option<&NSString>,
  ) -> bool;
  #[objrs(selector = "outlineView:shouldShowCellExpansionForTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldShowCellExpansionForTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> bool;
  #[objrs(selector = "outlineView:shouldTrackCell:forTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldTrackCell_forTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    cell: &NSCell,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> bool;
  #[objrs(selector = "outlineView:dataCellForTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_dataCellForTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> Option<Arc<NSCell>>;
  #[objrs(selector = "outlineView:isGroupItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_isGroupItem_(&self, outlineView: &NSOutlineView, item: &Object) -> bool;
  #[objrs(selector = "outlineView:shouldExpandItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldExpandItem_(&self, outlineView: &NSOutlineView, item: &Object) -> bool;
  #[objrs(selector = "outlineView:shouldCollapseItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldCollapseItem_(&self, outlineView: &NSOutlineView, item: &Object) -> bool;
  #[objrs(selector = "outlineView:willDisplayOutlineCell:forTableColumn:item:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_willDisplayOutlineCell_forTableColumn_item_(
    &self,
    outlineView: &NSOutlineView,
    cell: &Object,
    tableColumn: Option<&NSTableColumn>,
    item: &Object,
  ) -> ();
  #[objrs(selector = "outlineView:sizeToFitWidthOfColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_sizeToFitWidthOfColumn_(&self, outlineView: &NSOutlineView, column: isize) -> f64;
  #[objrs(selector = "outlineView:shouldReorderColumn:toColumn:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldReorderColumn_toColumn_(
    &self,
    outlineView: &NSOutlineView,
    columnIndex: isize,
    newColumnIndex: isize,
  ) -> bool;
  #[objrs(selector = "outlineView:shouldShowOutlineCellForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineView_shouldShowOutlineCellForItem_(
    &self,
    outlineView: &NSOutlineView,
    item: &Object,
  ) -> bool;
  #[objrs(selector = "outlineViewSelectionDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewSelectionDidChange_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewColumnDidMove:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewColumnDidMove_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewColumnDidResize:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewColumnDidResize_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewSelectionIsChanging:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewSelectionIsChanging_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewItemWillExpand:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewItemWillExpand_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewItemDidExpand:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewItemDidExpand_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewItemWillCollapse:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewItemWillCollapse_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "outlineViewItemDidCollapse:")]
  #[cfg(feature = "RK_AppKit")]
  fn outlineViewItemDidCollapse_(&self, notification: &NSNotification) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSInputServiceProviderProto {
  #[objrs(selector = "insertText:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn insertText_client_(&self, string: Option<&Object>, sender: Option<&Object>) -> ();
  #[objrs(selector = "doCommandBySelector:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn doCommandBySelector_client_(&self, selector: SelectorRef, sender: Option<&Object>) -> ();
  #[objrs(selector = "markedTextAbandoned:")]
  #[cfg(feature = "RK_AppKit")]
  fn markedTextAbandoned_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "markedTextSelectionChanged:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn markedTextSelectionChanged_client_(&self, newSel: NSRange, sender: Option<&Object>) -> ();
  #[objrs(selector = "terminate:")]
  #[cfg(feature = "RK_AppKit")]
  fn terminate_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "canBeDisabled")]
  #[cfg(feature = "RK_AppKit")]
  fn canBeDisabled(&self) -> bool;
  #[objrs(selector = "wantsToInterpretAllKeystrokes")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsToInterpretAllKeystrokes(&self) -> bool;
  #[objrs(selector = "wantsToHandleMouseEvents")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsToHandleMouseEvents(&self) -> bool;
  #[objrs(selector = "wantsToDelayTextChangeNotifications")]
  #[cfg(feature = "RK_AppKit")]
  fn wantsToDelayTextChangeNotifications(&self) -> bool;
  #[objrs(selector = "inputClientBecomeActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn inputClientBecomeActive_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "inputClientResignActive:")]
  #[cfg(feature = "RK_AppKit")]
  fn inputClientResignActive_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "inputClientEnabled:")]
  #[cfg(feature = "RK_AppKit")]
  fn inputClientEnabled_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "inputClientDisabled:")]
  #[cfg(feature = "RK_AppKit")]
  fn inputClientDisabled_(&self, sender: Option<&Object>) -> ();
  #[objrs(selector = "activeConversationWillChange:fromOldConversation:")]
  #[cfg(feature = "RK_AppKit")]
  fn activeConversationWillChange_fromOldConversation_(
    &self,
    sender: Option<&Object>,
    oldConversation: isize,
  ) -> ();
  #[objrs(selector = "activeConversationChanged:toNewConversation:")]
  #[cfg(feature = "RK_AppKit")]
  fn activeConversationChanged_toNewConversation_(
    &self,
    sender: Option<&Object>,
    newConversation: isize,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSInputServerMouseTrackerProto {
  #[objrs(selector = "mouseDownOnCharacterIndex:atCoordinate:withModifier:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn mouseDownOnCharacterIndex_atCoordinate_withModifier_client_(
    &self,
    index: usize,
    point: CGPoint,
    flags: usize,
    sender: Option<&Object>,
  ) -> bool;
  #[objrs(selector = "mouseDraggedOnCharacterIndex:atCoordinate:withModifier:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn mouseDraggedOnCharacterIndex_atCoordinate_withModifier_client_(
    &self,
    index: usize,
    point: CGPoint,
    flags: usize,
    sender: Option<&Object>,
  ) -> bool;
  #[objrs(selector = "mouseUpOnCharacterIndex:atCoordinate:withModifier:client:")]
  #[cfg(feature = "RK_AppKit")]
  fn mouseUpOnCharacterIndex_atCoordinate_withModifier_client_(
    &self,
    index: usize,
    point: CGPoint,
    flags: usize,
    sender: Option<&Object>,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSInputServer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStringDrawingContext;
bitflags! { # [ repr ( C ) ] pub struct NSStringDrawingOptions : isize { const NSStringDrawingUsesLineFragmentOrigin = 1 ; const NSStringDrawingUsesFontLeading = 2 ; const NSStringDrawingUsesDeviceMetrics = 8 ; const NSStringDrawingTruncatesLastVisibleLine = 32 ; const NSStringDrawingDisableScreenFontSubstitution = 4 ; const NSStringDrawingOneShot = 16 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSRulerMarker;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __rFlags {
  pub movable: u32,
  pub removable: u32,
  pub dragging: u32,
  pub pinned: u32,
  pub _reserved: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRulerOrientation {
  NSHorizontalRuler = 0,
  NSVerticalRuler = 1,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSRulerView;
# [ objrs ( class , super = NSTextField ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSecureTextField;
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSecureTextFieldCell;
pub type NSInterfaceStyle = usize;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSProgressIndicatorStyle {
  NSProgressIndicatorStyleBar = 0,
  NSProgressIndicatorStyleSpinning = 1,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSProgressIndicator;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSProgressIndicatorThickness {
  NSProgressIndicatorPreferredThickness = 14,
  NSProgressIndicatorPreferredSmallThickness = 10,
  NSProgressIndicatorPreferredLargeThickness = 18,
  NSProgressIndicatorPreferredAquaThickness = 12,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTabViewType {
  NSTopTabsBezelBorder = 0,
  NSLeftTabsBezelBorder = 1,
  NSBottomTabsBezelBorder = 2,
  NSRightTabsBezelBorder = 3,
  NSNoTabsBezelBorder = 4,
  NSNoTabsLineBorder = 5,
  NSNoTabsNoBorder = 6,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTabPosition {
  NSTabPositionNone = 0,
  NSTabPositionTop = 1,
  NSTabPositionLeft = 2,
  NSTabPositionBottom = 3,
  NSTabPositionRight = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTabViewBorderType {
  NSTabViewBorderTypeNone = 0,
  NSTabViewBorderTypeLine = 1,
  NSTabViewBorderTypeBezel = 2,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTabView;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __NSTabViewTypeFlags {
  pub tabViewBorderType: u32,
  pub tabPosition: u32,
  pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __NSTabViewDelegateRespondTo {
  pub shouldSelectTabViewItem: u32,
  pub willSelectTabViewItem: u32,
  pub didSelectTabViewItem: u32,
  pub didChangeNumberOfTabViewItems: u32,
  pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __NSTabViewFlags {
  pub needsLayout: u32,
  pub controlTint: u32,
  pub controlSize: u32,
  pub wiringNibConnections: u32,
  pub wiringInteriorLastKeyView: u32,
  pub originalNextKeyViewChanged: u32,
  pub liveResizeSkippedResetToolTips: u32,
  pub subviewsAddedForTabs: u32,
  pub allowsPropertyChange: u32,
  pub ownedByTabViewController: u32,
  pub reserved: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSTabViewDelegateProto {
  #[objrs(selector = "tabView:shouldSelectTabViewItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn tabView_shouldSelectTabViewItem_(
    &self,
    tabView: &NSTabView,
    tabViewItem: Option<&NSTabViewItem>,
  ) -> bool;
  #[objrs(selector = "tabView:willSelectTabViewItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn tabView_willSelectTabViewItem_(
    &self,
    tabView: &NSTabView,
    tabViewItem: Option<&NSTabViewItem>,
  ) -> ();
  #[objrs(selector = "tabView:didSelectTabViewItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn tabView_didSelectTabViewItem_(
    &self,
    tabView: &NSTabView,
    tabViewItem: Option<&NSTabViewItem>,
  ) -> ();
  #[objrs(selector = "tabViewDidChangeNumberOfTabViewItems:")]
  #[cfg(feature = "RK_AppKit")]
  fn tabViewDidChangeNumberOfTabViewItems_(&self, tabView: &NSTabView) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSToolbarDisplayMode {
  NSToolbarDisplayModeDefault = 0,
  NSToolbarDisplayModeIconAndLabel = 1,
  NSToolbarDisplayModeIconOnly = 2,
  NSToolbarDisplayModeLabelOnly = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSToolbarSizeMode {
  NSToolbarSizeModeDefault = 0,
  NSToolbarSizeModeRegular = 1,
  NSToolbarSizeModeSmall = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSToolbar;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __tbFlags {
  pub allowsUserCustomization: u32,
  pub autosavesUsingIdentifier: u32,
  pub initialConfigurationDone: u32,
  pub doesNotAttachToMenuBar: u32,
  pub delegateDefaultItemIdentifiers: u32,
  pub delegateAllowedItemIdentifiers: u32,
  pub delegateItemWithItemIdentifier: u32,
  pub delegateNotificationsEnabled: u32,
  pub prefersToBeShown: u32,
  pub loadItemsImmediately: u32,
  pub currentItemsContainsPlaceholder: u32,
  pub customizationPanelIsRunning: u32,
  pub usesCustomSheetWidth: u32,
  pub clickAndDragPerformsCustomization: u32,
  pub showsNoContextMenu: u32,
  pub currentlyLoadingPlaceholders: u32,
  pub delegateItemWithItemIdentifier2: u32,
  pub inGlobalWindow: u32,
  pub hasOwnedFullscreenViewController: u32,
  pub usesServicesItems: u32,
  pub usingFSMetrics: u32,
  pub keyboardLoopNeedsUpdating: u32,
  pub showHideDuringConfigurationChangeDisabled: u32,
  pub displayMode: u32,
  pub sizeMode: u32,
  pub doNotShowBaselineSeparator: u32,
  pub hideWithoutResizingWindowHint: u32,
  pub autovalidatesItemsDisabled: u32,
  pub inAutovalidation: u32,
  pub loadedMetrics: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSToolbarDelegateProto {
  #[objrs(selector = "toolbar:itemForItemIdentifier:willBeInsertedIntoToolbar:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbar_itemForItemIdentifier_willBeInsertedIntoToolbar_(
    &self,
    toolbar: &NSToolbar,
    itemIdentifier: &NSString,
    flag: bool,
  ) -> Option<Arc<NSToolbarItem>>;
  #[objrs(selector = "toolbarDefaultItemIdentifiers:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbarDefaultItemIdentifiers_(&self, toolbar: &NSToolbar) -> Arc<NSArray>;
  #[objrs(selector = "toolbarAllowedItemIdentifiers:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbarAllowedItemIdentifiers_(&self, toolbar: &NSToolbar) -> Arc<NSArray>;
  #[objrs(selector = "toolbarSelectableItemIdentifiers:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbarSelectableItemIdentifiers_(&self, toolbar: &NSToolbar) -> Arc<NSArray>;
  #[objrs(selector = "toolbarWillAddItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbarWillAddItem_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "toolbarDidRemoveItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn toolbarDidRemoveItem_(&self, notification: &NSNotification) -> ();
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTabViewControllerTabStyle {
  NSTabViewControllerTabStyleSegmentedControlOnTop = 0,
  NSTabViewControllerTabStyleSegmentedControlOnBottom = 1,
  NSTabViewControllerTabStyleToolbar = 2,
  NSTabViewControllerTabStyleUnspecified = -1,
}
# [ objrs ( class , super = NSViewController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTabViewController;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTabState {
  NSSelectedTab = 0,
  NSBackgroundTab = 1,
  NSPressedTab = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTabViewItem;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __tviFlags {
  pub hasCustomColor: u32,
  pub labelSizeIsValid: u32,
  pub autoGeneratedIFR: u32,
  pub isTabDisabled: u32,
  pub isActive: u32,
  pub RESERVED: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPopUpArrowPosition {
  NSPopUpNoArrow = 0,
  NSPopUpArrowAtCenter = 1,
  NSPopUpArrowAtBottom = 2,
}
# [ objrs ( class , super = NSMenuItemCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPopUpButtonCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pbcFlags {
  pub pullsDown: u32,
  pub preferredEdge: u32,
  pub menuIsAttached: u32,
  pub usesItemFromMenu: u32,
  pub altersStateOfSelectedItem: u32,
  pub decoding: u32,
  pub arrowPosition: u32,
  pub drawing: u32,
  pub menuShouldBeUniquedAgainstMain: u32,
  pub RESERVED: u32,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLineCapStyle {
  NSLineCapStyleButt = 0,
  NSLineCapStyleRound = 1,
  NSLineCapStyleSquare = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLineJoinStyle {
  NSLineJoinStyleMiter = 0,
  NSLineJoinStyleRound = 1,
  NSLineJoinStyleBevel = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSWindingRule {
  NSWindingRuleNonZero = 0,
  NSWindingRuleEvenOdd = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBezierPathElement {
  NSBezierPathElementMoveTo = 0,
  NSBezierPathElementLineTo = 1,
  NSBezierPathElementCurveTo = 2,
  NSBezierPathElementClosePath = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSBezierPath;
#[repr(C)]
pub struct PATHSEGMENT {
  opaque: u32,
}
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPICTImageRep;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStatusBar;
# [ objrs ( class , super = NSButton ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStatusBarButton;
bitflags! { # [ repr ( C ) ] pub struct NSStatusItemBehavior : usize { const NSStatusItemBehaviorRemovalAllowed = 2 ; const NSStatusItemBehaviorTerminationOnRemoval = 4 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStatusItem;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSound;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSoundDelegateProto {
  #[objrs(selector = "sound:didFinishPlaying:")]
  #[cfg(feature = "RK_AppKit")]
  fn sound_didFinishPlaying_(&self, sound: &NSSound, flag: bool) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSMovie;
# [ objrs ( class , super = NSImageRep ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPDFImageRep;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDrawerState {
  NSDrawerClosedState = 0,
  NSDrawerOpeningState = 1,
  NSDrawerOpenState = 2,
  NSDrawerClosingState = 3,
}
# [ objrs ( class , super = NSResponder ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDrawer;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDrawerDelegateProto {
  #[objrs(selector = "drawerShouldOpen:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerShouldOpen_(&self, sender: &NSDrawer) -> bool;
  #[objrs(selector = "drawerShouldClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerShouldClose_(&self, sender: &NSDrawer) -> bool;
  #[objrs(selector = "drawerWillResizeContents:toSize:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerWillResizeContents_toSize_(&self, sender: &NSDrawer, contentSize: CGSize) -> CGSize;
  #[objrs(selector = "drawerWillOpen:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerWillOpen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "drawerDidOpen:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerDidOpen_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "drawerWillClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerWillClose_(&self, notification: &NSNotification) -> ();
  #[objrs(selector = "drawerDidClose:")]
  #[cfg(feature = "RK_AppKit")]
  fn drawerDidClose_(&self, notification: &NSNotification) -> ();
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum NSOpenGLGlobalOption {
  NSOpenGLGOFormatCacheSize = 501,
  NSOpenGLGOClearFormatCache = 502,
  NSOpenGLGORetainRenderers = 503,
  NSOpenGLGOUseBuildCache = 506,
  NSOpenGLGOResetLibrary = 504,
}
pub type NSOpenGLPixelFormatAttribute = u32;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenGLPixelFormat;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenGLPixelBuffer;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSOpenGLContextParameter {
  NSOpenGLContextParameterSwapInterval = 222,
  NSOpenGLContextParameterSurfaceOrder = 235,
  NSOpenGLContextParameterSurfaceOpacity = 236,
  NSOpenGLContextParameterSurfaceBackingSize = 304,
  NSOpenGLContextParameterReclaimResources = 308,
  NSOpenGLContextParameterCurrentRendererID = 309,
  NSOpenGLContextParameterGPUVertexProcessing = 310,
  NSOpenGLContextParameterGPUFragmentProcessing = 311,
  NSOpenGLContextParameterHasDrawable = 314,
  NSOpenGLContextParameterMPSwapsInFlight = 315,
  NSOpenGLContextParameterSwapRectangle = 200,
  NSOpenGLContextParameterSwapRectangleEnable = 201,
  NSOpenGLContextParameterRasterizationEnable = 221,
  NSOpenGLContextParameterStateValidation = 301,
  NSOpenGLContextParameterSurfaceSurfaceVolatile = 306,
}
pub type NSOpenGLContextAuxiliary = _CGLContextObject;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenGLContext;
# [ objrs ( class , super = CAOpenGLLayer ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenGLLayer;
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSOpenGLView;
pub type NSToolbarItemVisibilityPriority = isize;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSToolbarItem;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __tbiFlags {
  pub viewRespondsToIsEnabled: u32,
  pub viewRespondsToSetEnabled: u32,
  pub viewRespondsToTag: u32,
  pub viewRespondsToSetTag: u32,
  pub viewRespondsToAction: u32,
  pub viewRespondsToSetAction: u32,
  pub viewRespondsToTarget: u32,
  pub viewRespondsToSetTarget: u32,
  pub viewRespondsToImage: u32,
  pub viewRespondsToSetImage: u32,
  pub isEnabled: u32,
  pub isUserRemovable: u32,
  pub menuHasBeenSet: u32,
  pub menuRepIsDefault: u32,
  pub viewHasBeenLoaded: u32,
  pub drawingForDragImage: u32,
  pub isCustomItemType: u32,
  pub hasValidatedAutoModeConfiguration: u32,
  pub useAutoModeConfiguration: u32,
  pub fromBaseLocalizedNib: u32,
  pub autovalidationDisabled: u32,
  pub tagHasBeenSet: u32,
  pub sizeHasBeenSet: u32,
  pub stateWasDisabledBeforeSheet: u32,
  pub wantsToBeCentered: u32,
  pub isMeasuring: u32,
  pub ignoresEncodedMinMaxValue: u32,
  pub usesStaticMinMaxValues: u32,
  pub RESERVED: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSToolbarItemValidationProto {
  #[objrs(selector = "validateToolbarItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn validateToolbarItem_(&self, item: &NSToolbarItem) -> bool;
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSCloudSharingValidationProto {}
# [ objrs ( class , super = NSToolbarItem ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSToolbarItemGroup;
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStepper;
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSStepperCell;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSGlyphInfo;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSCharacterCollection {
  NSIdentityMappingCharacterCollection = 0,
  NSAdobeCNS1CharacterCollection = 1,
  NSAdobeGB1CharacterCollection = 2,
  NSAdobeJapan1CharacterCollection = 3,
  NSAdobeJapan2CharacterCollection = 4,
  NSAdobeKorea1CharacterCollection = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSShadow;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTypesetter;
bitflags! { # [ repr ( C ) ] pub struct NSTypesetterControlCharacterAction : usize { const NSTypesetterZeroAdvancementAction = 1 ; const NSTypesetterWhitespaceAction = 2 ; const NSTypesetterHorizontalTabAction = 4 ; const NSTypesetterLineBreakAction = 8 ; const NSTypesetterParagraphBreakAction = 16 ; const NSTypesetterContainerBreakAction = 32 ; } }
# [ objrs ( class , super = NSTypesetter ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSATSTypesetter;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSSearchFieldDelegateProto {
  #[objrs(selector = "searchFieldDidStartSearching:")]
  #[cfg(feature = "RK_AppKit")]
  fn searchFieldDidStartSearching_(&self, sender: &NSSearchField) -> ();
  #[objrs(selector = "searchFieldDidEndSearching:")]
  #[cfg(feature = "RK_AppKit")]
  fn searchFieldDidEndSearching_(&self, sender: &NSSearchField) -> ();
}
# [ objrs ( class , super = NSTextField ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSearchField;
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSSearchFieldCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sfFlags {
  pub sendsWholeSearchString: u32,
  pub maximumRecents: u32,
  pub cancelVisible: u32,
  pub reserved2: u32,
  pub disableText: u32,
  pub menuTracking: u32,
  pub deferredUpdate: u32,
  pub sendsImmediately: u32,
  pub centeredLook: u32,
  pub renderingCentered: u32,
  pub becomeTransition: u32,
  pub resignTransition: u32,
  pub subclassOverridesRectForSearchButtonWhenCentered: u32,
  pub subclassOverridesRectForSearchTextWhenCentered: u32,
  pub subclassOverridesRectForCancelButtonWhenCentered: u32,
  pub resumeEditingOnCancel: u32,
  pub reserved: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __bindingsControllerFlags {
  pub _alwaysPresentsApplicationModalAlerts: u32,
  pub _refreshesAllModelKeys: u32,
  pub _multipleObservedModelObjects: u32,
  pub _isEditing: u32,
  pub _reservedController: u32,
}
# [ objrs ( class , super = NSController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSObjectController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __objectControllerFlags {
  pub _editable: u32,
  pub _automaticallyPreparesContent: u32,
  pub _hasLoadedData: u32,
  pub _explicitlyCannotAdd: u32,
  pub _explicitlyCannotRemove: u32,
  pub _isUsingManagedProxy: u32,
  pub _hasFetched: u32,
  pub _batches: u32,
  pub _reservedObjectController: u32,
}
# [ objrs ( class , super = NSObjectController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSArrayController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arrayControllerFlags {
  pub _avoidsEmptySelection: u32,
  pub _preservesSelection: u32,
  pub _selectsInsertedObjects: u32,
  pub _alwaysUsesMultipleValuesMarker: u32,
  pub _refreshesAllModelObjects: u32,
  pub _filterRestrictsInsertion: u32,
  pub _overridesArrangeObjects: u32,
  pub _overridesDidChangeArrangementCriteria: u32,
  pub _explicitlyCannotInsert: u32,
  pub _generatedEmptyArray: u32,
  pub _isObservingKeyPathsThroughArrangedObjects: u32,
  pub _arrangedObjectsIsMutable: u32,
  pub _clearsFilterPredicateOnInsertion: u32,
  pub _skipSortingAfterFetch: u32,
  pub _automaticallyRearrangesObjects: u32,
  pub _reservedArrayController: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDictionaryControllerKeyValuePair;
# [ objrs ( class , super = NSArrayController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDictionaryController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __dictionaryControllerFlags {
  pub _deepCopiesValues: u32,
  pub _suppressBuildingDictionary: u32,
  pub _reservedDictionaryController: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTreeNode;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __NSTreeNodeFlags {
  pub ignoreObserving: u32,
  pub reserved: u32,
}
# [ objrs ( class , super = NSObjectController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTreeController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __treeControllerFlags {
  pub _avoidsEmptySelection: u32,
  pub _preservesSelection: u32,
  pub _selectsInsertedObjects: u32,
  pub _explicitlyCannotInsert: u32,
  pub _explicitlyCannotInsertChild: u32,
  pub _explicitlyCannotAddChild: u32,
  pub _alwaysUsesMultipleValuesMarker: u32,
  pub _observingThroughArrangedObjects: u32,
  pub _mutatingNodes: u32,
  pub _performingFetch: u32,
  pub _skipSortingAfterFetch: u32,
  pub _usesIdenticalComparisonOfModelObjects: u32,
  pub _reservedTreeController: u32,
}
# [ objrs ( class , super = NSController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSUserDefaultsController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __userDefaultsControllerFlags {
  pub _sharedInstance: u32,
  pub _appliesImmediately: u32,
  pub _reservedUserDefaultsController: u32,
}
bitflags! { # [ repr ( C ) ] pub struct NSTextListOptions : usize { const NSTextListPrependEnclosingMarker = 1 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextList;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextBlockValueType {
  NSTextBlockAbsoluteValueType = 0,
  NSTextBlockPercentageValueType = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextBlockDimension {
  NSTextBlockWidth = 0,
  NSTextBlockMinimumWidth = 1,
  NSTextBlockMaximumWidth = 2,
  NSTextBlockHeight = 4,
  NSTextBlockMinimumHeight = 5,
  NSTextBlockMaximumHeight = 6,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSTextBlockLayer {
  NSTextBlockPadding = -1,
  NSTextBlockBorder = 0,
  NSTextBlockMargin = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextBlockVerticalAlignment {
  NSTextBlockTopAlignment = 0,
  NSTextBlockMiddleAlignment = 1,
  NSTextBlockBottomAlignment = 2,
  NSTextBlockBaselineAlignment = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSTextTableLayoutAlgorithm {
  NSTextTableAutomaticLayoutAlgorithm = 0,
  NSTextTableFixedLayoutAlgorithm = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextBlock;
# [ objrs ( class , super = NSTextBlock ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextTableBlock;
# [ objrs ( class , super = NSTextBlock ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextTable;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDatePickerStyle {
  NSDatePickerStyleTextFieldAndStepper = 0,
  NSDatePickerStyleClockAndCalendar = 1,
  NSDatePickerStyleTextField = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDatePickerMode {
  NSDatePickerModeSingle = 0,
  NSDatePickerModeRange = 1,
}
bitflags! { # [ repr ( C ) ] pub struct NSDatePickerElementFlags : usize { const NSDatePickerElementFlagHourMinute = 12 ; const NSDatePickerElementFlagHourMinuteSecond = 14 ; const NSDatePickerElementFlagTimeZone = 16 ; const NSDatePickerElementFlagYearMonth = 192 ; const NSDatePickerElementFlagYearMonthDay = 224 ; const NSDatePickerElementFlagEra = 256 ; } }
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDatePickerCell;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __dateCellFlags {
  pub elements: u32,
  pub controlStyle: u32,
  pub controlMode: u32,
  pub trackingHand: u32,
  pub reserved2: u32,
  pub drawsBackground: u32,
  pub digitsEntered: u32,
  pub forcesLeadingZeroes: u32,
  pub wrapsDateComponentArithmetic: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSDatePickerCellDelegateProto {
  #[objrs(selector = "datePickerCell:validateProposedDateValue:timeInterval:")]
  #[cfg(feature = "RK_AppKit")]
  fn datePickerCell_validateProposedDateValue_timeInterval_(
    &self,
    datePickerCell: &NSDatePickerCell,
    proposedDateValue: &mut &Arc<NSDate>,
    proposedTimeInterval: Option<&f64>,
  ) -> ();
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDatePicker;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSLevelIndicatorStyle {
  NSLevelIndicatorStyleRelevancy = 0,
  NSLevelIndicatorStyleContinuousCapacity = 1,
  NSLevelIndicatorStyleDiscreteCapacity = 2,
  NSLevelIndicatorStyleRating = 3,
}
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLevelIndicatorCell;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSLevelIndicatorPlaceholderVisibility {
  NSLevelIndicatorPlaceholderVisibilityAutomatic = 0,
  NSLevelIndicatorPlaceholderVisibilityAlways = 1,
  NSLevelIndicatorPlaceholderVisibilityWhileEditing = 2,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSLevelIndicator;
# [ objrs ( class , super = NSDocument ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPersistentDocument;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRuleEditorNestingMode {
  NSRuleEditorNestingModeSingle = 0,
  NSRuleEditorNestingModeList = 1,
  NSRuleEditorNestingModeCompound = 2,
  NSRuleEditorNestingModeSimple = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSRuleEditorRowType {
  NSRuleEditorRowTypeSimple = 0,
  NSRuleEditorRowTypeCompound = 1,
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSRuleEditor;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSRuleEditorDelegateProto {
  #[objrs(selector = "ruleEditor:numberOfChildrenForCriterion:withRowType:")]
  #[cfg(feature = "RK_AppKit")]
  fn ruleEditor_numberOfChildrenForCriterion_withRowType_(
    &self,
    editor: &NSRuleEditor,
    criterion: Option<&Object>,
    rowType: NSRuleEditorRowType,
  ) -> isize;
  #[objrs(selector = "ruleEditor:child:forCriterion:withRowType:")]
  #[cfg(feature = "RK_AppKit")]
  fn ruleEditor_child_forCriterion_withRowType_(
    &self,
    editor: &NSRuleEditor,
    index: isize,
    criterion: Option<&Object>,
    rowType: NSRuleEditorRowType,
  ) -> Arc<Object>;
  #[objrs(selector = "ruleEditor:displayValueForCriterion:inRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn ruleEditor_displayValueForCriterion_inRow_(
    &self,
    editor: &NSRuleEditor,
    criterion: &Object,
    row: isize,
  ) -> Arc<Object>;
  #[objrs(selector = "ruleEditor:predicatePartsForCriterion:withDisplayValue:inRow:")]
  #[cfg(feature = "RK_AppKit")]
  fn ruleEditor_predicatePartsForCriterion_withDisplayValue_inRow_(
    &self,
    editor: &NSRuleEditor,
    criterion: &Object,
    value: &Object,
    row: isize,
  ) -> Option<Arc<NSDictionary>>;
  #[objrs(selector = "ruleEditorRowsDidChange:")]
  #[cfg(feature = "RK_AppKit")]
  fn ruleEditorRowsDidChange_(&self, notification: &NSNotification) -> ();
}
# [ objrs ( class , super = NSRuleEditor ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPredicateEditor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPredicateEditorRowTemplate;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPathStyle {
  NSPathStyleStandard = 0,
  NSPathStylePopUp = 2,
  NSPathStyleNavigationBar = 1,
}
# [ objrs ( class , super = NSActionCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPathCell;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPathCellDelegateProto {
  #[objrs(selector = "pathCell:willDisplayOpenPanel:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathCell_willDisplayOpenPanel_(&self, pathCell: &NSPathCell, openPanel: &NSOpenPanel) -> ();
  #[objrs(selector = "pathCell:willPopUpMenu:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathCell_willPopUpMenu_(&self, pathCell: &NSPathCell, menu: &NSMenu) -> ();
}
# [ objrs ( class , super = NSControl ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPathControl;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPathControlDelegateProto {
  #[objrs(selector = "pathControl:shouldDragItem:withPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_shouldDragItem_withPasteboard_(
    &self,
    pathControl: &NSPathControl,
    pathItem: &NSPathControlItem,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "pathControl:shouldDragPathComponentCell:withPasteboard:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_shouldDragPathComponentCell_withPasteboard_(
    &self,
    pathControl: &NSPathControl,
    pathComponentCell: &NSPathComponentCell,
    pasteboard: &NSPasteboard,
  ) -> bool;
  #[objrs(selector = "pathControl:validateDrop:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_validateDrop_(
    &self,
    pathControl: &NSPathControl,
    info: &Object,
  ) -> NSDragOperation;
  #[objrs(selector = "pathControl:acceptDrop:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_acceptDrop_(&self, pathControl: &NSPathControl, info: &Object) -> bool;
  #[objrs(selector = "pathControl:willDisplayOpenPanel:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_willDisplayOpenPanel_(
    &self,
    pathControl: &NSPathControl,
    openPanel: &NSOpenPanel,
  ) -> ();
  #[objrs(selector = "pathControl:willPopUpMenu:")]
  #[cfg(feature = "RK_AppKit")]
  fn pathControl_willPopUpMenu_(&self, pathControl: &NSPathControl, menu: &NSMenu) -> ();
}
# [ objrs ( class , super = NSTextFieldCell ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPathComponentCell;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPathControlItem;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPageControllerTransitionStyle {
  NSPageControllerTransitionStyleStackHistory = 0,
  NSPageControllerTransitionStyleStackBook = 1,
  NSPageControllerTransitionStyleHorizontalStrip = 2,
}
# [ objrs ( class , super = NSViewController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPageController;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pcDelegateFlags {
  pub delegateRespondsToIdentifierForRepresentedObject: u32,
  pub delegateRespondsToViewControllerForIdentifier: u32,
  pub delegateRespondsToFrameForRepresentedObject: u32,
  pub delegateRespondsToPrepareView: u32,
  pub delegateRespondsToDidTransition: u32,
  pub delegateRespondsToWillLiveTransition: u32,
  pub delegateRespondsToDidLiveTransition: u32,
  pub delegateRespondsToReserved1: u32,
  pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pcFlags {
  pub templateCacheIsInvalid: u32,
  pub private1: u32,
  pub private2: u32,
  pub inSwipeGesture: u32,
  pub reserved: u32,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSPageControllerDelegateProto {
  #[objrs(selector = "pageController:identifierForObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageController_identifierForObject_(
    &self,
    pageController: &NSPageController,
    object: &Object,
  ) -> Arc<NSString>;
  #[objrs(selector = "pageController:viewControllerForIdentifier:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageController_viewControllerForIdentifier_(
    &self,
    pageController: &NSPageController,
    identifier: &NSString,
  ) -> Arc<NSViewController>;
  #[objrs(selector = "pageController:frameForObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageController_frameForObject_(
    &self,
    pageController: &NSPageController,
    object: Option<&Object>,
  ) -> CGRect;
  #[objrs(selector = "pageController:prepareViewController:withObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageController_prepareViewController_withObject_(
    &self,
    pageController: &NSPageController,
    viewController: &NSViewController,
    object: Option<&Object>,
  ) -> ();
  #[objrs(selector = "pageController:didTransitionToObject:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageController_didTransitionToObject_(
    &self,
    pageController: &NSPageController,
    object: &Object,
  ) -> ();
  #[objrs(selector = "pageControllerWillStartLiveTransition:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageControllerWillStartLiveTransition_(&self, pageController: &NSPageController) -> ();
  #[objrs(selector = "pageControllerDidEndLiveTransition:")]
  #[cfg(feature = "RK_AppKit")]
  fn pageControllerDidEndLiveTransition_(&self, pageController: &NSPageController) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextInputContext;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSUserInterfaceItemSearchingProto {
  #[objrs(selector = "searchForItemsWithSearchString:resultLimit:matchedItemHandler:")]
  #[cfg(feature = "RK_AppKit")]
  fn searchForItemsWithSearchString_resultLimit_matchedItemHandler_(
    &self,
    searchString: &NSString,
    resultLimit: isize,
    handleMatchedItems: (),
  ) -> ();
  #[objrs(selector = "localizedTitlesForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn localizedTitlesForItem_(&self, item: &Object) -> Arc<NSArray>;
  #[objrs(selector = "performActionForItem:")]
  #[cfg(feature = "RK_AppKit")]
  fn performActionForItem_(&self, item: &Object) -> ();
  #[objrs(selector = "showAllHelpTopicsForSearchString:")]
  #[cfg(feature = "RK_AppKit")]
  fn showAllHelpTopicsForSearchString_(&self, searchString: &NSString) -> ();
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSWindowRestorationProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTextAlternatives;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSVisualEffectMaterial {
  NSVisualEffectMaterialTitlebar = 3,
  NSVisualEffectMaterialSelection = 4,
  NSVisualEffectMaterialMenu = 5,
  NSVisualEffectMaterialPopover = 6,
  NSVisualEffectMaterialSidebar = 7,
  NSVisualEffectMaterialHeaderView = 10,
  NSVisualEffectMaterialSheet = 11,
  NSVisualEffectMaterialWindowBackground = 12,
  NSVisualEffectMaterialHUDWindow = 13,
  NSVisualEffectMaterialFullScreenUI = 15,
  NSVisualEffectMaterialToolTip = 17,
  NSVisualEffectMaterialContentBackground = 18,
  NSVisualEffectMaterialUnderWindowBackground = 21,
  NSVisualEffectMaterialUnderPageBackground = 22,
  NSVisualEffectMaterialAppearanceBased = 0,
  NSVisualEffectMaterialLight = 1,
  NSVisualEffectMaterialDark = 2,
  NSVisualEffectMaterialMediumLight = 8,
  NSVisualEffectMaterialUltraDark = 9,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSVisualEffectBlendingMode {
  NSVisualEffectBlendingModeBehindWindow = 0,
  NSVisualEffectBlendingModeWithinWindow = 1,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSVisualEffectState {
  NSVisualEffectStateFollowsWindowActiveState = 0,
  NSVisualEffectStateActive = 1,
  NSVisualEffectStateInactive = 2,
}
# [ objrs ( class , super = NSView ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSVisualEffectView;
#[repr(C)]
pub struct NSVisualEffectViewInternal {
  opaque: u32,
}
# [ objrs ( class , super = NSViewController ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSTitlebarAccessoryViewController;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSDataAsset;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSHapticFeedbackPattern {
  NSHapticFeedbackPatternGeneric = 0,
  NSHapticFeedbackPatternAlignment = 1,
  NSHapticFeedbackPatternLevelChange = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSHapticFeedbackPerformanceTime {
  NSHapticFeedbackPerformanceTimeDefault = 0,
  NSHapticFeedbackPerformanceTimeNow = 1,
  NSHapticFeedbackPerformanceTimeDrawCompleted = 2,
}
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSHapticFeedbackPerformerProto {
  #[objrs(selector = "performFeedbackPattern:performanceTime:")]
  #[cfg(feature = "RK_AppKit")]
  fn performFeedbackPattern_performanceTime_(
    &self,
    pattern: NSHapticFeedbackPattern,
    performanceTime: NSHapticFeedbackPerformanceTime,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSHapticFeedbackManager;
#[objrs(protocol)]
#[link(name = "AppKit", kind = "framework")]
pub trait NSAlignmentFeedbackTokenProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSAlignmentFeedbackFilter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "AppKit", kind = "framework")]
pub struct NSPressureConfiguration;
#[cfg(feature = "RK_AppKit")]
#[link(name = "AppKit", kind = "framework")]
extern "C" {
  pub fn NSBestDepth(
    colorSpace: *mut NSString,
    bps: isize,
    bpp: isize,
    planar: bool,
    exactMatch: *mut bool,
  ) -> NSWindowDepth;
  pub fn NSPlanarFromDepth(depth: NSWindowDepth) -> bool;
  pub fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> *mut NSString;
  pub fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> isize;
  pub fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> isize;
  pub fn NSNumberOfColorComponents(colorSpaceName: *mut NSString) -> isize;
  pub fn NSAvailableWindowDepths() -> *mut NSWindowDepth;
  pub fn NSRectFill(rect: CGRect) -> ();
  pub fn NSRectFillList(rects: *mut CGRect, count: isize) -> ();
  pub fn NSRectFillListWithGrays(rects: *mut CGRect, grays: *mut f64, num: isize) -> ();
  pub fn NSRectFillListWithColors(rects: *mut CGRect, colors: *mut *mut NSColor, num: isize) -> ();
  pub fn NSRectFillUsingOperation(rect: CGRect, op: NSCompositingOperation) -> ();
  pub fn NSRectFillListUsingOperation(
    rects: *mut CGRect,
    count: isize,
    op: NSCompositingOperation,
  ) -> ();
  pub fn NSRectFillListWithColorsUsingOperation(
    rects: *mut CGRect,
    colors: *mut *mut NSColor,
    num: isize,
    op: NSCompositingOperation,
  ) -> ();
  pub fn NSFrameRect(rect: CGRect) -> ();
  pub fn NSFrameRectWithWidth(rect: CGRect, frameWidth: f64) -> ();
  pub fn NSFrameRectWithWidthUsingOperation(
    rect: CGRect,
    frameWidth: f64,
    op: NSCompositingOperation,
  ) -> ();
  pub fn NSRectClip(rect: CGRect) -> ();
  pub fn NSRectClipList(rects: *mut CGRect, count: isize) -> ();
  pub fn NSDrawTiledRects(
    boundsRect: CGRect,
    clipRect: CGRect,
    sides: *mut NSRectEdge,
    grays: *mut f64,
    count: isize,
  ) -> CGRect;
  pub fn NSDrawGrayBezel(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSDrawGroove(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSDrawWhiteBezel(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSDrawButton(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSEraseRect(rect: CGRect) -> ();
  pub fn NSReadPixel(passedPoint: CGPoint) -> *mut NSColor;
  pub fn NSDrawBitmap(
    rect: CGRect,
    width: isize,
    height: isize,
    bps: isize,
    spp: isize,
    bpp: isize,
    bpr: isize,
    isPlanar: bool,
    hasAlpha: bool,
    colorSpaceName: *mut NSString,
    data: [*const u8; 5],
  ) -> ();
  pub fn NSHighlightRect(rect: CGRect) -> ();
  pub fn NSBeep() -> ();
  pub fn NSGetWindowServerMemory(
    context: isize,
    virtualMemory: *mut isize,
    windowBackingMemory: *mut isize,
    windowDumpString: *mut *mut NSString,
  ) -> isize;
  pub fn NSDrawColorTiledRects(
    boundsRect: CGRect,
    clipRect: CGRect,
    sides: *mut NSRectEdge,
    colors: *mut *mut NSColor,
    count: isize,
  ) -> CGRect;
  pub fn NSDrawDarkBezel(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSDrawLightBezel(rect: CGRect, clipRect: CGRect) -> ();
  pub fn NSDottedFrameRect(rect: CGRect) -> ();
  pub fn NSDrawWindowBackground(rect: CGRect) -> ();
  pub fn NSSetFocusRingStyle(placement: NSFocusRingPlacement) -> ();
  pub fn NSDisableScreenUpdates() -> ();
  pub fn NSEnableScreenUpdates() -> ();
  pub fn NSShowAnimationEffect(
    animationEffect: NSAnimationEffect,
    centerLocation: CGPoint,
    size: CGSize,
    animationDelegate: *mut Object,
    didEndSelector: SelectorRef,
    contextInfo: *mut c_void,
  ) -> ();
  pub fn NSCountWindows(count: *mut isize) -> ();
  pub fn NSWindowList(size: isize, list: *mut isize) -> ();
  pub fn NSCountWindowsForContext(context: isize, count: *mut isize) -> ();
  pub fn NSWindowListForContext(context: isize, size: isize, list: *mut isize) -> ();
  pub fn NSCopyBits(srcGState: isize, srcRect: CGRect, destPoint: CGPoint) -> ();
  pub fn NSAccessibilityPostNotificationWithUserInfo(
    element: *mut Object,
    notification: *mut NSString,
    userInfo: *mut NSDictionary,
  ) -> ();
  pub fn NSAccessibilityFrameInView(parentView: *mut NSView, frame: CGRect) -> CGRect;
  pub fn NSAccessibilityPointInView(parentView: *mut NSView, point: CGPoint) -> CGPoint;
  pub fn NSAccessibilitySetMayContainProtectedContent(flag: bool) -> bool;
  pub fn NSAccessibilityRoleDescription(
    role: *mut NSString,
    subrole: *mut NSString,
  ) -> *mut NSString;
  pub fn NSAccessibilityRoleDescriptionForUIElement(element: *mut Object) -> *mut NSString;
  pub fn NSAccessibilityActionDescription(action: *mut NSString) -> *mut NSString;
  pub fn NSAccessibilityRaiseBadArgumentException(
    element: *mut Object,
    attribute: *mut NSString,
    value: *mut Object,
  ) -> ();
  pub fn NSAccessibilityUnignoredAncestor(element: *mut Object) -> *mut Object;
  pub fn NSAccessibilityUnignoredDescendant(element: *mut Object) -> *mut Object;
  pub fn NSAccessibilityUnignoredChildren(originalChildren: *mut NSArray) -> *mut NSArray;
  pub fn NSAccessibilityUnignoredChildrenForOnlyChild(originalChild: *mut Object) -> *mut NSArray;
  pub fn NSAccessibilityPostNotification(element: *mut Object, notification: *mut NSString) -> ();
  pub fn NSCreateFilenamePboardType(fileType: *mut NSString) -> *mut NSString;
  pub fn NSCreateFileContentsPboardType(fileType: *mut NSString) -> *mut NSString;
  pub fn NSGetFileType(pboardType: *mut NSString) -> *mut NSString;
  pub fn NSGetFileTypes(pboardTypes: *mut NSArray) -> *mut NSArray;
  pub fn NSTouchTypeMaskFromType(type_: NSTouchType) -> NSTouchTypeMask;
  pub fn NSEventMaskFromType(type_: NSEventType) -> NSEventMask;
  pub fn NSDrawThreePartImage(
    frame: CGRect,
    startCap: *mut NSImage,
    centerFill: *mut NSImage,
    endCap: *mut NSImage,
    vertical: bool,
    op: NSCompositingOperation,
    alphaFraction: f64,
    flipped: bool,
  ) -> ();
  pub fn NSDrawNinePartImage(
    frame: CGRect,
    topLeftCorner: *mut NSImage,
    topEdgeFill: *mut NSImage,
    topRightCorner: *mut NSImage,
    leftEdgeFill: *mut NSImage,
    centerFill: *mut NSImage,
    rightEdgeFill: *mut NSImage,
    bottomLeftCorner: *mut NSImage,
    bottomEdgeFill: *mut NSImage,
    bottomRightCorner: *mut NSImage,
    op: NSCompositingOperation,
    alphaFraction: f64,
    flipped: bool,
  ) -> ();
  pub fn NSIsControllerMarker(object: *mut Object) -> bool;
  pub fn NSApplicationMain(argc: i32, argv: *mut *mut i8) -> i32;
  pub fn NSApplicationLoad() -> bool;
  pub fn NSShowsServicesMenuItem(itemName: *mut NSString) -> bool;
  pub fn NSSetShowsServicesMenuItem(itemName: *mut NSString, enabled: bool) -> isize;
  pub fn NSUpdateDynamicServices() -> ();
  pub fn NSPerformService(itemName: *mut NSString, pboard: *mut NSPasteboard) -> bool;
  pub fn NSRegisterServicesProvider(provider: *mut Object, name: *mut NSString) -> ();
  pub fn NSUnregisterServicesProvider(name: *mut NSString) -> ();
  pub fn NSConvertGlyphsToPackedGlyphs(
    glBuf: *mut u32,
    count: isize,
    packing: NSMultibyteGlyphPacking,
    packedGlyphs: *mut i8,
  ) -> isize;
  pub fn NSRunAlertPanelRelativeToWindow(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    ...
  ) -> isize;
  pub fn NSRunInformationalAlertPanelRelativeToWindow(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    ...
  ) -> isize;
  pub fn NSRunCriticalAlertPanelRelativeToWindow(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    ...
  ) -> isize;
  pub fn NSRunAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> isize;
  pub fn NSRunInformationalAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> isize;
  pub fn NSRunCriticalAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> isize;
  pub fn NSBeginAlertSheet(
    title: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    modalDelegate: *mut Object,
    didEndSelector: SelectorRef,
    didDismissSelector: SelectorRef,
    contextInfo: *mut c_void,
    msgFormat: *mut NSString,
    ...
  ) -> ();
  pub fn NSBeginInformationalAlertSheet(
    title: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    modalDelegate: *mut Object,
    didEndSelector: SelectorRef,
    didDismissSelector: SelectorRef,
    contextInfo: *mut c_void,
    msgFormat: *mut NSString,
    ...
  ) -> ();
  pub fn NSBeginCriticalAlertSheet(
    title: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    docWindow: *mut NSWindow,
    modalDelegate: *mut Object,
    didEndSelector: SelectorRef,
    didDismissSelector: SelectorRef,
    contextInfo: *mut c_void,
    msgFormat: *mut NSString,
    ...
  ) -> ();
  pub fn NSGetAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> *mut Object;
  pub fn NSGetInformationalAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> *mut Object;
  pub fn NSGetCriticalAlertPanel(
    title: *mut NSString,
    msgFormat: *mut NSString,
    defaultButton: *mut NSString,
    alternateButton: *mut NSString,
    otherButton: *mut NSString,
    ...
  ) -> *mut Object;
  pub fn NSReleaseAlertPanel(panel: *mut Object) -> ();
  pub fn _NSDictionaryOfVariableBindings(
    commaSeparatedKeysString: *mut NSString,
    firstValue: *mut Object,
    ...
  ) -> *mut NSDictionary;
  pub fn NSInterfaceStyleForKey(key: *mut NSString, responder: *mut NSResponder) -> usize;
  pub fn NSOpenGLSetOption(pname: NSOpenGLGlobalOption, param: i32) -> ();
  pub fn NSOpenGLGetOption(pname: NSOpenGLGlobalOption, param: *mut i32) -> ();
  pub fn NSOpenGLGetVersion(major: *mut i32, minor: *mut i32) -> ();
}
