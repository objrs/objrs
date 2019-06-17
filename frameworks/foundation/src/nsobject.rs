// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// See https://github.com/opensource-apple/objc4/blob/master/runtime/NSObject.h
// Even though NSObject is provided by libobjc (and not the Foundation framework), we need to provide NSObject from the Foundation crate. This is because of a few issues:
//   1. Only part of NSObject is provided by libobjc. Part of it is provided by CoreFoundation (e.g. +[description], -[description], -[methodSignatureForSelector:], etc.). Part of it is provided by Foundation (e.g. -[autoContentAccessingProxy]). We can't work around the CoreFoundation issue (CoreFoundation doesn't just add new category methods; it replaces method implementations), and might as well embrace Foundation while we're at it.
//   2. Some of NSObject's methods (provided by libobjc/NSObject.h) use other Foundation types (e.g. NSString, NSInvocation, etc.). We need to provide NSObject in this crate to avoid a cyclic dependency (which isn't allowed between crates).

extern crate core;
extern crate objrs;

use objrs::objrs;

#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
#[repr(C)]
pub struct NSObject {
  #[deprecated]
  isa: *mut objrs::Class,
}

#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSObject {
  #[objrs(selector = "new")]
  pub fn new() -> objrs::Strong<NSObject> {}

  #[objrs(selector = "alloc")]
  pub fn alloc() -> objrs::Alloc<NSObject> {}

  #[objrs(selector = "init", instance)]
  pub fn init(this: objrs::Alloc<Self>) -> objrs::Strong<NSObject> {}

  // TODO: make this optionally available based on features.
  // #[objrs(selector = "init")]
  // pub fn init(self: objrs::Alloc<NSObject>) -> objrs::Strong<NSObject> {}

  // #[objrs(selector = "copy")]
  // pub unsafe fn copy(&self) -> objrs::Strong<NSObject> {}

  // #[objrs(selector = "mutableCopy")]
  // pub unsafe fn mutable_copy(&self) -> objrs::Strong<NSObject> {}

  #[objrs(selector = "instancesRespondToSelector:")]
  pub fn instances_respond_to_selector(sel: &objrs::Sel) -> bool {}

  // #[objrs(selector = "methodForSelector:")]
  // pub unsafe fn method_for_selector(&self, sel: &objrs::Sel) -> *mut objrs::Imp {}

  // #[objrs(selector = "instanceMethodForSelector:")]
  // pub unsafe fn instance_method_for_selector(sel: &objrs::Sel) -> *mut objrs::Imp {}

  // #[objrs(selector = "doesNotRecognizeSelector:")]
  // pub unsafe fn does_not_recognize_selector(&self, sel: &objrs::Sel) {}

  // #[objrs(selector = "forwardingTargetForSelector:")]
  // pub unsafe fn forwarding_target_for_selector(
  //   &self,
  //   sel: &objrs::Sel,
  // ) -> *mut objrs::Id {
  // }

  #[objrs(selector = "hash")]
  pub fn hash() -> usize {}

  #[objrs(selector = "superclass")]
  pub fn superclass() -> *mut objrs::Class {}

  #[objrs(selector = "class")]
  pub fn class() -> *mut objrs::Class {}

  // #[objrs(selector = "isSubclassOfClass:")]
  // pub unsafe fn is_subclass_of_class(class: *const objrs::Class) -> bool {}

  // #[objrs(selector = "resolveClassMethod:")]
  // pub unsafe fn resolve_class_method(sel: &'static objrs::Sel) -> bool {}

  // #[objrs(selector = "resolveInstanceMethod:")]
  // pub unsafe fn resolve_instance_method(sel: &'static objrs::Sel) -> bool {}
}

// @protocol NSObject

// - (BOOL)isEqual:(id)object;
// @property (readonly) NSUInteger hash;

// @property (readonly) Class superclass;
// - (Class)class OBJC_SWIFT_UNAVAILABLE("use 'anObject.dynamicType' instead");
// - (instancetype)self;

