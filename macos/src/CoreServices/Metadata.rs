#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::CFArrayCallBacks;
use crate::CoreFoundation::CFComparisonResult;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::CoreFoundation::__CFUUID;
use crate::NSObject::NSObject;
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
pub struct __MDItem {
  opaque: u32,
}
#[repr(C)]
pub struct __MDQuery {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MDQueryOptionFlags {
  kMDQuerySynchronous = 1,
  kMDQueryWantsUpdates = 4,
  kMDQueryAllowFSTranslation = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MDQueryBatchingParams {
  pub first_max_num: usize,
  pub first_max_ms: usize,
  pub progress_max_num: usize,
  pub progress_max_ms: usize,
  pub update_max_num: usize,
  pub update_max_ms: usize,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MDQuerySortOptionFlags {
  kMDQueryReverseSortOrderFlag = 1,
}
#[repr(C)]
pub struct __MDLabel {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MDLabelDomain {
  kMDLabelUserDomain = 0,
  kMDLabelLocalDomain = 1,
}
extern "C" {
  pub fn MDItemGetTypeID() -> usize;
  pub fn MDItemCreate(allocator: *mut __CFAllocator, path: *mut __CFString) -> *mut __MDItem;
  pub fn MDItemCreateWithURL(allocator: *mut __CFAllocator, url: *mut __CFURL) -> *mut __MDItem;
  pub fn MDItemsCreateWithURLs(
    allocator: *mut __CFAllocator,
    urls: *mut __CFArray,
  ) -> *mut __CFArray;
  pub fn MDItemCopyAttribute(item: *mut __MDItem, name: *mut __CFString) -> *mut c_void;
  pub fn MDItemCopyAttributes(item: *mut __MDItem, names: *mut __CFArray) -> *mut __CFDictionary;
  pub fn MDItemCopyAttributeList(item: *mut __MDItem, ...) -> *mut __CFDictionary;
  pub fn MDItemCopyAttributeNames(item: *mut __MDItem) -> *mut __CFArray;
  pub fn MDItemsCopyAttributes(items: *mut __CFArray, names: *mut __CFArray) -> *mut __CFArray;
  pub fn __MDItemCopyAttributesEllipsis1(item: *mut __MDItem, ...) -> *mut __CFDictionary;
  pub fn MDQueryGetTypeID() -> usize;
  pub fn MDQueryCreate(
    allocator: *mut __CFAllocator,
    queryString: *mut __CFString,
    valueListAttrs: *mut __CFArray,
    sortingAttrs: *mut __CFArray,
  ) -> *mut __MDQuery;
  pub fn MDQueryCreateSubset(
    allocator: *mut __CFAllocator,
    query: *mut __MDQuery,
    queryString: *mut __CFString,
    valueListAttrs: *mut __CFArray,
    sortingAttrs: *mut __CFArray,
  ) -> *mut __MDQuery;
  pub fn MDQueryCreateForItems(
    allocator: *mut __CFAllocator,
    queryString: *mut __CFString,
    valueListAttrs: *mut __CFArray,
    sortingAttrs: *mut __CFArray,
    items: *mut __CFArray,
  ) -> *mut __MDQuery;
  pub fn MDQueryCopyQueryString(query: *mut __MDQuery) -> *mut __CFString;
  pub fn MDQueryCopyValueListAttributes(query: *mut __MDQuery) -> *mut __CFArray;
  pub fn MDQueryCopySortingAttributes(query: *mut __MDQuery) -> *mut __CFArray;
  pub fn MDQueryGetBatchingParameters(query: *mut __MDQuery) -> MDQueryBatchingParams;
  pub fn MDQuerySetBatchingParameters(query: *mut __MDQuery, params: MDQueryBatchingParams) -> ();
  pub fn MDQuerySetCreateResultFunction(
    query: *mut __MDQuery,
    func: Option<extern "C" fn(*mut __MDQuery, *mut __MDItem, *mut c_void) -> *mut c_void>,
    context: *mut c_void,
    cb: *mut CFArrayCallBacks,
  ) -> ();
  pub fn MDQuerySetCreateValueFunction(
    query: *mut __MDQuery,
    func: Option<
      extern "C" fn(*mut __MDQuery, *mut __CFString, *mut c_void, *mut c_void) -> *mut c_void,
    >,
    context: *mut c_void,
    cb: *mut CFArrayCallBacks,
  ) -> ();
  pub fn MDQuerySetDispatchQueue(query: *mut __MDQuery, queue: *mut NSObject) -> ();
  pub fn MDQueryExecute(query: *mut __MDQuery, optionFlags: usize) -> u8;
  pub fn MDQueryStop(query: *mut __MDQuery) -> ();
  pub fn MDQueryDisableUpdates(query: *mut __MDQuery) -> ();
  pub fn MDQueryEnableUpdates(query: *mut __MDQuery) -> ();
  pub fn MDQueryIsGatheringComplete(query: *mut __MDQuery) -> u8;
  pub fn MDQueryGetResultCount(query: *mut __MDQuery) -> isize;
  pub fn MDQueryGetResultAtIndex(query: *mut __MDQuery, idx: isize) -> *mut c_void;
  pub fn MDQueryGetIndexOfResult(query: *mut __MDQuery, result: *mut c_void) -> isize;
  pub fn MDQueryGetAttributeValueOfResultAtIndex(
    query: *mut __MDQuery,
    name: *mut __CFString,
    idx: isize,
  ) -> *mut c_void;
  pub fn MDQueryCopyValuesOfAttribute(
    query: *mut __MDQuery,
    name: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn MDQueryGetCountOfResultsWithAttributeValue(
    query: *mut __MDQuery,
    name: *mut __CFString,
    value: *mut c_void,
  ) -> isize;
  pub fn MDQuerySetSortOrder(query: *mut __MDQuery, sortingAttrs: *mut __CFArray) -> u8;
  pub fn MDQuerySetSortOptionFlagsForAttribute(
    query: *mut __MDQuery,
    fieldName: *mut __CFString,
    flags: u32,
  ) -> u8;
  pub fn MDQueryGetSortOptionFlagsForAttribute(
    query: *mut __MDQuery,
    fieldName: *mut __CFString,
  ) -> u32;
  pub fn MDQuerySetSortComparator(
    query: *mut __MDQuery,
    comparator: Option<
      extern "C" fn(*mut *mut c_void, *mut *mut c_void, *mut c_void) -> CFComparisonResult,
    >,
    context: *mut c_void,
  ) -> ();
  pub fn MDQuerySetSortComparatorBlock(query: *mut __MDQuery, comparator: ()) -> ();
  pub fn MDQuerySetSearchScope(
    query: *mut __MDQuery,
    scopeDirectories: *mut __CFArray,
    scopeOptions: u32,
  ) -> ();
  pub fn MDQuerySetMaxCount(query: *mut __MDQuery, size: isize) -> ();
  pub fn MDLabelGetTypeID() -> usize;
  pub fn MDItemCopyLabels(item: *mut __MDItem) -> *mut __CFArray;
  pub fn MDItemSetLabel(item: *mut __MDItem, label: *mut __MDLabel) -> u8;
  pub fn MDItemRemoveLabel(item: *mut __MDItem, label: *mut __MDLabel) -> u8;
  pub fn MDLabelCreate(
    allocator: *mut __CFAllocator,
    displayName: *mut __CFString,
    kind: *mut __CFString,
    domain: MDLabelDomain,
  ) -> *mut __MDLabel;
  pub fn MDLabelCopyAttribute(label: *mut __MDLabel, name: *mut __CFString) -> *mut c_void;
  pub fn MDLabelCopyAttributeName(label: *mut __MDLabel) -> *mut __CFString;
  pub fn MDLabelDelete(label: *mut __MDLabel) -> u8;
  pub fn MDLabelSetAttributes(label: *mut __MDLabel, attrs: *mut __CFDictionary) -> u8;
  pub fn MDCopyLabelKinds() -> *mut __CFArray;
  pub fn MDCopyLabelsMatchingExpression(simpleQueryString: *mut __CFString) -> *mut __CFArray;
  pub fn MDCopyLabelsWithKind(kind: *mut __CFString) -> *mut __CFArray;
  pub fn MDCopyLabelWithUUID(labelUUID: *mut __CFUUID) -> *mut __MDLabel;
  pub fn MDSchemaCopyAttributesForContentType(
    contentTypeUTI: *mut __CFString,
  ) -> *mut __CFDictionary;
  pub fn MDSchemaCopyMetaAttributesForAttribute(name: *mut __CFString) -> *mut __CFDictionary;
  pub fn MDSchemaCopyAllAttributes() -> *mut __CFArray;
  pub fn MDSchemaCopyDisplayNameForAttribute(name: *mut __CFString) -> *mut __CFString;
  pub fn MDSchemaCopyDisplayDescriptionForAttribute(name: *mut __CFString) -> *mut __CFString;
}
