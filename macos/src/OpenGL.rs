#[allow(unused_imports)]
use crate::objc::*;
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
pub struct _CGLContextObject {
  opaque: u32,
}
#[repr(C)]
pub struct _CGLPixelFormatObject {
  opaque: u32,
}
#[repr(C)]
pub struct _CGLRendererInfoObject {
  opaque: u32,
}
#[repr(C)]
pub struct _CGLPBufferObject {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLPixelFormatAttribute {
  kCGLPFAAllRenderers = 1,
  kCGLPFATripleBuffer = 3,
  kCGLPFADoubleBuffer = 5,
  kCGLPFAColorSize = 8,
  kCGLPFAAlphaSize = 11,
  kCGLPFADepthSize = 12,
  kCGLPFAStencilSize = 13,
  kCGLPFAMinimumPolicy = 51,
  kCGLPFAMaximumPolicy = 52,
  kCGLPFASampleBuffers = 55,
  kCGLPFASamples = 56,
  kCGLPFAColorFloat = 58,
  kCGLPFAMultisample = 59,
  kCGLPFASupersample = 60,
  kCGLPFASampleAlpha = 61,
  kCGLPFARendererID = 70,
  kCGLPFANoRecovery = 72,
  kCGLPFAAccelerated = 73,
  kCGLPFAClosestPolicy = 74,
  kCGLPFABackingStore = 76,
  kCGLPFABackingVolatile = 77,
  kCGLPFADisplayMask = 84,
  kCGLPFAAllowOfflineRenderers = 96,
  kCGLPFAAcceleratedCompute = 97,
  kCGLPFAOpenGLProfile = 99,
  kCGLPFASupportsAutomaticGraphicsSwitching = 101,
  kCGLPFAVirtualScreenCount = 128,
  kCGLPFAAuxBuffers = 7,
  kCGLPFAAccumSize = 14,
  kCGLPFAAuxDepthStencil = 57,
  kCGLPFAStereo = 6,
  kCGLPFAOffScreen = 53,
  kCGLPFAWindow = 80,
  kCGLPFACompliant = 83,
  kCGLPFAPBuffer = 90,
  kCGLPFARemotePBuffer = 91,
  kCGLPFASingleRenderer = 71,
  kCGLPFARobust = 75,
  kCGLPFAMPSafe = 78,
  kCGLPFAMultiScreen = 81,
  kCGLPFAFullScreen = 54,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLRendererProperty {
  kCGLRPOffScreen = 53,
  kCGLRPRendererID = 70,
  kCGLRPAccelerated = 73,
  kCGLRPBackingStore = 76,
  kCGLRPWindow = 80,
  kCGLRPCompliant = 83,
  kCGLRPDisplayMask = 84,
  kCGLRPBufferModes = 100,
  kCGLRPColorModes = 103,
  kCGLRPAccumModes = 104,
  kCGLRPDepthModes = 105,
  kCGLRPStencilModes = 106,
  kCGLRPMaxAuxBuffers = 107,
  kCGLRPMaxSampleBuffers = 108,
  kCGLRPMaxSamples = 109,
  kCGLRPSampleModes = 110,
  kCGLRPSampleAlpha = 111,
  kCGLRPGPUVertProcCapable = 122,
  kCGLRPGPUFragProcCapable = 123,
  kCGLRPRendererCount = 128,
  kCGLRPOnline = 129,
  kCGLRPAcceleratedCompute = 130,
  kCGLRPVideoMemoryMegabytes = 131,
  kCGLRPTextureMemoryMegabytes = 132,
  kCGLRPMajorGLVersion = 133,
  kCGLRPRegistryIDLow = 140,
  kCGLRPRegistryIDHigh = 141,
  kCGLRPRemovable = 142,
  kCGLRPRobust = 75,
  kCGLRPMPSafe = 78,
  kCGLRPMultiScreen = 81,
  kCGLRPFullScreen = 54,
  kCGLRPVideoMemory = 120,
  kCGLRPTextureMemory = 121,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLContextEnable {
  kCGLCESwapRectangle = 201,
  kCGLCESwapLimit = 203,
  kCGLCERasterization = 221,
  kCGLCEStateValidation = 301,
  kCGLCESurfaceBackingSize = 305,
  kCGLCEDisplayListOptimization = 307,
  kCGLCEMPEngine = 313,
  kCGLCECrashOnRemovedFunctions = 316,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLGPURestartStatus {
  kCGLCPGPURestartStatusNone = 0,
  kCGLCPGPURestartStatusCaused = 1,
  kCGLCPGPURestartStatusBlacklisted = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLContextParameter {
  kCGLCPSwapRectangle = 200,
  kCGLCPSwapInterval = 222,
  kCGLCPDispatchTableSize = 224,
  kCGLCPClientStorage = 226,
  kCGLCPSurfaceTexture = 228,
  kCGLCPSurfaceOrder = 235,
  kCGLCPSurfaceOpacity = 236,
  kCGLCPSurfaceBackingSize = 304,
  kCGLCPSurfaceSurfaceVolatile = 306,
  kCGLCPReclaimResources = 308,
  kCGLCPCurrentRendererID = 309,
  kCGLCPGPUVertexProcessing = 310,
  kCGLCPGPUFragmentProcessing = 311,
  kCGLCPHasDrawable = 314,
  kCGLCPMPSwapsInFlight = 315,
  kCGLCPGPURestartStatus = 317,
  kCGLCPAbortOnGPURestartStatusBlacklisted = 318,
  kCGLCPSupportGPURestart = 319,
  kCGLCPSupportSeparateAddressSpace = 320,
  kCGLCPContextPriorityRequest = 608,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLCPContextPriorityRequest {
  kCGLCPContextPriorityRequestHigh = 0,
  kCGLCPContextPriorityRequestNormal = 1,
  kCGLCPContextPriorityRequestLow = 2,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLGlobalOption {
  kCGLGOFormatCacheSize = 501,
  kCGLGOClearFormatCache = 502,
  kCGLGORetainRenderers = 503,
  kCGLGOUseBuildCache = 506,
  kCGLGOResetLibrary = 504,
  kCGLGOUseErrorHandler = 505,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLOpenGLProfile {
  kCGLOGLPVersion_Legacy = 4096,
  kCGLOGLPVersion_3_2_Core = 12800,
  kCGLOGLPVersion_GL4_Core = 16640,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGLError {
  kCGLNoError = 0,
  kCGLBadAttribute = 10000,
  kCGLBadProperty = 10001,
  kCGLBadPixelFormat = 10002,
  kCGLBadRendererInfo = 10003,
  kCGLBadContext = 10004,
  kCGLBadDrawable = 10005,
  kCGLBadDisplay = 10006,
  kCGLBadState = 10007,
  kCGLBadValue = 10008,
  kCGLBadMatch = 10009,
  kCGLBadEnumeration = 10010,
  kCGLBadOffScreen = 10011,
  kCGLBadFullScreen = 10012,
  kCGLBadWindow = 10013,
  kCGLBadAddress = 10014,
  kCGLBadCodeModule = 10015,
  kCGLBadAlloc = 10016,
  kCGLBadConnection = 10017,
}
pub type GLbitfield = u32;
pub type GLboolean = u8;
pub type GLbyte = i8;
pub type GLenum = u32;
pub type GLint = i32;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLushort = u16;
pub type GLfixed = i32;
pub type GLhalf = u16;
pub type GLhalfARB = u16;
pub type GLint64 = i64;
#[repr(C)]
pub struct __GLsync {
  opaque: u32,
}
pub type GLuint64 = u64;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;
pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
#[repr(C)]
pub struct CGLShareGroupRec {
  opaque: u32,
}
#[repr(C)]
pub struct _cl_device_id {
  opaque: u32,
}
#[repr(C)]
pub struct __IOSurface {
  opaque: u32,
}
#[cfg(feature = "RK_OpenGL")]
#[link(name = "OpenGL", kind = "framework")]
extern "C" {
  pub fn CGLSetCurrentContext(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLGetCurrentContext() -> *mut _CGLContextObject;
  pub fn CGLGetShareGroup(ctx: *mut _CGLContextObject) -> *mut CGLShareGroupRec;
  pub fn CGLGetDeviceFromGLRenderer(rendererID: i32) -> *mut _cl_device_id;
  pub fn CGLTexImageIOSurface2D(
    ctx: *mut _CGLContextObject,
    target: u32,
    internal_format: u32,
    width: i32,
    height: i32,
    format: u32,
    type_: u32,
    ioSurface: *mut __IOSurface,
    plane: u32,
  ) -> CGLError;
  pub fn CGLChoosePixelFormat(
    attribs: *mut CGLPixelFormatAttribute,
    pix: *mut *mut _CGLPixelFormatObject,
    npix: *mut i32,
  ) -> CGLError;
  pub fn CGLDestroyPixelFormat(pix: *mut _CGLPixelFormatObject) -> CGLError;
  pub fn CGLDescribePixelFormat(
    pix: *mut _CGLPixelFormatObject,
    pix_num: i32,
    attrib: CGLPixelFormatAttribute,
    value: *mut i32,
  ) -> CGLError;
  pub fn CGLReleasePixelFormat(pix: *mut _CGLPixelFormatObject) -> ();
  pub fn CGLRetainPixelFormat(pix: *mut _CGLPixelFormatObject) -> *mut _CGLPixelFormatObject;
  pub fn CGLGetPixelFormatRetainCount(pix: *mut _CGLPixelFormatObject) -> u32;
  pub fn CGLQueryRendererInfo(
    display_mask: u32,
    rend: *mut *mut _CGLRendererInfoObject,
    nrend: *mut i32,
  ) -> CGLError;
  pub fn CGLDestroyRendererInfo(rend: *mut _CGLRendererInfoObject) -> CGLError;
  pub fn CGLDescribeRenderer(
    rend: *mut _CGLRendererInfoObject,
    rend_num: i32,
    prop: CGLRendererProperty,
    value: *mut i32,
  ) -> CGLError;
  pub fn CGLCreateContext(
    pix: *mut _CGLPixelFormatObject,
    share: *mut _CGLContextObject,
    ctx: *mut *mut _CGLContextObject,
  ) -> CGLError;
  pub fn CGLDestroyContext(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLCopyContext(
    src: *mut _CGLContextObject,
    dst: *mut _CGLContextObject,
    mask: u32,
  ) -> CGLError;
  pub fn CGLRetainContext(ctx: *mut _CGLContextObject) -> *mut _CGLContextObject;
  pub fn CGLReleaseContext(ctx: *mut _CGLContextObject) -> ();
  pub fn CGLGetContextRetainCount(ctx: *mut _CGLContextObject) -> u32;
  pub fn CGLGetPixelFormat(ctx: *mut _CGLContextObject) -> *mut _CGLPixelFormatObject;
  pub fn CGLCreatePBuffer(
    width: i32,
    height: i32,
    target: u32,
    internalFormat: u32,
    max_level: i32,
    pbuffer: *mut *mut _CGLPBufferObject,
  ) -> CGLError;
  pub fn CGLDestroyPBuffer(pbuffer: *mut _CGLPBufferObject) -> CGLError;
  pub fn CGLDescribePBuffer(
    obj: *mut _CGLPBufferObject,
    width: *mut i32,
    height: *mut i32,
    target: *mut u32,
    internalFormat: *mut u32,
    mipmap: *mut i32,
  ) -> CGLError;
  pub fn CGLTexImagePBuffer(
    ctx: *mut _CGLContextObject,
    pbuffer: *mut _CGLPBufferObject,
    source: u32,
  ) -> CGLError;
  pub fn CGLRetainPBuffer(pbuffer: *mut _CGLPBufferObject) -> *mut _CGLPBufferObject;
  pub fn CGLReleasePBuffer(pbuffer: *mut _CGLPBufferObject) -> ();
  pub fn CGLGetPBufferRetainCount(pbuffer: *mut _CGLPBufferObject) -> u32;
  pub fn CGLSetOffScreen(
    ctx: *mut _CGLContextObject,
    width: i32,
    height: i32,
    rowbytes: i32,
    baseaddr: *mut c_void,
  ) -> CGLError;
  pub fn CGLGetOffScreen(
    ctx: *mut _CGLContextObject,
    width: *mut i32,
    height: *mut i32,
    rowbytes: *mut i32,
    baseaddr: *mut *mut c_void,
  ) -> CGLError;
  pub fn CGLSetFullScreen(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLSetFullScreenOnDisplay(ctx: *mut _CGLContextObject, display_mask: u32) -> CGLError;
  pub fn CGLSetPBuffer(
    ctx: *mut _CGLContextObject,
    pbuffer: *mut _CGLPBufferObject,
    face: u32,
    level: i32,
    screen: i32,
  ) -> CGLError;
  pub fn CGLGetPBuffer(
    ctx: *mut _CGLContextObject,
    pbuffer: *mut *mut _CGLPBufferObject,
    face: *mut u32,
    level: *mut i32,
    screen: *mut i32,
  ) -> CGLError;
  pub fn CGLClearDrawable(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLFlushDrawable(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLEnable(ctx: *mut _CGLContextObject, pname: CGLContextEnable) -> CGLError;
  pub fn CGLDisable(ctx: *mut _CGLContextObject, pname: CGLContextEnable) -> CGLError;
  pub fn CGLIsEnabled(
    ctx: *mut _CGLContextObject,
    pname: CGLContextEnable,
    enable: *mut i32,
  ) -> CGLError;
  pub fn CGLSetParameter(
    ctx: *mut _CGLContextObject,
    pname: CGLContextParameter,
    params: *mut i32,
  ) -> CGLError;
  pub fn CGLGetParameter(
    ctx: *mut _CGLContextObject,
    pname: CGLContextParameter,
    params: *mut i32,
  ) -> CGLError;
  pub fn CGLSetVirtualScreen(ctx: *mut _CGLContextObject, screen: i32) -> CGLError;
  pub fn CGLGetVirtualScreen(ctx: *mut _CGLContextObject, screen: *mut i32) -> CGLError;
  pub fn CGLUpdateContext(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLSetGlobalOption(pname: CGLGlobalOption, params: *mut i32) -> CGLError;
  pub fn CGLGetGlobalOption(pname: CGLGlobalOption, params: *mut i32) -> CGLError;
  pub fn CGLSetOption(pname: CGLGlobalOption, param: i32) -> CGLError;
  pub fn CGLGetOption(pname: CGLGlobalOption, param: *mut i32) -> CGLError;
  pub fn CGLLockContext(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLUnlockContext(ctx: *mut _CGLContextObject) -> CGLError;
  pub fn CGLGetVersion(majorvers: *mut i32, minorvers: *mut i32) -> ();
  pub fn CGLErrorString(error: CGLError) -> *mut i8;
}
