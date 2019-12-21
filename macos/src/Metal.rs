#[allow(unused_imports)]
use crate::objc::*;
use crate::Foundation::NSArray;
use crate::Foundation::NSBundle;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSError;
use crate::Foundation::NSRange;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSString;
use crate::Foundation::NSURL;
use crate::IOSurface::__IOSurface;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
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
#[derive(Copy, Clone)]
pub struct MTLOrigin {
  pub x: usize,
  pub y: usize,
  pub z: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLSize {
  pub width: usize,
  pub height: usize,
  pub depth: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLRegion {
  pub origin: MTLOrigin,
  pub size: MTLSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLSamplePosition {
  pub x: f32,
  pub y: f32,
}
bitflags! { # [ repr ( C ) ] pub struct MTLResourceUsage : usize { const MTLResourceUsageRead = 1 ; const MTLResourceUsageWrite = 2 ; const MTLResourceUsageSample = 4 ; } }
bitflags! { # [ repr ( C ) ] pub struct MTLBarrierScope : usize { const MTLBarrierScopeBuffers = 1 ; const MTLBarrierScopeTextures = 2 ; const MTLBarrierScopeRenderTargets = 4 ; } }
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandEncoderProto {
  #[objrs(selector = "endEncoding")]
  #[cfg(feature = "RK_Metal")]
  fn endEncoding(&self) -> ();
  #[objrs(selector = "insertDebugSignpost:")]
  #[cfg(feature = "RK_Metal")]
  fn insertDebugSignpost_(&self, string: &NSString) -> ();
  #[objrs(selector = "pushDebugGroup:")]
  #[cfg(feature = "RK_Metal")]
  fn pushDebugGroup_(&self, string: &NSString) -> ();
  #[objrs(selector = "popDebugGroup")]
  #[cfg(feature = "RK_Metal")]
  fn popDebugGroup(&self) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLPixelFormat {
  MTLPixelFormatInvalid = 0,
  MTLPixelFormatA8Unorm = 1,
  MTLPixelFormatR8Unorm = 10,
  MTLPixelFormatR8Unorm_sRGB = 11,
  MTLPixelFormatR8Snorm = 12,
  MTLPixelFormatR8Uint = 13,
  MTLPixelFormatR8Sint = 14,
  MTLPixelFormatR16Unorm = 20,
  MTLPixelFormatR16Snorm = 22,
  MTLPixelFormatR16Uint = 23,
  MTLPixelFormatR16Sint = 24,
  MTLPixelFormatR16Float = 25,
  MTLPixelFormatRG8Unorm = 30,
  MTLPixelFormatRG8Unorm_sRGB = 31,
  MTLPixelFormatRG8Snorm = 32,
  MTLPixelFormatRG8Uint = 33,
  MTLPixelFormatRG8Sint = 34,
  MTLPixelFormatB5G6R5Unorm = 40,
  MTLPixelFormatA1BGR5Unorm = 41,
  MTLPixelFormatABGR4Unorm = 42,
  MTLPixelFormatBGR5A1Unorm = 43,
  MTLPixelFormatR32Uint = 53,
  MTLPixelFormatR32Sint = 54,
  MTLPixelFormatR32Float = 55,
  MTLPixelFormatRG16Unorm = 60,
  MTLPixelFormatRG16Snorm = 62,
  MTLPixelFormatRG16Uint = 63,
  MTLPixelFormatRG16Sint = 64,
  MTLPixelFormatRG16Float = 65,
  MTLPixelFormatRGBA8Unorm = 70,
  MTLPixelFormatRGBA8Unorm_sRGB = 71,
  MTLPixelFormatRGBA8Snorm = 72,
  MTLPixelFormatRGBA8Uint = 73,
  MTLPixelFormatRGBA8Sint = 74,
  MTLPixelFormatBGRA8Unorm = 80,
  MTLPixelFormatBGRA8Unorm_sRGB = 81,
  MTLPixelFormatRGB10A2Unorm = 90,
  MTLPixelFormatRGB10A2Uint = 91,
  MTLPixelFormatRG11B10Float = 92,
  MTLPixelFormatRGB9E5Float = 93,
  MTLPixelFormatBGR10A2Unorm = 94,
  MTLPixelFormatBGR10_XR = 554,
  MTLPixelFormatBGR10_XR_sRGB = 555,
  MTLPixelFormatRG32Uint = 103,
  MTLPixelFormatRG32Sint = 104,
  MTLPixelFormatRG32Float = 105,
  MTLPixelFormatRGBA16Unorm = 110,
  MTLPixelFormatRGBA16Snorm = 112,
  MTLPixelFormatRGBA16Uint = 113,
  MTLPixelFormatRGBA16Sint = 114,
  MTLPixelFormatRGBA16Float = 115,
  MTLPixelFormatBGRA10_XR = 552,
  MTLPixelFormatBGRA10_XR_sRGB = 553,
  MTLPixelFormatRGBA32Uint = 123,
  MTLPixelFormatRGBA32Sint = 124,
  MTLPixelFormatRGBA32Float = 125,
  MTLPixelFormatBC1_RGBA = 130,
  MTLPixelFormatBC1_RGBA_sRGB = 131,
  MTLPixelFormatBC2_RGBA = 132,
  MTLPixelFormatBC2_RGBA_sRGB = 133,
  MTLPixelFormatBC3_RGBA = 134,
  MTLPixelFormatBC3_RGBA_sRGB = 135,
  MTLPixelFormatBC4_RUnorm = 140,
  MTLPixelFormatBC4_RSnorm = 141,
  MTLPixelFormatBC5_RGUnorm = 142,
  MTLPixelFormatBC5_RGSnorm = 143,
  MTLPixelFormatBC6H_RGBFloat = 150,
  MTLPixelFormatBC6H_RGBUfloat = 151,
  MTLPixelFormatBC7_RGBAUnorm = 152,
  MTLPixelFormatBC7_RGBAUnorm_sRGB = 153,
  MTLPixelFormatPVRTC_RGB_2BPP = 160,
  MTLPixelFormatPVRTC_RGB_2BPP_sRGB = 161,
  MTLPixelFormatPVRTC_RGB_4BPP = 162,
  MTLPixelFormatPVRTC_RGB_4BPP_sRGB = 163,
  MTLPixelFormatPVRTC_RGBA_2BPP = 164,
  MTLPixelFormatPVRTC_RGBA_2BPP_sRGB = 165,
  MTLPixelFormatPVRTC_RGBA_4BPP = 166,
  MTLPixelFormatPVRTC_RGBA_4BPP_sRGB = 167,
  MTLPixelFormatEAC_R11Unorm = 170,
  MTLPixelFormatEAC_R11Snorm = 172,
  MTLPixelFormatEAC_RG11Unorm = 174,
  MTLPixelFormatEAC_RG11Snorm = 176,
  MTLPixelFormatEAC_RGBA8 = 178,
  MTLPixelFormatEAC_RGBA8_sRGB = 179,
  MTLPixelFormatETC2_RGB8 = 180,
  MTLPixelFormatETC2_RGB8_sRGB = 181,
  MTLPixelFormatETC2_RGB8A1 = 182,
  MTLPixelFormatETC2_RGB8A1_sRGB = 183,
  MTLPixelFormatASTC_4x4_sRGB = 186,
  MTLPixelFormatASTC_5x4_sRGB = 187,
  MTLPixelFormatASTC_5x5_sRGB = 188,
  MTLPixelFormatASTC_6x5_sRGB = 189,
  MTLPixelFormatASTC_6x6_sRGB = 190,
  MTLPixelFormatASTC_8x5_sRGB = 192,
  MTLPixelFormatASTC_8x6_sRGB = 193,
  MTLPixelFormatASTC_8x8_sRGB = 194,
  MTLPixelFormatASTC_10x5_sRGB = 195,
  MTLPixelFormatASTC_10x6_sRGB = 196,
  MTLPixelFormatASTC_10x8_sRGB = 197,
  MTLPixelFormatASTC_10x10_sRGB = 198,
  MTLPixelFormatASTC_12x10_sRGB = 199,
  MTLPixelFormatASTC_12x12_sRGB = 200,
  MTLPixelFormatASTC_4x4_LDR = 204,
  MTLPixelFormatASTC_5x4_LDR = 205,
  MTLPixelFormatASTC_5x5_LDR = 206,
  MTLPixelFormatASTC_6x5_LDR = 207,
  MTLPixelFormatASTC_6x6_LDR = 208,
  MTLPixelFormatASTC_8x5_LDR = 210,
  MTLPixelFormatASTC_8x6_LDR = 211,
  MTLPixelFormatASTC_8x8_LDR = 212,
  MTLPixelFormatASTC_10x5_LDR = 213,
  MTLPixelFormatASTC_10x6_LDR = 214,
  MTLPixelFormatASTC_10x8_LDR = 215,
  MTLPixelFormatASTC_10x10_LDR = 216,
  MTLPixelFormatASTC_12x10_LDR = 217,
  MTLPixelFormatASTC_12x12_LDR = 218,
  MTLPixelFormatGBGR422 = 240,
  MTLPixelFormatBGRG422 = 241,
  MTLPixelFormatDepth16Unorm = 250,
  MTLPixelFormatDepth32Float = 252,
  MTLPixelFormatStencil8 = 253,
  MTLPixelFormatDepth24Unorm_Stencil8 = 255,
  MTLPixelFormatDepth32Float_Stencil8 = 260,
  MTLPixelFormatX32_Stencil8 = 261,
  MTLPixelFormatX24_Stencil8 = 262,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLPurgeableState {
  MTLPurgeableStateKeepCurrent = 1,
  MTLPurgeableStateNonVolatile = 2,
  MTLPurgeableStateVolatile = 3,
  MTLPurgeableStateEmpty = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLCPUCacheMode {
  MTLCPUCacheModeDefaultCache = 0,
  MTLCPUCacheModeWriteCombined = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLStorageMode {
  MTLStorageModeShared = 0,
  MTLStorageModeManaged = 1,
  MTLStorageModePrivate = 2,
  MTLStorageModeMemoryless = 3,
}
bitflags! { # [ repr ( C ) ] pub struct MTLResourceOptions : usize { const MTLResourceCPUCacheModeDefaultCache = 0 ; const MTLResourceCPUCacheModeWriteCombined = 1 ; const MTLResourceStorageModeManaged = 16 ; const MTLResourceStorageModePrivate = 32 ; const MTLResourceStorageModeMemoryless = 48 ; const MTLResourceHazardTrackingModeUntracked = 256 ; } }
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLResourceProto {
  #[objrs(selector = "setPurgeableState:")]
  #[cfg(feature = "RK_Metal")]
  fn setPurgeableState_(&self, state: MTLPurgeableState) -> MTLPurgeableState;
  #[objrs(selector = "makeAliasable")]
  #[cfg(feature = "RK_Metal")]
  fn makeAliasable(&self) -> ();
  #[objrs(selector = "isAliasable")]
  #[cfg(feature = "RK_Metal")]
  fn isAliasable(&self) -> bool;
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLBufferProto {
  #[objrs(selector = "contents")]
  #[cfg(feature = "RK_Metal")]
  fn contents(&self) -> &c_void;
  #[objrs(selector = "didModifyRange:")]
  #[cfg(feature = "RK_Metal")]
  fn didModifyRange_(&self, range: NSRange) -> ();
  #[objrs(selector = "newTextureWithDescriptor:offset:bytesPerRow:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureWithDescriptor_offset_bytesPerRow_(
    &self,
    descriptor: &MTLTextureDescriptor,
    offset: usize,
    bytesPerRow: usize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "addDebugMarker:range:")]
  #[cfg(feature = "RK_Metal")]
  fn addDebugMarker_range_(&self, marker: &NSString, range: NSRange) -> ();
  #[objrs(selector = "removeAllDebugMarkers")]
  #[cfg(feature = "RK_Metal")]
  fn removeAllDebugMarkers(&self) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTextureType {
  MTLTextureType1D = 0,
  MTLTextureType1DArray = 1,
  MTLTextureType2D = 2,
  MTLTextureType2DArray = 3,
  MTLTextureType2DMultisample = 4,
  MTLTextureTypeCube = 5,
  MTLTextureTypeCubeArray = 6,
  MTLTextureType3D = 7,
  MTLTextureType2DMultisampleArray = 8,
  MTLTextureTypeTextureBuffer = 9,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLSharedTextureHandle;
#[repr(C)]
pub struct MTLSharedTextureHandlePrivate {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct MTLTextureUsage : usize { const MTLTextureUsageUnknown = 0 ; const MTLTextureUsageShaderRead = 1 ; const MTLTextureUsageShaderWrite = 2 ; const MTLTextureUsageRenderTarget = 4 ; const MTLTextureUsagePixelFormatView = 16 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLTextureDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLTextureProto {
  #[objrs(selector = "getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:")]
  #[cfg(feature = "RK_Metal")]
  fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice_(
    &self,
    pixelBytes: &c_void,
    bytesPerRow: usize,
    bytesPerImage: usize,
    region: MTLRegion,
    level: usize,
    slice: usize,
  ) -> ();
  #[objrs(selector = "replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:")]
  #[cfg(feature = "RK_Metal")]
  fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage_(
    &self,
    region: MTLRegion,
    level: usize,
    slice: usize,
    pixelBytes: &c_void,
    bytesPerRow: usize,
    bytesPerImage: usize,
  ) -> ();
  #[objrs(selector = "getBytes:bytesPerRow:fromRegion:mipmapLevel:")]
  #[cfg(feature = "RK_Metal")]
  fn getBytes_bytesPerRow_fromRegion_mipmapLevel_(
    &self,
    pixelBytes: &c_void,
    bytesPerRow: usize,
    region: MTLRegion,
    level: usize,
  ) -> ();
  #[objrs(selector = "replaceRegion:mipmapLevel:withBytes:bytesPerRow:")]
  #[cfg(feature = "RK_Metal")]
  fn replaceRegion_mipmapLevel_withBytes_bytesPerRow_(
    &self,
    region: MTLRegion,
    level: usize,
    pixelBytes: &c_void,
    bytesPerRow: usize,
  ) -> ();
  #[objrs(selector = "newTextureViewWithPixelFormat:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureViewWithPixelFormat_(&self, pixelFormat: MTLPixelFormat) -> Option<Arc<Object>>;
  #[objrs(selector = "newTextureViewWithPixelFormat:textureType:levels:slices:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureViewWithPixelFormat_textureType_levels_slices_(
    &self,
    pixelFormat: MTLPixelFormat,
    textureType: MTLTextureType,
    levelRange: NSRange,
    sliceRange: NSRange,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newSharedTextureHandle")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedTextureHandle(&self) -> Option<Arc<MTLSharedTextureHandle>>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLDataType {
  MTLDataTypeNone = 0,
  MTLDataTypeStruct = 1,
  MTLDataTypeArray = 2,
  MTLDataTypeFloat = 3,
  MTLDataTypeFloat2 = 4,
  MTLDataTypeFloat3 = 5,
  MTLDataTypeFloat4 = 6,
  MTLDataTypeFloat2x2 = 7,
  MTLDataTypeFloat2x3 = 8,
  MTLDataTypeFloat2x4 = 9,
  MTLDataTypeFloat3x2 = 10,
  MTLDataTypeFloat3x3 = 11,
  MTLDataTypeFloat3x4 = 12,
  MTLDataTypeFloat4x2 = 13,
  MTLDataTypeFloat4x3 = 14,
  MTLDataTypeFloat4x4 = 15,
  MTLDataTypeHalf = 16,
  MTLDataTypeHalf2 = 17,
  MTLDataTypeHalf3 = 18,
  MTLDataTypeHalf4 = 19,
  MTLDataTypeHalf2x2 = 20,
  MTLDataTypeHalf2x3 = 21,
  MTLDataTypeHalf2x4 = 22,
  MTLDataTypeHalf3x2 = 23,
  MTLDataTypeHalf3x3 = 24,
  MTLDataTypeHalf3x4 = 25,
  MTLDataTypeHalf4x2 = 26,
  MTLDataTypeHalf4x3 = 27,
  MTLDataTypeHalf4x4 = 28,
  MTLDataTypeInt = 29,
  MTLDataTypeInt2 = 30,
  MTLDataTypeInt3 = 31,
  MTLDataTypeInt4 = 32,
  MTLDataTypeUInt = 33,
  MTLDataTypeUInt2 = 34,
  MTLDataTypeUInt3 = 35,
  MTLDataTypeUInt4 = 36,
  MTLDataTypeShort = 37,
  MTLDataTypeShort2 = 38,
  MTLDataTypeShort3 = 39,
  MTLDataTypeShort4 = 40,
  MTLDataTypeUShort = 41,
  MTLDataTypeUShort2 = 42,
  MTLDataTypeUShort3 = 43,
  MTLDataTypeUShort4 = 44,
  MTLDataTypeChar = 45,
  MTLDataTypeChar2 = 46,
  MTLDataTypeChar3 = 47,
  MTLDataTypeChar4 = 48,
  MTLDataTypeUChar = 49,
  MTLDataTypeUChar2 = 50,
  MTLDataTypeUChar3 = 51,
  MTLDataTypeUChar4 = 52,
  MTLDataTypeBool = 53,
  MTLDataTypeBool2 = 54,
  MTLDataTypeBool3 = 55,
  MTLDataTypeBool4 = 56,
  MTLDataTypeTexture = 58,
  MTLDataTypeSampler = 59,
  MTLDataTypePointer = 60,
  MTLDataTypeRenderPipeline = 78,
  MTLDataTypeIndirectCommandBuffer = 80,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLArgumentType {
  MTLArgumentTypeBuffer = 0,
  MTLArgumentTypeThreadgroupMemory = 1,
  MTLArgumentTypeTexture = 2,
  MTLArgumentTypeSampler = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLArgumentAccess {
  MTLArgumentAccessReadOnly = 0,
  MTLArgumentAccessReadWrite = 1,
  MTLArgumentAccessWriteOnly = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLType;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLStructMember;
# [ objrs ( class , super = MTLType ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLStructType;
# [ objrs ( class , super = MTLType ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLArrayType;
# [ objrs ( class , super = MTLType ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLPointerType;
# [ objrs ( class , super = MTLType ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLTextureReferenceType;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLArgument;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLPatchType {
  MTLPatchTypeNone = 0,
  MTLPatchTypeTriangle = 1,
  MTLPatchTypeQuad = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexAttribute;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLAttribute;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLFunctionType {
  MTLFunctionTypeVertex = 1,
  MTLFunctionTypeFragment = 2,
  MTLFunctionTypeKernel = 3,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLFunctionConstant;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLFunctionProto {
  #[objrs(selector = "newArgumentEncoderWithBufferIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn newArgumentEncoderWithBufferIndex_(&self, bufferIndex: usize) -> Arc<Object>;
  #[objrs(selector = "newArgumentEncoderWithBufferIndex:reflection:")]
  #[cfg(feature = "RK_Metal")]
  fn newArgumentEncoderWithBufferIndex_reflection_(
    &self,
    bufferIndex: usize,
    reflection: Option<&mut &Option<Arc<MTLArgument>>>,
  ) -> Arc<Object>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLLanguageVersion {
  MTLLanguageVersion1_0 = 65536,
  MTLLanguageVersion1_1 = 65537,
  MTLLanguageVersion1_2 = 65538,
  MTLLanguageVersion2_0 = 131072,
  MTLLanguageVersion2_1 = 131073,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLCompileOptions;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLLibraryError {
  MTLLibraryErrorUnsupported = 1,
  MTLLibraryErrorInternal = 2,
  MTLLibraryErrorCompileFailure = 3,
  MTLLibraryErrorCompileWarning = 4,
  MTLLibraryErrorFunctionNotFound = 5,
  MTLLibraryErrorFileNotFound = 6,
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLLibraryProto {
  #[objrs(selector = "newFunctionWithName:")]
  #[cfg(feature = "RK_Metal")]
  fn newFunctionWithName_(&self, functionName: &NSString) -> Option<Arc<Object>>;
  #[objrs(selector = "newFunctionWithName:constantValues:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newFunctionWithName_constantValues_error_(
    &self,
    name: &NSString,
    constantValues: &MTLFunctionConstantValues,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newFunctionWithName:constantValues:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newFunctionWithName_constantValues_completionHandler_(
    &self,
    name: &NSString,
    constantValues: &MTLFunctionConstantValues,
    completionHandler: (),
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLFeatureSet {
  MTLFeatureSet_iOS_GPUFamily1_v1 = 0,
  MTLFeatureSet_iOS_GPUFamily2_v1 = 1,
  MTLFeatureSet_iOS_GPUFamily1_v2 = 2,
  MTLFeatureSet_iOS_GPUFamily2_v2 = 3,
  MTLFeatureSet_iOS_GPUFamily3_v1 = 4,
  MTLFeatureSet_iOS_GPUFamily1_v3 = 5,
  MTLFeatureSet_iOS_GPUFamily2_v3 = 6,
  MTLFeatureSet_iOS_GPUFamily3_v2 = 7,
  MTLFeatureSet_iOS_GPUFamily1_v4 = 8,
  MTLFeatureSet_iOS_GPUFamily2_v4 = 9,
  MTLFeatureSet_iOS_GPUFamily3_v3 = 10,
  MTLFeatureSet_iOS_GPUFamily4_v1 = 11,
  MTLFeatureSet_iOS_GPUFamily1_v5 = 12,
  MTLFeatureSet_iOS_GPUFamily2_v5 = 13,
  MTLFeatureSet_iOS_GPUFamily3_v4 = 14,
  MTLFeatureSet_iOS_GPUFamily4_v2 = 15,
  MTLFeatureSet_macOS_GPUFamily1_v1 = 10000,
  MTLFeatureSet_macOS_GPUFamily1_v2 = 10001,
  MTLFeatureSet_macOS_ReadWriteTextureTier2 = 10002,
  MTLFeatureSet_macOS_GPUFamily1_v3 = 10003,
  MTLFeatureSet_macOS_GPUFamily1_v4 = 10004,
  MTLFeatureSet_macOS_GPUFamily2_v1 = 10005,
  MTLFeatureSet_tvOS_GPUFamily1_v1 = 30000,
  MTLFeatureSet_tvOS_GPUFamily1_v2 = 30001,
  MTLFeatureSet_tvOS_GPUFamily1_v3 = 30002,
  MTLFeatureSet_tvOS_GPUFamily1_v4 = 30004,
}
bitflags! { # [ repr ( C ) ] pub struct MTLPipelineOption : usize { const MTLPipelineOptionNone = 0 ; const MTLPipelineOptionArgumentInfo = 1 ; const MTLPipelineOptionBufferTypeInfo = 2 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLReadWriteTextureTier {
  MTLReadWriteTextureTierNone = 0,
  MTLReadWriteTextureTier1 = 1,
  MTLReadWriteTextureTier2 = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLArgumentBuffersTier {
  MTLArgumentBuffersTier1 = 0,
  MTLArgumentBuffersTier2 = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLSizeAndAlign {
  pub size: usize,
  pub align: usize,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLArgumentDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLDeviceProto {
  #[objrs(selector = "newCommandQueue")]
  #[cfg(feature = "RK_Metal")]
  fn newCommandQueue(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "newCommandQueueWithMaxCommandBufferCount:")]
  #[cfg(feature = "RK_Metal")]
  fn newCommandQueueWithMaxCommandBufferCount_(
    &self,
    maxCommandBufferCount: usize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "heapTextureSizeAndAlignWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn heapTextureSizeAndAlignWithDescriptor_(&self, desc: &MTLTextureDescriptor) -> MTLSizeAndAlign;
  #[objrs(selector = "heapBufferSizeAndAlignWithLength:options:")]
  #[cfg(feature = "RK_Metal")]
  fn heapBufferSizeAndAlignWithLength_options_(
    &self,
    length: usize,
    options: MTLResourceOptions,
  ) -> MTLSizeAndAlign;
  #[objrs(selector = "newHeapWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newHeapWithDescriptor_(&self, descriptor: &MTLHeapDescriptor) -> Option<Arc<Object>>;
  #[objrs(selector = "newBufferWithLength:options:")]
  #[cfg(feature = "RK_Metal")]
  fn newBufferWithLength_options_(
    &self,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newBufferWithBytes:length:options:")]
  #[cfg(feature = "RK_Metal")]
  fn newBufferWithBytes_length_options_(
    &self,
    pointer: &c_void,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newBufferWithBytesNoCopy:length:options:deallocator:")]
  #[cfg(feature = "RK_Metal")]
  fn newBufferWithBytesNoCopy_length_options_deallocator_(
    &self,
    pointer: &c_void,
    length: usize,
    options: MTLResourceOptions,
    deallocator: (),
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newDepthStencilStateWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newDepthStencilStateWithDescriptor_(
    &self,
    descriptor: &MTLDepthStencilDescriptor,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newTextureWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureWithDescriptor_(&self, descriptor: &MTLTextureDescriptor) -> Option<Arc<Object>>;
  #[objrs(selector = "newTextureWithDescriptor:iosurface:plane:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureWithDescriptor_iosurface_plane_(
    &self,
    descriptor: &MTLTextureDescriptor,
    iosurface: &__IOSurface,
    plane: usize,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newSharedTextureWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedTextureWithDescriptor_(
    &self,
    descriptor: &MTLTextureDescriptor,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newSharedTextureWithHandle:")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedTextureWithHandle_(
    &self,
    sharedHandle: &MTLSharedTextureHandle,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newSamplerStateWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newSamplerStateWithDescriptor_(
    &self,
    descriptor: &MTLSamplerDescriptor,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newDefaultLibrary")]
  #[cfg(feature = "RK_Metal")]
  fn newDefaultLibrary(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "newDefaultLibraryWithBundle:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newDefaultLibraryWithBundle_error_(
    &self,
    bundle: &NSBundle,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newLibraryWithFile:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newLibraryWithFile_error_(
    &self,
    filepath: &NSString,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newLibraryWithURL:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newLibraryWithURL_error_(
    &self,
    url: &NSURL,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newLibraryWithData:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newLibraryWithData_error_(
    &self,
    data: &NSObject,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newLibraryWithSource:options:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newLibraryWithSource_options_error_(
    &self,
    source: &NSString,
    options: Option<&MTLCompileOptions>,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newLibraryWithSource:options:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newLibraryWithSource_options_completionHandler_(
    &self,
    source: &NSString,
    options: Option<&MTLCompileOptions>,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newRenderPipelineStateWithDescriptor:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newRenderPipelineStateWithDescriptor_error_(
    &self,
    descriptor: &MTLRenderPipelineDescriptor,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newRenderPipelineStateWithDescriptor:options:reflection:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newRenderPipelineStateWithDescriptor_options_reflection_error_(
    &self,
    descriptor: &MTLRenderPipelineDescriptor,
    options: MTLPipelineOption,
    reflection: Option<&mut &Option<Arc<MTLRenderPipelineReflection>>>,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newRenderPipelineStateWithDescriptor:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newRenderPipelineStateWithDescriptor_completionHandler_(
    &self,
    descriptor: &MTLRenderPipelineDescriptor,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newRenderPipelineStateWithDescriptor:options:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newRenderPipelineStateWithDescriptor_options_completionHandler_(
    &self,
    descriptor: &MTLRenderPipelineDescriptor,
    options: MTLPipelineOption,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newComputePipelineStateWithFunction:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithFunction_error_(
    &self,
    computeFunction: &Object,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newComputePipelineStateWithFunction:options:reflection:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithFunction_options_reflection_error_(
    &self,
    computeFunction: &Object,
    options: MTLPipelineOption,
    reflection: Option<&mut &Option<Arc<MTLComputePipelineReflection>>>,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newComputePipelineStateWithFunction:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithFunction_completionHandler_(
    &self,
    computeFunction: &Object,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newComputePipelineStateWithFunction:options:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithFunction_options_completionHandler_(
    &self,
    computeFunction: &Object,
    options: MTLPipelineOption,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newComputePipelineStateWithDescriptor:options:reflection:error:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithDescriptor_options_reflection_error_(
    &self,
    descriptor: &MTLComputePipelineDescriptor,
    options: MTLPipelineOption,
    reflection: Option<&mut &Option<Arc<MTLComputePipelineReflection>>>,
    error: Option<&mut &Option<Arc<NSError>>>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newComputePipelineStateWithDescriptor:options:completionHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn newComputePipelineStateWithDescriptor_options_completionHandler_(
    &self,
    descriptor: &MTLComputePipelineDescriptor,
    options: MTLPipelineOption,
    completionHandler: (),
  ) -> ();
  #[objrs(selector = "newFence")]
  #[cfg(feature = "RK_Metal")]
  fn newFence(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "supportsFeatureSet:")]
  #[cfg(feature = "RK_Metal")]
  fn supportsFeatureSet_(&self, featureSet: MTLFeatureSet) -> bool;
  #[objrs(selector = "supportsTextureSampleCount:")]
  #[cfg(feature = "RK_Metal")]
  fn supportsTextureSampleCount_(&self, sampleCount: usize) -> bool;
  #[objrs(selector = "minimumLinearTextureAlignmentForPixelFormat:")]
  #[cfg(feature = "RK_Metal")]
  fn minimumLinearTextureAlignmentForPixelFormat_(&self, format: MTLPixelFormat) -> usize;
  #[objrs(selector = "minimumTextureBufferAlignmentForPixelFormat:")]
  #[cfg(feature = "RK_Metal")]
  fn minimumTextureBufferAlignmentForPixelFormat_(&self, format: MTLPixelFormat) -> usize;
  #[objrs(selector = "getDefaultSamplePositions:count:")]
  #[cfg(feature = "RK_Metal")]
  fn getDefaultSamplePositions_count_(&self, positions: &MTLSamplePosition, count: usize) -> ();
  #[objrs(selector = "newArgumentEncoderWithArguments:")]
  #[cfg(feature = "RK_Metal")]
  fn newArgumentEncoderWithArguments_(&self, arguments: &NSArray) -> Option<Arc<Object>>;
  #[objrs(selector = "newIndirectCommandBufferWithDescriptor:maxCommandCount:options:")]
  #[cfg(feature = "RK_Metal")]
  fn newIndirectCommandBufferWithDescriptor_maxCommandCount_options_(
    &self,
    descriptor: &MTLIndirectCommandBufferDescriptor,
    maxCount: usize,
    options: MTLResourceOptions,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newEvent")]
  #[cfg(feature = "RK_Metal")]
  fn newEvent(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "newSharedEvent")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedEvent(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "newSharedEventWithHandle:")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedEventWithHandle_(
    &self,
    sharedEventHandle: &MTLSharedEventHandle,
  ) -> Option<Arc<Object>>;
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLFenceProto {}
bitflags! { # [ repr ( C ) ] pub struct MTLBlitOption : usize { const MTLBlitOptionNone = 0 ; const MTLBlitOptionDepthFromDepthStencil = 1 ; const MTLBlitOptionStencilFromDepthStencil = 2 ; const MTLBlitOptionRowLinearPVRTC = 4 ; } }
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLBlitCommandEncoderProto {
  #[objrs(selector = "synchronizeResource:")]
  #[cfg(feature = "RK_Metal")]
  fn synchronizeResource_(&self, resource: &Object) -> ();
  #[objrs(selector = "synchronizeTexture:slice:level:")]
  #[cfg(feature = "RK_Metal")]
  fn synchronizeTexture_slice_level_(&self, texture: &Object, slice: usize, level: usize) -> ();
  #[objrs(
    selector = "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
    &self,
    sourceTexture: &Object,
    sourceSlice: usize,
    sourceLevel: usize,
    sourceOrigin: MTLOrigin,
    sourceSize: MTLSize,
    destinationTexture: &Object,
    destinationSlice: usize,
    destinationLevel: usize,
    destinationOrigin: MTLOrigin,
  ) -> ();
  #[objrs(
    selector = "copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_(
    &self,
    sourceBuffer: &Object,
    sourceOffset: usize,
    sourceBytesPerRow: usize,
    sourceBytesPerImage: usize,
    sourceSize: MTLSize,
    destinationTexture: &Object,
    destinationSlice: usize,
    destinationLevel: usize,
    destinationOrigin: MTLOrigin,
  ) -> ();
  #[objrs(
    selector = "copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options_(
    &self,
    sourceBuffer: &Object,
    sourceOffset: usize,
    sourceBytesPerRow: usize,
    sourceBytesPerImage: usize,
    sourceSize: MTLSize,
    destinationTexture: &Object,
    destinationSlice: usize,
    destinationLevel: usize,
    destinationOrigin: MTLOrigin,
    options: MTLBlitOption,
  ) -> ();
  #[objrs(
    selector = "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_(
    &self,
    sourceTexture: &Object,
    sourceSlice: usize,
    sourceLevel: usize,
    sourceOrigin: MTLOrigin,
    sourceSize: MTLSize,
    destinationBuffer: &Object,
    destinationOffset: usize,
    destinationBytesPerRow: usize,
    destinationBytesPerImage: usize,
  ) -> ();
  #[objrs(
    selector = "copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toBuffer_destinationOffset_destinationBytesPerRow_destinationBytesPerImage_options_(
    &self,
    sourceTexture: &Object,
    sourceSlice: usize,
    sourceLevel: usize,
    sourceOrigin: MTLOrigin,
    sourceSize: MTLSize,
    destinationBuffer: &Object,
    destinationOffset: usize,
    destinationBytesPerRow: usize,
    destinationBytesPerImage: usize,
    options: MTLBlitOption,
  ) -> ();
  #[objrs(selector = "generateMipmapsForTexture:")]
  #[cfg(feature = "RK_Metal")]
  fn generateMipmapsForTexture_(&self, texture: &Object) -> ();
  #[objrs(selector = "fillBuffer:range:value:")]
  #[cfg(feature = "RK_Metal")]
  fn fillBuffer_range_value_(&self, buffer: &Object, range: NSRange, value: u8) -> ();
  #[objrs(selector = "copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:")]
  #[cfg(feature = "RK_Metal")]
  fn copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size_(
    &self,
    sourceBuffer: &Object,
    sourceOffset: usize,
    destinationBuffer: &Object,
    destinationOffset: usize,
    size: usize,
  ) -> ();
  #[objrs(selector = "updateFence:")]
  #[cfg(feature = "RK_Metal")]
  fn updateFence_(&self, fence: &Object) -> ();
  #[objrs(selector = "waitForFence:")]
  #[cfg(feature = "RK_Metal")]
  fn waitForFence_(&self, fence: &Object) -> ();
  #[objrs(selector = "optimizeContentsForGPUAccess:")]
  #[cfg(feature = "RK_Metal")]
  fn optimizeContentsForGPUAccess_(&self, texture: &Object) -> ();
  #[objrs(selector = "optimizeContentsForGPUAccess:slice:level:")]
  #[cfg(feature = "RK_Metal")]
  fn optimizeContentsForGPUAccess_slice_level_(
    &self,
    texture: &Object,
    slice: usize,
    level: usize,
  ) -> ();
  #[objrs(selector = "optimizeContentsForCPUAccess:")]
  #[cfg(feature = "RK_Metal")]
  fn optimizeContentsForCPUAccess_(&self, texture: &Object) -> ();
  #[objrs(selector = "optimizeContentsForCPUAccess:slice:level:")]
  #[cfg(feature = "RK_Metal")]
  fn optimizeContentsForCPUAccess_slice_level_(
    &self,
    texture: &Object,
    slice: usize,
    level: usize,
  ) -> ();
  #[objrs(selector = "resetCommandsInBuffer:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn resetCommandsInBuffer_withRange_(&self, buffer: &Object, range: NSRange) -> ();
  #[objrs(selector = "copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn copyIndirectCommandBuffer_sourceRange_destination_destinationIndex_(
    &self,
    source: &Object,
    sourceRange: NSRange,
    destination: &Object,
    destinationIndex: usize,
  ) -> ();
  #[objrs(selector = "optimizeIndirectCommandBuffer:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn optimizeIndirectCommandBuffer_withRange_(
    &self,
    indirectCommandBuffer: &Object,
    range: NSRange,
  ) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLCommandBufferStatus {
  MTLCommandBufferStatusNotEnqueued = 0,
  MTLCommandBufferStatusEnqueued = 1,
  MTLCommandBufferStatusCommitted = 2,
  MTLCommandBufferStatusScheduled = 3,
  MTLCommandBufferStatusCompleted = 4,
  MTLCommandBufferStatusError = 5,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLCommandBufferError {
  MTLCommandBufferErrorNone = 0,
  MTLCommandBufferErrorInternal = 1,
  MTLCommandBufferErrorTimeout = 2,
  MTLCommandBufferErrorPageFault = 3,
  MTLCommandBufferErrorBlacklisted = 4,
  MTLCommandBufferErrorNotPermitted = 7,
  MTLCommandBufferErrorOutOfMemory = 8,
  MTLCommandBufferErrorInvalidResource = 9,
  MTLCommandBufferErrorMemoryless = 10,
  MTLCommandBufferErrorDeviceRemoved = 11,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLDispatchType {
  MTLDispatchTypeSerial = 0,
  MTLDispatchTypeConcurrent = 1,
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandBufferProto {
  #[objrs(selector = "enqueue")]
  #[cfg(feature = "RK_Metal")]
  fn enqueue(&self) -> ();
  #[objrs(selector = "commit")]
  #[cfg(feature = "RK_Metal")]
  fn commit(&self) -> ();
  #[objrs(selector = "addScheduledHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn addScheduledHandler_(&self, block: ()) -> ();
  #[objrs(selector = "presentDrawable:")]
  #[cfg(feature = "RK_Metal")]
  fn presentDrawable_(&self, drawable: &Object) -> ();
  #[objrs(selector = "presentDrawable:atTime:")]
  #[cfg(feature = "RK_Metal")]
  fn presentDrawable_atTime_(&self, drawable: &Object, presentationTime: f64) -> ();
  #[objrs(selector = "waitUntilScheduled")]
  #[cfg(feature = "RK_Metal")]
  fn waitUntilScheduled(&self) -> ();
  #[objrs(selector = "addCompletedHandler:")]
  #[cfg(feature = "RK_Metal")]
  fn addCompletedHandler_(&self, block: ()) -> ();
  #[objrs(selector = "waitUntilCompleted")]
  #[cfg(feature = "RK_Metal")]
  fn waitUntilCompleted(&self) -> ();
  #[objrs(selector = "blitCommandEncoder")]
  #[cfg(feature = "RK_Metal")]
  fn blitCommandEncoder(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "renderCommandEncoderWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn renderCommandEncoderWithDescriptor_(
    &self,
    renderPassDescriptor: &MTLRenderPassDescriptor,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "computeCommandEncoder")]
  #[cfg(feature = "RK_Metal")]
  fn computeCommandEncoder(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "computeCommandEncoderWithDispatchType:")]
  #[cfg(feature = "RK_Metal")]
  fn computeCommandEncoderWithDispatchType_(
    &self,
    dispatchType: MTLDispatchType,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "encodeWaitForEvent:value:")]
  #[cfg(feature = "RK_Metal")]
  fn encodeWaitForEvent_value_(&self, event: &Object, value: u64) -> ();
  #[objrs(selector = "encodeSignalEvent:value:")]
  #[cfg(feature = "RK_Metal")]
  fn encodeSignalEvent_value_(&self, event: &Object, value: u64) -> ();
  #[objrs(selector = "parallelRenderCommandEncoderWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn parallelRenderCommandEncoderWithDescriptor_(
    &self,
    renderPassDescriptor: &MTLRenderPassDescriptor,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "pushDebugGroup:")]
  #[cfg(feature = "RK_Metal")]
  fn pushDebugGroup_(&self, string: &NSString) -> ();
  #[objrs(selector = "popDebugGroup")]
  #[cfg(feature = "RK_Metal")]
  fn popDebugGroup(&self) -> ();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
  pub threadgroupsPerGrid: [u32; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLStageInRegionIndirectArguments {
  pub stageInOrigin: [u32; 3],
  pub stageInSize: [u32; 3],
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLComputeCommandEncoderProto {
  #[objrs(selector = "setComputePipelineState:")]
  #[cfg(feature = "RK_Metal")]
  fn setComputePipelineState_(&self, state: &Object) -> ();
  #[objrs(selector = "setBytes:length:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setBytes_length_atIndex_(&self, bytes: &c_void, length: usize, index: usize) -> ();
  #[objrs(selector = "setBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setBuffer_offset_atIndex_(&self, buffer: Option<&Object>, offset: usize, index: usize) -> ();
  #[objrs(selector = "setBufferOffset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setBufferOffset_atIndex_(&self, offset: usize, index: usize) -> ();
  #[objrs(selector = "setBuffers:offsets:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setBuffers_offsets_withRange_(
    &self,
    buffers: &mut &Option<Arc<Object>>,
    offsets: &usize,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setTexture:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setTexture_atIndex_(&self, texture: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setTextures:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setTextures_withRange_(&self, textures: &mut &Option<Arc<Object>>, range: NSRange) -> ();
  #[objrs(selector = "setSamplerState:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerState_atIndex_(&self, sampler: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setSamplerStates:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerStates_withRange_(&self, samplers: &mut &Option<Arc<Object>>, range: NSRange) -> ();
  #[objrs(selector = "setSamplerState:lodMinClamp:lodMaxClamp:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
    &self,
    sampler: Option<&Object>,
    lodMinClamp: f32,
    lodMaxClamp: f32,
    index: usize,
  ) -> ();
  #[objrs(selector = "setSamplerStates:lodMinClamps:lodMaxClamps:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
    &self,
    samplers: &mut &Option<Arc<Object>>,
    lodMinClamps: &f32,
    lodMaxClamps: &f32,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setThreadgroupMemoryLength:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setThreadgroupMemoryLength_atIndex_(&self, length: usize, index: usize) -> ();
  #[objrs(selector = "setStageInRegion:")]
  #[cfg(feature = "RK_Metal")]
  fn setStageInRegion_(&self, region: MTLRegion) -> ();
  #[objrs(selector = "setStageInRegionWithIndirectBuffer:indirectBufferOffset:")]
  #[cfg(feature = "RK_Metal")]
  fn setStageInRegionWithIndirectBuffer_indirectBufferOffset_(
    &self,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(selector = "dispatchThreadgroups:threadsPerThreadgroup:")]
  #[cfg(feature = "RK_Metal")]
  fn dispatchThreadgroups_threadsPerThreadgroup_(
    &self,
    threadgroupsPerGrid: MTLSize,
    threadsPerThreadgroup: MTLSize,
  ) -> ();
  #[objrs(
    selector = "dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn dispatchThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerThreadgroup_(
    &self,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
    threadsPerThreadgroup: MTLSize,
  ) -> ();
  #[objrs(selector = "dispatchThreads:threadsPerThreadgroup:")]
  #[cfg(feature = "RK_Metal")]
  fn dispatchThreads_threadsPerThreadgroup_(
    &self,
    threadsPerGrid: MTLSize,
    threadsPerThreadgroup: MTLSize,
  ) -> ();
  #[objrs(selector = "updateFence:")]
  #[cfg(feature = "RK_Metal")]
  fn updateFence_(&self, fence: &Object) -> ();
  #[objrs(selector = "waitForFence:")]
  #[cfg(feature = "RK_Metal")]
  fn waitForFence_(&self, fence: &Object) -> ();
  #[objrs(selector = "useResource:usage:")]
  #[cfg(feature = "RK_Metal")]
  fn useResource_usage_(&self, resource: &Object, usage: MTLResourceUsage) -> ();
  #[objrs(selector = "useResources:count:usage:")]
  #[cfg(feature = "RK_Metal")]
  fn useResources_count_usage_(
    &self,
    resources: &mut &Arc<Object>,
    count: usize,
    usage: MTLResourceUsage,
  ) -> ();
  #[objrs(selector = "useHeap:")]
  #[cfg(feature = "RK_Metal")]
  fn useHeap_(&self, heap: &Object) -> ();
  #[objrs(selector = "useHeaps:count:")]
  #[cfg(feature = "RK_Metal")]
  fn useHeaps_count_(&self, heaps: &mut &Arc<Object>, count: usize) -> ();
  #[objrs(selector = "memoryBarrierWithScope:")]
  #[cfg(feature = "RK_Metal")]
  fn memoryBarrierWithScope_(&self, scope: MTLBarrierScope) -> ();
  #[objrs(selector = "memoryBarrierWithResources:count:")]
  #[cfg(feature = "RK_Metal")]
  fn memoryBarrierWithResources_count_(&self, resources: &mut &Arc<Object>, count: usize) -> ();
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCommandQueueProto {
  #[objrs(selector = "commandBuffer")]
  #[cfg(feature = "RK_Metal")]
  fn commandBuffer(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "commandBufferWithUnretainedReferences")]
  #[cfg(feature = "RK_Metal")]
  fn commandBufferWithUnretainedReferences(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "insertDebugCaptureBoundary")]
  #[cfg(feature = "RK_Metal")]
  fn insertDebugCaptureBoundary(&self) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLCompareFunction {
  MTLCompareFunctionNever = 0,
  MTLCompareFunctionLess = 1,
  MTLCompareFunctionEqual = 2,
  MTLCompareFunctionLessEqual = 3,
  MTLCompareFunctionGreater = 4,
  MTLCompareFunctionNotEqual = 5,
  MTLCompareFunctionGreaterEqual = 6,
  MTLCompareFunctionAlways = 7,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLStencilOperation {
  MTLStencilOperationKeep = 0,
  MTLStencilOperationZero = 1,
  MTLStencilOperationReplace = 2,
  MTLStencilOperationIncrementClamp = 3,
  MTLStencilOperationDecrementClamp = 4,
  MTLStencilOperationInvert = 5,
  MTLStencilOperationIncrementWrap = 6,
  MTLStencilOperationDecrementWrap = 7,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLStencilDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLDepthStencilDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLDepthStencilStateProto {}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLDrawableProto {
  #[objrs(selector = "present")]
  #[cfg(feature = "RK_Metal")]
  fn present(&self) -> ();
  #[objrs(selector = "presentAtTime:")]
  #[cfg(feature = "RK_Metal")]
  fn presentAtTime_(&self, presentationTime: f64) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLLoadAction {
  MTLLoadActionDontCare = 0,
  MTLLoadActionLoad = 1,
  MTLLoadActionClear = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLStoreAction {
  MTLStoreActionDontCare = 0,
  MTLStoreActionStore = 1,
  MTLStoreActionMultisampleResolve = 2,
  MTLStoreActionStoreAndMultisampleResolve = 3,
  MTLStoreActionUnknown = 4,
  MTLStoreActionCustomSampleDepthStore = 5,
}
bitflags! { # [ repr ( C ) ] pub struct MTLStoreActionOptions : usize { const MTLStoreActionOptionNone = 0 ; const MTLStoreActionOptionCustomSamplePositions = 1 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLClearColor {
  pub red: f64,
  pub green: f64,
  pub blue: f64,
  pub alpha: f64,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassAttachmentDescriptor;
# [ objrs ( class , super = MTLRenderPassAttachmentDescriptor ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassColorAttachmentDescriptor;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLMultisampleDepthResolveFilter {
  MTLMultisampleDepthResolveFilterSample0 = 0,
  MTLMultisampleDepthResolveFilterMin = 1,
  MTLMultisampleDepthResolveFilterMax = 2,
}
# [ objrs ( class , super = MTLRenderPassAttachmentDescriptor ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassDepthAttachmentDescriptor;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLMultisampleStencilResolveFilter {
  MTLMultisampleStencilResolveFilterSample0 = 0,
  MTLMultisampleStencilResolveFilterDepthResolvedSample = 1,
}
# [ objrs ( class , super = MTLRenderPassAttachmentDescriptor ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassStencilAttachmentDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassColorAttachmentDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPassDescriptor;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLAttributeFormat {
  MTLAttributeFormatInvalid = 0,
  MTLAttributeFormatUChar2 = 1,
  MTLAttributeFormatUChar3 = 2,
  MTLAttributeFormatUChar4 = 3,
  MTLAttributeFormatChar2 = 4,
  MTLAttributeFormatChar3 = 5,
  MTLAttributeFormatChar4 = 6,
  MTLAttributeFormatUChar2Normalized = 7,
  MTLAttributeFormatUChar3Normalized = 8,
  MTLAttributeFormatUChar4Normalized = 9,
  MTLAttributeFormatChar2Normalized = 10,
  MTLAttributeFormatChar3Normalized = 11,
  MTLAttributeFormatChar4Normalized = 12,
  MTLAttributeFormatUShort2 = 13,
  MTLAttributeFormatUShort3 = 14,
  MTLAttributeFormatUShort4 = 15,
  MTLAttributeFormatShort2 = 16,
  MTLAttributeFormatShort3 = 17,
  MTLAttributeFormatShort4 = 18,
  MTLAttributeFormatUShort2Normalized = 19,
  MTLAttributeFormatUShort3Normalized = 20,
  MTLAttributeFormatUShort4Normalized = 21,
  MTLAttributeFormatShort2Normalized = 22,
  MTLAttributeFormatShort3Normalized = 23,
  MTLAttributeFormatShort4Normalized = 24,
  MTLAttributeFormatHalf2 = 25,
  MTLAttributeFormatHalf3 = 26,
  MTLAttributeFormatHalf4 = 27,
  MTLAttributeFormatFloat = 28,
  MTLAttributeFormatFloat2 = 29,
  MTLAttributeFormatFloat3 = 30,
  MTLAttributeFormatFloat4 = 31,
  MTLAttributeFormatInt = 32,
  MTLAttributeFormatInt2 = 33,
  MTLAttributeFormatInt3 = 34,
  MTLAttributeFormatInt4 = 35,
  MTLAttributeFormatUInt = 36,
  MTLAttributeFormatUInt2 = 37,
  MTLAttributeFormatUInt3 = 38,
  MTLAttributeFormatUInt4 = 39,
  MTLAttributeFormatInt1010102Normalized = 40,
  MTLAttributeFormatUInt1010102Normalized = 41,
  MTLAttributeFormatUChar4Normalized_BGRA = 42,
  MTLAttributeFormatUChar = 45,
  MTLAttributeFormatChar = 46,
  MTLAttributeFormatUCharNormalized = 47,
  MTLAttributeFormatCharNormalized = 48,
  MTLAttributeFormatUShort = 49,
  MTLAttributeFormatShort = 50,
  MTLAttributeFormatUShortNormalized = 51,
  MTLAttributeFormatShortNormalized = 52,
  MTLAttributeFormatHalf = 53,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLIndexType {
  MTLIndexTypeUInt16 = 0,
  MTLIndexTypeUInt32 = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLStepFunction {
  MTLStepFunctionConstant = 0,
  MTLStepFunctionPerVertex = 1,
  MTLStepFunctionPerInstance = 2,
  MTLStepFunctionPerPatch = 3,
  MTLStepFunctionPerPatchControlPoint = 4,
  MTLStepFunctionThreadPositionInGridX = 5,
  MTLStepFunctionThreadPositionInGridY = 6,
  MTLStepFunctionThreadPositionInGridXIndexed = 7,
  MTLStepFunctionThreadPositionInGridYIndexed = 8,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLBufferLayoutDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLBufferLayoutDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLAttributeDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLAttributeDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLStageInputOutputDescriptor;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLMutability {
  MTLMutabilityDefault = 0,
  MTLMutabilityMutable = 1,
  MTLMutabilityImmutable = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLPipelineBufferDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLPipelineBufferDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLComputePipelineReflection;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLComputePipelineDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLComputePipelineStateProto {}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLPrimitiveType {
  MTLPrimitiveTypePoint = 0,
  MTLPrimitiveTypeLine = 1,
  MTLPrimitiveTypeLineStrip = 2,
  MTLPrimitiveTypeTriangle = 3,
  MTLPrimitiveTypeTriangleStrip = 4,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLVisibilityResultMode {
  MTLVisibilityResultModeDisabled = 0,
  MTLVisibilityResultModeBoolean = 1,
  MTLVisibilityResultModeCounting = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLScissorRect {
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLViewport {
  pub originX: f64,
  pub originY: f64,
  pub width: f64,
  pub height: f64,
  pub znear: f64,
  pub zfar: f64,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLCullMode {
  MTLCullModeNone = 0,
  MTLCullModeFront = 1,
  MTLCullModeBack = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLWinding {
  MTLWindingClockwise = 0,
  MTLWindingCounterClockwise = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLDepthClipMode {
  MTLDepthClipModeClip = 0,
  MTLDepthClipModeClamp = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTriangleFillMode {
  MTLTriangleFillModeFill = 0,
  MTLTriangleFillModeLines = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLDrawPrimitivesIndirectArguments {
  pub vertexCount: u32,
  pub instanceCount: u32,
  pub vertexStart: u32,
  pub baseInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
  pub indexCount: u32,
  pub instanceCount: u32,
  pub indexStart: u32,
  pub baseVertex: i32,
  pub baseInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLDrawPatchIndirectArguments {
  pub patchCount: u32,
  pub instanceCount: u32,
  pub patchStart: u32,
  pub baseInstance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLQuadTessellationFactorsHalf {
  pub edgeTessellationFactor: [u16; 4],
  pub insideTessellationFactor: [u16; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLTriangleTessellationFactorsHalf {
  pub edgeTessellationFactor: [u16; 3],
  pub insideTessellationFactor: u16,
}
bitflags! { # [ repr ( C ) ] pub struct MTLRenderStages : usize { const MTLRenderStageVertex = 1 ; const MTLRenderStageFragment = 2 ; } }
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLRenderCommandEncoderProto {
  #[objrs(selector = "setRenderPipelineState:")]
  #[cfg(feature = "RK_Metal")]
  fn setRenderPipelineState_(&self, pipelineState: &Object) -> ();
  #[objrs(selector = "setVertexBytes:length:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexBytes_length_atIndex_(&self, bytes: &c_void, length: usize, index: usize) -> ();
  #[objrs(selector = "setVertexBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexBuffer_offset_atIndex_(
    &self,
    buffer: Option<&Object>,
    offset: usize,
    index: usize,
  ) -> ();
  #[objrs(selector = "setVertexBufferOffset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexBufferOffset_atIndex_(&self, offset: usize, index: usize) -> ();
  #[objrs(selector = "setVertexBuffers:offsets:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexBuffers_offsets_withRange_(
    &self,
    buffers: &mut &Option<Arc<Object>>,
    offsets: &usize,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setVertexTexture:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexTexture_atIndex_(&self, texture: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setVertexTextures:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexTextures_withRange_(&self, textures: &mut &Option<Arc<Object>>, range: NSRange)
    -> ();
  #[objrs(selector = "setVertexSamplerState:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexSamplerState_atIndex_(&self, sampler: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setVertexSamplerStates:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexSamplerStates_withRange_(
    &self,
    samplers: &mut &Option<Arc<Object>>,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setVertexSamplerState:lodMinClamp:lodMaxClamp:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
    &self,
    sampler: Option<&Object>,
    lodMinClamp: f32,
    lodMaxClamp: f32,
    index: usize,
  ) -> ();
  #[objrs(selector = "setVertexSamplerStates:lodMinClamps:lodMaxClamps:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
    &self,
    samplers: &mut &Option<Arc<Object>>,
    lodMinClamps: &f32,
    lodMaxClamps: &f32,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setViewport:")]
  #[cfg(feature = "RK_Metal")]
  fn setViewport_(&self, viewport: MTLViewport) -> ();
  #[objrs(selector = "setViewports:count:")]
  #[cfg(feature = "RK_Metal")]
  fn setViewports_count_(&self, viewports: &MTLViewport, count: usize) -> ();
  #[objrs(selector = "setFrontFacingWinding:")]
  #[cfg(feature = "RK_Metal")]
  fn setFrontFacingWinding_(&self, frontFacingWinding: MTLWinding) -> ();
  #[objrs(selector = "setCullMode:")]
  #[cfg(feature = "RK_Metal")]
  fn setCullMode_(&self, cullMode: MTLCullMode) -> ();
  #[objrs(selector = "setDepthClipMode:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthClipMode_(&self, depthClipMode: MTLDepthClipMode) -> ();
  #[objrs(selector = "setDepthBias:slopeScale:clamp:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthBias_slopeScale_clamp_(&self, depthBias: f32, slopeScale: f32, clamp: f32) -> ();
  #[objrs(selector = "setScissorRect:")]
  #[cfg(feature = "RK_Metal")]
  fn setScissorRect_(&self, rect: MTLScissorRect) -> ();
  #[objrs(selector = "setScissorRects:count:")]
  #[cfg(feature = "RK_Metal")]
  fn setScissorRects_count_(&self, scissorRects: &MTLScissorRect, count: usize) -> ();
  #[objrs(selector = "setTriangleFillMode:")]
  #[cfg(feature = "RK_Metal")]
  fn setTriangleFillMode_(&self, fillMode: MTLTriangleFillMode) -> ();
  #[objrs(selector = "setFragmentBytes:length:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentBytes_length_atIndex_(&self, bytes: &c_void, length: usize, index: usize) -> ();
  #[objrs(selector = "setFragmentBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentBuffer_offset_atIndex_(
    &self,
    buffer: Option<&Object>,
    offset: usize,
    index: usize,
  ) -> ();
  #[objrs(selector = "setFragmentBufferOffset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentBufferOffset_atIndex_(&self, offset: usize, index: usize) -> ();
  #[objrs(selector = "setFragmentBuffers:offsets:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentBuffers_offsets_withRange_(
    &self,
    buffers: &mut &Option<Arc<Object>>,
    offsets: &usize,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setFragmentTexture:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentTexture_atIndex_(&self, texture: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setFragmentTextures:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentTextures_withRange_(
    &self,
    textures: &mut &Option<Arc<Object>>,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setFragmentSamplerState:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentSamplerState_atIndex_(&self, sampler: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setFragmentSamplerStates:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentSamplerStates_withRange_(
    &self,
    samplers: &mut &Option<Arc<Object>>,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setFragmentSamplerState:lodMinClamp:lodMaxClamp:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentSamplerState_lodMinClamp_lodMaxClamp_atIndex_(
    &self,
    sampler: Option<&Object>,
    lodMinClamp: f32,
    lodMaxClamp: f32,
    index: usize,
  ) -> ();
  #[objrs(selector = "setFragmentSamplerStates:lodMinClamps:lodMaxClamps:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentSamplerStates_lodMinClamps_lodMaxClamps_withRange_(
    &self,
    samplers: &mut &Option<Arc<Object>>,
    lodMinClamps: &f32,
    lodMaxClamps: &f32,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setBlendColorRed:green:blue:alpha:")]
  #[cfg(feature = "RK_Metal")]
  fn setBlendColorRed_green_blue_alpha_(&self, red: f32, green: f32, blue: f32, alpha: f32) -> ();
  #[objrs(selector = "setDepthStencilState:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthStencilState_(&self, depthStencilState: Option<&Object>) -> ();
  #[objrs(selector = "setStencilReferenceValue:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilReferenceValue_(&self, referenceValue: u32) -> ();
  #[objrs(selector = "setStencilFrontReferenceValue:backReferenceValue:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilFrontReferenceValue_backReferenceValue_(
    &self,
    frontReferenceValue: u32,
    backReferenceValue: u32,
  ) -> ();
  #[objrs(selector = "setVisibilityResultMode:offset:")]
  #[cfg(feature = "RK_Metal")]
  fn setVisibilityResultMode_offset_(&self, mode: MTLVisibilityResultMode, offset: usize) -> ();
  #[objrs(selector = "setColorStoreAction:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setColorStoreAction_atIndex_(
    &self,
    storeAction: MTLStoreAction,
    colorAttachmentIndex: usize,
  ) -> ();
  #[objrs(selector = "setDepthStoreAction:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthStoreAction_(&self, storeAction: MTLStoreAction) -> ();
  #[objrs(selector = "setStencilStoreAction:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilStoreAction_(&self, storeAction: MTLStoreAction) -> ();
  #[objrs(selector = "setColorStoreActionOptions:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setColorStoreActionOptions_atIndex_(
    &self,
    storeActionOptions: MTLStoreActionOptions,
    colorAttachmentIndex: usize,
  ) -> ();
  #[objrs(selector = "setDepthStoreActionOptions:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions) -> ();
  #[objrs(selector = "setStencilStoreActionOptions:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions) -> ();
  #[objrs(selector = "drawPrimitives:vertexStart:vertexCount:instanceCount:")]
  #[cfg(feature = "RK_Metal")]
  fn drawPrimitives_vertexStart_vertexCount_instanceCount_(
    &self,
    primitiveType: MTLPrimitiveType,
    vertexStart: usize,
    vertexCount: usize,
    instanceCount: usize,
  ) -> ();
  #[objrs(selector = "drawPrimitives:vertexStart:vertexCount:")]
  #[cfg(feature = "RK_Metal")]
  fn drawPrimitives_vertexStart_vertexCount_(
    &self,
    primitiveType: MTLPrimitiveType,
    vertexStart: usize,
    vertexCount: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_(
    &self,
    primitiveType: MTLPrimitiveType,
    indexCount: usize,
    indexType: MTLIndexType,
    indexBuffer: &Object,
    indexBufferOffset: usize,
    instanceCount: usize,
  ) -> ();
  #[objrs(selector = "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:")]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_(
    &self,
    primitiveType: MTLPrimitiveType,
    indexCount: usize,
    indexType: MTLIndexType,
    indexBuffer: &Object,
    indexBufferOffset: usize,
  ) -> ();
  #[objrs(selector = "drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:")]
  #[cfg(feature = "RK_Metal")]
  fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance_(
    &self,
    primitiveType: MTLPrimitiveType,
    vertexStart: usize,
    vertexCount: usize,
    instanceCount: usize,
    baseInstance: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance_(
    &self,
    primitiveType: MTLPrimitiveType,
    indexCount: usize,
    indexType: MTLIndexType,
    indexBuffer: &Object,
    indexBufferOffset: usize,
    instanceCount: usize,
    baseVertex: isize,
    baseInstance: usize,
  ) -> ();
  #[objrs(selector = "drawPrimitives:indirectBuffer:indirectBufferOffset:")]
  #[cfg(feature = "RK_Metal")]
  fn drawPrimitives_indirectBuffer_indirectBufferOffset_(
    &self,
    primitiveType: MTLPrimitiveType,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPrimitives:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset_(
    &self,
    primitiveType: MTLPrimitiveType,
    indexType: MTLIndexType,
    indexBuffer: &Object,
    indexBufferOffset: usize,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(selector = "textureBarrier")]
  #[cfg(feature = "RK_Metal")]
  fn textureBarrier(&self) -> ();
  #[objrs(selector = "updateFence:afterStages:")]
  #[cfg(feature = "RK_Metal")]
  fn updateFence_afterStages_(&self, fence: &Object, stages: MTLRenderStages) -> ();
  #[objrs(selector = "waitForFence:beforeStages:")]
  #[cfg(feature = "RK_Metal")]
  fn waitForFence_beforeStages_(&self, fence: &Object, stages: MTLRenderStages) -> ();
  #[objrs(selector = "setTessellationFactorBuffer:offset:instanceStride:")]
  #[cfg(feature = "RK_Metal")]
  fn setTessellationFactorBuffer_offset_instanceStride_(
    &self,
    buffer: Option<&Object>,
    offset: usize,
    instanceStride: usize,
  ) -> ();
  #[objrs(selector = "setTessellationFactorScale:")]
  #[cfg(feature = "RK_Metal")]
  fn setTessellationFactorScale_(&self, scale: f32) -> ();
  #[objrs(
    selector = "drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_(
    &self,
    numberOfPatchControlPoints: usize,
    patchStart: usize,
    patchCount: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    instanceCount: usize,
    baseInstance: usize,
  ) -> ();
  #[objrs(
    selector = "drawPatches:patchIndexBuffer:patchIndexBufferOffset:indirectBuffer:indirectBufferOffset:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawPatches_patchIndexBuffer_patchIndexBufferOffset_indirectBuffer_indirectBufferOffset_(
    &self,
    numberOfPatchControlPoints: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_(
    &self,
    numberOfPatchControlPoints: usize,
    patchStart: usize,
    patchCount: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    controlPointIndexBuffer: &Object,
    controlPointIndexBufferOffset: usize,
    instanceCount: usize,
    baseInstance: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPatches:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPatches_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_indirectBuffer_indirectBufferOffset_(
    &self,
    numberOfPatchControlPoints: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    controlPointIndexBuffer: &Object,
    controlPointIndexBufferOffset: usize,
    indirectBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(selector = "useResource:usage:")]
  #[cfg(feature = "RK_Metal")]
  fn useResource_usage_(&self, resource: &Object, usage: MTLResourceUsage) -> ();
  #[objrs(selector = "useResources:count:usage:")]
  #[cfg(feature = "RK_Metal")]
  fn useResources_count_usage_(
    &self,
    resources: &mut &Arc<Object>,
    count: usize,
    usage: MTLResourceUsage,
  ) -> ();
  #[objrs(selector = "useHeap:")]
  #[cfg(feature = "RK_Metal")]
  fn useHeap_(&self, heap: &Object) -> ();
  #[objrs(selector = "useHeaps:count:")]
  #[cfg(feature = "RK_Metal")]
  fn useHeaps_count_(&self, heaps: &mut &Arc<Object>, count: usize) -> ();
  #[objrs(selector = "executeCommandsInBuffer:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn executeCommandsInBuffer_withRange_(
    &self,
    indirectCommandBuffer: &Object,
    executionRange: NSRange,
  ) -> ();
  #[objrs(selector = "executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:")]
  #[cfg(feature = "RK_Metal")]
  fn executeCommandsInBuffer_indirectBuffer_indirectBufferOffset_(
    &self,
    indirectCommandbuffer: &Object,
    indirectRangeBuffer: &Object,
    indirectBufferOffset: usize,
  ) -> ();
  #[objrs(selector = "memoryBarrierWithScope:afterStages:beforeStages:")]
  #[cfg(feature = "RK_Metal")]
  fn memoryBarrierWithScope_afterStages_beforeStages_(
    &self,
    scope: MTLBarrierScope,
    after: MTLRenderStages,
    before: MTLRenderStages,
  ) -> ();
  #[objrs(selector = "memoryBarrierWithResources:count:afterStages:beforeStages:")]
  #[cfg(feature = "RK_Metal")]
  fn memoryBarrierWithResources_count_afterStages_beforeStages_(
    &self,
    resources: &mut &Arc<Object>,
    count: usize,
    after: MTLRenderStages,
    before: MTLRenderStages,
  ) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLFunctionConstantValues;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLBlendFactor {
  MTLBlendFactorZero = 0,
  MTLBlendFactorOne = 1,
  MTLBlendFactorSourceColor = 2,
  MTLBlendFactorOneMinusSourceColor = 3,
  MTLBlendFactorSourceAlpha = 4,
  MTLBlendFactorOneMinusSourceAlpha = 5,
  MTLBlendFactorDestinationColor = 6,
  MTLBlendFactorOneMinusDestinationColor = 7,
  MTLBlendFactorDestinationAlpha = 8,
  MTLBlendFactorOneMinusDestinationAlpha = 9,
  MTLBlendFactorSourceAlphaSaturated = 10,
  MTLBlendFactorBlendColor = 11,
  MTLBlendFactorOneMinusBlendColor = 12,
  MTLBlendFactorBlendAlpha = 13,
  MTLBlendFactorOneMinusBlendAlpha = 14,
  MTLBlendFactorSource1Color = 15,
  MTLBlendFactorOneMinusSource1Color = 16,
  MTLBlendFactorSource1Alpha = 17,
  MTLBlendFactorOneMinusSource1Alpha = 18,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLBlendOperation {
  MTLBlendOperationAdd = 0,
  MTLBlendOperationSubtract = 1,
  MTLBlendOperationReverseSubtract = 2,
  MTLBlendOperationMin = 3,
  MTLBlendOperationMax = 4,
}
bitflags! { # [ repr ( C ) ] pub struct MTLColorWriteMask : usize { const MTLColorWriteMaskNone = 0 ; const MTLColorWriteMaskRed = 8 ; const MTLColorWriteMaskGreen = 4 ; const MTLColorWriteMaskBlue = 2 ; const MTLColorWriteMaskAlpha = 1 ; const MTLColorWriteMaskAll = 15 ; } }
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLPrimitiveTopologyClass {
  MTLPrimitiveTopologyClassUnspecified = 0,
  MTLPrimitiveTopologyClassPoint = 1,
  MTLPrimitiveTopologyClassLine = 2,
  MTLPrimitiveTopologyClassTriangle = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTessellationPartitionMode {
  MTLTessellationPartitionModePow2 = 0,
  MTLTessellationPartitionModeInteger = 1,
  MTLTessellationPartitionModeFractionalOdd = 2,
  MTLTessellationPartitionModeFractionalEven = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTessellationFactorStepFunction {
  MTLTessellationFactorStepFunctionConstant = 0,
  MTLTessellationFactorStepFunctionPerPatch = 1,
  MTLTessellationFactorStepFunctionPerInstance = 2,
  MTLTessellationFactorStepFunctionPerPatchAndPerInstance = 3,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTessellationFactorFormat {
  MTLTessellationFactorFormatHalf = 0,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLTessellationControlPointIndexType {
  MTLTessellationControlPointIndexTypeNone = 0,
  MTLTessellationControlPointIndexTypeUInt16 = 1,
  MTLTessellationControlPointIndexTypeUInt32 = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineColorAttachmentDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineReflection;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLRenderPipelineStateProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLRenderPipelineColorAttachmentDescriptorArray;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLVertexFormat {
  MTLVertexFormatInvalid = 0,
  MTLVertexFormatUChar2 = 1,
  MTLVertexFormatUChar3 = 2,
  MTLVertexFormatUChar4 = 3,
  MTLVertexFormatChar2 = 4,
  MTLVertexFormatChar3 = 5,
  MTLVertexFormatChar4 = 6,
  MTLVertexFormatUChar2Normalized = 7,
  MTLVertexFormatUChar3Normalized = 8,
  MTLVertexFormatUChar4Normalized = 9,
  MTLVertexFormatChar2Normalized = 10,
  MTLVertexFormatChar3Normalized = 11,
  MTLVertexFormatChar4Normalized = 12,
  MTLVertexFormatUShort2 = 13,
  MTLVertexFormatUShort3 = 14,
  MTLVertexFormatUShort4 = 15,
  MTLVertexFormatShort2 = 16,
  MTLVertexFormatShort3 = 17,
  MTLVertexFormatShort4 = 18,
  MTLVertexFormatUShort2Normalized = 19,
  MTLVertexFormatUShort3Normalized = 20,
  MTLVertexFormatUShort4Normalized = 21,
  MTLVertexFormatShort2Normalized = 22,
  MTLVertexFormatShort3Normalized = 23,
  MTLVertexFormatShort4Normalized = 24,
  MTLVertexFormatHalf2 = 25,
  MTLVertexFormatHalf3 = 26,
  MTLVertexFormatHalf4 = 27,
  MTLVertexFormatFloat = 28,
  MTLVertexFormatFloat2 = 29,
  MTLVertexFormatFloat3 = 30,
  MTLVertexFormatFloat4 = 31,
  MTLVertexFormatInt = 32,
  MTLVertexFormatInt2 = 33,
  MTLVertexFormatInt3 = 34,
  MTLVertexFormatInt4 = 35,
  MTLVertexFormatUInt = 36,
  MTLVertexFormatUInt2 = 37,
  MTLVertexFormatUInt3 = 38,
  MTLVertexFormatUInt4 = 39,
  MTLVertexFormatInt1010102Normalized = 40,
  MTLVertexFormatUInt1010102Normalized = 41,
  MTLVertexFormatUChar4Normalized_BGRA = 42,
  MTLVertexFormatUChar = 45,
  MTLVertexFormatChar = 46,
  MTLVertexFormatUCharNormalized = 47,
  MTLVertexFormatCharNormalized = 48,
  MTLVertexFormatUShort = 49,
  MTLVertexFormatShort = 50,
  MTLVertexFormatUShortNormalized = 51,
  MTLVertexFormatShortNormalized = 52,
  MTLVertexFormatHalf = 53,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLVertexStepFunction {
  MTLVertexStepFunctionConstant = 0,
  MTLVertexStepFunctionPerVertex = 1,
  MTLVertexStepFunctionPerInstance = 2,
  MTLVertexStepFunctionPerPatch = 3,
  MTLVertexStepFunctionPerPatchControlPoint = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexBufferLayoutDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexBufferLayoutDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexAttributeDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexAttributeDescriptorArray;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLVertexDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLParallelRenderCommandEncoderProto {
  #[objrs(selector = "renderCommandEncoder")]
  #[cfg(feature = "RK_Metal")]
  fn renderCommandEncoder(&self) -> Option<Arc<Object>>;
  #[objrs(selector = "setColorStoreAction:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setColorStoreAction_atIndex_(
    &self,
    storeAction: MTLStoreAction,
    colorAttachmentIndex: usize,
  ) -> ();
  #[objrs(selector = "setDepthStoreAction:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthStoreAction_(&self, storeAction: MTLStoreAction) -> ();
  #[objrs(selector = "setStencilStoreAction:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilStoreAction_(&self, storeAction: MTLStoreAction) -> ();
  #[objrs(selector = "setColorStoreActionOptions:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setColorStoreActionOptions_atIndex_(
    &self,
    storeActionOptions: MTLStoreActionOptions,
    colorAttachmentIndex: usize,
  ) -> ();
  #[objrs(selector = "setDepthStoreActionOptions:")]
  #[cfg(feature = "RK_Metal")]
  fn setDepthStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions) -> ();
  #[objrs(selector = "setStencilStoreActionOptions:")]
  #[cfg(feature = "RK_Metal")]
  fn setStencilStoreActionOptions_(&self, storeActionOptions: MTLStoreActionOptions) -> ();
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLSamplerMinMagFilter {
  MTLSamplerMinMagFilterNearest = 0,
  MTLSamplerMinMagFilterLinear = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLSamplerMipFilter {
  MTLSamplerMipFilterNotMipmapped = 0,
  MTLSamplerMipFilterNearest = 1,
  MTLSamplerMipFilterLinear = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLSamplerAddressMode {
  MTLSamplerAddressModeClampToEdge = 0,
  MTLSamplerAddressModeMirrorClampToEdge = 1,
  MTLSamplerAddressModeRepeat = 2,
  MTLSamplerAddressModeMirrorRepeat = 3,
  MTLSamplerAddressModeClampToZero = 4,
  MTLSamplerAddressModeClampToBorderColor = 5,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MTLSamplerBorderColor {
  MTLSamplerBorderColorTransparentBlack = 0,
  MTLSamplerBorderColorOpaqueBlack = 1,
  MTLSamplerBorderColorOpaqueWhite = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLSamplerDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLSamplerStateProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLHeapDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLHeapProto {
  #[objrs(selector = "maxAvailableSizeWithAlignment:")]
  #[cfg(feature = "RK_Metal")]
  fn maxAvailableSizeWithAlignment_(&self, alignment: usize) -> usize;
  #[objrs(selector = "newBufferWithLength:options:")]
  #[cfg(feature = "RK_Metal")]
  fn newBufferWithLength_options_(
    &self,
    length: usize,
    options: MTLResourceOptions,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newTextureWithDescriptor:")]
  #[cfg(feature = "RK_Metal")]
  fn newTextureWithDescriptor_(&self, desc: &MTLTextureDescriptor) -> Option<Arc<Object>>;
  #[objrs(selector = "setPurgeableState:")]
  #[cfg(feature = "RK_Metal")]
  fn setPurgeableState_(&self, state: MTLPurgeableState) -> MTLPurgeableState;
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLArgumentEncoderProto {
  #[objrs(selector = "setArgumentBuffer:offset:")]
  #[cfg(feature = "RK_Metal")]
  fn setArgumentBuffer_offset_(&self, argumentBuffer: Option<&Object>, offset: usize) -> ();
  #[objrs(selector = "setArgumentBuffer:startOffset:arrayElement:")]
  #[cfg(feature = "RK_Metal")]
  fn setArgumentBuffer_startOffset_arrayElement_(
    &self,
    argumentBuffer: Option<&Object>,
    startOffset: usize,
    arrayElement: usize,
  ) -> ();
  #[objrs(selector = "setBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setBuffer_offset_atIndex_(&self, buffer: Option<&Object>, offset: usize, index: usize) -> ();
  #[objrs(selector = "setBuffers:offsets:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setBuffers_offsets_withRange_(
    &self,
    buffers: &mut &Option<Arc<Object>>,
    offsets: &usize,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setTexture:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setTexture_atIndex_(&self, texture: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setTextures:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setTextures_withRange_(&self, textures: &mut &Option<Arc<Object>>, range: NSRange) -> ();
  #[objrs(selector = "setSamplerState:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerState_atIndex_(&self, sampler: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setSamplerStates:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setSamplerStates_withRange_(&self, samplers: &mut &Option<Arc<Object>>, range: NSRange) -> ();
  #[objrs(selector = "constantDataAtIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn constantDataAtIndex_(&self, index: usize) -> *mut c_void;
  #[objrs(selector = "setRenderPipelineState:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setRenderPipelineState_atIndex_(&self, pipeline: Option<&Object>, index: usize) -> ();
  #[objrs(selector = "setRenderPipelineStates:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setRenderPipelineStates_withRange_(
    &self,
    pipelines: &mut &Option<Arc<Object>>,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "setIndirectCommandBuffer:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setIndirectCommandBuffer_atIndex_(
    &self,
    indirectCommandBuffer: Option<&Object>,
    index: usize,
  ) -> ();
  #[objrs(selector = "setIndirectCommandBuffers:withRange:")]
  #[cfg(feature = "RK_Metal")]
  fn setIndirectCommandBuffers_withRange_(
    &self,
    buffers: &mut &Option<Arc<Object>>,
    range: NSRange,
  ) -> ();
  #[objrs(selector = "newArgumentEncoderForBufferAtIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn newArgumentEncoderForBufferAtIndex_(&self, index: usize) -> Option<Arc<Object>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLCaptureManager;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLCaptureScopeProto {
  #[objrs(selector = "beginScope")]
  #[cfg(feature = "RK_Metal")]
  fn beginScope(&self) -> ();
  #[objrs(selector = "endScope")]
  #[cfg(feature = "RK_Metal")]
  fn endScope(&self) -> ();
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLIndirectRenderCommandProto {
  #[objrs(selector = "setRenderPipelineState:")]
  #[cfg(feature = "RK_Metal")]
  fn setRenderPipelineState_(&self, pipelineState: &Object) -> ();
  #[objrs(selector = "setVertexBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setVertexBuffer_offset_atIndex_(&self, buffer: &Object, offset: usize, index: usize) -> ();
  #[objrs(selector = "setFragmentBuffer:offset:atIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn setFragmentBuffer_offset_atIndex_(&self, buffer: &Object, offset: usize, index: usize) -> ();
  #[objrs(
    selector = "drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride_(
    &self,
    numberOfPatchControlPoints: usize,
    patchStart: usize,
    patchCount: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    instanceCount: usize,
    baseInstance: usize,
    buffer: &Object,
    offset: usize,
    instanceStride: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPatches_patchStart_patchCount_patchIndexBuffer_patchIndexBufferOffset_controlPointIndexBuffer_controlPointIndexBufferOffset_instanceCount_baseInstance_tessellationFactorBuffer_tessellationFactorBufferOffset_tessellationFactorBufferInstanceStride_(
    &self,
    numberOfPatchControlPoints: usize,
    patchStart: usize,
    patchCount: usize,
    patchIndexBuffer: Option<&Object>,
    patchIndexBufferOffset: usize,
    controlPointIndexBuffer: &Object,
    controlPointIndexBufferOffset: usize,
    instanceCount: usize,
    baseInstance: usize,
    buffer: &Object,
    offset: usize,
    instanceStride: usize,
  ) -> ();
  #[objrs(selector = "drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:")]
  #[cfg(feature = "RK_Metal")]
  fn drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance_(
    &self,
    primitiveType: MTLPrimitiveType,
    vertexStart: usize,
    vertexCount: usize,
    instanceCount: usize,
    baseInstance: usize,
  ) -> ();
  #[objrs(
    selector = "drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:"
  )]
  #[cfg(feature = "RK_Metal")]
  fn drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance_(
    &self,
    primitiveType: MTLPrimitiveType,
    indexCount: usize,
    indexType: MTLIndexType,
    indexBuffer: &Object,
    indexBufferOffset: usize,
    instanceCount: usize,
    baseVertex: isize,
    baseInstance: usize,
  ) -> ();
  #[objrs(selector = "reset")]
  #[cfg(feature = "RK_Metal")]
  fn reset(&self) -> ();
}
bitflags! { # [ repr ( C ) ] pub struct MTLIndirectCommandType : usize { const MTLIndirectCommandTypeDraw = 1 ; const MTLIndirectCommandTypeDrawIndexed = 2 ; const MTLIndirectCommandTypeDrawPatches = 4 ; const MTLIndirectCommandTypeDrawIndexedPatches = 8 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MTLIndirectCommandBufferExecutionRange {
  pub location: u32,
  pub length: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLIndirectCommandBufferDescriptor;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLIndirectCommandBufferProto {
  #[objrs(selector = "resetWithRange:")]
  #[cfg(feature = "RK_Metal")]
  fn resetWithRange_(&self, range: NSRange) -> ();
  #[objrs(selector = "indirectRenderCommandAtIndex:")]
  #[cfg(feature = "RK_Metal")]
  fn indirectRenderCommandAtIndex_(&self, commandIndex: usize) -> Arc<Object>;
}
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLEventProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLSharedEventListener;
#[objrs(protocol)]
#[link(name = "Metal", kind = "framework")]
pub trait MTLSharedEventProto {
  #[objrs(selector = "notifyListener:atValue:block:")]
  #[cfg(feature = "RK_Metal")]
  fn notifyListener_atValue_block_(
    &self,
    listener: &MTLSharedEventListener,
    value: u64,
    block: (),
  ) -> ();
  #[objrs(selector = "newSharedEventHandle")]
  #[cfg(feature = "RK_Metal")]
  fn newSharedEventHandle(&self) -> Arc<MTLSharedEventHandle>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "Metal", kind = "framework")]
pub struct MTLSharedEventHandle;
#[repr(C)]
pub struct MTLSharedEventHandlePrivate {
  opaque: u32,
}
#[cfg(feature = "RK_Metal")]
#[link(name = "Metal", kind = "framework")]
extern "C" {
  pub fn MTLOriginMake(x: usize, y: usize, z: usize) -> MTLOrigin;
  pub fn MTLSizeMake(width: usize, height: usize, depth: usize) -> MTLSize;
  pub fn MTLRegionMake1D(x: usize, width: usize) -> MTLRegion;
  pub fn MTLRegionMake2D(x: usize, y: usize, width: usize, height: usize) -> MTLRegion;
  pub fn MTLRegionMake3D(
    x: usize,
    y: usize,
    z: usize,
    width: usize,
    height: usize,
    depth: usize,
  ) -> MTLRegion;
  pub fn MTLSamplePositionMake(x: f32, y: f32) -> MTLSamplePosition;
  pub fn MTLCreateSystemDefaultDevice() -> *mut Object;
  pub fn MTLCopyAllDevices() -> *mut NSArray;
  pub fn MTLCopyAllDevicesWithObserver(observer: *mut *mut Object, handler: ()) -> *mut NSArray;
  pub fn MTLRemoveDeviceObserver(observer: *mut Object) -> ();
  pub fn MTLClearColorMake(red: f64, green: f64, blue: f64, alpha: f64) -> MTLClearColor;
  pub fn MTLIndirectCommandBufferExecutionRangeMake(
    location: u32,
    length: u32,
  ) -> MTLIndirectCommandBufferExecutionRange;
}
