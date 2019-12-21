#[allow(unused_imports)]
use crate::objc::*;
use crate::types::simd_double4x4;
use crate::types::simd_float4x4;
use crate::types::simd_quatd;
use crate::types::simd_quatf;
use crate::CoreFoundation::__CFString;
use crate::CoreGraphics::CGColor;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGImage;
use crate::Foundation::NSArray;
use crate::Foundation::NSBundle;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSData;
use crate::Foundation::NSError;
use crate::Foundation::NSFastEnumerationProto;
use crate::Foundation::NSMutableArray;
use crate::Foundation::NSNumber;
use crate::Foundation::NSString;
use crate::Foundation::NSURL;
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
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLAssetResolverProto {
  #[objrs(selector = "canResolveAssetNamed:")]
  #[cfg(feature = "RK_ModelIO")]
  fn canResolveAssetNamed_(&self, name: &NSString) -> bool;
  #[objrs(selector = "resolveAssetNamed:")]
  #[cfg(feature = "RK_ModelIO")]
  fn resolveAssetNamed_(&self, name: &NSString) -> Arc<NSURL>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLRelativeAssetResolver;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLPathAssetResolver;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLBundleAssetResolver;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLIndexBitDepth {
  MDLIndexBitDepthInvalid = 0,
  MDLIndexBitDepthUInt8 = 8,
  MDLIndexBitDepthUInt16 = 16,
  MDLIndexBitDepthUInt32 = 32,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum MDLGeometryType {
  MDLGeometryTypePoints = 0,
  MDLGeometryTypeLines = 1,
  MDLGeometryTypeTriangles = 2,
  MDLGeometryTypeTriangleStrips = 3,
  MDLGeometryTypeQuads = 4,
  MDLGeometryTypeVariableTopology = 5,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum MDLProbePlacement {
  MDLProbePlacementUniformGrid = 0,
  MDLProbePlacementIrradianceDistribution = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLDataPrecision {
  MDLDataPrecisionUndefined = 0,
  MDLDataPrecisionFloat = 1,
  MDLDataPrecisionDouble = 2,
}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLNamedProto {}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLComponentProto {}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLObjectContainerComponentProto {
  #[objrs(selector = "addObject:")]
  #[cfg(feature = "RK_ModelIO")]
  fn addObject_(&self, object: &MDLObject) -> ();
  #[objrs(selector = "removeObject:")]
  #[cfg(feature = "RK_ModelIO")]
  fn removeObject_(&self, object: &MDLObject) -> ();
  #[objrs(selector = "objectAtIndexedSubscript:")]
  #[cfg(feature = "RK_ModelIO")]
  fn objectAtIndexedSubscript_(&self, index: usize) -> Arc<MDLObject>;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MDLAxisAlignedBoundingBox {
  pub maxBounds: (),
  pub minBounds: (),
}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLTransformComponentProto {
  #[objrs(selector = "setLocalTransform:forTime:")]
  #[cfg(feature = "RK_ModelIO")]
  fn setLocalTransform_forTime_(&self, transform: simd_float4x4, time: f64) -> ();
  #[objrs(selector = "setLocalTransform:")]
  #[cfg(feature = "RK_ModelIO")]
  fn setLocalTransform_(&self, transform: simd_float4x4) -> ();
  #[objrs(selector = "localTransformAtTime:")]
  #[cfg(feature = "RK_ModelIO")]
  fn localTransformAtTime_(&self, time: f64) -> simd_float4x4;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransform;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLObject;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLObjectContainer;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMeshBufferType {
  MDLMeshBufferTypeVertex = 1,
  MDLMeshBufferTypeIndex = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMeshBufferMap;
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLMeshBufferProto {
  #[objrs(selector = "fillData:offset:")]
  #[cfg(feature = "RK_ModelIO")]
  fn fillData_offset_(&self, data: &NSData, offset: usize) -> ();
  #[objrs(selector = "map")]
  #[cfg(feature = "RK_ModelIO")]
  fn map(&self) -> Arc<MDLMeshBufferMap>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMeshBufferData;
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLMeshBufferZoneProto {}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLMeshBufferAllocatorProto {
  #[objrs(selector = "newZone:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newZone_(&self, capacity: usize) -> Arc<Object>;
  #[objrs(selector = "newZoneForBuffersWithSize:andType:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newZoneForBuffersWithSize_andType_(&self, sizes: &NSArray, types: &NSArray) -> Arc<Object>;
  #[objrs(selector = "newBuffer:type:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newBuffer_type_(&self, length: usize, type_: MDLMeshBufferType) -> Arc<Object>;
  #[objrs(selector = "newBufferWithData:type:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newBufferWithData_type_(&self, data: &NSData, type_: MDLMeshBufferType) -> Arc<Object>;
  #[objrs(selector = "newBufferFromZone:length:type:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newBufferFromZone_length_type_(
    &self,
    zone: Option<&Object>,
    length: usize,
    type_: MDLMeshBufferType,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "newBufferFromZone:data:type:")]
  #[cfg(feature = "RK_ModelIO")]
  fn newBufferFromZone_data_type_(
    &self,
    zone: Option<&Object>,
    data: &NSData,
    type_: MDLMeshBufferType,
  ) -> Option<Arc<Object>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMeshBufferDataAllocator;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMeshBufferZoneDefault;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLVertexFormat {
  MDLVertexFormatInvalid = 0,
  MDLVertexFormatPackedBit = 4096,
  MDLVertexFormatUCharBits = 65536,
  MDLVertexFormatCharBits = 131072,
  MDLVertexFormatUCharNormalizedBits = 196608,
  MDLVertexFormatCharNormalizedBits = 262144,
  MDLVertexFormatUShortBits = 327680,
  MDLVertexFormatShortBits = 393216,
  MDLVertexFormatUShortNormalizedBits = 458752,
  MDLVertexFormatShortNormalizedBits = 524288,
  MDLVertexFormatUIntBits = 589824,
  MDLVertexFormatIntBits = 655360,
  MDLVertexFormatHalfBits = 720896,
  MDLVertexFormatFloatBits = 786432,
  MDLVertexFormatUChar = 65537,
  MDLVertexFormatUChar2 = 65538,
  MDLVertexFormatUChar3 = 65539,
  MDLVertexFormatUChar4 = 65540,
  MDLVertexFormatChar = 131073,
  MDLVertexFormatChar2 = 131074,
  MDLVertexFormatChar3 = 131075,
  MDLVertexFormatChar4 = 131076,
  MDLVertexFormatUCharNormalized = 196609,
  MDLVertexFormatUChar2Normalized = 196610,
  MDLVertexFormatUChar3Normalized = 196611,
  MDLVertexFormatUChar4Normalized = 196612,
  MDLVertexFormatCharNormalized = 262145,
  MDLVertexFormatChar2Normalized = 262146,
  MDLVertexFormatChar3Normalized = 262147,
  MDLVertexFormatChar4Normalized = 262148,
  MDLVertexFormatUShort = 327681,
  MDLVertexFormatUShort2 = 327682,
  MDLVertexFormatUShort3 = 327683,
  MDLVertexFormatUShort4 = 327684,
  MDLVertexFormatShort = 393217,
  MDLVertexFormatShort2 = 393218,
  MDLVertexFormatShort3 = 393219,
  MDLVertexFormatShort4 = 393220,
  MDLVertexFormatUShortNormalized = 458753,
  MDLVertexFormatUShort2Normalized = 458754,
  MDLVertexFormatUShort3Normalized = 458755,
  MDLVertexFormatUShort4Normalized = 458756,
  MDLVertexFormatShortNormalized = 524289,
  MDLVertexFormatShort2Normalized = 524290,
  MDLVertexFormatShort3Normalized = 524291,
  MDLVertexFormatShort4Normalized = 524292,
  MDLVertexFormatUInt = 589825,
  MDLVertexFormatUInt2 = 589826,
  MDLVertexFormatUInt3 = 589827,
  MDLVertexFormatUInt4 = 589828,
  MDLVertexFormatInt = 655361,
  MDLVertexFormatInt2 = 655362,
  MDLVertexFormatInt3 = 655363,
  MDLVertexFormatInt4 = 655364,
  MDLVertexFormatHalf = 720897,
  MDLVertexFormatHalf2 = 720898,
  MDLVertexFormatHalf3 = 720899,
  MDLVertexFormatHalf4 = 720900,
  MDLVertexFormatFloat = 786433,
  MDLVertexFormatFloat2 = 786434,
  MDLVertexFormatFloat3 = 786435,
  MDLVertexFormatFloat4 = 786436,
  MDLVertexFormatInt1010102Normalized = 659460,
  MDLVertexFormatUInt1010102Normalized = 593924,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLVertexBufferLayout;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLVertexAttribute;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLVertexDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMatrix4x4Array;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLAnimatedValueInterpolation {
  MDLAnimatedValueInterpolationConstant = 0,
  MDLAnimatedValueInterpolationLinear = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedValue;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedScalarArray;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedVector3Array;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedQuaternionArray;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedScalar;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedVector2;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedVector3;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedVector4;
# [ objrs ( class , super = MDLAnimatedValue ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimatedMatrix4x4;
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLSkeleton;
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLJointAnimationProto {}
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLPackedJointAnimation;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAnimationBindComponent;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAsset;
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLLightProbeIrradianceDataSourceProto {
  #[objrs(selector = "sphericalHarmonicsCoefficientsAtPosition:")]
  #[cfg(feature = "RK_ModelIO")]
  fn sphericalHarmonicsCoefficientsAtPosition_(&self, position: ()) -> Arc<NSData>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLCameraProjection {
  MDLCameraProjectionPerspective = 0,
  MDLCameraProjectionOrthographic = 1,
}
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLCamera;
# [ objrs ( class , super = MDLCamera ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLStereoscopicCamera;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLLightType {
  MDLLightTypeUnknown = 0,
  MDLLightTypeAmbient = 1,
  MDLLightTypeDirectional = 2,
  MDLLightTypeSpot = 3,
  MDLLightTypePoint = 4,
  MDLLightTypeLinear = 5,
  MDLLightTypeDiscArea = 6,
  MDLLightTypeRectangularArea = 7,
  MDLLightTypeSuperElliptical = 8,
  MDLLightTypePhotometric = 9,
  MDLLightTypeProbe = 10,
  MDLLightTypeEnvironment = 11,
}
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLLight;
# [ objrs ( class , super = MDLLight ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLPhysicallyPlausibleLight;
# [ objrs ( class , super = MDLPhysicallyPlausibleLight ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLAreaLight;
# [ objrs ( class , super = MDLPhysicallyPlausibleLight ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLPhotometricLight;
# [ objrs ( class , super = MDLLight ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLLightProbe;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialSemantic {
  MDLMaterialSemanticBaseColor = 0,
  MDLMaterialSemanticSubsurface = 1,
  MDLMaterialSemanticMetallic = 2,
  MDLMaterialSemanticSpecular = 3,
  MDLMaterialSemanticSpecularExponent = 4,
  MDLMaterialSemanticSpecularTint = 5,
  MDLMaterialSemanticRoughness = 6,
  MDLMaterialSemanticAnisotropic = 7,
  MDLMaterialSemanticAnisotropicRotation = 8,
  MDLMaterialSemanticSheen = 9,
  MDLMaterialSemanticSheenTint = 10,
  MDLMaterialSemanticClearcoat = 11,
  MDLMaterialSemanticClearcoatGloss = 12,
  MDLMaterialSemanticEmission = 13,
  MDLMaterialSemanticBump = 14,
  MDLMaterialSemanticOpacity = 15,
  MDLMaterialSemanticInterfaceIndexOfRefraction = 16,
  MDLMaterialSemanticMaterialIndexOfRefraction = 17,
  MDLMaterialSemanticObjectSpaceNormal = 18,
  MDLMaterialSemanticTangentSpaceNormal = 19,
  MDLMaterialSemanticDisplacement = 20,
  MDLMaterialSemanticDisplacementScale = 21,
  MDLMaterialSemanticAmbientOcclusion = 22,
  MDLMaterialSemanticAmbientOcclusionScale = 23,
  MDLMaterialSemanticNone = 32768,
  MDLMaterialSemanticUserDefined = 32769,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialPropertyType {
  MDLMaterialPropertyTypeNone = 0,
  MDLMaterialPropertyTypeString = 1,
  MDLMaterialPropertyTypeURL = 2,
  MDLMaterialPropertyTypeTexture = 3,
  MDLMaterialPropertyTypeColor = 4,
  MDLMaterialPropertyTypeFloat = 5,
  MDLMaterialPropertyTypeFloat2 = 6,
  MDLMaterialPropertyTypeFloat3 = 7,
  MDLMaterialPropertyTypeFloat4 = 8,
  MDLMaterialPropertyTypeMatrix44 = 9,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialTextureWrapMode {
  MDLMaterialTextureWrapModeClamp = 0,
  MDLMaterialTextureWrapModeRepeat = 1,
  MDLMaterialTextureWrapModeMirror = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialTextureFilterMode {
  MDLMaterialTextureFilterModeNearest = 0,
  MDLMaterialTextureFilterModeLinear = 1,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialMipMapFilterMode {
  MDLMaterialMipMapFilterModeNearest = 0,
  MDLMaterialMipMapFilterModeLinear = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTextureFilter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTextureSampler;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMaterialProperty;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMaterialPropertyConnection;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMaterialPropertyNode;
# [ objrs ( class , super = MDLMaterialPropertyNode ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMaterialPropertyGraph;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLScatteringFunction;
# [ objrs ( class , super = MDLScatteringFunction ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLPhysicallyPlausibleScatteringFunction;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLMaterialFace {
  MDLMaterialFaceFront = 0,
  MDLMaterialFaceBack = 1,
  MDLMaterialFaceDoubleSided = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMaterial;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLSubmeshTopology;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLSubmesh;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLVertexAttributeData;
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLMesh;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum MDLTextureChannelEncoding {
  MDLTextureChannelEncodingUInt8 = 1,
  MDLTextureChannelEncodingUInt16 = 2,
  MDLTextureChannelEncodingUInt24 = 3,
  MDLTextureChannelEncodingUInt32 = 4,
  MDLTextureChannelEncodingFloat16 = 258,
  MDLTextureChannelEncodingFloat16SR = 770,
  MDLTextureChannelEncodingFloat32 = 260,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLURLTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLCheckerboardTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLSkyCubeTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLColorSwatchTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLNoiseTexture;
# [ objrs ( class , super = MDLTexture ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLNormalMapTexture;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MDLTransformOpRotationOrder {
  MDLTransformOpRotationOrderXYZ = 1,
  MDLTransformOpRotationOrderXZY = 2,
  MDLTransformOpRotationOrderYXZ = 3,
  MDLTransformOpRotationOrderYZX = 4,
  MDLTransformOpRotationOrderZXY = 5,
  MDLTransformOpRotationOrderZYX = 6,
}
#[objrs(protocol)]
#[link(name = "ModelIO", kind = "framework")]
pub trait MDLTransformOpProto {
  #[objrs(selector = "float4x4AtTime:")]
  #[cfg(feature = "RK_ModelIO")]
  fn float4x4AtTime_(&self, time: f64) -> simd_float4x4;
  #[objrs(selector = "double4x4AtTime:")]
  #[cfg(feature = "RK_ModelIO")]
  fn double4x4AtTime_(&self, time: f64) -> simd_double4x4;
  #[objrs(selector = "IsInverseOp")]
  #[cfg(feature = "RK_ModelIO")]
  fn IsInverseOp(&self) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformRotateXOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformRotateYOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformRotateZOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformRotateOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformTranslateOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformScaleOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformMatrixOp;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLTransformStack;
pub type MDLVoxelIndex = ();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MDLVoxelIndexExtent {
  pub minimumExtent: (),
  pub maximumExtent: (),
}
# [ objrs ( class , super = MDLObject ) ]
#[link(name = "ModelIO", kind = "framework")]
pub struct MDLVoxelArray;
#[cfg(feature = "RK_ModelIO")]
#[link(name = "ModelIO", kind = "framework")]
extern "C" {}
