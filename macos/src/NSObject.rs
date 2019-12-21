#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use objrs::objrs;
use Foundation::NSString;
#[objrs(protocol)]
pub trait NSObjectProto {
  #[objrs(selector = "isEqual:")]
  fn isEqual_(&self, object: Option<&Object>) -> bool;
  #[objrs(selector = "self")]
  fn self_(&self) -> Option<Arc<Self>>;
  #[objrs(selector = "performSelector:")]
  fn performSelector_(&self, aSelector: SelectorRef) -> Option<Arc<Object>>;
  #[objrs(selector = "performSelector:withObject:")]
  fn performSelector_withObject_(
    &self,
    aSelector: SelectorRef,
    object: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "performSelector:withObject:withObject:")]
  fn performSelector_withObject_withObject_(
    &self,
    aSelector: SelectorRef,
    object1: Option<&Object>,
    object2: Option<&Object>,
  ) -> Option<Arc<Object>>;
  #[objrs(selector = "isProxy")]
  fn isProxy(&self) -> bool;
  #[objrs(selector = "isKindOfClass:")]
  fn isKindOfClass_(&self, aClass: Option<&Class>) -> bool;
  #[objrs(selector = "isMemberOfClass:")]
  fn isMemberOfClass_(&self, aClass: Option<&Class>) -> bool;
  #[objrs(selector = "conformsToProtocol:")]
  fn conformsToProtocol_(&self, aProtocol: Option<&Protocol>) -> bool;
  #[objrs(selector = "respondsToSelector:")]
  fn respondsToSelector_(&self, aSelector: SelectorRef) -> bool;
}
#[repr(C)]
pub struct _NSZone {
  opaque: u32,
}
#[objrs(class, root_class)]
pub struct NSObject;
