// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

#![feature(arbitrary_self_types, extern_types, rust_2018_preview)]
#![no_std]

extern crate libc;
extern crate objrs;
extern crate objrs_frameworks_foundation;

mod mtlblit_command_encoder;
mod mtlbuffer;
mod mtlcommand_buffer;
mod mtlcommand_encoder;
mod mtlcommand_queue;
mod mtlcompile_options;
mod mtldevice;
mod mtldrawable;
mod mtllibrary;
mod mtlpixel_format;
mod mtlrender_command_encoder;
mod mtlrender_pass;
mod mtlrender_pipeline;
mod mtlresource;

pub use mtlblit_command_encoder::*;
pub use mtlbuffer::*;
pub use mtlcommand_buffer::*;
pub use mtlcommand_encoder::*;
pub use mtlcommand_queue::*;
pub use mtlcompile_options::*;
pub use mtldevice::*;
pub use mtldrawable::*;
pub use mtllibrary::*;
pub use mtlpixel_format::*;
pub use mtlrender_command_encoder::*;
pub use mtlrender_pass::*;
pub use mtlrender_pipeline::*;
pub use mtlresource::*;
