#[allow(unused_imports)]
use crate::objc::*;
use crate::AppKit::NSDisplayGamut;
use crate::AppKit::NSView;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGImage;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::Foundation::NSArray;
use crate::Foundation::NSBundle;
use crate::Foundation::NSCoder;
use crate::Foundation::NSCodingProto;
use crate::Foundation::NSData;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSError;
use crate::Foundation::NSString;
use crate::Foundation::NSURL;
use crate::Metal::MTLBufferProto;
use crate::Metal::MTLClearColor;
use crate::Metal::MTLDeviceProto;
use crate::Metal::MTLIndexType;
use crate::Metal::MTLPixelFormat;
use crate::Metal::MTLPrimitiveType;
use crate::Metal::MTLRenderPassDescriptor;
use crate::Metal::MTLTextureProto;
use crate::Metal::MTLVertexDescriptor;
use crate::Metal::MTLVertexFormat;
use crate::ModelIO::MDLAsset;
use crate::ModelIO::MDLMesh;
use crate::ModelIO::MDLMeshBufferAllocatorProto;
use crate::ModelIO::MDLMeshBufferProto;
use crate::ModelIO::MDLMeshBufferType;
use crate::ModelIO::MDLMeshBufferZoneProto;
use crate::ModelIO::MDLNamedProto;
use crate::ModelIO::MDLTexture;
use crate::ModelIO::MDLVertexDescriptor;
use crate::ModelIO::MDLVertexFormat;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
use crate::QuartzCore::CALayerDelegateProto;
use crate::QuartzCore::CAMetalDrawableProto;
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
# [ objrs ( class , super = NSView ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKView;
#[objrs(protocol)]
#[link(name = "MetalKit", kind = "framework")]
pub trait MTKViewDelegateProto {
  #[objrs(selector = "mtkView:drawableSizeWillChange:")]
  #[cfg(feature = "RK_MetalKit")]
  fn mtkView_drawableSizeWillChange_(&self, view: &MTKView, size: CGSize) -> ();
  #[objrs(selector = "drawInMTKView:")]
  #[cfg(feature = "RK_MetalKit")]
  fn drawInMTKView_(&self, view: &MTKView) -> ();
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKTextureLoader;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKMeshBufferAllocator;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKMeshBuffer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKSubmesh;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "MetalKit", kind = "framework")]
pub struct MTKMesh;
#[cfg(feature = "RK_MetalKit")]
#[link(name = "MetalKit", kind = "framework")]
extern "C" {
  pub fn MTKModelIOVertexDescriptorFromMetal(
    metalDescriptor: *mut MTLVertexDescriptor,
  ) -> *mut MDLVertexDescriptor;
  pub fn MTKModelIOVertexDescriptorFromMetalWithError(
    metalDescriptor: *mut MTLVertexDescriptor,
    error: *mut *mut NSError,
  ) -> *mut MDLVertexDescriptor;
  pub fn MTKMetalVertexDescriptorFromModelIO(
    modelIODescriptor: *mut MDLVertexDescriptor,
  ) -> *mut MTLVertexDescriptor;
  pub fn MTKMetalVertexDescriptorFromModelIOWithError(
    modelIODescriptor: *mut MDLVertexDescriptor,
    error: *mut *mut NSError,
  ) -> *mut MTLVertexDescriptor;
  pub fn MTKModelIOVertexFormatFromMetal(vertexFormat: MTLVertexFormat) -> MDLVertexFormat;
  pub fn MTKMetalVertexFormatFromModelIO(vertexFormat: MDLVertexFormat) -> MTLVertexFormat;
}
