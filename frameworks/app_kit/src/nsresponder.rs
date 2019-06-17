// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

use objrs::objrs;
use objrs_frameworks_foundation::NSObject;

#[objrs(class, super = NSObject)]
#[link(name = "AppKit", kind = "framework")]
pub struct NSResponder;

#[objrs(impl)]
#[link(name = "AppKit", kind = "framework")]
impl NSResponder {}
