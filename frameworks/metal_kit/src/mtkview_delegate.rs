// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use mtkview::MTKView;
use objrs::objrs;
use objrs_frameworks_core_graphics::CGSize;

#[objrs(protocol, id_ident = MTKViewDelegateId)]
#[link(name = "MetalKit", kind = "framework")]
pub trait MTKViewDelegate {
  #[objrs(selector = "mtkView:drawableSizeWillChange:")]
  fn mtkview_drawable_size_will_change(&mut self, view: &mut MTKView, size: CGSize);

  #[objrs(selector = "drawInMTKView:")]
  fn draw_in_mtkview(&mut self, view: &mut MTKView);
}
