#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreGraphics::CGAffineTransform;
use crate::CoreGraphics::CGColor;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGContext;
use crate::CoreGraphics::CGImage;
use crate::CoreGraphics::CGLayer;
use crate::CoreGraphics::CGPoint;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::CoreVideo::__CVBuffer;
use crate::Foundation::NSArray;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSData;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSError;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSString;
use crate::Foundation::NSURL;
use crate::IOSurface::IOSurface;
use crate::IOSurface::__IOSurface;
use crate::ImageIO::CGImagePropertyOrientation;
use crate::Metal::MTLPixelFormat;
use crate::NSObject::NSObject;
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
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIVector;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIColor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIImage;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIContext;
#[objrs(protocol)]
#[link(name = "CoreImage", kind = "framework")]
pub trait CIFilterConstructorProto {
  #[objrs(selector = "filterWithName:")]
  #[cfg(feature = "RK_CoreImage")]
  fn filterWithName_(&self, name: &NSString) -> Option<Arc<CIFilter>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIFilter;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIKernel;
# [ objrs ( class , super = CIKernel ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIColorKernel;
# [ objrs ( class , super = CIKernel ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIWarpKernel;
# [ objrs ( class , super = CIColorKernel ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIBlendKernel;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIDetector;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIFeature;
# [ objrs ( class , super = CIFeature ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIFaceFeature;
# [ objrs ( class , super = CIFeature ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIRectangleFeature;
# [ objrs ( class , super = CIFeature ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIQRCodeFeature;
# [ objrs ( class , super = CIFeature ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CITextFeature;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIImageProcessorKernel;
#[objrs(protocol)]
#[link(name = "CoreImage", kind = "framework")]
pub trait CIImageProcessorInputProto {}
#[objrs(protocol)]
#[link(name = "CoreImage", kind = "framework")]
pub trait CIImageProcessorOutputProto {}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIImageAccumulator;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIFilterShape;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CISampler;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIRenderDestination;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum CIRenderDestinationAlphaMode {
  CIRenderDestinationAlphaNone = 0,
  CIRenderDestinationAlphaPremultiplied = 1,
  CIRenderDestinationAlphaUnpremultiplied = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIRenderInfo;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIRenderTask;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIBarcodeDescriptor;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CIQRCodeErrorCorrectionLevel {
  CIQRCodeErrorCorrectionLevelL = 76,
  CIQRCodeErrorCorrectionLevelM = 77,
  CIQRCodeErrorCorrectionLevelQ = 81,
  CIQRCodeErrorCorrectionLevelH = 72,
}
# [ objrs ( class , super = CIBarcodeDescriptor ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIQRCodeDescriptor;
# [ objrs ( class , super = CIBarcodeDescriptor ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIAztecCodeDescriptor;
# [ objrs ( class , super = CIBarcodeDescriptor ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIPDF417CodeDescriptor;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum CIDataMatrixCodeECCVersion {
  CIDataMatrixCodeECCVersion000 = 0,
  CIDataMatrixCodeECCVersion050 = 50,
  CIDataMatrixCodeECCVersion080 = 80,
  CIDataMatrixCodeECCVersion100 = 100,
  CIDataMatrixCodeECCVersion140 = 140,
  CIDataMatrixCodeECCVersion200 = 200,
}
# [ objrs ( class , super = CIBarcodeDescriptor ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIDataMatrixCodeDescriptor;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIFilterGenerator;
#[repr(C)]
pub struct CIFilterGeneratorStruct {
  opaque: u32,
}
#[objrs(protocol)]
#[link(name = "CoreImage", kind = "framework")]
pub trait CIPlugInRegistrationProto {
  #[objrs(selector = "load:")]
  #[cfg(feature = "RK_CoreImage")]
  fn load_(&self, host: Option<&c_void>) -> bool;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreImage", kind = "framework")]
pub struct CIPlugIn;
#[cfg(feature = "RK_CoreImage")]
#[link(name = "CoreImage", kind = "framework")]
extern "C" {}
