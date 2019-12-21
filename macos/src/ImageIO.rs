#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFData;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFError;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::CoreGraphics::CGDataConsumer;
use crate::CoreGraphics::CGDataProvider;
use crate::CoreGraphics::CGImage;
#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use objrs::objrs;
#[allow(unused_imports)]
use std::mem;
#[allow(unused_imports)]
use std::ptr;
#[repr(C)]
pub struct CGImageSource {
  opaque: u32,
}
#[repr(C)]
pub struct CGImageMetadata {
  opaque: u32,
}
#[repr(C)]
pub struct CGImageMetadataTag {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGImageMetadataType {
  kCGImageMetadataTypeInvalid = -1,
  kCGImageMetadataTypeDefault = 0,
  kCGImageMetadataTypeString = 1,
  kCGImageMetadataTypeArrayUnordered = 2,
  kCGImageMetadataTypeArrayOrdered = 3,
  kCGImageMetadataTypeAlternateArray = 4,
  kCGImageMetadataTypeAlternateText = 5,
  kCGImageMetadataTypeStructure = 6,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGImageMetadataErrors {
  kCGImageMetadataErrorUnknown = 0,
  kCGImageMetadataErrorUnsupportedFormat = 1,
  kCGImageMetadataErrorBadArgument = 2,
  kCGImageMetadataErrorConflictingArguments = 3,
  kCGImageMetadataErrorPrefixConflict = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CGImageSourceStatus {
  kCGImageStatusUnexpectedEOF = -5,
  kCGImageStatusInvalidData = -4,
  kCGImageStatusUnknownType = -3,
  kCGImageStatusReadingHeader = -2,
  kCGImageStatusIncomplete = -1,
  kCGImageStatusComplete = 0,
}
#[repr(C)]
pub struct CGImageDestination {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CGImagePropertyOrientation {
  kCGImagePropertyOrientationUp = 1,
  kCGImagePropertyOrientationUpMirrored = 2,
  kCGImagePropertyOrientationDown = 3,
  kCGImagePropertyOrientationDownMirrored = 4,
  kCGImagePropertyOrientationLeftMirrored = 5,
  kCGImagePropertyOrientationRight = 6,
  kCGImagePropertyOrientationRightMirrored = 7,
  kCGImagePropertyOrientationLeft = 8,
}
#[cfg(feature = "RK_ImageIO")]
#[link(name = "ImageIO", kind = "framework")]
extern "C" {
  pub fn CGImageMetadataGetTypeID() -> usize;
  pub fn CGImageMetadataCreateMutable() -> *mut CGImageMetadata;
  pub fn CGImageMetadataCreateMutableCopy(metadata: *mut CGImageMetadata) -> *mut CGImageMetadata;
  pub fn CGImageMetadataTagGetTypeID() -> usize;
  pub fn CGImageMetadataTagCreate(
    xmlns: *mut __CFString,
    prefix: *mut __CFString,
    name: *mut __CFString,
    type_: CGImageMetadataType,
    value: *mut c_void,
  ) -> *mut CGImageMetadataTag;
  pub fn CGImageMetadataTagCopyNamespace(tag: *mut CGImageMetadataTag) -> *mut __CFString;
  pub fn CGImageMetadataTagCopyPrefix(tag: *mut CGImageMetadataTag) -> *mut __CFString;
  pub fn CGImageMetadataTagCopyName(tag: *mut CGImageMetadataTag) -> *mut __CFString;
  pub fn CGImageMetadataTagCopyValue(tag: *mut CGImageMetadataTag) -> *mut c_void;
  pub fn CGImageMetadataTagGetType(tag: *mut CGImageMetadataTag) -> CGImageMetadataType;
  pub fn CGImageMetadataTagCopyQualifiers(tag: *mut CGImageMetadataTag) -> *mut __CFArray;
  pub fn CGImageMetadataCopyTags(metadata: *mut CGImageMetadata) -> *mut __CFArray;
  pub fn CGImageMetadataCopyTagWithPath(
    metadata: *mut CGImageMetadata,
    parent: *mut CGImageMetadataTag,
    path: *mut __CFString,
  ) -> *mut CGImageMetadataTag;
  pub fn CGImageMetadataCopyStringValueWithPath(
    metadata: *mut CGImageMetadata,
    parent: *mut CGImageMetadataTag,
    path: *mut __CFString,
  ) -> *mut __CFString;
  pub fn CGImageMetadataRegisterNamespaceForPrefix(
    metadata: *mut CGImageMetadata,
    xmlns: *mut __CFString,
    prefix: *mut __CFString,
    err: *mut *mut __CFError,
  ) -> bool;
  pub fn CGImageMetadataSetTagWithPath(
    metadata: *mut CGImageMetadata,
    parent: *mut CGImageMetadataTag,
    path: *mut __CFString,
    tag: *mut CGImageMetadataTag,
  ) -> bool;
  pub fn CGImageMetadataSetValueWithPath(
    metadata: *mut CGImageMetadata,
    parent: *mut CGImageMetadataTag,
    path: *mut __CFString,
    value: *mut c_void,
  ) -> bool;
  pub fn CGImageMetadataRemoveTagWithPath(
    metadata: *mut CGImageMetadata,
    parent: *mut CGImageMetadataTag,
    path: *mut __CFString,
  ) -> bool;
  pub fn CGImageMetadataEnumerateTagsUsingBlock(
    metadata: *mut CGImageMetadata,
    rootPath: *mut __CFString,
    options: *mut __CFDictionary,
    block: (),
  ) -> ();
  pub fn CGImageMetadataCopyTagMatchingImageProperty(
    metadata: *mut CGImageMetadata,
    dictionaryName: *mut __CFString,
    propertyName: *mut __CFString,
  ) -> *mut CGImageMetadataTag;
  pub fn CGImageMetadataSetValueMatchingImageProperty(
    metadata: *mut CGImageMetadata,
    dictionaryName: *mut __CFString,
    propertyName: *mut __CFString,
    value: *mut c_void,
  ) -> bool;
  pub fn CGImageMetadataCreateXMPData(
    metadata: *mut CGImageMetadata,
    options: *mut __CFDictionary,
  ) -> *mut __CFData;
  pub fn CGImageMetadataCreateFromXMPData(data: *mut __CFData) -> *mut CGImageMetadata;
  pub fn CGImageSourceGetTypeID() -> usize;
  pub fn CGImageSourceCopyTypeIdentifiers() -> *mut __CFArray;
  pub fn CGImageSourceCreateWithDataProvider(
    provider: *mut CGDataProvider,
    options: *mut __CFDictionary,
  ) -> *mut CGImageSource;
  pub fn CGImageSourceCreateWithData(
    data: *mut __CFData,
    options: *mut __CFDictionary,
  ) -> *mut CGImageSource;
  pub fn CGImageSourceCreateWithURL(
    url: *mut __CFURL,
    options: *mut __CFDictionary,
  ) -> *mut CGImageSource;
  pub fn CGImageSourceGetType(isrc: *mut CGImageSource) -> *mut __CFString;
  pub fn CGImageSourceGetCount(isrc: *mut CGImageSource) -> usize;
  pub fn CGImageSourceCopyProperties(
    isrc: *mut CGImageSource,
    options: *mut __CFDictionary,
  ) -> *mut __CFDictionary;
  pub fn CGImageSourceCopyPropertiesAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
    options: *mut __CFDictionary,
  ) -> *mut __CFDictionary;
  pub fn CGImageSourceCopyMetadataAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImageMetadata;
  pub fn CGImageSourceCreateImageAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImage;
  pub fn CGImageSourceRemoveCacheAtIndex(isrc: *mut CGImageSource, index: usize) -> ();
  pub fn CGImageSourceCreateThumbnailAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImage;
  pub fn CGImageSourceCreateIncremental(options: *mut __CFDictionary) -> *mut CGImageSource;
  pub fn CGImageSourceUpdateData(isrc: *mut CGImageSource, data: *mut __CFData, final_: bool)
    -> ();
  pub fn CGImageSourceUpdateDataProvider(
    isrc: *mut CGImageSource,
    provider: *mut CGDataProvider,
    final_: bool,
  ) -> ();
  pub fn CGImageSourceGetStatus(isrc: *mut CGImageSource) -> CGImageSourceStatus;
  pub fn CGImageSourceGetStatusAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
  ) -> CGImageSourceStatus;
  pub fn CGImageSourceGetPrimaryImageIndex(isrc: *mut CGImageSource) -> usize;
  pub fn CGImageSourceCopyAuxiliaryDataInfoAtIndex(
    isrc: *mut CGImageSource,
    index: usize,
    auxiliaryImageDataType: *mut __CFString,
  ) -> *mut __CFDictionary;
  pub fn CGImageDestinationGetTypeID() -> usize;
  pub fn CGImageDestinationCopyTypeIdentifiers() -> *mut __CFArray;
  pub fn CGImageDestinationCreateWithDataConsumer(
    consumer: *mut CGDataConsumer,
    type_: *mut __CFString,
    count: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImageDestination;
  pub fn CGImageDestinationCreateWithData(
    data: *mut __CFData,
    type_: *mut __CFString,
    count: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImageDestination;
  pub fn CGImageDestinationCreateWithURL(
    url: *mut __CFURL,
    type_: *mut __CFString,
    count: usize,
    options: *mut __CFDictionary,
  ) -> *mut CGImageDestination;
  pub fn CGImageDestinationSetProperties(
    idst: *mut CGImageDestination,
    properties: *mut __CFDictionary,
  ) -> ();
  pub fn CGImageDestinationAddImage(
    idst: *mut CGImageDestination,
    image: *mut CGImage,
    properties: *mut __CFDictionary,
  ) -> ();
  pub fn CGImageDestinationAddImageFromSource(
    idst: *mut CGImageDestination,
    isrc: *mut CGImageSource,
    index: usize,
    properties: *mut __CFDictionary,
  ) -> ();
  pub fn CGImageDestinationFinalize(idst: *mut CGImageDestination) -> bool;
  pub fn CGImageDestinationAddImageAndMetadata(
    idst: *mut CGImageDestination,
    image: *mut CGImage,
    metadata: *mut CGImageMetadata,
    options: *mut __CFDictionary,
  ) -> ();
  pub fn CGImageDestinationCopyImageSource(
    idst: *mut CGImageDestination,
    isrc: *mut CGImageSource,
    options: *mut __CFDictionary,
    err: *mut *mut __CFError,
  ) -> bool;
  pub fn CGImageDestinationAddAuxiliaryDataInfo(
    idst: *mut CGImageDestination,
    auxiliaryImageDataType: *mut __CFString,
    auxiliaryDataInfoDictionary: *mut __CFDictionary,
  ) -> ();
}
