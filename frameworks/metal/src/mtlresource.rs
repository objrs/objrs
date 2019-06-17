// This file and its contents are licensed by their authors and copyright holders under the Apache
// License (Version 2.0), MIT license, or Mozilla Public License (Version 2.0), at your option, and
// may not be copied, modified, or distributed except according to those terms. For copies of these
// licenses and more information, see the COPYRIGHT file in this distribution's top-level directory.

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct MTLStorageMode(usize);

// pub const MTLStorageModeShared: MTLStorageMode  = MTLStorageMode(0);
// pub const MTLStorageModeManaged: MTLStorageMode = MTLStorageMode(1);
// pub const MTLStorageModePrivate: MTLStorageMode = MTLStorageMode(2);
// pub const MTLStorageModeMemoryless: MTLStorageMode = MTLStorageMode(3);

impl MTLStorageMode {
  pub const SHARED: MTLStorageMode = MTLStorageMode(0);
  pub const MANAGED: MTLStorageMode = MTLStorageMode(1);
  pub const PRIVATE: MTLStorageMode = MTLStorageMode(2);
  pub const MEMORYLESS: MTLStorageMode = MTLStorageMode(3);
}

unsafe impl objrs::marker::Zeroed for MTLStorageMode {}
impl objrs::marker::Forgettable for MTLStorageMode {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct MTLCPUCacheMode(usize);

// pub const MTLCPUCacheModeDefaultCache: MTLCPUCacheMode = MTLCPUCacheMode(0);
// pub const MTLCPUCacheModeWriteCombined: MTLCPUCacheMode = MTLCPUCacheMode(1);

impl MTLCPUCacheMode {
  pub const DEFAULT_CACHE: MTLCPUCacheMode = MTLCPUCacheMode(0);
  pub const WRITE_COMBINED: MTLCPUCacheMode = MTLCPUCacheMode(1);
}

unsafe impl objrs::marker::Zeroed for MTLCPUCacheMode {}
impl objrs::marker::Forgettable for MTLCPUCacheMode {}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct MTLResourceOptions(usize);

// pub const MTLResourceCPUCacheModeDefaultCache: MTLResourceOptions = MTLResourceOptions(MTLCPUCacheMode::DEFAULT_CACHE.0  << 0);
// pub const MTLResourceCPUCacheModeWriteCombined: MTLResourceOptions = MTLResourceOptions(MTLCPUCacheMode::WRITE_COMBINED.0 << 0);
// pub const MTLResourceStorageModeShared: MTLResourceOptions = MTLResourceOptions(MTLStorageMode::SHARED.0 << 4);
// pub const MTLResourceStorageModeManaged: MTLResourceOptions = MTLResourceOptions(MTLStorageMode::MANAGED.0 << 4);
// pub const MTLResourceStorageModePrivate: MTLResourceOptions = MTLResourceOptions(MTLStorageMode::PRIVATE.0 << 4);
// pub const MTLResourceStorageModeMemoryless: MTLResourceOptions = MTLResourceOptions(MTLStorageMode::MEMORYLESS.0 << 4);
// pub const MTLResourceHazardTrackingModeUntracked: MTLResourceOptions = MTLResourceOptions(0x1 << 8);

impl MTLResourceOptions {
  pub const CPU_CACHE_MODE_DEFAULT_CACHE: MTLResourceOptions =
    MTLResourceOptions(MTLCPUCacheMode::DEFAULT_CACHE.0 << 0);
  pub const CPU_CACHE_MODE_WRITE_COMBINED: MTLResourceOptions =
    MTLResourceOptions(MTLCPUCacheMode::WRITE_COMBINED.0 << 0);
  pub const STORAGE_MODE_SHARED: MTLResourceOptions =
    MTLResourceOptions(MTLStorageMode::SHARED.0 << 4);
  pub const STORAGE_MODE_MANAGED: MTLResourceOptions =
    MTLResourceOptions(MTLStorageMode::MANAGED.0 << 4);
  pub const STORAGE_MODE_PRIVATE: MTLResourceOptions =
    MTLResourceOptions(MTLStorageMode::PRIVATE.0 << 4);
  pub const STORAGE_MODE_MEMORYLESS: MTLResourceOptions =
    MTLResourceOptions(MTLStorageMode::MEMORYLESS.0 << 4);
  pub const HAZARD_TRACKING_MODE_UNTRACKED: MTLResourceOptions = MTLResourceOptions(0x1 << 8);
}

unsafe impl objrs::marker::Zeroed for MTLResourceOptions {}
impl objrs::marker::Forgettable for MTLResourceOptions {}
