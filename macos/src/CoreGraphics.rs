#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFData;
use crate::CoreFoundation::__CFDate;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFMachPort;
use crate::CoreFoundation::__CFRunLoopSource;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
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
pub struct __IOSurface {
  opaque: u32,
}
#[repr(C)]
pub struct CGAffineTransform {
  opaque: u32,
}
#[repr(C)]
pub struct CGPoint {
  opaque: u32,
}
#[repr(C)]
pub struct CGSize {
  opaque: u32,
}
#[repr(C)]
pub struct CGVector {
  opaque: u32,
}
#[repr(C)]
pub struct CGRect {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGRectEdge {
  CGRectMinXEdge = 0,
  CGRectMinYEdge = 1,
  CGRectMaxXEdge = 2,
  CGRectMaxYEdge = 3,
}
#[repr(C)]
pub struct CGContext {
  opaque: u32,
}
#[repr(C)]
pub struct CGColor {
  opaque: u32,
}
#[repr(C)]
pub struct CGColorSpace {
  opaque: u32,
}
#[repr(C)]
pub struct CGDataProvider {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGDataProviderSequentialCallbacks {
  pub version: u32,
  pub getBytes: Option<extern "C" fn(*mut c_void, *mut c_void, usize) -> usize>,
  pub skipForward: Option<extern "C" fn(*mut c_void, i64) -> i64>,
  pub rewind: Option<extern "C" fn(*mut c_void) -> ()>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGDataProviderDirectCallbacks {
  pub version: u32,
  pub getBytePointer: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub releaseBytePointer: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
  pub getBytesAtPosition: Option<extern "C" fn(*mut c_void, *mut c_void, i64, usize) -> usize>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGColorRenderingIntent {
  kCGRenderingIntentDefault = 0,
  kCGRenderingIntentAbsoluteColorimetric = 1,
  kCGRenderingIntentRelativeColorimetric = 2,
  kCGRenderingIntentPerceptual = 3,
  kCGRenderingIntentSaturation = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGColorSpaceModel {
  kCGColorSpaceModelUnknown = -1,
  kCGColorSpaceModelMonochrome = 0,
  kCGColorSpaceModelRGB = 1,
  kCGColorSpaceModelCMYK = 2,
  kCGColorSpaceModelLab = 3,
  kCGColorSpaceModelDeviceN = 4,
  kCGColorSpaceModelIndexed = 5,
  kCGColorSpaceModelPattern = 6,
  kCGColorSpaceModelXYZ = 7,
}
#[repr(C)]
pub struct CGPattern {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPatternTiling {
  kCGPatternTilingNoDistortion = 0,
  kCGPatternTilingConstantSpacingMinimalDistortion = 1,
  kCGPatternTilingConstantSpacing = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGPatternCallbacks {
  pub version: u32,
  pub drawPattern: Option<extern "C" fn(*mut c_void, *mut CGContext) -> ()>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(C)]
pub struct CGFont {
  opaque: u32,
}
pub type CGGlyph = u16;
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGFontPostScriptFormat {
  kCGFontPostScriptFormatType1 = 1,
  kCGFontPostScriptFormatType3 = 3,
  kCGFontPostScriptFormatType42 = 42,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGGlyphDeprecatedEnum {
  CGGlyphMin = 0,
  CGGlyphMax = 1,
}
#[repr(C)]
pub struct CGGradient {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CGGradientDrawingOptions : u32 { const kCGGradientDrawsBeforeStartLocation = 1 ; const kCGGradientDrawsAfterEndLocation = 2 ; } }
#[repr(C)]
pub struct CGImage {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGImageAlphaInfo {
  kCGImageAlphaNone = 0,
  kCGImageAlphaPremultipliedLast = 1,
  kCGImageAlphaPremultipliedFirst = 2,
  kCGImageAlphaLast = 3,
  kCGImageAlphaFirst = 4,
  kCGImageAlphaNoneSkipLast = 5,
  kCGImageAlphaNoneSkipFirst = 6,
  kCGImageAlphaOnly = 7,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGImageByteOrderInfo {
  kCGImageByteOrderMask = 28672,
  kCGImageByteOrderDefault = 0,
  kCGImageByteOrder16Little = 4096,
  kCGImageByteOrder32Little = 8192,
  kCGImageByteOrder16Big = 12288,
  kCGImageByteOrder32Big = 16384,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGImagePixelFormatInfo {
  kCGImagePixelFormatMask = 983040,
  kCGImagePixelFormatPacked = 0,
  kCGImagePixelFormatRGB555 = 65536,
  kCGImagePixelFormatRGB565 = 131072,
  kCGImagePixelFormatRGB101010 = 196608,
  kCGImagePixelFormatRGBCIF10 = 262144,
}
bitflags! { # [ repr ( C ) ] pub struct CGBitmapInfo : u32 { const kCGBitmapAlphaInfoMask = 31 ; const kCGBitmapFloatInfoMask = 3840 ; const kCGBitmapFloatComponents = 256 ; const kCGBitmapByteOrderMask = 28672 ; const kCGBitmapByteOrderDefault = 0 ; const kCGBitmapByteOrder16Little = 4096 ; const kCGBitmapByteOrder32Little = 8192 ; const kCGBitmapByteOrder16Big = 12288 ; const kCGBitmapByteOrder32Big = 16384 ; } }
#[repr(C)]
pub struct CGPath {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGLineJoin {
  kCGLineJoinMiter = 0,
  kCGLineJoinRound = 1,
  kCGLineJoinBevel = 2,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGLineCap {
  kCGLineCapButt = 0,
  kCGLineCapRound = 1,
  kCGLineCapSquare = 2,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPathElementType {
  kCGPathElementMoveToPoint = 0,
  kCGPathElementAddLineToPoint = 1,
  kCGPathElementAddQuadCurveToPoint = 2,
  kCGPathElementAddCurveToPoint = 3,
  kCGPathElementCloseSubpath = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGPathElement {
  pub type_: CGPathElementType,
  pub points: *mut CGPoint,
}
#[repr(C)]
pub struct CGPDFDocument {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFPage {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFDictionary {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFArray {
  opaque: u32,
}
pub type CGPDFReal = f64;
#[repr(C)]
pub struct CGPDFObject {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPDFObjectType {
  kCGPDFObjectTypeNull = 1,
  kCGPDFObjectTypeBoolean = 2,
  kCGPDFObjectTypeInteger = 3,
  kCGPDFObjectTypeReal = 4,
  kCGPDFObjectTypeName = 5,
  kCGPDFObjectTypeString = 6,
  kCGPDFObjectTypeArray = 7,
  kCGPDFObjectTypeDictionary = 8,
  kCGPDFObjectTypeStream = 9,
}
#[repr(C)]
pub struct CGPDFStream {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPDFDataFormat {
  CGPDFDataFormatRaw = 0,
  CGPDFDataFormatJPEGEncoded = 1,
  CGPDFDataFormatJPEG2000 = 2,
}
#[repr(C)]
pub struct CGPDFString {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPDFBox {
  kCGPDFMediaBox = 0,
  kCGPDFCropBox = 1,
  kCGPDFBleedBox = 2,
  kCGPDFTrimBox = 3,
  kCGPDFArtBox = 4,
}
bitflags! { # [ repr ( C ) ] pub struct CGPDFAccessPermissions : u32 { const kCGPDFAllowsLowQualityPrinting = 1 ; const kCGPDFAllowsHighQualityPrinting = 2 ; const kCGPDFAllowsDocumentChanges = 4 ; const kCGPDFAllowsDocumentAssembly = 8 ; const kCGPDFAllowsContentCopying = 16 ; const kCGPDFAllowsContentAccessibility = 32 ; const kCGPDFAllowsCommenting = 64 ; const kCGPDFAllowsFormFieldEntry = 128 ; } }
#[repr(C)]
pub struct CGShading {
  opaque: u32,
}
#[repr(C)]
pub struct CGFunction {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGFunctionCallbacks {
  pub version: u32,
  pub evaluate: Option<extern "C" fn(*mut c_void, *mut f64, *mut f64) -> ()>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGPathDrawingMode {
  kCGPathFill = 0,
  kCGPathEOFill = 1,
  kCGPathStroke = 2,
  kCGPathFillStroke = 3,
  kCGPathEOFillStroke = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGTextDrawingMode {
  kCGTextFill = 0,
  kCGTextStroke = 1,
  kCGTextFillStroke = 2,
  kCGTextInvisible = 3,
  kCGTextFillClip = 4,
  kCGTextStrokeClip = 5,
  kCGTextFillStrokeClip = 6,
  kCGTextClip = 7,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGTextEncoding {
  kCGEncodingFontSpecific = 0,
  kCGEncodingMacRoman = 1,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGInterpolationQuality {
  kCGInterpolationDefault = 0,
  kCGInterpolationNone = 1,
  kCGInterpolationLow = 2,
  kCGInterpolationMedium = 4,
  kCGInterpolationHigh = 3,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGBlendMode {
  kCGBlendModeNormal = 0,
  kCGBlendModeMultiply = 1,
  kCGBlendModeScreen = 2,
  kCGBlendModeOverlay = 3,
  kCGBlendModeDarken = 4,
  kCGBlendModeLighten = 5,
  kCGBlendModeColorDodge = 6,
  kCGBlendModeColorBurn = 7,
  kCGBlendModeSoftLight = 8,
  kCGBlendModeHardLight = 9,
  kCGBlendModeDifference = 10,
  kCGBlendModeExclusion = 11,
  kCGBlendModeHue = 12,
  kCGBlendModeSaturation = 13,
  kCGBlendModeColor = 14,
  kCGBlendModeLuminosity = 15,
  kCGBlendModeClear = 16,
  kCGBlendModeCopy = 17,
  kCGBlendModeSourceIn = 18,
  kCGBlendModeSourceOut = 19,
  kCGBlendModeSourceAtop = 20,
  kCGBlendModeDestinationOver = 21,
  kCGBlendModeDestinationIn = 22,
  kCGBlendModeDestinationOut = 23,
  kCGBlendModeDestinationAtop = 24,
  kCGBlendModeXOR = 25,
  kCGBlendModePlusDarker = 26,
  kCGBlendModePlusLighter = 27,
}
#[repr(C)]
pub struct CGColorConversionInfo {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGColorConversionInfoTransformType {
  kCGColorConversionTransformFromSpace = 0,
  kCGColorConversionTransformToSpace = 1,
  kCGColorConversionTransformApplySpace = 2,
}
#[repr(C)]
pub struct CGDataConsumer {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGDataConsumerCallbacks {
  pub putBytes: Option<extern "C" fn(*mut c_void, *mut c_void, usize) -> usize>,
  pub releaseConsumer: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGError {
  kCGErrorSuccess = 0,
  kCGErrorFailure = 1000,
  kCGErrorIllegalArgument = 1001,
  kCGErrorInvalidConnection = 1002,
  kCGErrorInvalidContext = 1003,
  kCGErrorCannotComplete = 1004,
  kCGErrorNotImplemented = 1006,
  kCGErrorRangeCheck = 1007,
  kCGErrorTypeCheck = 1008,
  kCGErrorInvalidOperation = 1010,
  kCGErrorNoneAvailable = 1011,
}
#[repr(C)]
pub struct CGLayer {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFContentStream {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFOperatorTable {
  opaque: u32,
}
#[repr(C)]
pub struct CGPDFScanner {
  opaque: u32,
}
pub type CGWindowID = u32;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGWindowSharingType {
  kCGWindowSharingNone = 0,
  kCGWindowSharingReadOnly = 1,
  kCGWindowSharingReadWrite = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGWindowBackingType {
  kCGBackingStoreRetained = 0,
  kCGBackingStoreNonretained = 1,
  kCGBackingStoreBuffered = 2,
}
bitflags! { # [ repr ( C ) ] pub struct CGWindowListOption : u32 { const kCGWindowListOptionAll = 0 ; const kCGWindowListOptionOnScreenOnly = 1 ; const kCGWindowListOptionOnScreenAboveWindow = 2 ; const kCGWindowListOptionOnScreenBelowWindow = 4 ; const kCGWindowListOptionIncludingWindow = 8 ; const kCGWindowListExcludeDesktopElements = 16 ; } }
bitflags! { # [ repr ( C ) ] pub struct CGWindowImageOption : u32 { const kCGWindowImageDefault = 0 ; const kCGWindowImageBoundsIgnoreFraming = 1 ; const kCGWindowImageShouldBeOpaque = 2 ; const kCGWindowImageOnlyShadows = 4 ; const kCGWindowImageBestResolution = 8 ; const kCGWindowImageNominalResolution = 16 ; } }
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGWindowLevelKey {
  kCGBaseWindowLevelKey = 0,
  kCGMinimumWindowLevelKey = 1,
  kCGDesktopWindowLevelKey = 2,
  kCGBackstopMenuLevelKey = 3,
  kCGNormalWindowLevelKey = 4,
  kCGFloatingWindowLevelKey = 5,
  kCGTornOffMenuWindowLevelKey = 6,
  kCGDockWindowLevelKey = 7,
  kCGMainMenuWindowLevelKey = 8,
  kCGStatusWindowLevelKey = 9,
  kCGModalPanelWindowLevelKey = 10,
  kCGPopUpMenuWindowLevelKey = 11,
  kCGDraggingWindowLevelKey = 12,
  kCGScreenSaverWindowLevelKey = 13,
  kCGMaximumWindowLevelKey = 14,
  kCGOverlayWindowLevelKey = 15,
  kCGHelpWindowLevelKey = 16,
  kCGUtilityWindowLevelKey = 17,
  kCGDesktopIconWindowLevelKey = 18,
  kCGCursorWindowLevelKey = 19,
  kCGAssistiveTechHighWindowLevelKey = 20,
  kCGNumberOfWindowLevelKeys = 21,
}
pub type CGWindowLevel = i32;
pub type CGDirectDisplayID = u32;
pub type CGOpenGLDisplayMask = u32;
#[repr(C)]
pub struct CGDisplayMode {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CGCaptureOptions : u32 { const kCGCaptureNoOptions = 0 ; const kCGCaptureNoFill = 1 ; } }
pub type CGDisplayCount = u32;
pub type CGDisplayErr = CGError;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGDeviceColor {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}
#[repr(C)]
pub struct _CGDisplayConfigRef {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CGConfigureOption : u32 { const kCGConfigureForAppOnly = 0 ; const kCGConfigureForSession = 1 ; const kCGConfigurePermanently = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct CGDisplayChangeSummaryFlags : u32 { const kCGDisplayBeginConfigurationFlag = 1 ; const kCGDisplayMovedFlag = 2 ; const kCGDisplaySetMainFlag = 4 ; const kCGDisplaySetModeFlag = 8 ; const kCGDisplayAddFlag = 16 ; const kCGDisplayRemoveFlag = 32 ; const kCGDisplayEnabledFlag = 256 ; const kCGDisplayDisabledFlag = 512 ; const kCGDisplayMirrorFlag = 1024 ; const kCGDisplayUnMirrorFlag = 2048 ; const kCGDisplayDesktopShapeChangedFlag = 4096 ; } }
pub type CGDisplayFadeReservationToken = u32;
#[repr(C)]
pub struct CGDisplayStream {
  opaque: u32,
}
#[repr(C)]
pub struct CGDisplayStreamUpdate {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGDisplayStreamUpdateRectType {
  kCGDisplayStreamUpdateRefreshedRects = 0,
  kCGDisplayStreamUpdateMovedRects = 1,
  kCGDisplayStreamUpdateDirtyRects = 2,
  kCGDisplayStreamUpdateReducedDirtyRects = 3,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGDisplayStreamFrameStatus {
  kCGDisplayStreamFrameStatusFrameComplete = 0,
  kCGDisplayStreamFrameStatusFrameIdle = 1,
  kCGDisplayStreamFrameStatusFrameBlank = 2,
  kCGDisplayStreamFrameStatusStopped = 3,
}
pub type CGEventErr = CGError;
pub type CGButtonCount = u32;
pub type CGWheelCount = u32;
pub type CGCharCode = u16;
pub type CGKeyCode = u16;
bitflags! { # [ repr ( C ) ] pub struct CGScreenUpdateOperation : u32 { const kCGScreenUpdateOperationRefresh = 0 ; const kCGScreenUpdateOperationMove = 1 ; const kCGScreenUpdateOperationReducedDirtyRectangleCount = 2147483648 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGScreenUpdateMoveDelta {
  pub dX: i32,
  pub dY: i32,
}
bitflags! { # [ repr ( C ) ] pub struct CGEventFilterMask : u32 { const kCGEventFilterMaskPermitLocalMouseEvents = 1 ; const kCGEventFilterMaskPermitLocalKeyboardEvents = 2 ; const kCGEventFilterMaskPermitSystemDefinedEvents = 4 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventSuppressionState {
  kCGEventSuppressionStateSuppressionInterval = 0,
  kCGEventSuppressionStateRemoteMouseDrag = 1,
  kCGNumberOfEventSuppressionStates = 2,
}
pub type CGRectCount = u32;
#[repr(C)]
pub struct __CGEvent {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGMouseButton {
  kCGMouseButtonLeft = 0,
  kCGMouseButtonRight = 1,
  kCGMouseButtonCenter = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGScrollEventUnit {
  kCGScrollEventUnitPixel = 0,
  kCGScrollEventUnitLine = 1,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGMomentumScrollPhase {
  kCGMomentumScrollPhaseNone = 0,
  kCGMomentumScrollPhaseBegin = 1,
  kCGMomentumScrollPhaseContinue = 2,
  kCGMomentumScrollPhaseEnd = 3,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGScrollPhase {
  kCGScrollPhaseBegan = 1,
  kCGScrollPhaseChanged = 2,
  kCGScrollPhaseEnded = 4,
  kCGScrollPhaseCancelled = 8,
  kCGScrollPhaseMayBegin = 128,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGGesturePhase {
  kCGGesturePhaseNone = 0,
  kCGGesturePhaseBegan = 1,
  kCGGesturePhaseChanged = 2,
  kCGGesturePhaseEnded = 4,
  kCGGesturePhaseCancelled = 8,
  kCGGesturePhaseMayBegin = 128,
}
bitflags! { # [ repr ( C ) ] pub struct CGEventFlags : u64 { const kCGEventFlagMaskAlphaShift = 65536 ; const kCGEventFlagMaskShift = 131072 ; const kCGEventFlagMaskControl = 262144 ; const kCGEventFlagMaskAlternate = 524288 ; const kCGEventFlagMaskCommand = 1048576 ; const kCGEventFlagMaskHelp = 4194304 ; const kCGEventFlagMaskSecondaryFn = 8388608 ; const kCGEventFlagMaskNumericPad = 2097152 ; const kCGEventFlagMaskNonCoalesced = 256 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventType {
  kCGEventNull = 0,
  kCGEventLeftMouseDown = 1,
  kCGEventLeftMouseUp = 2,
  kCGEventRightMouseDown = 3,
  kCGEventRightMouseUp = 4,
  kCGEventMouseMoved = 5,
  kCGEventLeftMouseDragged = 6,
  kCGEventRightMouseDragged = 7,
  kCGEventKeyDown = 10,
  kCGEventKeyUp = 11,
  kCGEventFlagsChanged = 12,
  kCGEventScrollWheel = 22,
  kCGEventTabletPointer = 23,
  kCGEventTabletProximity = 24,
  kCGEventOtherMouseDown = 25,
  kCGEventOtherMouseUp = 26,
  kCGEventOtherMouseDragged = 27,
  kCGEventTapDisabledByTimeout = 4294967294,
  kCGEventTapDisabledByUserInput = 4294967295,
}
pub type CGEventTimestamp = u64;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventField {
  kCGMouseEventNumber = 0,
  kCGMouseEventClickState = 1,
  kCGMouseEventPressure = 2,
  kCGMouseEventButtonNumber = 3,
  kCGMouseEventDeltaX = 4,
  kCGMouseEventDeltaY = 5,
  kCGMouseEventInstantMouser = 6,
  kCGMouseEventSubtype = 7,
  kCGKeyboardEventAutorepeat = 8,
  kCGKeyboardEventKeycode = 9,
  kCGKeyboardEventKeyboardType = 10,
  kCGScrollWheelEventDeltaAxis1 = 11,
  kCGScrollWheelEventDeltaAxis2 = 12,
  kCGScrollWheelEventDeltaAxis3 = 13,
  kCGScrollWheelEventFixedPtDeltaAxis1 = 93,
  kCGScrollWheelEventFixedPtDeltaAxis2 = 94,
  kCGScrollWheelEventFixedPtDeltaAxis3 = 95,
  kCGScrollWheelEventPointDeltaAxis1 = 96,
  kCGScrollWheelEventPointDeltaAxis2 = 97,
  kCGScrollWheelEventPointDeltaAxis3 = 98,
  kCGScrollWheelEventScrollPhase = 99,
  kCGScrollWheelEventScrollCount = 100,
  kCGScrollWheelEventMomentumPhase = 123,
  kCGScrollWheelEventInstantMouser = 14,
  kCGTabletEventPointX = 15,
  kCGTabletEventPointY = 16,
  kCGTabletEventPointZ = 17,
  kCGTabletEventPointButtons = 18,
  kCGTabletEventPointPressure = 19,
  kCGTabletEventTiltX = 20,
  kCGTabletEventTiltY = 21,
  kCGTabletEventRotation = 22,
  kCGTabletEventTangentialPressure = 23,
  kCGTabletEventDeviceID = 24,
  kCGTabletEventVendor1 = 25,
  kCGTabletEventVendor2 = 26,
  kCGTabletEventVendor3 = 27,
  kCGTabletProximityEventVendorID = 28,
  kCGTabletProximityEventTabletID = 29,
  kCGTabletProximityEventPointerID = 30,
  kCGTabletProximityEventDeviceID = 31,
  kCGTabletProximityEventSystemTabletID = 32,
  kCGTabletProximityEventVendorPointerType = 33,
  kCGTabletProximityEventVendorPointerSerialNumber = 34,
  kCGTabletProximityEventVendorUniqueID = 35,
  kCGTabletProximityEventCapabilityMask = 36,
  kCGTabletProximityEventPointerType = 37,
  kCGTabletProximityEventEnterProximity = 38,
  kCGEventTargetProcessSerialNumber = 39,
  kCGEventTargetUnixProcessID = 40,
  kCGEventSourceUnixProcessID = 41,
  kCGEventSourceUserData = 42,
  kCGEventSourceUserID = 43,
  kCGEventSourceGroupID = 44,
  kCGEventSourceStateID = 45,
  kCGScrollWheelEventIsContinuous = 88,
  kCGMouseEventWindowUnderMousePointer = 91,
  kCGMouseEventWindowUnderMousePointerThatCanHandleThisEvent = 92,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventMouseSubtype {
  kCGEventMouseSubtypeDefault = 0,
  kCGEventMouseSubtypeTabletPoint = 1,
  kCGEventMouseSubtypeTabletProximity = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventTapLocation {
  kCGHIDEventTap = 0,
  kCGSessionEventTap = 1,
  kCGAnnotatedSessionEventTap = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventTapPlacement {
  kCGHeadInsertEventTap = 0,
  kCGTailAppendEventTap = 1,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGEventTapOptions {
  kCGEventTapOptionDefault = 0,
  kCGEventTapOptionListenOnly = 1,
}
pub type CGEventMask = u64;
#[repr(C)]
pub struct __CGEventTapProxy {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CGEventTapInformation {
  pub eventTapID: u32,
  pub tapPoint: CGEventTapLocation,
  pub options: CGEventTapOptions,
  pub eventsOfInterest: u64,
  pub tappingProcess: i32,
  pub processBeingTapped: i32,
  pub enabled: bool,
  pub minUsecLatency: f32,
  pub avgUsecLatency: f32,
  pub maxUsecLatency: f32,
}
pub type CGEventTapInformation = __CGEventTapInformation;
#[repr(C)]
pub struct __CGEventSource {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGEventSourceStateID {
  kCGEventSourceStatePrivate = -1,
  kCGEventSourceStateCombinedSessionState = 0,
  kCGEventSourceStateHIDSystemState = 1,
}
pub type CGEventSourceKeyboardType = u32;
#[repr(C)]
pub struct CGPSConverter {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGPSConverterCallbacks {
  pub version: u32,
  pub beginDocument: Option<extern "C" fn(*mut c_void) -> ()>,
  pub endDocument: Option<extern "C" fn(*mut c_void, bool) -> ()>,
  pub beginPage: Option<extern "C" fn(*mut c_void, usize, *mut __CFDictionary) -> ()>,
  pub endPage: Option<extern "C" fn(*mut c_void, usize, *mut __CFDictionary) -> ()>,
  pub noteProgress: Option<extern "C" fn(*mut c_void) -> ()>,
  pub noteMessage: Option<extern "C" fn(*mut c_void, *mut __CFString) -> ()>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
}
#[cfg(feature = "RK_CoreGraphics")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  pub fn CGPointMake(x: f64, y: f64) -> CGPoint;
  pub fn CGSizeMake(width: f64, height: f64) -> CGSize;
  pub fn CGVectorMake(dx: f64, dy: f64) -> CGVector;
  pub fn CGRectMake(x: f64, y: f64, width: f64, height: f64) -> CGRect;
  pub fn CGRectGetMinX(rect: CGRect) -> f64;
  pub fn CGRectGetMidX(rect: CGRect) -> f64;
  pub fn CGRectGetMaxX(rect: CGRect) -> f64;
  pub fn CGRectGetMinY(rect: CGRect) -> f64;
  pub fn CGRectGetMidY(rect: CGRect) -> f64;
  pub fn CGRectGetMaxY(rect: CGRect) -> f64;
  pub fn CGRectGetWidth(rect: CGRect) -> f64;
  pub fn CGRectGetHeight(rect: CGRect) -> f64;
  pub fn CGPointEqualToPoint(point1: CGPoint, point2: CGPoint) -> bool;
  pub fn CGSizeEqualToSize(size1: CGSize, size2: CGSize) -> bool;
  pub fn CGRectEqualToRect(rect1: CGRect, rect2: CGRect) -> bool;
  pub fn CGRectStandardize(rect: CGRect) -> CGRect;
  pub fn CGRectIsEmpty(rect: CGRect) -> bool;
  pub fn CGRectIsNull(rect: CGRect) -> bool;
  pub fn CGRectIsInfinite(rect: CGRect) -> bool;
  pub fn CGRectInset(rect: CGRect, dx: f64, dy: f64) -> CGRect;
  pub fn CGRectIntegral(rect: CGRect) -> CGRect;
  pub fn CGRectUnion(r1: CGRect, r2: CGRect) -> CGRect;
  pub fn CGRectIntersection(r1: CGRect, r2: CGRect) -> CGRect;
  pub fn CGRectOffset(rect: CGRect, dx: f64, dy: f64) -> CGRect;
  pub fn CGRectDivide(
    rect: CGRect,
    slice: *mut CGRect,
    remainder: *mut CGRect,
    amount: f64,
    edge: CGRectEdge,
  ) -> ();
  pub fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> bool;
  pub fn CGRectContainsRect(rect1: CGRect, rect2: CGRect) -> bool;
  pub fn CGRectIntersectsRect(rect1: CGRect, rect2: CGRect) -> bool;
  pub fn CGPointCreateDictionaryRepresentation(point: CGPoint) -> *mut __CFDictionary;
  pub fn CGPointMakeWithDictionaryRepresentation(
    dict: *mut __CFDictionary,
    point: *mut CGPoint,
  ) -> bool;
  pub fn CGSizeCreateDictionaryRepresentation(size: CGSize) -> *mut __CFDictionary;
  pub fn CGSizeMakeWithDictionaryRepresentation(
    dict: *mut __CFDictionary,
    size: *mut CGSize,
  ) -> bool;
  pub fn CGRectCreateDictionaryRepresentation(_: CGRect) -> *mut __CFDictionary;
  pub fn CGRectMakeWithDictionaryRepresentation(
    dict: *mut __CFDictionary,
    rect: *mut CGRect,
  ) -> bool;
  pub fn __CGPointEqualToPoint(point1: CGPoint, point2: CGPoint) -> bool;
  pub fn __CGSizeEqualToSize(size1: CGSize, size2: CGSize) -> bool;
  pub fn CGAffineTransformMake(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: f64,
    ty: f64,
  ) -> CGAffineTransform;
  pub fn CGAffineTransformMakeTranslation(tx: f64, ty: f64) -> CGAffineTransform;
  pub fn CGAffineTransformMakeScale(sx: f64, sy: f64) -> CGAffineTransform;
  pub fn CGAffineTransformMakeRotation(angle: f64) -> CGAffineTransform;
  pub fn CGAffineTransformIsIdentity(t: CGAffineTransform) -> bool;
  pub fn CGAffineTransformTranslate(t: CGAffineTransform, tx: f64, ty: f64) -> CGAffineTransform;
  pub fn CGAffineTransformScale(t: CGAffineTransform, sx: f64, sy: f64) -> CGAffineTransform;
  pub fn CGAffineTransformRotate(t: CGAffineTransform, angle: f64) -> CGAffineTransform;
  pub fn CGAffineTransformInvert(t: CGAffineTransform) -> CGAffineTransform;
  pub fn CGAffineTransformConcat(t1: CGAffineTransform, t2: CGAffineTransform)
    -> CGAffineTransform;
  pub fn CGAffineTransformEqualToTransform(t1: CGAffineTransform, t2: CGAffineTransform) -> bool;
  pub fn CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
  pub fn CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;
  pub fn CGRectApplyAffineTransform(rect: CGRect, t: CGAffineTransform) -> CGRect;
  pub fn __CGAffineTransformMake(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    tx: f64,
    ty: f64,
  ) -> CGAffineTransform;
  pub fn __CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
  pub fn __CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;
  pub fn CGDataProviderGetTypeID() -> usize;
  pub fn CGDataProviderCreateSequential(
    info: *mut c_void,
    callbacks: *mut CGDataProviderSequentialCallbacks,
  ) -> *mut CGDataProvider;
  pub fn CGDataProviderCreateDirect(
    info: *mut c_void,
    size: i64,
    callbacks: *mut CGDataProviderDirectCallbacks,
  ) -> *mut CGDataProvider;
  pub fn CGDataProviderCreateWithData(
    info: *mut c_void,
    data: *mut c_void,
    size: usize,
    releaseData: Option<extern "C" fn(*mut c_void, *mut c_void, usize) -> ()>,
  ) -> *mut CGDataProvider;
  pub fn CGDataProviderCreateWithCFData(data: *mut __CFData) -> *mut CGDataProvider;
  pub fn CGDataProviderCreateWithURL(url: *mut __CFURL) -> *mut CGDataProvider;
  pub fn CGDataProviderCreateWithFilename(filename: *mut i8) -> *mut CGDataProvider;
  pub fn CGDataProviderRetain(provider: *mut CGDataProvider) -> *mut CGDataProvider;
  pub fn CGDataProviderRelease(provider: *mut CGDataProvider) -> ();
  pub fn CGDataProviderCopyData(provider: *mut CGDataProvider) -> *mut __CFData;
  pub fn CGDataProviderGetInfo(provider: *mut CGDataProvider) -> *mut c_void;
  pub fn CGColorSpaceCreateDeviceGray() -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateDeviceRGB() -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateDeviceCMYK() -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateCalibratedGray(
    whitePoint: [f64; 3],
    blackPoint: [f64; 3],
    gamma: f64,
  ) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateCalibratedRGB(
    whitePoint: [f64; 3],
    blackPoint: [f64; 3],
    gamma: [f64; 3],
    matrix: [f64; 9],
  ) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateLab(
    whitePoint: [f64; 3],
    blackPoint: [f64; 3],
    range: [f64; 4],
  ) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateWithICCData(data: *mut c_void) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateICCBased(
    nComponents: usize,
    range: *mut f64,
    profile: *mut CGDataProvider,
    alternate: *mut CGColorSpace,
  ) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateIndexed(
    baseSpace: *mut CGColorSpace,
    lastIndex: usize,
    colorTable: *mut u8,
  ) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreatePattern(baseSpace: *mut CGColorSpace) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateWithPlatformColorSpace(ref_: *mut c_void) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateWithName(name: *mut __CFString) -> *mut CGColorSpace;
  pub fn CGColorSpaceRetain(space: *mut CGColorSpace) -> *mut CGColorSpace;
  pub fn CGColorSpaceRelease(space: *mut CGColorSpace) -> ();
  pub fn CGColorSpaceGetName(space: *mut CGColorSpace) -> *mut __CFString;
  pub fn CGColorSpaceCopyName(space: *mut CGColorSpace) -> *mut __CFString;
  pub fn CGColorSpaceGetTypeID() -> usize;
  pub fn CGColorSpaceGetNumberOfComponents(space: *mut CGColorSpace) -> usize;
  pub fn CGColorSpaceGetModel(space: *mut CGColorSpace) -> CGColorSpaceModel;
  pub fn CGColorSpaceGetBaseColorSpace(space: *mut CGColorSpace) -> *mut CGColorSpace;
  pub fn CGColorSpaceGetColorTableCount(space: *mut CGColorSpace) -> usize;
  pub fn CGColorSpaceGetColorTable(space: *mut CGColorSpace, table: *mut u8) -> ();
  pub fn CGColorSpaceCopyICCData(space: *mut CGColorSpace) -> *mut __CFData;
  pub fn CGColorSpaceIsWideGamutRGB(_: *mut CGColorSpace) -> bool;
  pub fn CGColorSpaceSupportsOutput(space: *mut CGColorSpace) -> bool;
  pub fn CGColorSpaceCopyPropertyList(space: *mut CGColorSpace) -> *mut c_void;
  pub fn CGColorSpaceCreateWithPropertyList(plist: *mut c_void) -> *mut CGColorSpace;
  pub fn CGColorSpaceCreateWithICCProfile(data: *mut __CFData) -> *mut CGColorSpace;
  pub fn CGColorSpaceCopyICCProfile(space: *mut CGColorSpace) -> *mut __CFData;
  pub fn CGPatternGetTypeID() -> usize;
  pub fn CGPatternCreate(
    info: *mut c_void,
    bounds: CGRect,
    matrix: CGAffineTransform,
    xStep: f64,
    yStep: f64,
    tiling: CGPatternTiling,
    isColored: bool,
    callbacks: *mut CGPatternCallbacks,
  ) -> *mut CGPattern;
  pub fn CGPatternRetain(pattern: *mut CGPattern) -> *mut CGPattern;
  pub fn CGPatternRelease(pattern: *mut CGPattern) -> ();
  pub fn CGColorCreate(space: *mut CGColorSpace, components: *mut f64) -> *mut CGColor;
  pub fn CGColorCreateGenericGray(gray: f64, alpha: f64) -> *mut CGColor;
  pub fn CGColorCreateGenericRGB(red: f64, green: f64, blue: f64, alpha: f64) -> *mut CGColor;
  pub fn CGColorCreateGenericCMYK(
    cyan: f64,
    magenta: f64,
    yellow: f64,
    black: f64,
    alpha: f64,
  ) -> *mut CGColor;
  pub fn CGColorGetConstantColor(colorName: *mut __CFString) -> *mut CGColor;
  pub fn CGColorCreateWithPattern(
    space: *mut CGColorSpace,
    pattern: *mut CGPattern,
    components: *mut f64,
  ) -> *mut CGColor;
  pub fn CGColorCreateCopy(color: *mut CGColor) -> *mut CGColor;
  pub fn CGColorCreateCopyWithAlpha(color: *mut CGColor, alpha: f64) -> *mut CGColor;
  pub fn CGColorCreateCopyByMatchingToColorSpace(
    _: *mut CGColorSpace,
    intent: CGColorRenderingIntent,
    color: *mut CGColor,
    options: *mut __CFDictionary,
  ) -> *mut CGColor;
  pub fn CGColorRetain(color: *mut CGColor) -> *mut CGColor;
  pub fn CGColorRelease(color: *mut CGColor) -> ();
  pub fn CGColorEqualToColor(color1: *mut CGColor, color2: *mut CGColor) -> bool;
  pub fn CGColorGetNumberOfComponents(color: *mut CGColor) -> usize;
  pub fn CGColorGetComponents(color: *mut CGColor) -> *mut f64;
  pub fn CGColorGetAlpha(color: *mut CGColor) -> f64;
  pub fn CGColorGetColorSpace(color: *mut CGColor) -> *mut CGColorSpace;
  pub fn CGColorGetPattern(color: *mut CGColor) -> *mut CGPattern;
  pub fn CGColorGetTypeID() -> usize;
  pub fn CGFontGetTypeID() -> usize;
  pub fn CGFontCreateWithPlatformFont(platformFontReference: *mut c_void) -> *mut CGFont;
  pub fn CGFontCreateWithDataProvider(provider: *mut CGDataProvider) -> *mut CGFont;
  pub fn CGFontCreateWithFontName(name: *mut __CFString) -> *mut CGFont;
  pub fn CGFontCreateCopyWithVariations(
    font: *mut CGFont,
    variations: *mut __CFDictionary,
  ) -> *mut CGFont;
  pub fn CGFontRetain(font: *mut CGFont) -> *mut CGFont;
  pub fn CGFontRelease(font: *mut CGFont) -> ();
  pub fn CGFontGetNumberOfGlyphs(font: *mut CGFont) -> usize;
  pub fn CGFontGetUnitsPerEm(font: *mut CGFont) -> i32;
  pub fn CGFontCopyPostScriptName(font: *mut CGFont) -> *mut __CFString;
  pub fn CGFontCopyFullName(font: *mut CGFont) -> *mut __CFString;
  pub fn CGFontGetAscent(font: *mut CGFont) -> i32;
  pub fn CGFontGetDescent(font: *mut CGFont) -> i32;
  pub fn CGFontGetLeading(font: *mut CGFont) -> i32;
  pub fn CGFontGetCapHeight(font: *mut CGFont) -> i32;
  pub fn CGFontGetXHeight(font: *mut CGFont) -> i32;
  pub fn CGFontGetFontBBox(font: *mut CGFont) -> CGRect;
  pub fn CGFontGetItalicAngle(font: *mut CGFont) -> f64;
  pub fn CGFontGetStemV(font: *mut CGFont) -> f64;
  pub fn CGFontCopyVariationAxes(font: *mut CGFont) -> *mut __CFArray;
  pub fn CGFontCopyVariations(font: *mut CGFont) -> *mut __CFDictionary;
  pub fn CGFontGetGlyphAdvances(
    font: *mut CGFont,
    glyphs: *mut u16,
    count: usize,
    advances: *mut i32,
  ) -> bool;
  pub fn CGFontGetGlyphBBoxes(
    font: *mut CGFont,
    glyphs: *mut u16,
    count: usize,
    bboxes: *mut CGRect,
  ) -> bool;
  pub fn CGFontGetGlyphWithGlyphName(font: *mut CGFont, name: *mut __CFString) -> u16;
  pub fn CGFontCopyGlyphNameForGlyph(font: *mut CGFont, glyph: u16) -> *mut __CFString;
  pub fn CGFontCanCreatePostScriptSubset(font: *mut CGFont, format: CGFontPostScriptFormat)
    -> bool;
  pub fn CGFontCreatePostScriptSubset(
    font: *mut CGFont,
    subsetName: *mut __CFString,
    format: CGFontPostScriptFormat,
    glyphs: *mut u16,
    count: usize,
    encoding: [u16; 256],
  ) -> *mut __CFData;
  pub fn CGFontCreatePostScriptEncoding(font: *mut CGFont, encoding: [u16; 256]) -> *mut __CFData;
  pub fn CGFontCopyTableTags(font: *mut CGFont) -> *mut __CFArray;
  pub fn CGFontCopyTableForTag(font: *mut CGFont, tag: u32) -> *mut __CFData;
  pub fn CGGradientGetTypeID() -> usize;
  pub fn CGGradientCreateWithColorComponents(
    space: *mut CGColorSpace,
    components: *mut f64,
    locations: *mut f64,
    count: usize,
  ) -> *mut CGGradient;
  pub fn CGGradientCreateWithColors(
    space: *mut CGColorSpace,
    colors: *mut __CFArray,
    locations: *mut f64,
  ) -> *mut CGGradient;
  pub fn CGGradientRetain(gradient: *mut CGGradient) -> *mut CGGradient;
  pub fn CGGradientRelease(gradient: *mut CGGradient) -> ();
  pub fn CGImageGetTypeID() -> usize;
  pub fn CGImageCreate(
    width: usize,
    height: usize,
    bitsPerComponent: usize,
    bitsPerPixel: usize,
    bytesPerRow: usize,
    space: *mut CGColorSpace,
    bitmapInfo: CGBitmapInfo,
    provider: *mut CGDataProvider,
    decode: *mut f64,
    shouldInterpolate: bool,
    intent: CGColorRenderingIntent,
  ) -> *mut CGImage;
  pub fn CGImageMaskCreate(
    width: usize,
    height: usize,
    bitsPerComponent: usize,
    bitsPerPixel: usize,
    bytesPerRow: usize,
    provider: *mut CGDataProvider,
    decode: *mut f64,
    shouldInterpolate: bool,
  ) -> *mut CGImage;
  pub fn CGImageCreateCopy(image: *mut CGImage) -> *mut CGImage;
  pub fn CGImageCreateWithJPEGDataProvider(
    source: *mut CGDataProvider,
    decode: *mut f64,
    shouldInterpolate: bool,
    intent: CGColorRenderingIntent,
  ) -> *mut CGImage;
  pub fn CGImageCreateWithPNGDataProvider(
    source: *mut CGDataProvider,
    decode: *mut f64,
    shouldInterpolate: bool,
    intent: CGColorRenderingIntent,
  ) -> *mut CGImage;
  pub fn CGImageCreateWithImageInRect(image: *mut CGImage, rect: CGRect) -> *mut CGImage;
  pub fn CGImageCreateWithMask(image: *mut CGImage, mask: *mut CGImage) -> *mut CGImage;
  pub fn CGImageCreateWithMaskingColors(image: *mut CGImage, components: *mut f64) -> *mut CGImage;
  pub fn CGImageCreateCopyWithColorSpace(
    image: *mut CGImage,
    space: *mut CGColorSpace,
  ) -> *mut CGImage;
  pub fn CGImageRetain(image: *mut CGImage) -> *mut CGImage;
  pub fn CGImageRelease(image: *mut CGImage) -> ();
  pub fn CGImageIsMask(image: *mut CGImage) -> bool;
  pub fn CGImageGetWidth(image: *mut CGImage) -> usize;
  pub fn CGImageGetHeight(image: *mut CGImage) -> usize;
  pub fn CGImageGetBitsPerComponent(image: *mut CGImage) -> usize;
  pub fn CGImageGetBitsPerPixel(image: *mut CGImage) -> usize;
  pub fn CGImageGetBytesPerRow(image: *mut CGImage) -> usize;
  pub fn CGImageGetColorSpace(image: *mut CGImage) -> *mut CGColorSpace;
  pub fn CGImageGetAlphaInfo(image: *mut CGImage) -> CGImageAlphaInfo;
  pub fn CGImageGetDataProvider(image: *mut CGImage) -> *mut CGDataProvider;
  pub fn CGImageGetDecode(image: *mut CGImage) -> *mut f64;
  pub fn CGImageGetShouldInterpolate(image: *mut CGImage) -> bool;
  pub fn CGImageGetRenderingIntent(image: *mut CGImage) -> CGColorRenderingIntent;
  pub fn CGImageGetBitmapInfo(image: *mut CGImage) -> CGBitmapInfo;
  pub fn CGImageGetByteOrderInfo(image: *mut CGImage) -> CGImageByteOrderInfo;
  pub fn CGImageGetPixelFormatInfo(image: *mut CGImage) -> CGImagePixelFormatInfo;
  pub fn CGImageGetUTType(image: *mut CGImage) -> *mut __CFString;
  pub fn CGPathGetTypeID() -> usize;
  pub fn CGPathCreateMutable() -> *mut CGPath;
  pub fn CGPathCreateCopy(path: *mut CGPath) -> *mut CGPath;
  pub fn CGPathCreateCopyByTransformingPath(
    path: *mut CGPath,
    transform: *mut CGAffineTransform,
  ) -> *mut CGPath;
  pub fn CGPathCreateMutableCopy(path: *mut CGPath) -> *mut CGPath;
  pub fn CGPathCreateMutableCopyByTransformingPath(
    path: *mut CGPath,
    transform: *mut CGAffineTransform,
  ) -> *mut CGPath;
  pub fn CGPathCreateWithRect(rect: CGRect, transform: *mut CGAffineTransform) -> *mut CGPath;
  pub fn CGPathCreateWithEllipseInRect(
    rect: CGRect,
    transform: *mut CGAffineTransform,
  ) -> *mut CGPath;
  pub fn CGPathCreateWithRoundedRect(
    rect: CGRect,
    cornerWidth: f64,
    cornerHeight: f64,
    transform: *mut CGAffineTransform,
  ) -> *mut CGPath;
  pub fn CGPathAddRoundedRect(
    path: *mut CGPath,
    transform: *mut CGAffineTransform,
    rect: CGRect,
    cornerWidth: f64,
    cornerHeight: f64,
  ) -> ();
  pub fn CGPathCreateCopyByDashingPath(
    path: *mut CGPath,
    transform: *mut CGAffineTransform,
    phase: f64,
    lengths: *mut f64,
    count: usize,
  ) -> *mut CGPath;
  pub fn CGPathCreateCopyByStrokingPath(
    path: *mut CGPath,
    transform: *mut CGAffineTransform,
    lineWidth: f64,
    lineCap: CGLineCap,
    lineJoin: CGLineJoin,
    miterLimit: f64,
  ) -> *mut CGPath;
  pub fn CGPathRetain(path: *mut CGPath) -> *mut CGPath;
  pub fn CGPathRelease(path: *mut CGPath) -> ();
  pub fn CGPathEqualToPath(path1: *mut CGPath, path2: *mut CGPath) -> bool;
  pub fn CGPathMoveToPoint(path: *mut CGPath, m: *mut CGAffineTransform, x: f64, y: f64) -> ();
  pub fn CGPathAddLineToPoint(path: *mut CGPath, m: *mut CGAffineTransform, x: f64, y: f64) -> ();
  pub fn CGPathAddQuadCurveToPoint(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    cpx: f64,
    cpy: f64,
    x: f64,
    y: f64,
  ) -> ();
  pub fn CGPathAddCurveToPoint(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    cp1x: f64,
    cp1y: f64,
    cp2x: f64,
    cp2y: f64,
    x: f64,
    y: f64,
  ) -> ();
  pub fn CGPathCloseSubpath(path: *mut CGPath) -> ();
  pub fn CGPathAddRect(path: *mut CGPath, m: *mut CGAffineTransform, rect: CGRect) -> ();
  pub fn CGPathAddRects(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    rects: *mut CGRect,
    count: usize,
  ) -> ();
  pub fn CGPathAddLines(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    points: *mut CGPoint,
    count: usize,
  ) -> ();
  pub fn CGPathAddEllipseInRect(path: *mut CGPath, m: *mut CGAffineTransform, rect: CGRect) -> ();
  pub fn CGPathAddRelativeArc(
    path: *mut CGPath,
    matrix: *mut CGAffineTransform,
    x: f64,
    y: f64,
    radius: f64,
    startAngle: f64,
    delta: f64,
  ) -> ();
  pub fn CGPathAddArc(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    x: f64,
    y: f64,
    radius: f64,
    startAngle: f64,
    endAngle: f64,
    clockwise: bool,
  ) -> ();
  pub fn CGPathAddArcToPoint(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    radius: f64,
  ) -> ();
  pub fn CGPathAddPath(path1: *mut CGPath, m: *mut CGAffineTransform, path2: *mut CGPath) -> ();
  pub fn CGPathIsEmpty(path: *mut CGPath) -> bool;
  pub fn CGPathIsRect(path: *mut CGPath, rect: *mut CGRect) -> bool;
  pub fn CGPathGetCurrentPoint(path: *mut CGPath) -> CGPoint;
  pub fn CGPathGetBoundingBox(path: *mut CGPath) -> CGRect;
  pub fn CGPathGetPathBoundingBox(path: *mut CGPath) -> CGRect;
  pub fn CGPathContainsPoint(
    path: *mut CGPath,
    m: *mut CGAffineTransform,
    point: CGPoint,
    eoFill: bool,
  ) -> bool;
  pub fn CGPathApply(
    path: *mut CGPath,
    info: *mut c_void,
    function: Option<extern "C" fn(*mut c_void, *mut CGPathElement) -> ()>,
  ) -> ();
  pub fn CGPathApplyWithBlock(path: *mut CGPath, block: ()) -> ();
  pub fn CGPDFObjectGetType(object: *mut CGPDFObject) -> CGPDFObjectType;
  pub fn CGPDFObjectGetValue(
    object: *mut CGPDFObject,
    type_: CGPDFObjectType,
    value: *mut c_void,
  ) -> bool;
  pub fn CGPDFStreamGetDictionary(stream: *mut CGPDFStream) -> *mut CGPDFDictionary;
  pub fn CGPDFStreamCopyData(
    stream: *mut CGPDFStream,
    format: *mut CGPDFDataFormat,
  ) -> *mut __CFData;
  pub fn CGPDFStringGetLength(string: *mut CGPDFString) -> usize;
  pub fn CGPDFStringGetBytePtr(string: *mut CGPDFString) -> *mut u8;
  pub fn CGPDFStringCopyTextString(string: *mut CGPDFString) -> *mut __CFString;
  pub fn CGPDFStringCopyDate(string: *mut CGPDFString) -> *mut __CFDate;
  pub fn CGPDFArrayGetCount(array: *mut CGPDFArray) -> usize;
  pub fn CGPDFArrayGetObject(
    array: *mut CGPDFArray,
    index: usize,
    value: *mut *mut CGPDFObject,
  ) -> bool;
  pub fn CGPDFArrayGetNull(array: *mut CGPDFArray, index: usize) -> bool;
  pub fn CGPDFArrayGetBoolean(array: *mut CGPDFArray, index: usize, value: *mut u8) -> bool;
  pub fn CGPDFArrayGetInteger(array: *mut CGPDFArray, index: usize, value: *mut isize) -> bool;
  pub fn CGPDFArrayGetNumber(array: *mut CGPDFArray, index: usize, value: *mut f64) -> bool;
  pub fn CGPDFArrayGetName(array: *mut CGPDFArray, index: usize, value: *mut *mut i8) -> bool;
  pub fn CGPDFArrayGetString(
    array: *mut CGPDFArray,
    index: usize,
    value: *mut *mut CGPDFString,
  ) -> bool;
  pub fn CGPDFArrayGetArray(
    array: *mut CGPDFArray,
    index: usize,
    value: *mut *mut CGPDFArray,
  ) -> bool;
  pub fn CGPDFArrayGetDictionary(
    array: *mut CGPDFArray,
    index: usize,
    value: *mut *mut CGPDFDictionary,
  ) -> bool;
  pub fn CGPDFArrayGetStream(
    array: *mut CGPDFArray,
    index: usize,
    value: *mut *mut CGPDFStream,
  ) -> bool;
  pub fn CGPDFArrayApplyBlock(array: *mut CGPDFArray, block: (), info: *mut c_void) -> ();
  pub fn CGPDFDictionaryGetCount(dict: *mut CGPDFDictionary) -> usize;
  pub fn CGPDFDictionaryGetObject(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut CGPDFObject,
  ) -> bool;
  pub fn CGPDFDictionaryGetBoolean(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut u8,
  ) -> bool;
  pub fn CGPDFDictionaryGetInteger(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut isize,
  ) -> bool;
  pub fn CGPDFDictionaryGetNumber(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut f64,
  ) -> bool;
  pub fn CGPDFDictionaryGetName(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut i8,
  ) -> bool;
  pub fn CGPDFDictionaryGetString(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut CGPDFString,
  ) -> bool;
  pub fn CGPDFDictionaryGetArray(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut CGPDFArray,
  ) -> bool;
  pub fn CGPDFDictionaryGetDictionary(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut CGPDFDictionary,
  ) -> bool;
  pub fn CGPDFDictionaryGetStream(
    dict: *mut CGPDFDictionary,
    key: *mut i8,
    value: *mut *mut CGPDFStream,
  ) -> bool;
  pub fn CGPDFDictionaryApplyFunction(
    dict: *mut CGPDFDictionary,
    function: Option<extern "C" fn(*mut i8, *mut CGPDFObject, *mut c_void) -> ()>,
    info: *mut c_void,
  ) -> ();
  pub fn CGPDFDictionaryApplyBlock(dict: *mut CGPDFDictionary, block: (), info: *mut c_void) -> ();
  pub fn CGPDFPageRetain(page: *mut CGPDFPage) -> *mut CGPDFPage;
  pub fn CGPDFPageRelease(page: *mut CGPDFPage) -> ();
  pub fn CGPDFPageGetDocument(page: *mut CGPDFPage) -> *mut CGPDFDocument;
  pub fn CGPDFPageGetPageNumber(page: *mut CGPDFPage) -> usize;
  pub fn CGPDFPageGetBoxRect(page: *mut CGPDFPage, box_: CGPDFBox) -> CGRect;
  pub fn CGPDFPageGetRotationAngle(page: *mut CGPDFPage) -> i32;
  pub fn CGPDFPageGetDrawingTransform(
    page: *mut CGPDFPage,
    box_: CGPDFBox,
    rect: CGRect,
    rotate: i32,
    preserveAspectRatio: bool,
  ) -> CGAffineTransform;
  pub fn CGPDFPageGetDictionary(page: *mut CGPDFPage) -> *mut CGPDFDictionary;
  pub fn CGPDFPageGetTypeID() -> usize;
  pub fn CGPDFDocumentCreateWithProvider(provider: *mut CGDataProvider) -> *mut CGPDFDocument;
  pub fn CGPDFDocumentCreateWithURL(url: *mut __CFURL) -> *mut CGPDFDocument;
  pub fn CGPDFDocumentRetain(document: *mut CGPDFDocument) -> *mut CGPDFDocument;
  pub fn CGPDFDocumentRelease(document: *mut CGPDFDocument) -> ();
  pub fn CGPDFDocumentGetVersion(
    document: *mut CGPDFDocument,
    majorVersion: *mut i32,
    minorVersion: *mut i32,
  ) -> ();
  pub fn CGPDFDocumentIsEncrypted(document: *mut CGPDFDocument) -> bool;
  pub fn CGPDFDocumentUnlockWithPassword(document: *mut CGPDFDocument, password: *mut i8) -> bool;
  pub fn CGPDFDocumentIsUnlocked(document: *mut CGPDFDocument) -> bool;
  pub fn CGPDFDocumentAllowsPrinting(document: *mut CGPDFDocument) -> bool;
  pub fn CGPDFDocumentAllowsCopying(document: *mut CGPDFDocument) -> bool;
  pub fn CGPDFDocumentGetNumberOfPages(document: *mut CGPDFDocument) -> usize;
  pub fn CGPDFDocumentGetPage(document: *mut CGPDFDocument, pageNumber: usize) -> *mut CGPDFPage;
  pub fn CGPDFDocumentGetCatalog(document: *mut CGPDFDocument) -> *mut CGPDFDictionary;
  pub fn CGPDFDocumentGetInfo(document: *mut CGPDFDocument) -> *mut CGPDFDictionary;
  pub fn CGPDFDocumentGetID(document: *mut CGPDFDocument) -> *mut CGPDFArray;
  pub fn CGPDFDocumentGetTypeID() -> usize;
  pub fn CGPDFDocumentGetOutline(document: *mut CGPDFDocument) -> *mut __CFDictionary;
  pub fn CGPDFDocumentGetAccessPermissions(document: *mut CGPDFDocument) -> CGPDFAccessPermissions;
  pub fn CGPDFDocumentGetMediaBox(document: *mut CGPDFDocument, page: i32) -> CGRect;
  pub fn CGPDFDocumentGetCropBox(document: *mut CGPDFDocument, page: i32) -> CGRect;
  pub fn CGPDFDocumentGetBleedBox(document: *mut CGPDFDocument, page: i32) -> CGRect;
  pub fn CGPDFDocumentGetTrimBox(document: *mut CGPDFDocument, page: i32) -> CGRect;
  pub fn CGPDFDocumentGetArtBox(document: *mut CGPDFDocument, page: i32) -> CGRect;
  pub fn CGPDFDocumentGetRotationAngle(document: *mut CGPDFDocument, page: i32) -> i32;
  pub fn CGFunctionGetTypeID() -> usize;
  pub fn CGFunctionCreate(
    info: *mut c_void,
    domainDimension: usize,
    domain: *mut f64,
    rangeDimension: usize,
    range: *mut f64,
    callbacks: *mut CGFunctionCallbacks,
  ) -> *mut CGFunction;
  pub fn CGFunctionRetain(function: *mut CGFunction) -> *mut CGFunction;
  pub fn CGFunctionRelease(function: *mut CGFunction) -> ();
  pub fn CGShadingGetTypeID() -> usize;
  pub fn CGShadingCreateAxial(
    space: *mut CGColorSpace,
    start: CGPoint,
    end: CGPoint,
    function: *mut CGFunction,
    extendStart: bool,
    extendEnd: bool,
  ) -> *mut CGShading;
  pub fn CGShadingCreateRadial(
    space: *mut CGColorSpace,
    start: CGPoint,
    startRadius: f64,
    end: CGPoint,
    endRadius: f64,
    function: *mut CGFunction,
    extendStart: bool,
    extendEnd: bool,
  ) -> *mut CGShading;
  pub fn CGShadingRetain(shading: *mut CGShading) -> *mut CGShading;
  pub fn CGShadingRelease(shading: *mut CGShading) -> ();
  pub fn CGContextGetTypeID() -> usize;
  pub fn CGContextSaveGState(c: *mut CGContext) -> ();
  pub fn CGContextRestoreGState(c: *mut CGContext) -> ();
  pub fn CGContextScaleCTM(c: *mut CGContext, sx: f64, sy: f64) -> ();
  pub fn CGContextTranslateCTM(c: *mut CGContext, tx: f64, ty: f64) -> ();
  pub fn CGContextRotateCTM(c: *mut CGContext, angle: f64) -> ();
  pub fn CGContextConcatCTM(c: *mut CGContext, transform: CGAffineTransform) -> ();
  pub fn CGContextGetCTM(c: *mut CGContext) -> CGAffineTransform;
  pub fn CGContextSetLineWidth(c: *mut CGContext, width: f64) -> ();
  pub fn CGContextSetLineCap(c: *mut CGContext, cap: CGLineCap) -> ();
  pub fn CGContextSetLineJoin(c: *mut CGContext, join: CGLineJoin) -> ();
  pub fn CGContextSetMiterLimit(c: *mut CGContext, limit: f64) -> ();
  pub fn CGContextSetLineDash(c: *mut CGContext, phase: f64, lengths: *mut f64, count: usize)
    -> ();
  pub fn CGContextSetFlatness(c: *mut CGContext, flatness: f64) -> ();
  pub fn CGContextSetAlpha(c: *mut CGContext, alpha: f64) -> ();
  pub fn CGContextSetBlendMode(c: *mut CGContext, mode: CGBlendMode) -> ();
  pub fn CGContextBeginPath(c: *mut CGContext) -> ();
  pub fn CGContextMoveToPoint(c: *mut CGContext, x: f64, y: f64) -> ();
  pub fn CGContextAddLineToPoint(c: *mut CGContext, x: f64, y: f64) -> ();
  pub fn CGContextAddCurveToPoint(
    c: *mut CGContext,
    cp1x: f64,
    cp1y: f64,
    cp2x: f64,
    cp2y: f64,
    x: f64,
    y: f64,
  ) -> ();
  pub fn CGContextAddQuadCurveToPoint(c: *mut CGContext, cpx: f64, cpy: f64, x: f64, y: f64) -> ();
  pub fn CGContextClosePath(c: *mut CGContext) -> ();
  pub fn CGContextAddRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextAddRects(c: *mut CGContext, rects: *mut CGRect, count: usize) -> ();
  pub fn CGContextAddLines(c: *mut CGContext, points: *mut CGPoint, count: usize) -> ();
  pub fn CGContextAddEllipseInRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextAddArc(
    c: *mut CGContext,
    x: f64,
    y: f64,
    radius: f64,
    startAngle: f64,
    endAngle: f64,
    clockwise: i32,
  ) -> ();
  pub fn CGContextAddArcToPoint(
    c: *mut CGContext,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    radius: f64,
  ) -> ();
  pub fn CGContextAddPath(c: *mut CGContext, path: *mut CGPath) -> ();
  pub fn CGContextReplacePathWithStrokedPath(c: *mut CGContext) -> ();
  pub fn CGContextIsPathEmpty(c: *mut CGContext) -> bool;
  pub fn CGContextGetPathCurrentPoint(c: *mut CGContext) -> CGPoint;
  pub fn CGContextGetPathBoundingBox(c: *mut CGContext) -> CGRect;
  pub fn CGContextCopyPath(c: *mut CGContext) -> *mut CGPath;
  pub fn CGContextPathContainsPoint(
    c: *mut CGContext,
    point: CGPoint,
    mode: CGPathDrawingMode,
  ) -> bool;
  pub fn CGContextDrawPath(c: *mut CGContext, mode: CGPathDrawingMode) -> ();
  pub fn CGContextFillPath(c: *mut CGContext) -> ();
  pub fn CGContextEOFillPath(c: *mut CGContext) -> ();
  pub fn CGContextStrokePath(c: *mut CGContext) -> ();
  pub fn CGContextFillRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextFillRects(c: *mut CGContext, rects: *mut CGRect, count: usize) -> ();
  pub fn CGContextStrokeRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextStrokeRectWithWidth(c: *mut CGContext, rect: CGRect, width: f64) -> ();
  pub fn CGContextClearRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextFillEllipseInRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextStrokeEllipseInRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextStrokeLineSegments(c: *mut CGContext, points: *mut CGPoint, count: usize) -> ();
  pub fn CGContextClip(c: *mut CGContext) -> ();
  pub fn CGContextEOClip(c: *mut CGContext) -> ();
  pub fn CGContextResetClip(c: *mut CGContext) -> ();
  pub fn CGContextClipToMask(c: *mut CGContext, rect: CGRect, mask: *mut CGImage) -> ();
  pub fn CGContextGetClipBoundingBox(c: *mut CGContext) -> CGRect;
  pub fn CGContextClipToRect(c: *mut CGContext, rect: CGRect) -> ();
  pub fn CGContextClipToRects(c: *mut CGContext, rects: *mut CGRect, count: usize) -> ();
  pub fn CGContextSetFillColorWithColor(c: *mut CGContext, color: *mut CGColor) -> ();
  pub fn CGContextSetStrokeColorWithColor(c: *mut CGContext, color: *mut CGColor) -> ();
  pub fn CGContextSetFillColorSpace(c: *mut CGContext, space: *mut CGColorSpace) -> ();
  pub fn CGContextSetStrokeColorSpace(c: *mut CGContext, space: *mut CGColorSpace) -> ();
  pub fn CGContextSetFillColor(c: *mut CGContext, components: *mut f64) -> ();
  pub fn CGContextSetStrokeColor(c: *mut CGContext, components: *mut f64) -> ();
  pub fn CGContextSetFillPattern(
    c: *mut CGContext,
    pattern: *mut CGPattern,
    components: *mut f64,
  ) -> ();
  pub fn CGContextSetStrokePattern(
    c: *mut CGContext,
    pattern: *mut CGPattern,
    components: *mut f64,
  ) -> ();
  pub fn CGContextSetPatternPhase(c: *mut CGContext, phase: CGSize) -> ();
  pub fn CGContextSetGrayFillColor(c: *mut CGContext, gray: f64, alpha: f64) -> ();
  pub fn CGContextSetGrayStrokeColor(c: *mut CGContext, gray: f64, alpha: f64) -> ();
  pub fn CGContextSetRGBFillColor(
    c: *mut CGContext,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
  ) -> ();
  pub fn CGContextSetRGBStrokeColor(
    c: *mut CGContext,
    red: f64,
    green: f64,
    blue: f64,
    alpha: f64,
  ) -> ();
  pub fn CGContextSetCMYKFillColor(
    c: *mut CGContext,
    cyan: f64,
    magenta: f64,
    yellow: f64,
    black: f64,
    alpha: f64,
  ) -> ();
  pub fn CGContextSetCMYKStrokeColor(
    c: *mut CGContext,
    cyan: f64,
    magenta: f64,
    yellow: f64,
    black: f64,
    alpha: f64,
  ) -> ();
  pub fn CGContextSetRenderingIntent(c: *mut CGContext, intent: CGColorRenderingIntent) -> ();
  pub fn CGContextDrawImage(c: *mut CGContext, rect: CGRect, image: *mut CGImage) -> ();
  pub fn CGContextDrawTiledImage(c: *mut CGContext, rect: CGRect, image: *mut CGImage) -> ();
  pub fn CGContextGetInterpolationQuality(c: *mut CGContext) -> CGInterpolationQuality;
  pub fn CGContextSetInterpolationQuality(c: *mut CGContext, quality: CGInterpolationQuality)
    -> ();
  pub fn CGContextSetShadowWithColor(
    c: *mut CGContext,
    offset: CGSize,
    blur: f64,
    color: *mut CGColor,
  ) -> ();
  pub fn CGContextSetShadow(c: *mut CGContext, offset: CGSize, blur: f64) -> ();
  pub fn CGContextDrawLinearGradient(
    c: *mut CGContext,
    gradient: *mut CGGradient,
    startPoint: CGPoint,
    endPoint: CGPoint,
    options: CGGradientDrawingOptions,
  ) -> ();
  pub fn CGContextDrawRadialGradient(
    c: *mut CGContext,
    gradient: *mut CGGradient,
    startCenter: CGPoint,
    startRadius: f64,
    endCenter: CGPoint,
    endRadius: f64,
    options: CGGradientDrawingOptions,
  ) -> ();
  pub fn CGContextDrawShading(c: *mut CGContext, shading: *mut CGShading) -> ();
  pub fn CGContextSetCharacterSpacing(c: *mut CGContext, spacing: f64) -> ();
  pub fn CGContextSetTextPosition(c: *mut CGContext, x: f64, y: f64) -> ();
  pub fn CGContextGetTextPosition(c: *mut CGContext) -> CGPoint;
  pub fn CGContextSetTextMatrix(c: *mut CGContext, t: CGAffineTransform) -> ();
  pub fn CGContextGetTextMatrix(c: *mut CGContext) -> CGAffineTransform;
  pub fn CGContextSetTextDrawingMode(c: *mut CGContext, mode: CGTextDrawingMode) -> ();
  pub fn CGContextSetFont(c: *mut CGContext, font: *mut CGFont) -> ();
  pub fn CGContextSetFontSize(c: *mut CGContext, size: f64) -> ();
  pub fn CGContextShowGlyphsAtPositions(
    c: *mut CGContext,
    glyphs: *mut u16,
    Lpositions: *mut CGPoint,
    count: usize,
  ) -> ();
  pub fn CGContextDrawPDFPage(c: *mut CGContext, page: *mut CGPDFPage) -> ();
  pub fn CGContextBeginPage(c: *mut CGContext, mediaBox: *mut CGRect) -> ();
  pub fn CGContextEndPage(c: *mut CGContext) -> ();
  pub fn CGContextRetain(c: *mut CGContext) -> *mut CGContext;
  pub fn CGContextRelease(c: *mut CGContext) -> ();
  pub fn CGContextFlush(c: *mut CGContext) -> ();
  pub fn CGContextSynchronize(c: *mut CGContext) -> ();
  pub fn CGContextSetShouldAntialias(c: *mut CGContext, shouldAntialias: bool) -> ();
  pub fn CGContextSetAllowsAntialiasing(c: *mut CGContext, allowsAntialiasing: bool) -> ();
  pub fn CGContextSetShouldSmoothFonts(c: *mut CGContext, shouldSmoothFonts: bool) -> ();
  pub fn CGContextSetAllowsFontSmoothing(c: *mut CGContext, allowsFontSmoothing: bool) -> ();
  pub fn CGContextSetShouldSubpixelPositionFonts(
    c: *mut CGContext,
    shouldSubpixelPositionFonts: bool,
  ) -> ();
  pub fn CGContextSetAllowsFontSubpixelPositioning(
    c: *mut CGContext,
    allowsFontSubpixelPositioning: bool,
  ) -> ();
  pub fn CGContextSetShouldSubpixelQuantizeFonts(
    c: *mut CGContext,
    shouldSubpixelQuantizeFonts: bool,
  ) -> ();
  pub fn CGContextSetAllowsFontSubpixelQuantization(
    c: *mut CGContext,
    allowsFontSubpixelQuantization: bool,
  ) -> ();
  pub fn CGContextBeginTransparencyLayer(
    c: *mut CGContext,
    auxiliaryInfo: *mut __CFDictionary,
  ) -> ();
  pub fn CGContextBeginTransparencyLayerWithRect(
    c: *mut CGContext,
    rect: CGRect,
    auxInfo: *mut __CFDictionary,
  ) -> ();
  pub fn CGContextEndTransparencyLayer(c: *mut CGContext) -> ();
  pub fn CGContextGetUserSpaceToDeviceSpaceTransform(c: *mut CGContext) -> CGAffineTransform;
  pub fn CGContextConvertPointToDeviceSpace(c: *mut CGContext, point: CGPoint) -> CGPoint;
  pub fn CGContextConvertPointToUserSpace(c: *mut CGContext, point: CGPoint) -> CGPoint;
  pub fn CGContextConvertSizeToDeviceSpace(c: *mut CGContext, size: CGSize) -> CGSize;
  pub fn CGContextConvertSizeToUserSpace(c: *mut CGContext, size: CGSize) -> CGSize;
  pub fn CGContextConvertRectToDeviceSpace(c: *mut CGContext, rect: CGRect) -> CGRect;
  pub fn CGContextConvertRectToUserSpace(c: *mut CGContext, rect: CGRect) -> CGRect;
  pub fn CGContextSelectFont(
    c: *mut CGContext,
    name: *mut i8,
    size: f64,
    textEncoding: CGTextEncoding,
  ) -> ();
  pub fn CGContextShowText(c: *mut CGContext, string: *mut i8, length: usize) -> ();
  pub fn CGContextShowTextAtPoint(
    c: *mut CGContext,
    x: f64,
    y: f64,
    string: *mut i8,
    length: usize,
  ) -> ();
  pub fn CGContextShowGlyphs(c: *mut CGContext, g: *mut u16, count: usize) -> ();
  pub fn CGContextShowGlyphsAtPoint(
    c: *mut CGContext,
    x: f64,
    y: f64,
    glyphs: *mut u16,
    count: usize,
  ) -> ();
  pub fn CGContextShowGlyphsWithAdvances(
    c: *mut CGContext,
    glyphs: *mut u16,
    advances: *mut CGSize,
    count: usize,
  ) -> ();
  pub fn CGContextDrawPDFDocument(
    c: *mut CGContext,
    rect: CGRect,
    document: *mut CGPDFDocument,
    page: i32,
  ) -> ();
  pub fn CGBitmapContextCreateWithData(
    data: *mut c_void,
    width: usize,
    height: usize,
    bitsPerComponent: usize,
    bytesPerRow: usize,
    space: *mut CGColorSpace,
    bitmapInfo: u32,
    releaseCallback: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    releaseInfo: *mut c_void,
  ) -> *mut CGContext;
  pub fn CGBitmapContextCreate(
    data: *mut c_void,
    width: usize,
    height: usize,
    bitsPerComponent: usize,
    bytesPerRow: usize,
    space: *mut CGColorSpace,
    bitmapInfo: u32,
  ) -> *mut CGContext;
  pub fn CGBitmapContextGetData(context: *mut CGContext) -> *mut c_void;
  pub fn CGBitmapContextGetWidth(context: *mut CGContext) -> usize;
  pub fn CGBitmapContextGetHeight(context: *mut CGContext) -> usize;
  pub fn CGBitmapContextGetBitsPerComponent(context: *mut CGContext) -> usize;
  pub fn CGBitmapContextGetBitsPerPixel(context: *mut CGContext) -> usize;
  pub fn CGBitmapContextGetBytesPerRow(context: *mut CGContext) -> usize;
  pub fn CGBitmapContextGetColorSpace(context: *mut CGContext) -> *mut CGColorSpace;
  pub fn CGBitmapContextGetAlphaInfo(context: *mut CGContext) -> CGImageAlphaInfo;
  pub fn CGBitmapContextGetBitmapInfo(context: *mut CGContext) -> CGBitmapInfo;
  pub fn CGBitmapContextCreateImage(context: *mut CGContext) -> *mut CGImage;
  pub fn CGColorConversionInfoGetTypeID() -> usize;
  pub fn CGColorConversionInfoCreate(
    src: *mut CGColorSpace,
    dst: *mut CGColorSpace,
  ) -> *mut CGColorConversionInfo;
  pub fn CGColorConversionInfoCreateWithOptions(
    src: *mut CGColorSpace,
    dst: *mut CGColorSpace,
    options: *mut __CFDictionary,
  ) -> *mut CGColorConversionInfo;
  pub fn CGColorConversionInfoCreateFromList(
    options: *mut __CFDictionary,
    _: *mut CGColorSpace,
    _: CGColorConversionInfoTransformType,
    _: CGColorRenderingIntent,
    ...
  ) -> *mut CGColorConversionInfo;
  pub fn CGDataConsumerGetTypeID() -> usize;
  pub fn CGDataConsumerCreate(
    info: *mut c_void,
    cbks: *mut CGDataConsumerCallbacks,
  ) -> *mut CGDataConsumer;
  pub fn CGDataConsumerCreateWithURL(url: *mut __CFURL) -> *mut CGDataConsumer;
  pub fn CGDataConsumerCreateWithCFData(data: *mut __CFData) -> *mut CGDataConsumer;
  pub fn CGDataConsumerRetain(consumer: *mut CGDataConsumer) -> *mut CGDataConsumer;
  pub fn CGDataConsumerRelease(consumer: *mut CGDataConsumer) -> ();
  pub fn CGLayerCreateWithContext(
    context: *mut CGContext,
    size: CGSize,
    auxiliaryInfo: *mut __CFDictionary,
  ) -> *mut CGLayer;
  pub fn CGLayerRetain(layer: *mut CGLayer) -> *mut CGLayer;
  pub fn CGLayerRelease(layer: *mut CGLayer) -> ();
  pub fn CGLayerGetSize(layer: *mut CGLayer) -> CGSize;
  pub fn CGLayerGetContext(layer: *mut CGLayer) -> *mut CGContext;
  pub fn CGContextDrawLayerInRect(context: *mut CGContext, rect: CGRect, layer: *mut CGLayer)
    -> ();
  pub fn CGContextDrawLayerAtPoint(
    context: *mut CGContext,
    point: CGPoint,
    layer: *mut CGLayer,
  ) -> ();
  pub fn CGLayerGetTypeID() -> usize;
  pub fn CGPDFContentStreamCreateWithPage(page: *mut CGPDFPage) -> *mut CGPDFContentStream;
  pub fn CGPDFContentStreamCreateWithStream(
    stream: *mut CGPDFStream,
    streamResources: *mut CGPDFDictionary,
    parent: *mut CGPDFContentStream,
  ) -> *mut CGPDFContentStream;
  pub fn CGPDFContentStreamRetain(cs: *mut CGPDFContentStream) -> *mut CGPDFContentStream;
  pub fn CGPDFContentStreamRelease(cs: *mut CGPDFContentStream) -> ();
  pub fn CGPDFContentStreamGetStreams(cs: *mut CGPDFContentStream) -> *mut __CFArray;
  pub fn CGPDFContentStreamGetResource(
    cs: *mut CGPDFContentStream,
    category: *mut i8,
    name: *mut i8,
  ) -> *mut CGPDFObject;
  pub fn CGPDFContextCreate(
    consumer: *mut CGDataConsumer,
    mediaBox: *mut CGRect,
    auxiliaryInfo: *mut __CFDictionary,
  ) -> *mut CGContext;
  pub fn CGPDFContextCreateWithURL(
    url: *mut __CFURL,
    mediaBox: *mut CGRect,
    auxiliaryInfo: *mut __CFDictionary,
  ) -> *mut CGContext;
  pub fn CGPDFContextClose(context: *mut CGContext) -> ();
  pub fn CGPDFContextBeginPage(context: *mut CGContext, pageInfo: *mut __CFDictionary) -> ();
  pub fn CGPDFContextEndPage(context: *mut CGContext) -> ();
  pub fn CGPDFContextAddDocumentMetadata(context: *mut CGContext, metadata: *mut __CFData) -> ();
  pub fn CGPDFContextSetURLForRect(context: *mut CGContext, url: *mut __CFURL, rect: CGRect) -> ();
  pub fn CGPDFContextAddDestinationAtPoint(
    context: *mut CGContext,
    name: *mut __CFString,
    point: CGPoint,
  ) -> ();
  pub fn CGPDFContextSetDestinationForRect(
    context: *mut CGContext,
    name: *mut __CFString,
    rect: CGRect,
  ) -> ();
  pub fn CGPDFContextSetOutline(context: *mut CGContext, outline: *mut __CFDictionary) -> ();
  pub fn CGPDFScannerCreate(
    cs: *mut CGPDFContentStream,
    table: *mut CGPDFOperatorTable,
    info: *mut c_void,
  ) -> *mut CGPDFScanner;
  pub fn CGPDFScannerRetain(scanner: *mut CGPDFScanner) -> *mut CGPDFScanner;
  pub fn CGPDFScannerRelease(scanner: *mut CGPDFScanner) -> ();
  pub fn CGPDFScannerScan(scanner: *mut CGPDFScanner) -> bool;
  pub fn CGPDFScannerGetContentStream(scanner: *mut CGPDFScanner) -> *mut CGPDFContentStream;
  pub fn CGPDFScannerPopObject(scanner: *mut CGPDFScanner, value: *mut *mut CGPDFObject) -> bool;
  pub fn CGPDFScannerPopBoolean(scanner: *mut CGPDFScanner, value: *mut u8) -> bool;
  pub fn CGPDFScannerPopInteger(scanner: *mut CGPDFScanner, value: *mut isize) -> bool;
  pub fn CGPDFScannerPopNumber(scanner: *mut CGPDFScanner, value: *mut f64) -> bool;
  pub fn CGPDFScannerPopName(scanner: *mut CGPDFScanner, value: *mut *mut i8) -> bool;
  pub fn CGPDFScannerPopString(scanner: *mut CGPDFScanner, value: *mut *mut CGPDFString) -> bool;
  pub fn CGPDFScannerPopArray(scanner: *mut CGPDFScanner, value: *mut *mut CGPDFArray) -> bool;
  pub fn CGPDFScannerPopDictionary(
    scanner: *mut CGPDFScanner,
    value: *mut *mut CGPDFDictionary,
  ) -> bool;
  pub fn CGPDFScannerPopStream(scanner: *mut CGPDFScanner, value: *mut *mut CGPDFStream) -> bool;
  pub fn CGPDFOperatorTableCreate() -> *mut CGPDFOperatorTable;
  pub fn CGPDFOperatorTableRetain(table: *mut CGPDFOperatorTable) -> *mut CGPDFOperatorTable;
  pub fn CGPDFOperatorTableRelease(table: *mut CGPDFOperatorTable) -> ();
  pub fn CGPDFOperatorTableSetCallback(
    table: *mut CGPDFOperatorTable,
    name: *mut i8,
    callback: Option<extern "C" fn(*mut CGPDFScanner, *mut c_void) -> ()>,
  ) -> ();
  pub fn CGWindowListCopyWindowInfo(
    option: CGWindowListOption,
    relativeToWindow: u32,
  ) -> *mut __CFArray;
  pub fn CGWindowListCreateDescriptionFromArray(windowArray: *mut __CFArray) -> *mut __CFArray;
  pub fn CGWindowListCreateImage(
    screenBounds: CGRect,
    listOption: CGWindowListOption,
    windowID: u32,
    imageOption: CGWindowImageOption,
  ) -> *mut CGImage;
  pub fn CGWindowListCreateImageFromArray(
    screenBounds: CGRect,
    windowArray: *mut __CFArray,
    imageOption: CGWindowImageOption,
  ) -> *mut CGImage;
  pub fn CGWindowLevelForKey(key: CGWindowLevelKey) -> i32;
  pub fn CGMainDisplayID() -> u32;
  pub fn CGGetDisplaysWithPoint(
    point: CGPoint,
    maxDisplays: u32,
    displays: *mut u32,
    matchingDisplayCount: *mut u32,
  ) -> CGError;
  pub fn CGGetDisplaysWithRect(
    rect: CGRect,
    maxDisplays: u32,
    displays: *mut u32,
    matchingDisplayCount: *mut u32,
  ) -> CGError;
  pub fn CGGetDisplaysWithOpenGLDisplayMask(
    mask: u32,
    maxDisplays: u32,
    displays: *mut u32,
    matchingDisplayCount: *mut u32,
  ) -> CGError;
  pub fn CGGetActiveDisplayList(
    maxDisplays: u32,
    activeDisplays: *mut u32,
    displayCount: *mut u32,
  ) -> CGError;
  pub fn CGGetOnlineDisplayList(
    maxDisplays: u32,
    onlineDisplays: *mut u32,
    displayCount: *mut u32,
  ) -> CGError;
  pub fn CGDisplayIDToOpenGLDisplayMask(display: u32) -> u32;
  pub fn CGOpenGLDisplayMaskToDisplayID(mask: u32) -> u32;
  pub fn CGDisplayBounds(display: u32) -> CGRect;
  pub fn CGDisplayPixelsWide(display: u32) -> usize;
  pub fn CGDisplayPixelsHigh(display: u32) -> usize;
  pub fn CGDisplayCopyAllDisplayModes(display: u32, options: *mut __CFDictionary)
    -> *mut __CFArray;
  pub fn CGDisplayCopyDisplayMode(display: u32) -> *mut CGDisplayMode;
  pub fn CGDisplaySetDisplayMode(
    display: u32,
    mode: *mut CGDisplayMode,
    options: *mut __CFDictionary,
  ) -> CGError;
  pub fn CGDisplayModeGetWidth(mode: *mut CGDisplayMode) -> usize;
  pub fn CGDisplayModeGetHeight(mode: *mut CGDisplayMode) -> usize;
  pub fn CGDisplayModeCopyPixelEncoding(mode: *mut CGDisplayMode) -> *mut __CFString;
  pub fn CGDisplayModeGetRefreshRate(mode: *mut CGDisplayMode) -> f64;
  pub fn CGDisplayModeGetIOFlags(mode: *mut CGDisplayMode) -> u32;
  pub fn CGDisplayModeGetIODisplayModeID(mode: *mut CGDisplayMode) -> i32;
  pub fn CGDisplayModeIsUsableForDesktopGUI(mode: *mut CGDisplayMode) -> bool;
  pub fn CGDisplayModeGetTypeID() -> usize;
  pub fn CGDisplayModeRetain(mode: *mut CGDisplayMode) -> *mut CGDisplayMode;
  pub fn CGDisplayModeRelease(mode: *mut CGDisplayMode) -> ();
  pub fn CGDisplayModeGetPixelWidth(mode: *mut CGDisplayMode) -> usize;
  pub fn CGDisplayModeGetPixelHeight(mode: *mut CGDisplayMode) -> usize;
  pub fn CGSetDisplayTransferByFormula(
    display: u32,
    redMin: f32,
    redMax: f32,
    redGamma: f32,
    greenMin: f32,
    greenMax: f32,
    greenGamma: f32,
    blueMin: f32,
    blueMax: f32,
    blueGamma: f32,
  ) -> CGError;
  pub fn CGGetDisplayTransferByFormula(
    display: u32,
    redMin: *mut f32,
    redMax: *mut f32,
    redGamma: *mut f32,
    greenMin: *mut f32,
    greenMax: *mut f32,
    greenGamma: *mut f32,
    blueMin: *mut f32,
    blueMax: *mut f32,
    blueGamma: *mut f32,
  ) -> CGError;
  pub fn CGDisplayGammaTableCapacity(display: u32) -> u32;
  pub fn CGSetDisplayTransferByTable(
    display: u32,
    tableSize: u32,
    redTable: *mut f32,
    greenTable: *mut f32,
    blueTable: *mut f32,
  ) -> CGError;
  pub fn CGGetDisplayTransferByTable(
    display: u32,
    capacity: u32,
    redTable: *mut f32,
    greenTable: *mut f32,
    blueTable: *mut f32,
    sampleCount: *mut u32,
  ) -> CGError;
  pub fn CGSetDisplayTransferByByteTable(
    display: u32,
    tableSize: u32,
    redTable: *mut u8,
    greenTable: *mut u8,
    blueTable: *mut u8,
  ) -> CGError;
  pub fn CGDisplayRestoreColorSyncSettings() -> ();
  pub fn CGDisplayIsCaptured(display: u32) -> u32;
  pub fn CGDisplayCapture(display: u32) -> CGError;
  pub fn CGDisplayCaptureWithOptions(display: u32, options: CGCaptureOptions) -> CGError;
  pub fn CGDisplayRelease(display: u32) -> CGError;
  pub fn CGCaptureAllDisplays() -> CGError;
  pub fn CGCaptureAllDisplaysWithOptions(options: CGCaptureOptions) -> CGError;
  pub fn CGReleaseAllDisplays() -> CGError;
  pub fn CGShieldingWindowID(display: u32) -> u32;
  pub fn CGShieldingWindowLevel() -> i32;
  pub fn CGDisplayCreateImage(displayID: u32) -> *mut CGImage;
  pub fn CGDisplayCreateImageForRect(display: u32, rect: CGRect) -> *mut CGImage;
  pub fn CGDisplayHideCursor(display: u32) -> CGError;
  pub fn CGDisplayShowCursor(display: u32) -> CGError;
  pub fn CGDisplayMoveCursorToPoint(display: u32, point: CGPoint) -> CGError;
  pub fn CGGetLastMouseDelta(deltaX: *mut i32, deltaY: *mut i32) -> ();
  pub fn CGDisplayGetDrawingContext(display: u32) -> *mut CGContext;
  pub fn CGDisplayAvailableModes(dsp: u32) -> *mut __CFArray;
  pub fn CGDisplayBestModeForParameters(
    display: u32,
    bitsPerPixel: usize,
    width: usize,
    height: usize,
    exactMatch: *mut u32,
  ) -> *mut __CFDictionary;
  pub fn CGDisplayBestModeForParametersAndRefreshRate(
    display: u32,
    bitsPerPixel: usize,
    width: usize,
    height: usize,
    refreshRate: f64,
    exactMatch: *mut u32,
  ) -> *mut __CFDictionary;
  pub fn CGDisplayCurrentMode(display: u32) -> *mut __CFDictionary;
  pub fn CGDisplaySwitchToMode(display: u32, mode: *mut __CFDictionary) -> CGError;
  pub fn CGBeginDisplayConfiguration(config: *mut *mut _CGDisplayConfigRef) -> CGError;
  pub fn CGConfigureDisplayOrigin(
    config: *mut _CGDisplayConfigRef,
    display: u32,
    x: i32,
    y: i32,
  ) -> CGError;
  pub fn CGConfigureDisplayWithDisplayMode(
    config: *mut _CGDisplayConfigRef,
    display: u32,
    mode: *mut CGDisplayMode,
    options: *mut __CFDictionary,
  ) -> CGError;
  pub fn CGConfigureDisplayStereoOperation(
    config: *mut _CGDisplayConfigRef,
    display: u32,
    stereo: u32,
    forceBlueLine: u32,
  ) -> CGError;
  pub fn CGConfigureDisplayMirrorOfDisplay(
    config: *mut _CGDisplayConfigRef,
    display: u32,
    master: u32,
  ) -> CGError;
  pub fn CGCancelDisplayConfiguration(config: *mut _CGDisplayConfigRef) -> CGError;
  pub fn CGCompleteDisplayConfiguration(
    config: *mut _CGDisplayConfigRef,
    option: CGConfigureOption,
  ) -> CGError;
  pub fn CGRestorePermanentDisplayConfiguration() -> ();
  pub fn CGDisplayRegisterReconfigurationCallback(
    callback: Option<extern "C" fn(u32, CGDisplayChangeSummaryFlags, *mut c_void) -> ()>,
    userInfo: *mut c_void,
  ) -> CGError;
  pub fn CGDisplayRemoveReconfigurationCallback(
    callback: Option<extern "C" fn(u32, CGDisplayChangeSummaryFlags, *mut c_void) -> ()>,
    userInfo: *mut c_void,
  ) -> CGError;
  pub fn CGDisplaySetStereoOperation(
    display: u32,
    stereo: u32,
    forceBlueLine: u32,
    option: CGConfigureOption,
  ) -> CGError;
  pub fn CGDisplayIsActive(display: u32) -> u32;
  pub fn CGDisplayIsAsleep(display: u32) -> u32;
  pub fn CGDisplayIsOnline(display: u32) -> u32;
  pub fn CGDisplayIsMain(display: u32) -> u32;
  pub fn CGDisplayIsBuiltin(display: u32) -> u32;
  pub fn CGDisplayIsInMirrorSet(display: u32) -> u32;
  pub fn CGDisplayIsAlwaysInMirrorSet(display: u32) -> u32;
  pub fn CGDisplayIsInHWMirrorSet(display: u32) -> u32;
  pub fn CGDisplayMirrorsDisplay(display: u32) -> u32;
  pub fn CGDisplayUsesOpenGLAcceleration(display: u32) -> u32;
  pub fn CGDisplayIsStereo(display: u32) -> u32;
  pub fn CGDisplayPrimaryDisplay(display: u32) -> u32;
  pub fn CGDisplayUnitNumber(display: u32) -> u32;
  pub fn CGDisplayVendorNumber(display: u32) -> u32;
  pub fn CGDisplayModelNumber(display: u32) -> u32;
  pub fn CGDisplaySerialNumber(display: u32) -> u32;
  pub fn CGDisplayIOServicePort(display: u32) -> u32;
  pub fn CGDisplayScreenSize(display: u32) -> CGSize;
  pub fn CGDisplayRotation(display: u32) -> f64;
  pub fn CGDisplayCopyColorSpace(display: u32) -> *mut CGColorSpace;
  pub fn CGConfigureDisplayMode(
    config: *mut _CGDisplayConfigRef,
    display: u32,
    mode: *mut __CFDictionary,
  ) -> CGError;
  pub fn CGConfigureDisplayFadeEffect(
    config: *mut _CGDisplayConfigRef,
    fadeOutSeconds: f32,
    fadeInSeconds: f32,
    fadeRed: f32,
    fadeGreen: f32,
    fadeBlue: f32,
  ) -> CGError;
  pub fn CGAcquireDisplayFadeReservation(seconds: f32, token: *mut u32) -> CGError;
  pub fn CGReleaseDisplayFadeReservation(token: u32) -> CGError;
  pub fn CGDisplayFade(
    token: u32,
    duration: f32,
    startBlend: f32,
    endBlend: f32,
    redBlend: f32,
    greenBlend: f32,
    blueBlend: f32,
    synchronous: u32,
  ) -> CGError;
  pub fn CGDisplayFadeOperationInProgress() -> u32;
  pub fn CGDisplayStreamUpdateGetTypeID() -> usize;
  pub fn CGDisplayStreamUpdateGetRects(
    updateRef: *mut CGDisplayStreamUpdate,
    rectType: CGDisplayStreamUpdateRectType,
    rectCount: *mut usize,
  ) -> *mut CGRect;
  pub fn CGDisplayStreamUpdateCreateMergedUpdate(
    firstUpdate: *mut CGDisplayStreamUpdate,
    secondUpdate: *mut CGDisplayStreamUpdate,
  ) -> *mut CGDisplayStreamUpdate;
  pub fn CGDisplayStreamUpdateGetMovedRectsDelta(
    updateRef: *mut CGDisplayStreamUpdate,
    dx: *mut f64,
    dy: *mut f64,
  ) -> ();
  pub fn CGDisplayStreamUpdateGetDropCount(updateRef: *mut CGDisplayStreamUpdate) -> usize;
  pub fn CGDisplayStreamGetTypeID() -> usize;
  pub fn CGDisplayStreamCreate(
    display: u32,
    outputWidth: usize,
    outputHeight: usize,
    pixelFormat: i32,
    properties: *mut __CFDictionary,
    handler: (),
  ) -> *mut CGDisplayStream;
  pub fn CGDisplayStreamCreateWithDispatchQueue(
    display: u32,
    outputWidth: usize,
    outputHeight: usize,
    pixelFormat: i32,
    properties: *mut __CFDictionary,
    queue: *mut NSObject,
    handler: (),
  ) -> *mut CGDisplayStream;
  pub fn CGDisplayStreamStart(displayStream: *mut CGDisplayStream) -> CGError;
  pub fn CGDisplayStreamStop(displayStream: *mut CGDisplayStream) -> CGError;
  pub fn CGDisplayStreamGetRunLoopSource(
    displayStream: *mut CGDisplayStream,
  ) -> *mut __CFRunLoopSource;
  pub fn CGRegisterScreenRefreshCallback(
    callback: extern "C" fn(u32, *mut CGRect, *mut c_void) -> (),
    userInfo: *mut c_void,
  ) -> CGError;
  pub fn CGUnregisterScreenRefreshCallback(
    callback: extern "C" fn(u32, *mut CGRect, *mut c_void) -> (),
    userInfo: *mut c_void,
  ) -> ();
  pub fn CGWaitForScreenRefreshRects(rects: *mut *mut CGRect, count: *mut u32) -> CGError;
  pub fn CGScreenRegisterMoveCallback(
    callback: extern "C" fn(CGScreenUpdateMoveDelta, usize, *mut CGRect, *mut c_void) -> (),
    userInfo: *mut c_void,
  ) -> CGError;
  pub fn CGScreenUnregisterMoveCallback(
    callback: extern "C" fn(CGScreenUpdateMoveDelta, usize, *mut CGRect, *mut c_void) -> (),
    userInfo: *mut c_void,
  ) -> ();
  pub fn CGWaitForScreenUpdateRects(
    requestedOperations: CGScreenUpdateOperation,
    currentOperation: *mut CGScreenUpdateOperation,
    rects: *mut *mut CGRect,
    rectCount: *mut usize,
    delta: *mut CGScreenUpdateMoveDelta,
  ) -> CGError;
  pub fn CGReleaseScreenRefreshRects(rects: *mut CGRect) -> ();
  pub fn CGCursorIsVisible() -> u32;
  pub fn CGCursorIsDrawnInFramebuffer() -> u32;
  pub fn CGWarpMouseCursorPosition(newCursorPosition: CGPoint) -> CGError;
  pub fn CGAssociateMouseAndMouseCursorPosition(connected: u32) -> CGError;
  pub fn CGWindowServerCreateServerPort() -> *mut __CFMachPort;
  pub fn CGEnableEventStateCombining(combineState: u32) -> CGError;
  pub fn CGInhibitLocalEvents(inhibit: u32) -> CGError;
  pub fn CGPostMouseEvent(
    mouseCursorPosition: CGPoint,
    updateMouseCursorPosition: u32,
    buttonCount: u32,
    mouseButtonDown: u32,
    ...
  ) -> CGError;
  pub fn CGPostScrollWheelEvent(wheelCount: u32, wheel1: i32, ...) -> CGError;
  pub fn CGPostKeyboardEvent(keyChar: u16, virtualKey: u16, keyDown: u32) -> CGError;
  pub fn CGSetLocalEventsFilterDuringSuppressionState(
    filter: CGEventFilterMask,
    state: CGEventSuppressionState,
  ) -> CGError;
  pub fn CGSetLocalEventsSuppressionInterval(seconds: f64) -> CGError;
  pub fn CGWindowServerCFMachPort() -> *mut __CFMachPort;
  pub fn CGEventGetTypeID() -> usize;
  pub fn CGEventCreate(source: *mut __CGEventSource) -> *mut __CGEvent;
  pub fn CGEventCreateData(allocator: *mut __CFAllocator, event: *mut __CGEvent) -> *mut __CFData;
  pub fn CGEventCreateFromData(
    allocator: *mut __CFAllocator,
    data: *mut __CFData,
  ) -> *mut __CGEvent;
  pub fn CGEventCreateMouseEvent(
    source: *mut __CGEventSource,
    mouseType: CGEventType,
    mouseCursorPosition: CGPoint,
    mouseButton: CGMouseButton,
  ) -> *mut __CGEvent;
  pub fn CGEventCreateKeyboardEvent(
    source: *mut __CGEventSource,
    virtualKey: u16,
    keyDown: bool,
  ) -> *mut __CGEvent;
  pub fn CGEventCreateScrollWheelEvent(
    source: *mut __CGEventSource,
    units: CGScrollEventUnit,
    wheelCount: u32,
    wheel1: i32,
    ...
  ) -> *mut __CGEvent;
  pub fn CGEventCreateScrollWheelEvent2(
    source: *mut __CGEventSource,
    units: CGScrollEventUnit,
    wheelCount: u32,
    wheel1: i32,
    wheel2: i32,
    wheel3: i32,
  ) -> *mut __CGEvent;
  pub fn CGEventCreateCopy(event: *mut __CGEvent) -> *mut __CGEvent;
  pub fn CGEventCreateSourceFromEvent(event: *mut __CGEvent) -> *mut __CGEventSource;
  pub fn CGEventSetSource(event: *mut __CGEvent, source: *mut __CGEventSource) -> ();
  pub fn CGEventGetType(event: *mut __CGEvent) -> CGEventType;
  pub fn CGEventSetType(event: *mut __CGEvent, type_: CGEventType) -> ();
  pub fn CGEventGetTimestamp(event: *mut __CGEvent) -> u64;
  pub fn CGEventSetTimestamp(event: *mut __CGEvent, timestamp: u64) -> ();
  pub fn CGEventGetLocation(event: *mut __CGEvent) -> CGPoint;
  pub fn CGEventGetUnflippedLocation(event: *mut __CGEvent) -> CGPoint;
  pub fn CGEventSetLocation(event: *mut __CGEvent, location: CGPoint) -> ();
  pub fn CGEventGetFlags(event: *mut __CGEvent) -> CGEventFlags;
  pub fn CGEventSetFlags(event: *mut __CGEvent, flags: CGEventFlags) -> ();
  pub fn CGEventKeyboardGetUnicodeString(
    event: *mut __CGEvent,
    maxStringLength: usize,
    actualStringLength: *mut usize,
    unicodeString: *mut u16,
  ) -> ();
  pub fn CGEventKeyboardSetUnicodeString(
    event: *mut __CGEvent,
    stringLength: usize,
    unicodeString: *mut u16,
  ) -> ();
  pub fn CGEventGetIntegerValueField(event: *mut __CGEvent, field: CGEventField) -> i64;
  pub fn CGEventSetIntegerValueField(event: *mut __CGEvent, field: CGEventField, value: i64) -> ();
  pub fn CGEventGetDoubleValueField(event: *mut __CGEvent, field: CGEventField) -> f64;
  pub fn CGEventSetDoubleValueField(event: *mut __CGEvent, field: CGEventField, value: f64) -> ();
  pub fn CGEventTapCreate(
    tap: CGEventTapLocation,
    place: CGEventTapPlacement,
    options: CGEventTapOptions,
    eventsOfInterest: u64,
    callback: extern "C" fn(
      *mut __CGEventTapProxy,
      CGEventType,
      *mut __CGEvent,
      *mut c_void,
    ) -> *mut __CGEvent,
    userInfo: *mut c_void,
  ) -> *mut __CFMachPort;
  pub fn CGEventTapCreateForPSN(
    processSerialNumber: *mut c_void,
    place: CGEventTapPlacement,
    options: CGEventTapOptions,
    eventsOfInterest: u64,
    callback: extern "C" fn(
      *mut __CGEventTapProxy,
      CGEventType,
      *mut __CGEvent,
      *mut c_void,
    ) -> *mut __CGEvent,
    userInfo: *mut c_void,
  ) -> *mut __CFMachPort;
  pub fn CGEventTapCreateForPid(
    pid: i32,
    place: CGEventTapPlacement,
    options: CGEventTapOptions,
    eventsOfInterest: u64,
    callback: extern "C" fn(
      *mut __CGEventTapProxy,
      CGEventType,
      *mut __CGEvent,
      *mut c_void,
    ) -> *mut __CGEvent,
    userInfo: *mut c_void,
  ) -> *mut __CFMachPort;
  pub fn CGEventTapEnable(tap: *mut __CFMachPort, enable: bool) -> ();
  pub fn CGEventTapIsEnabled(tap: *mut __CFMachPort) -> bool;
  pub fn CGEventTapPostEvent(proxy: *mut __CGEventTapProxy, event: *mut __CGEvent) -> ();
  pub fn CGEventPost(tap: CGEventTapLocation, event: *mut __CGEvent) -> ();
  pub fn CGEventPostToPSN(processSerialNumber: *mut c_void, event: *mut __CGEvent) -> ();
  pub fn CGEventPostToPid(pid: i32, event: *mut __CGEvent) -> ();
  pub fn CGGetEventTapList(
    maxNumberOfTaps: u32,
    tapList: *mut CGEventTapInformation,
    eventTapCount: *mut u32,
  ) -> CGError;
  pub fn CGEventSourceGetTypeID() -> usize;
  pub fn CGEventSourceCreate(stateID: CGEventSourceStateID) -> *mut __CGEventSource;
  pub fn CGEventSourceGetKeyboardType(source: *mut __CGEventSource) -> u32;
  pub fn CGEventSourceSetKeyboardType(source: *mut __CGEventSource, keyboardType: u32) -> ();
  pub fn CGEventSourceGetPixelsPerLine(source: *mut __CGEventSource) -> f64;
  pub fn CGEventSourceSetPixelsPerLine(source: *mut __CGEventSource, pixelsPerLine: f64) -> ();
  pub fn CGEventSourceGetSourceStateID(source: *mut __CGEventSource) -> CGEventSourceStateID;
  pub fn CGEventSourceButtonState(stateID: CGEventSourceStateID, button: CGMouseButton) -> bool;
  pub fn CGEventSourceKeyState(stateID: CGEventSourceStateID, key: u16) -> bool;
  pub fn CGEventSourceFlagsState(stateID: CGEventSourceStateID) -> CGEventFlags;
  pub fn CGEventSourceSecondsSinceLastEventType(
    stateID: CGEventSourceStateID,
    eventType: CGEventType,
  ) -> f64;
  pub fn CGEventSourceCounterForEventType(
    stateID: CGEventSourceStateID,
    eventType: CGEventType,
  ) -> u32;
  pub fn CGEventSourceSetUserData(source: *mut __CGEventSource, userData: i64) -> ();
  pub fn CGEventSourceGetUserData(source: *mut __CGEventSource) -> i64;
  pub fn CGEventSourceSetLocalEventsFilterDuringSuppressionState(
    source: *mut __CGEventSource,
    filter: CGEventFilterMask,
    state: CGEventSuppressionState,
  ) -> ();
  pub fn CGEventSourceGetLocalEventsFilterDuringSuppressionState(
    source: *mut __CGEventSource,
    state: CGEventSuppressionState,
  ) -> CGEventFilterMask;
  pub fn CGEventSourceSetLocalEventsSuppressionInterval(
    source: *mut __CGEventSource,
    seconds: f64,
  ) -> ();
  pub fn CGEventSourceGetLocalEventsSuppressionInterval(source: *mut __CGEventSource) -> f64;
  pub fn CGPSConverterCreate(
    info: *mut c_void,
    callbacks: *mut CGPSConverterCallbacks,
    options: *mut __CFDictionary,
  ) -> *mut CGPSConverter;
  pub fn CGPSConverterConvert(
    converter: *mut CGPSConverter,
    provider: *mut CGDataProvider,
    consumer: *mut CGDataConsumer,
    options: *mut __CFDictionary,
  ) -> bool;
  pub fn CGPSConverterAbort(converter: *mut CGPSConverter) -> bool;
  pub fn CGPSConverterIsConverting(converter: *mut CGPSConverter) -> bool;
  pub fn CGPSConverterGetTypeID() -> usize;
  pub fn CGSessionCopyCurrentDictionary() -> *mut __CFDictionary;
  pub fn CGDirectDisplayCopyCurrentMetalDevice(display: u32) -> *mut Object;
}
