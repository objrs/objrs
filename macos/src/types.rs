#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use objrs::objrs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float2x2 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float3x2 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float4x2 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float2x3 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float3x3 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float4x3 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float2x4 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float3x4 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_float4x4 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double2x2 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double3x2 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double4x2 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double2x3 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double3x3 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double4x3 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double2x4 {
  pub columns: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double3x4 {
  pub columns: [(); 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_double4x4 {
  pub columns: [(); 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_quatf {
  pub vector: (),
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simd_quatd {
  pub vector: (),
}
