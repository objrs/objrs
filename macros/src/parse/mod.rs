// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

// TODO: Add available() parameter for attributes (see API_AVAILABLE macro and __attribute__((availability())))? Possible syntax
// #[objrs(class, super = NSObject, available(macos = X.Y, ios = X.Y, watchos = X.Y, tvos = X.Y))]
// struct Foo;
//
// The available() parameter would work for:
// #[objrs(class)]
// #[objrs(selector)]
// #[objrs(protocol)]
// #[objrs(ivar)]? Is this ever used in real life?
// #[objrs(impl)]? Is this ever used in real life, either for a category or protocol impl?
//
// Omitting an OS from the list means it's not available for that platform.

pub mod attr;
pub mod class_attr;
pub mod class_ref_attr;
mod drain_ext;
pub mod impl_attr;
pub mod ivar_attr;
mod property_attr;
pub mod protocol_attr;
pub mod sel_ref_attr;
pub mod selector_attr;
mod util;
