// The contents of this file is licensed by its authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option. The
// contents of this file may not be copied, modified, or distributed except according to those
// terms. See the COPYRIGHT file at the top-level directory of this distribution for copies of these
// licenses and more information.

use core;
use mesh::load_triangles;
use objrs::objrs;
use objrs_frameworks_core_graphics::CGSize;
use objrs_frameworks_foundation::{nsstring, NSObject, NSString};
use objrs_frameworks_metal::{
  MTLBlitCommandEncoder, MTLBufferId, MTLClearColorMake, MTLCommandBuffer, MTLCommandQueue,
  MTLCommandQueueId, MTLDevice, MTLDeviceId, MTLLibrary, MTLPrimitiveType, MTLRenderCommandEncoder,
  MTLRenderPipelineDescriptor, MTLRenderPipelineStateId, MTLResourceOptions, MTLViewport,
};
use objrs_frameworks_metal_kit::{MTKView, MTKViewDelegate};

const VERTICES_ID: usize = 0;
const UNIFORMS_ID: usize = 1;

static SHADERS: &'static NSString = nsstring!(
  r#"
#include <metal_stdlib>

using namespace metal;

constant constexpr uint VERTICES_ID = 0;
constant constexpr uint UNIFORMS_ID = 1;

// Takes an input in the range [0, 65535] and returns a float in the range [0, 1].
constexpr float hash(uint value) {
  // Hashing algorithm from Thomas Mueller: https://stackoverflow.com/a/12996028/1287251
  // In our case, value is always less than 65536, so there's no point in doing the shift + xor
  // in the first round.
  uint hash = value * 0x45d9f3b;
  hash = ((hash >> 16) ^ hash) * 0x45d9f3b;
  hash = (hash >> 16) ^ hash;
  return static_cast<float>(hash) * (1.0f / 4294967295.0f);
}

float2 distort_vertex(float2 vertex_pos, uint vertex_id, float2 white_hole_pos) {
  float2 delta = vertex_pos - white_hole_pos;
  float repel = length_squared(delta);
  repel = repel * repel;
  repel = 0.0001f * hash(vertex_id) / repel;
  repel = min(repel, 0.2f);
  float sign = (vertex_id & 1) ? -1.0f : 1.0f;
  return vertex_pos + sign * repel * delta;
}

struct VertexUniforms {
  float2 viewport_scale;
  float2 mouse_pos;
};

struct VertexOutput {
  float4 position [[position]];
};

vertex VertexOutput vertex_shader(uint vertex_id [[vertex_id]],
                                  const device float2 *vertices [[buffer(VERTICES_ID)]],
                                  constant VertexUniforms *uniforms [[buffer(UNIFORMS_ID)]]) {
  float2 vertex_pos = vertices[vertex_id];

  float2 viewport_scale = uniforms->viewport_scale;
  float2 mouse_pos = uniforms->mouse_pos;

  float2 out_position = distort_vertex(vertex_pos, vertex_id, mouse_pos);
  out_position *= viewport_scale;

  VertexOutput out;
  out.position = vector_float4(out_position, 0.0, 1.0);
  return out;
}

fragment float4 fragment_shader() {
  return float4(0.0, 0.0, 0.0, 1.0);
}
"#
);

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C)]
struct VertexUniforms {
  viewport_scale: [f32; 2],
  mouse_pos: [f32; 2],
}

fn gpu_buffer_with_triangles(
  triangles: &[[[f32; 2]; 3]],
  device: &mut MTLDeviceId,
  command_queue: &mut MTLCommandQueueId,
) -> Result<objrs::Strong<MTLBufferId>, String> {
  let cpu_buffer = device
    .new_buffer_with_slice_options(triangles, MTLResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED)
    .ok_or(String::from("Failed to create the CPU buffer"))?;
  let tirangles_len = core::mem::size_of_val(triangles);
  let mut gpu_buffer = device
    .new_buffer_with_length_options(tirangles_len, MTLResourceOptions::STORAGE_MODE_PRIVATE)
    .ok_or(String::from("Failed to create the GPU buffer"))?;

  let command_buffer = command_queue
    .command_buffer()
    .ok_or(String::from("Failed to get a command buffer from the command queue"))?;

  {
    let blit_encoder = command_buffer
      .blit_command_encoder()
      .ok_or(String::from("Failed to get a blit command encoder from the commadn buffer"))?;
    blit_encoder.copy_from_buffer_to_buffer(&cpu_buffer, &mut gpu_buffer);
    blit_encoder.end_encoding();
  }
  command_buffer.commit();
  command_buffer.wait_until_completed();

  return Ok(gpu_buffer);
}

#[objrs(class, super = NSObject)]
pub struct Renderer {
  drawable_size: [f32; 2],
  pipeline_state: Option<objrs::Strong<MTLRenderPipelineStateId>>,
  command_queue: Option<objrs::Strong<MTLCommandQueueId>>,
  triangles: Option<objrs::Strong<MTLBufferId>>,
}

#[objrs(impl)]
impl Renderer {
  #[objrs(selector = "new", no_impl)]
  pub fn new() -> objrs::Strong<Renderer> {}

