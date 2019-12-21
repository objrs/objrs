
mod objc;
mod NSObject;
mod MacTypes;
mod acl;
mod hfs_unistr;
mod message;
mod types;

pub mod MetalKit;
pub mod ModelIO;
pub mod CoreFoundation;
pub mod Metal;
pub mod IOSurface;
pub mod QuartzCore;
pub mod OpenGL;
pub mod CoreVideo;
pub mod CoreGraphics;
pub mod AppKit;
pub mod CoreData;
pub mod CoreServices;
pub mod DiskArbitration;
pub mod Security;
pub mod CoreImage;
pub mod ImageIO;
pub mod Foundation;

use std::mem;
use std::ptr;

mod stdio {
	#[repr(C)]
	pub struct FILE {
		opaque: [u8; 0]
	}
}

#[repr(C)]
pub struct c_void {
    opaque: [u8; 0]
}

/* This probably won't work for bitcode. Need to use LLVM IR metadata.
 * See llvm/docs/LangRef.rst */
#[allow(dead_code)]
#[no_mangle]
#[link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip"]
pub static IMAGEINFO: objc::ObjCImageInfo = objc::ObjCImageInfo {
    version: 0,
    flags: 0,
};
