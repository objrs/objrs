#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreGraphics::CGAffineTransform;
use crate::CoreGraphics::CGColor;
use crate::CoreGraphics::CGColorSpace;
use crate::CoreGraphics::CGContext;
use crate::CoreGraphics::CGPath;
use crate::CoreGraphics::CGPoint;
use crate::CoreGraphics::CGRect;
use crate::CoreGraphics::CGSize;
use crate::CoreVideo::CVTimeStamp;
use crate::Foundation::NSArray;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSNumber;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSString;
use crate::Metal::MTLDrawableProto;
use crate::Metal::MTLPixelFormat;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
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
#[repr(C)]
pub struct CATransform3D {
  opaque: u32,
}
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CAMediaTimingProto {}
bitflags! { # [ repr ( C ) ] pub struct CAAutoresizingMask : u32 { const kCALayerNotSizable = 0 ; const kCALayerMinXMargin = 1 ; const kCALayerWidthSizable = 2 ; const kCALayerMaxXMargin = 4 ; const kCALayerMinYMargin = 8 ; const kCALayerHeightSizable = 16 ; const kCALayerMaxYMargin = 32 ; } }
bitflags! { # [ repr ( C ) ] pub struct CAEdgeAntialiasingMask : u32 { const kCALayerLeftEdge = 1 ; const kCALayerRightEdge = 2 ; const kCALayerBottomEdge = 4 ; const kCALayerTopEdge = 8 ; } }
bitflags! { # [ repr ( C ) ] pub struct CACornerMask : usize { const kCALayerMinXMinYCorner = 1 ; const kCALayerMaxXMinYCorner = 2 ; const kCALayerMinXMaxYCorner = 4 ; const kCALayerMaxXMaxYCorner = 8 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CALayer;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CALayerIvars {
  pub refcount: i32,
  pub magic: u32,
  pub layer: *mut c_void,
}
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CALayoutManagerProto {
  #[objrs(selector = "preferredSizeOfLayer:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn preferredSizeOfLayer_(&self, layer: &CALayer) -> CGSize;
  #[objrs(selector = "invalidateLayoutOfLayer:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn invalidateLayoutOfLayer_(&self, layer: &CALayer) -> ();
  #[objrs(selector = "layoutSublayersOfLayer:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn layoutSublayersOfLayer_(&self, layer: &CALayer) -> ();
}
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CAActionProto {
  #[objrs(selector = "runActionForKey:object:arguments:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn runActionForKey_object_arguments_(
    &self,
    event: &NSString,
    anObject: &Object,
    dict: Option<&NSDictionary>,
  ) -> ();
}
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CALayerDelegateProto {
  #[objrs(selector = "displayLayer:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn displayLayer_(&self, layer: &CALayer) -> ();
  #[objrs(selector = "drawLayer:inContext:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn drawLayer_inContext_(&self, layer: &CALayer, ctx: &CGContext) -> ();
  #[objrs(selector = "layerWillDraw:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn layerWillDraw_(&self, layer: &CALayer) -> ();
  #[objrs(selector = "layoutSublayersOfLayer:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn layoutSublayersOfLayer_(&self, layer: &CALayer) -> ();
  #[objrs(selector = "actionForLayer:forKey:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn actionForLayer_forKey_(&self, layer: &CALayer, event: &NSString) -> Option<Arc<Object>>;
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAAnimation;
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CAAnimationDelegateProto {
  #[objrs(selector = "animationDidStart:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn animationDidStart_(&self, anim: &CAAnimation) -> ();
  #[objrs(selector = "animationDidStop:finished:")]
  #[cfg(feature = "RK_QuartzCore")]
  fn animationDidStop_finished_(&self, anim: &CAAnimation, flag: bool) -> ();
}
# [ objrs ( class , super = CAAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAPropertyAnimation;
# [ objrs ( class , super = CAPropertyAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CABasicAnimation;
# [ objrs ( class , super = CAPropertyAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAKeyframeAnimation;
# [ objrs ( class , super = CABasicAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CASpringAnimation;
# [ objrs ( class , super = CAAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CATransition;
# [ objrs ( class , super = CAAnimation ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAAnimationGroup;
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CAConstraintAttribute {
  kCAConstraintMinX = 0,
  kCAConstraintMidX = 1,
  kCAConstraintMaxX = 2,
  kCAConstraintWidth = 3,
  kCAConstraintMinY = 4,
  kCAConstraintMidY = 5,
  kCAConstraintMaxY = 6,
  kCAConstraintHeight = 7,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAConstraintLayoutManager;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAConstraint;
#[objrs(protocol)]
#[link(name = "QuartzCore", kind = "framework")]
pub trait CAMetalDrawableProto {}
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAMetalLayer;
#[repr(C)]
pub struct _CAMetalLayerPrivate {
  opaque: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAEmitterCell;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAEmitterLayer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAMediaTimingFunction;
#[repr(C)]
pub struct CAMediaTimingFunctionPrivate {
  opaque: u32,
}
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAGradientLayer;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAOpenGLLayer;
#[repr(C)]
pub struct CAOpenGLLayerPrivate {
  opaque: u32,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CARemoteLayerClient;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CARemoteLayerServer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CARenderer;
#[repr(C)]
pub struct CARendererPriv {
  opaque: u32,
}
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAReplicatorLayer;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAScrollLayer;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAShapeLayer;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CATextLayer;
#[repr(C)]
pub struct CATextLayerPrivate {
  opaque: u32,
}
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CATiledLayer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CATransaction;
# [ objrs ( class , super = CALayer ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CATransformLayer;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "QuartzCore", kind = "framework")]
pub struct CAValueFunction;
#[cfg(feature = "RK_QuartzCore")]
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
  pub fn CACurrentMediaTime() -> f64;
  pub fn CATransform3DIsIdentity(t: CATransform3D) -> bool;
  pub fn CATransform3DEqualToTransform(a: CATransform3D, b: CATransform3D) -> bool;
  pub fn CATransform3DMakeTranslation(tx: f64, ty: f64, tz: f64) -> CATransform3D;
  pub fn CATransform3DMakeScale(sx: f64, sy: f64, sz: f64) -> CATransform3D;
  pub fn CATransform3DMakeRotation(angle: f64, x: f64, y: f64, z: f64) -> CATransform3D;
  pub fn CATransform3DTranslate(t: CATransform3D, tx: f64, ty: f64, tz: f64) -> CATransform3D;
  pub fn CATransform3DScale(t: CATransform3D, sx: f64, sy: f64, sz: f64) -> CATransform3D;
  pub fn CATransform3DRotate(t: CATransform3D, angle: f64, x: f64, y: f64, z: f64)
    -> CATransform3D;
  pub fn CATransform3DConcat(a: CATransform3D, b: CATransform3D) -> CATransform3D;
  pub fn CATransform3DInvert(t: CATransform3D) -> CATransform3D;
  pub fn CATransform3DMakeAffineTransform(m: CGAffineTransform) -> CATransform3D;
  pub fn CATransform3DIsAffine(t: CATransform3D) -> bool;
  pub fn CATransform3DGetAffineTransform(t: CATransform3D) -> CGAffineTransform;
}
