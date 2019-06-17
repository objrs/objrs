// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#![feature(arbitrary_self_types, extern_types)]
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

pub use crate::mtlblit_command_encoder::*;
pub use crate::mtlbuffer::*;
pub use crate::mtlcommand_buffer::*;
pub use crate::mtlcommand_encoder::*;
pub use crate::mtlcommand_queue::*;
pub use crate::mtlcompile_options::*;
pub use crate::mtldevice::*;
pub use crate::mtldrawable::*;
pub use crate::mtllibrary::*;
pub use crate::mtlpixel_format::*;
pub use crate::mtlrender_command_encoder::*;
pub use crate::mtlrender_pass::*;
pub use crate::mtlrender_pipeline::*;
pub use crate::mtlresource::*;