// - (id)performSelector:(SEL)aSelector;
// - (id)performSelector:(SEL)aSelector withObject:(id)object;
// - (id)performSelector:(SEL)aSelector withObject:(id)object1 withObject:(id)object2;

// - (BOOL)isProxy;

// - (BOOL)isKindOfClass:(Class)aClass;
// - (BOOL)isMemberOfClass:(Class)aClass;
// - (BOOL)conformsToProtocol:(Protocol *)aProtocol;

// - (BOOL)respondsToSelector:(SEL)aSelector;

// - (instancetype)retain OBJC_ARC_UNAVAILABLE;
// - (oneway void)release OBJC_ARC_UNAVAILABLE;
// - (instancetype)autorelease OBJC_ARC_UNAVAILABLE;
// - (NSUInteger)retainCount OBJC_ARC_UNAVAILABLE;

// - (struct _NSZone *)zone OBJC_ARC_UNAVAILABLE;

// @property (readonly, copy) NSString *description;
// @optional
// @property (readonly, copy) NSString *debugDescription;

// @end

// __OSX_AVAILABLE_STARTING(__MAC_10_0, __IPHONE_2_0)
// OBJC_ROOT_CLASS
// OBJC_EXPORT
// @interface NSObject <NSObject> {
//     Class isa  OBJC_ISA_AVAILABILITY;
// }

// + (void)load;

// + (void)initialize;
// - (instancetype)init
// #if NS_ENFORCE_NSOBJECT_DESIGNATED_INITIALIZER
//     NS_DESIGNATED_INITIALIZER
// #endif
//     ;

// + (instancetype)new OBJC_SWIFT_UNAVAILABLE("use object initializers instead");
// + (instancetype)allocWithZone:(struct _NSZone *)zone OBJC_SWIFT_UNAVAILABLE("use object initializers instead");
// + (instancetype)alloc OBJC_SWIFT_UNAVAILABLE("use object initializers instead");
// - (void)dealloc OBJC_SWIFT_UNAVAILABLE("use 'deinit' to define a de-initializer");

// - (void)finalize;

// - (id)copy;
// - (id)mutableCopy;

// + (id)copyWithZone:(struct _NSZone *)zone OBJC_ARC_UNAVAILABLE;
// + (id)mutableCopyWithZone:(struct _NSZone *)zone OBJC_ARC_UNAVAILABLE;

// + (BOOL)instancesRespondToSelector:(SEL)aSelector;
// + (BOOL)conformsToProtocol:(Protocol *)protocol;
// - (IMP)methodForSelector:(SEL)aSelector;
// + (IMP)instanceMethodForSelector:(SEL)aSelector;
// - (void)doesNotRecognizeSelector:(SEL)aSelector;

// - (id)forwardingTargetForSelector:(SEL)aSelector __OSX_AVAILABLE_STARTING(__MAC_10_5, __IPHONE_2_0);
// - (void)forwardInvocation:(NSInvocation *)anInvocation OBJC_SWIFT_UNAVAILABLE("");
// - (NSMethodSignature *)methodSignatureForSelector:(SEL)aSelector OBJC_SWIFT_UNAVAILABLE("");

// + (NSMethodSignature *)instanceMethodSignatureForSelector:(SEL)aSelector OBJC_SWIFT_UNAVAILABLE("");

// - (BOOL)allowsWeakReference UNAVAILABLE_ATTRIBUTE;
// - (BOOL)retainWeakReference UNAVAILABLE_ATTRIBUTE;

// + (BOOL)isSubclassOfClass:(Class)aClass;

// + (BOOL)resolveClassMethod:(SEL)sel __OSX_AVAILABLE_STARTING(__MAC_10_5, __IPHONE_2_0);
// + (BOOL)resolveInstanceMethod:(SEL)sel __OSX_AVAILABLE_STARTING(__MAC_10_5, __IPHONE_2_0);

// + (NSUInteger)hash;
// + (Class)superclass;
// + (Class)class OBJC_SWIFT_UNAVAILABLE("use 'aClass.self' instead");
// + (NSString *)description;
// + (NSString *)debugDescription;

// @end
