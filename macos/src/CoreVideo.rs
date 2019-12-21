#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFString;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::IOSurface::__IOSurface;
use crate::Metal::MTLPixelFormat;
use crate::OpenGL::_CGLContextObject;
use crate::OpenGL::_CGLPixelFormatObject;
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
pub type CVOptionFlags = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVSMPTETime {
  pub subframes: i16,
  pub subframeDivisor: i16,
  pub counter: u32,
  pub type_: u32,
  pub flags: u32,
  pub hours: i16,
  pub minutes: i16,
  pub seconds: i16,
  pub frames: i16,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CVSMPTETimeType {
  kCVSMPTETimeType24 = 0,
  kCVSMPTETimeType25 = 1,
  kCVSMPTETimeType30Drop = 2,
  kCVSMPTETimeType30 = 3,
  kCVSMPTETimeType2997 = 4,
  kCVSMPTETimeType2997Drop = 5,
  kCVSMPTETimeType60 = 6,
  kCVSMPTETimeType5994 = 7,
}
bitflags! { # [ repr ( C ) ] pub struct CVSMPTETimeFlags : u32 { const kCVSMPTETimeValid = 1 ; const kCVSMPTETimeRunning = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct CVTimeFlags : i32 { const kCVTimeIsIndefinite = 1 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVTime {
  pub timeValue: i64,
  pub timeScale: i32,
  pub flags: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVTimeStamp {
  pub version: u32,
  pub videoTimeScale: i32,
  pub videoTime: i64,
  pub hostTime: u64,
  pub rateScalar: f64,
  pub videoRefreshPeriod: i64,
  pub smpteTime: CVSMPTETime,
  pub flags: u64,
  pub reserved: u64,
}
bitflags! { # [ repr ( C ) ] pub struct CVTimeStampFlags : u64 { const kCVTimeStampVideoTimeValid = 1 ; const kCVTimeStampHostTimeValid = 2 ; const kCVTimeStampSMPTETimeValid = 4 ; const kCVTimeStampVideoRefreshPeriodValid = 8 ; const kCVTimeStampRateScalarValid = 16 ; const kCVTimeStampTopField = 65536 ; const kCVTimeStampBottomField = 131072 ; const kCVTimeStampVideoHostTimeValid = 3 ; const kCVTimeStampIsInterlaced = 196608 ; } }
pub type CVReturn = i32;
#[repr(C)]
pub struct __CVDisplayLink {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CVAttachmentMode {
  kCVAttachmentMode_ShouldNotPropagate = 0,
  kCVAttachmentMode_ShouldPropagate = 1,
}
#[repr(C)]
pub struct __CVBuffer {
  opaque: u32,
}
pub type CVImageBufferRef = *mut __CVBuffer;
bitflags! { # [ repr ( C ) ] pub struct CVPixelBufferLockFlags : u64 { const kCVPixelBufferLock_ReadOnly = 1 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVPlanarComponentInfo {
  pub offset: i32,
  pub rowBytes: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVPlanarPixelBufferInfo {
  pub componentInfo: [CVPlanarComponentInfo; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrPlanar {
  pub componentInfoY: CVPlanarComponentInfo,
  pub componentInfoCb: CVPlanarComponentInfo,
  pub componentInfoCr: CVPlanarComponentInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVPlanarPixelBufferInfo_YCbCrBiPlanar {
  pub componentInfoY: CVPlanarComponentInfo,
  pub componentInfoCbCr: CVPlanarComponentInfo,
}
pub type CVPixelBufferRef = *mut __CVBuffer;
#[repr(C)]
pub struct __CVPixelBufferPool {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CVPixelBufferPoolFlushFlags : u64 { const kCVPixelBufferPoolFlushExcessBuffers = 1 ; } }
pub type CVOpenGLBufferRef = *mut __CVBuffer;
#[repr(C)]
pub struct __CVOpenGLBufferPool {
  opaque: u32,
}
pub type CVOpenGLTextureRef = *mut __CVBuffer;
#[repr(C)]
pub struct __CVOpenGLTextureCache {
  opaque: u32,
}
pub type CVMetalTextureRef = *mut __CVBuffer;
#[repr(C)]
pub struct __CVMetalTextureCache {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVFillExtendedPixelsCallBackData {
  pub version: isize,
  pub fillCallBack: Option<extern "C" fn(*mut __CVBuffer, *mut c_void) -> u8>,
  pub refCon: *mut c_void,
}
#[cfg(feature = "RK_CoreVideo")]
#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
  pub fn CVGetCurrentHostTime() -> u64;
  pub fn CVGetHostClockFrequency() -> f64;
  pub fn CVGetHostClockMinimumTimeDelta() -> u32;
  pub fn CVDisplayLinkGetTypeID() -> usize;
  pub fn CVDisplayLinkCreateWithCGDisplays(
    displayArray: *mut u32,
    count: isize,
    displayLinkOut: *mut *mut __CVDisplayLink,
  ) -> i32;
  pub fn CVDisplayLinkCreateWithOpenGLDisplayMask(
    mask: u32,
    displayLinkOut: *mut *mut __CVDisplayLink,
  ) -> i32;
  pub fn CVDisplayLinkCreateWithCGDisplay(
    displayID: u32,
    displayLinkOut: *mut *mut __CVDisplayLink,
  ) -> i32;
  pub fn CVDisplayLinkCreateWithActiveCGDisplays(displayLinkOut: *mut *mut __CVDisplayLink) -> i32;
  pub fn CVDisplayLinkSetCurrentCGDisplay(displayLink: *mut __CVDisplayLink, displayID: u32)
    -> i32;
  pub fn CVDisplayLinkSetCurrentCGDisplayFromOpenGLContext(
    displayLink: *mut __CVDisplayLink,
    cglContext: *mut _CGLContextObject,
    cglPixelFormat: *mut _CGLPixelFormatObject,
  ) -> i32;
  pub fn CVDisplayLinkGetCurrentCGDisplay(displayLink: *mut __CVDisplayLink) -> u32;
  pub fn CVDisplayLinkSetOutputCallback(
    displayLink: *mut __CVDisplayLink,
    callback: Option<
      extern "C" fn(
        *mut __CVDisplayLink,
        *mut CVTimeStamp,
        *mut CVTimeStamp,
        u64,
        *mut u64,
        *mut c_void,
      ) -> i32,
    >,
    userInfo: *mut c_void,
  ) -> i32;
  pub fn CVDisplayLinkSetOutputHandler(displayLink: *mut __CVDisplayLink, handler: ()) -> i32;
  pub fn CVDisplayLinkStart(displayLink: *mut __CVDisplayLink) -> i32;
  pub fn CVDisplayLinkStop(displayLink: *mut __CVDisplayLink) -> i32;
  pub fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(
    displayLink: *mut __CVDisplayLink,
  ) -> CVTime;
  pub fn CVDisplayLinkGetOutputVideoLatency(displayLink: *mut __CVDisplayLink) -> CVTime;
  pub fn CVDisplayLinkGetActualOutputVideoRefreshPeriod(displayLink: *mut __CVDisplayLink) -> f64;
  pub fn CVDisplayLinkIsRunning(displayLink: *mut __CVDisplayLink) -> u8;
  pub fn CVDisplayLinkGetCurrentTime(
    displayLink: *mut __CVDisplayLink,
    outTime: *mut CVTimeStamp,
  ) -> i32;
  pub fn CVDisplayLinkTranslateTime(
    displayLink: *mut __CVDisplayLink,
    inTime: *mut CVTimeStamp,
    outTime: *mut CVTimeStamp,
  ) -> i32;
  pub fn CVDisplayLinkRetain(displayLink: *mut __CVDisplayLink) -> *mut __CVDisplayLink;
  pub fn CVDisplayLinkRelease(displayLink: *mut __CVDisplayLink) -> ();
  pub fn CVBufferRetain(buffer: *mut __CVBuffer) -> *mut __CVBuffer;
  pub fn CVBufferRelease(buffer: *mut __CVBuffer) -> ();
  pub fn CVBufferSetAttachment(
    buffer: *mut __CVBuffer,
    key: *mut __CFString,
    value: *mut c_void,
    attachmentMode: CVAttachmentMode,
  ) -> ();
  pub fn CVBufferGetAttachment(
    buffer: *mut __CVBuffer,
    key: *mut __CFString,
    attachmentMode: *mut CVAttachmentMode,
  ) -> *mut c_void;
  pub fn CVBufferRemoveAttachment(buffer: *mut __CVBuffer, key: *mut __CFString) -> ();
  pub fn CVBufferRemoveAllAttachments(buffer: *mut __CVBuffer) -> ();
  pub fn CVBufferGetAttachments(
    buffer: *mut __CVBuffer,
    attachmentMode: CVAttachmentMode,
  ) -> *mut __CFDictionary;
  pub fn CVBufferSetAttachments(
    buffer: *mut __CVBuffer,
    theAttachments: *mut __CFDictionary,
    attachmentMode: CVAttachmentMode,
  ) -> ();
  pub fn CVBufferPropagateAttachments(
    sourceBuffer: *mut __CVBuffer,
    destinationBuffer: *mut __CVBuffer,
  ) -> ();
  pub fn CVYCbCrMatrixGetIntegerCodePointForString(yCbCrMatrixString: *mut __CFString) -> i32;
  pub fn CVColorPrimariesGetIntegerCodePointForString(colorPrimariesString: *mut __CFString)
    -> i32;
  pub fn CVTransferFunctionGetIntegerCodePointForString(
    transferFunctionString: *mut __CFString,
  ) -> i32;
  pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(yCbCrMatrixCodePoint: i32) -> *mut __CFString;
  pub fn CVColorPrimariesGetStringForIntegerCodePoint(
    colorPrimariesCodePoint: i32,
  ) -> *mut __CFString;
  pub fn CVTransferFunctionGetStringForIntegerCodePoint(
    transferFunctionCodePoint: i32,
  ) -> *mut __CFString;
  pub fn CVImageBufferGetEncodedSize(imageBuffer: *mut __CVBuffer) -> CGSize;
  pub fn CVImageBufferGetDisplaySize(imageBuffer: *mut __CVBuffer) -> CGSize;
  pub fn CVImageBufferGetCleanRect(imageBuffer: *mut __CVBuffer) -> CGRect;
  pub fn CVImageBufferIsFlipped(imageBuffer: *mut __CVBuffer) -> u8;
  pub fn CVImageBufferGetColorSpace(imageBuffer: *mut __CVBuffer) -> *mut CGColorSpace;
  pub fn CVImageBufferCreateColorSpaceFromAttachments(
    attachments: *mut __CFDictionary,
  ) -> *mut CGColorSpace;
  pub fn CVPixelBufferGetTypeID() -> usize;
  pub fn CVPixelBufferRetain(texture: *mut __CVBuffer) -> *mut __CVBuffer;
  pub fn CVPixelBufferRelease(texture: *mut __CVBuffer) -> ();
  pub fn CVPixelBufferCreateResolvedAttributesDictionary(
    allocator: *mut __CFAllocator,
    attributes: *mut __CFArray,
    resolvedDictionaryOut: *mut *mut __CFDictionary,
  ) -> i32;
  pub fn CVPixelBufferCreate(
    allocator: *mut __CFAllocator,
    width: usize,
    height: usize,
    pixelFormatType: u32,
    pixelBufferAttributes: *mut __CFDictionary,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferCreateWithBytes(
    allocator: *mut __CFAllocator,
    width: usize,
    height: usize,
    pixelFormatType: u32,
    baseAddress: *mut c_void,
    bytesPerRow: usize,
    releaseCallback: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
    releaseRefCon: *mut c_void,
    pixelBufferAttributes: *mut __CFDictionary,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferCreateWithPlanarBytes(
    allocator: *mut __CFAllocator,
    width: usize,
    height: usize,
    pixelFormatType: u32,
    dataPtr: *mut c_void,
    dataSize: usize,
    numberOfPlanes: usize,
    planeBaseAddress: *mut *mut c_void,
    planeWidth: *mut usize,
    planeHeight: *mut usize,
    planeBytesPerRow: *mut usize,
    releaseCallback: Option<
      extern "C" fn(*mut c_void, *mut c_void, usize, usize, *mut *mut c_void) -> (),
    >,
    releaseRefCon: *mut c_void,
    pixelBufferAttributes: *mut __CFDictionary,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferLockBaseAddress(
    pixelBuffer: *mut __CVBuffer,
    lockFlags: CVPixelBufferLockFlags,
  ) -> i32;
  pub fn CVPixelBufferUnlockBaseAddress(
    pixelBuffer: *mut __CVBuffer,
    unlockFlags: CVPixelBufferLockFlags,
  ) -> i32;
  pub fn CVPixelBufferGetWidth(pixelBuffer: *mut __CVBuffer) -> usize;
  pub fn CVPixelBufferGetHeight(pixelBuffer: *mut __CVBuffer) -> usize;
  pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: *mut __CVBuffer) -> u32;
  pub fn CVPixelBufferGetBaseAddress(pixelBuffer: *mut __CVBuffer) -> *mut c_void;
  pub fn CVPixelBufferGetBytesPerRow(pixelBuffer: *mut __CVBuffer) -> usize;
  pub fn CVPixelBufferGetDataSize(pixelBuffer: *mut __CVBuffer) -> usize;
  pub fn CVPixelBufferIsPlanar(pixelBuffer: *mut __CVBuffer) -> u8;
  pub fn CVPixelBufferGetPlaneCount(pixelBuffer: *mut __CVBuffer) -> usize;
  pub fn CVPixelBufferGetWidthOfPlane(pixelBuffer: *mut __CVBuffer, planeIndex: usize) -> usize;
  pub fn CVPixelBufferGetHeightOfPlane(pixelBuffer: *mut __CVBuffer, planeIndex: usize) -> usize;
  pub fn CVPixelBufferGetBaseAddressOfPlane(
    pixelBuffer: *mut __CVBuffer,
    planeIndex: usize,
  ) -> *mut c_void;
  pub fn CVPixelBufferGetBytesPerRowOfPlane(
    pixelBuffer: *mut __CVBuffer,
    planeIndex: usize,
  ) -> usize;
  pub fn CVPixelBufferGetExtendedPixels(
    pixelBuffer: *mut __CVBuffer,
    extraColumnsOnLeft: *mut usize,
    extraColumnsOnRight: *mut usize,
    extraRowsOnTop: *mut usize,
    extraRowsOnBottom: *mut usize,
  ) -> ();
  pub fn CVPixelBufferFillExtendedPixels(pixelBuffer: *mut __CVBuffer) -> i32;
  pub fn CVPixelBufferGetIOSurface(pixelBuffer: *mut __CVBuffer) -> *mut __IOSurface;
  pub fn CVPixelBufferCreateWithIOSurface(
    allocator: *mut __CFAllocator,
    surface: *mut __IOSurface,
    pixelBufferAttributes: *mut __CFDictionary,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferPoolGetTypeID() -> usize;
  pub fn CVPixelBufferPoolRetain(
    pixelBufferPool: *mut __CVPixelBufferPool,
  ) -> *mut __CVPixelBufferPool;
  pub fn CVPixelBufferPoolRelease(pixelBufferPool: *mut __CVPixelBufferPool) -> ();
  pub fn CVPixelBufferPoolCreate(
    allocator: *mut __CFAllocator,
    poolAttributes: *mut __CFDictionary,
    pixelBufferAttributes: *mut __CFDictionary,
    poolOut: *mut *mut __CVPixelBufferPool,
  ) -> i32;
  pub fn CVPixelBufferPoolGetAttributes(pool: *mut __CVPixelBufferPool) -> *mut __CFDictionary;
  pub fn CVPixelBufferPoolGetPixelBufferAttributes(
    pool: *mut __CVPixelBufferPool,
  ) -> *mut __CFDictionary;
  pub fn CVPixelBufferPoolCreatePixelBuffer(
    allocator: *mut __CFAllocator,
    pixelBufferPool: *mut __CVPixelBufferPool,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
    allocator: *mut __CFAllocator,
    pixelBufferPool: *mut __CVPixelBufferPool,
    auxAttributes: *mut __CFDictionary,
    pixelBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVPixelBufferPoolFlush(
    pool: *mut __CVPixelBufferPool,
    options: CVPixelBufferPoolFlushFlags,
  ) -> ();
  pub fn CVOpenGLBufferGetTypeID() -> usize;
  pub fn CVOpenGLBufferRetain(buffer: *mut __CVBuffer) -> *mut __CVBuffer;
  pub fn CVOpenGLBufferRelease(buffer: *mut __CVBuffer) -> ();
  pub fn CVOpenGLBufferCreate(
    allocator: *mut __CFAllocator,
    width: usize,
    height: usize,
    attributes: *mut __CFDictionary,
    bufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVOpenGLBufferGetAttributes(openGLBuffer: *mut __CVBuffer) -> *mut __CFDictionary;
  pub fn CVOpenGLBufferAttach(
    openGLBuffer: *mut __CVBuffer,
    cglContext: *mut _CGLContextObject,
    face: u32,
    level: i32,
    screen: i32,
  ) -> i32;
  pub fn CVOpenGLBufferPoolGetTypeID() -> usize;
  pub fn CVOpenGLBufferPoolRetain(
    openGLBufferPool: *mut __CVOpenGLBufferPool,
  ) -> *mut __CVOpenGLBufferPool;
  pub fn CVOpenGLBufferPoolRelease(openGLBufferPool: *mut __CVOpenGLBufferPool) -> ();
  pub fn CVOpenGLBufferPoolCreate(
    allocator: *mut __CFAllocator,
    poolAttributes: *mut __CFDictionary,
    openGLBufferAttributes: *mut __CFDictionary,
    poolOut: *mut *mut __CVOpenGLBufferPool,
  ) -> i32;
  pub fn CVOpenGLBufferPoolGetAttributes(pool: *mut __CVOpenGLBufferPool) -> *mut __CFDictionary;
  pub fn CVOpenGLBufferPoolGetOpenGLBufferAttributes(
    pool: *mut __CVOpenGLBufferPool,
  ) -> *mut __CFDictionary;
  pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
    allocator: *mut __CFAllocator,
    openGLBufferPool: *mut __CVOpenGLBufferPool,
    openGLBufferOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVOpenGLTextureGetTypeID() -> usize;
  pub fn CVOpenGLTextureRetain(texture: *mut __CVBuffer) -> *mut __CVBuffer;
  pub fn CVOpenGLTextureRelease(texture: *mut __CVBuffer) -> ();
  pub fn CVOpenGLTextureGetTarget(image: *mut __CVBuffer) -> u32;
  pub fn CVOpenGLTextureGetName(image: *mut __CVBuffer) -> u32;
  pub fn CVOpenGLTextureIsFlipped(image: *mut __CVBuffer) -> u8;
  pub fn CVOpenGLTextureGetCleanTexCoords(
    image: *mut __CVBuffer,
    lowerLeft: [f32; 2],
    lowerRight: [f32; 2],
    upperRight: [f32; 2],
    upperLeft: [f32; 2],
  ) -> ();
  pub fn CVOpenGLTextureCacheGetTypeID() -> usize;
  pub fn CVOpenGLTextureCacheRetain(
    textureCache: *mut __CVOpenGLTextureCache,
  ) -> *mut __CVOpenGLTextureCache;
  pub fn CVOpenGLTextureCacheRelease(textureCache: *mut __CVOpenGLTextureCache) -> ();
  pub fn CVOpenGLTextureCacheCreate(
    allocator: *mut __CFAllocator,
    cacheAttributes: *mut __CFDictionary,
    cglContext: *mut _CGLContextObject,
    cglPixelFormat: *mut _CGLPixelFormatObject,
    textureAttributes: *mut __CFDictionary,
    cacheOut: *mut *mut __CVOpenGLTextureCache,
  ) -> i32;
  pub fn CVOpenGLTextureCacheCreateTextureFromImage(
    allocator: *mut __CFAllocator,
    textureCache: *mut __CVOpenGLTextureCache,
    sourceImage: *mut __CVBuffer,
    attributes: *mut __CFDictionary,
    textureOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVOpenGLTextureCacheFlush(textureCache: *mut __CVOpenGLTextureCache, options: u64) -> ();
  pub fn CVMetalTextureGetTypeID() -> usize;
  pub fn CVMetalTextureGetTexture(image: *mut __CVBuffer) -> *mut Object;
  pub fn CVMetalTextureIsFlipped(image: *mut __CVBuffer) -> u8;
  pub fn CVMetalTextureGetCleanTexCoords(
    image: *mut __CVBuffer,
    lowerLeft: [f32; 2],
    lowerRight: [f32; 2],
    upperRight: [f32; 2],
    upperLeft: [f32; 2],
  ) -> ();
  pub fn CVMetalTextureCacheGetTypeID() -> usize;
  pub fn CVMetalTextureCacheCreate(
    allocator: *mut __CFAllocator,
    cacheAttributes: *mut __CFDictionary,
    metalDevice: *mut Object,
    textureAttributes: *mut __CFDictionary,
    cacheOut: *mut *mut __CVMetalTextureCache,
  ) -> i32;
  pub fn CVMetalTextureCacheCreateTextureFromImage(
    allocator: *mut __CFAllocator,
    textureCache: *mut __CVMetalTextureCache,
    sourceImage: *mut __CVBuffer,
    textureAttributes: *mut __CFDictionary,
    pixelFormat: MTLPixelFormat,
    width: usize,
    height: usize,
    planeIndex: usize,
    textureOut: *mut *mut __CVBuffer,
  ) -> i32;
  pub fn CVMetalTextureCacheFlush(textureCache: *mut __CVMetalTextureCache, options: u64) -> ();
  pub fn CVPixelFormatDescriptionCreateWithPixelFormatType(
    allocator: *mut __CFAllocator,
    pixelFormat: u32,
  ) -> *mut __CFDictionary;
  pub fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(
    allocator: *mut __CFAllocator,
  ) -> *mut __CFArray;
  pub fn CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType(
    description: *mut __CFDictionary,
    pixelFormat: u32,
  ) -> ();
}