  fn initialize_metal<'a>(&mut self, view: &'a mut MTKView) -> Result<(), String> {
    if self.triangles.is_some() {
      // We've already initialized in this case.
      return Ok(());
    }

    let sample_count = 4;
    let color_pixel_format = view.color_pixel_format();

    view.set_clear_color(MTLClearColorMake(1.0, 1.0, 1.0, 1.0));
    view.set_sample_count(sample_count);

    let device = view.device().ok_or(String::from("MTKView has no Metal device"))?;
    let mut error = None;
    let library = device.new_library_with_source_options_error(SHADERS, None, Some(&mut error));
    if let Some(ref error) = error {
      return Err(error.localized_description().as_ref().to_string());
    }
    let mut library = library.ok_or(String::from("Library is null but the error was not set"))?;

    let vertex_function = library.new_function_with_name(nsstring!("vertex_shader"));
    let fragment_function = library.new_function_with_name(nsstring!("fragment_shader"));

    let mut pipeline_state_descriptor = MTLRenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_raster_sample_count(sample_count);
    pipeline_state_descriptor.set_vertex_function(vertex_function);
    pipeline_state_descriptor.set_fragment_function(fragment_function);
    pipeline_state_descriptor
      .color_attachments()
      .object_at_indexed_subscript(0)
      .set_pixel_format(color_pixel_format);

    let pipeline_state = device.new_render_pipeline_state_with_descriptor_error(
      &pipeline_state_descriptor,
      Some(&mut error),
    );
    if let Some(ref error) = error {
      return Err(error.localized_description().as_ref().to_string());
    }
    self.pipeline_state = pipeline_state;

    self.command_queue = device.new_command_queue();
    let command_queue = self
      .command_queue
      .as_mut()
      .ok_or(String::from("Failed to get a command queue from the device"))?;

    let buffer = gpu_buffer_with_triangles(&load_triangles(), device, command_queue)?;
    self.triangles = Some(buffer);

    return Ok(());
  }
}

#[objrs(impl)]
impl MTKViewDelegate for Renderer {
  #[objrs(selector = "mtkView:drawableSizeWillChange:")]
  fn mtkview_drawable_size_will_change(&mut self, view: &mut MTKView, size: CGSize) {
    self.initialize_metal(view).expect("Failed to initialize Metal");
    self.drawable_size = [size.width as f32, size.height as f32];
  }

  #[objrs(selector = "drawInMTKView:")]
  fn draw_in_mtkview(&mut self, view: &mut MTKView) {
    self.initialize_metal(view).expect("Failed to initialize Metal");

    let command_queue = self
      .command_queue
      .as_mut()
      .expect("BUG: the command queue shouldn't be empty if Metal initialization has succeeded");
    let command_buffer = command_queue
      .command_buffer()
      .expect("Failed to get a command buffer from the command queue");

    {
      let pos = view.window().mouse_location_outside_of_event_stream();

      let render_pass_descriptor;
      match view.current_render_pass_descriptor() {
        Some(value) => render_pass_descriptor = value,
        None => return,
      }

      {
        let render_encoder = command_buffer
          .render_command_encoder_with_descriptor(render_pass_descriptor)
          .expect("Failed to get a render command encoder from the command buffer");

        let drawable_size = self.drawable_size;

        render_encoder.set_viewport(MTLViewport {
          originX: 0.0,
          originY: 0.0,
          width: drawable_size[0] as f64,
          height: drawable_size[1] as f64,
          znear: -1.0,
          zfar: 1.0,
        });

        let pipeline_state = self.pipeline_state.as_mut().expect(
          "BUG: the pipeline state shouldn't be empty if Metal initialization has succeeded",
        );
        render_encoder.set_render_pipeline_state(pipeline_state);

        let triangles: Option<&mut MTLBufferId> = self.triangles.as_mut().map(|x| &mut **x);

        let mut viewport_scale: [f32; 2] = [1.0, 1.0];
        if drawable_size[0] < drawable_size[1] {
          viewport_scale[1] = drawable_size[0] / drawable_size[1];
        } else if drawable_size[0] > drawable_size[1] {
          viewport_scale[0] = drawable_size[1] / drawable_size[0];
        }

        let mouse_pos: [f32; 2] = [
          (2.0 * pos.x as f32 - drawable_size[0]) / (drawable_size[0] * viewport_scale[0]),
          (2.0 * pos.y as f32 - drawable_size[1]) / (drawable_size[1] * viewport_scale[1]),
        ];

        let vertex_uniforms = VertexUniforms {
          viewport_scale: viewport_scale,
          mouse_pos: mouse_pos,
        };

        unsafe {
          render_encoder.set_vertex_buffer_at_index(triangles, 0, VERTICES_ID);
          render_encoder.set_vertex_struct(&vertex_uniforms, UNIFORMS_ID);
          render_encoder.draw_primitives_vertex_start_vertex_count(
            MTLPrimitiveType::TRIANGLE,
            0,
            16141 * 3,
          );
        }

        render_encoder.end_encoding();
      }
    }

    let current_drawable =
      view.current_drawable().expect("Failed to get the current drawable for the view");
    command_buffer.present_drawable(current_drawable);
    command_buffer.commit();
  }
}
