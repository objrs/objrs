#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::FSRef;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFError;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::CoreServices::CarbonCore::ComponentRecord;
use crate::CoreServices::CarbonCore::FSCatalogInfo;
use crate::CoreServices::OSServices::IconFamilyResource;
use crate::CoreServices::AE::AEDesc;
use crate::CoreServices::AE::AEKeyDesc;
use crate::MacTypes::ProcessSerialNumber;
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
pub struct OpaqueIconRef {
  opaque: u32,
}
pub type IconServicesUsageFlags = u32;
bitflags! { # [ repr ( C ) ] pub struct LSRolesMask : u32 { const kLSRolesNone = 1 ; const kLSRolesViewer = 2 ; const kLSRolesEditor = 4 ; const kLSRolesShell = 8 ; const kLSRolesAll = 4294967295 ; } }
bitflags! { # [ repr ( C ) ] pub struct LSAcceptanceFlags : u32 { const kLSAcceptDefault = 1 ; const kLSAcceptAllowLoginUI = 2 ; } }
bitflags! { # [ repr ( C ) ] pub struct LSRequestedInfo : u32 { const kLSRequestExtension = 1 ; const kLSRequestTypeCreator = 2 ; const kLSRequestBasicFlagsOnly = 4 ; const kLSRequestAppTypeFlags = 8 ; const kLSRequestAllFlags = 16 ; const kLSRequestIconAndKind = 32 ; const kLSRequestExtensionFlagsOnly = 64 ; const kLSRequestAllInfo = 4294967295 ; } }
bitflags! { # [ repr ( C ) ] pub struct LSItemInfoFlags : u32 { const kLSItemInfoIsPlainFile = 1 ; const kLSItemInfoIsPackage = 2 ; const kLSItemInfoIsApplication = 4 ; const kLSItemInfoIsContainer = 8 ; const kLSItemInfoIsAliasFile = 16 ; const kLSItemInfoIsSymlink = 32 ; const kLSItemInfoIsInvisible = 64 ; const kLSItemInfoIsNativeApp = 128 ; const kLSItemInfoIsClassicApp = 256 ; const kLSItemInfoAppPrefersNative = 512 ; const kLSItemInfoAppPrefersClassic = 1024 ; const kLSItemInfoAppIsScriptable = 2048 ; const kLSItemInfoIsVolume = 4096 ; const kLSItemInfoExtensionIsHidden = 1048576 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LSItemInfoRecord {
  pub flags: LSItemInfoFlags,
  pub filetype: u32,
  pub creator: u32,
  pub extension: *mut __CFString,
}
bitflags! { # [ repr ( C ) ] pub struct LSHandlerOptions : u32 { const kLSHandlerOptionsDefault = 0 ; const kLSHandlerOptionsIgnoreCreator = 1 ; } }
bitflags! { # [ repr ( C ) ] pub struct LSLaunchFlags : u32 { const kLSLaunchDefaults = 1 ; const kLSLaunchAndPrint = 2 ; const kLSLaunchAndDisplayErrors = 64 ; const kLSLaunchDontAddToRecents = 256 ; const kLSLaunchDontSwitch = 512 ; const kLSLaunchAsync = 65536 ; const kLSLaunchNewInstance = 524288 ; const kLSLaunchAndHide = 1048576 ; const kLSLaunchAndHideOthers = 2097152 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LSLaunchURLSpec {
  pub appURL: *mut __CFURL,
  pub itemURLs: *mut __CFArray,
  pub passThruParams: *mut AEDesc,
  pub launchFlags: LSLaunchFlags,
  pub asyncRefCon: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LSLaunchFSRefSpec {
  pub appRef: *mut FSRef,
  pub numDocs: usize,
  pub itemRefs: *mut FSRef,
  pub passThruParams: *mut AEDesc,
  pub launchFlags: LSLaunchFlags,
  pub asyncRefCon: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LSApplicationParameters {
  pub version: isize,
  pub flags: LSLaunchFlags,
  pub application: *mut FSRef,
  pub asyncLaunchRefCon: *mut c_void,
  pub environment: *mut __CFDictionary,
  pub argv: *mut __CFArray,
  pub initialEvent: *mut AEDesc,
}
extern "C" {
  pub fn GetIconRefOwners(theIconRef: *mut OpaqueIconRef, owners: *mut u16) -> i16;
  pub fn AcquireIconRef(theIconRef: *mut OpaqueIconRef) -> i16;
  pub fn ReleaseIconRef(theIconRef: *mut OpaqueIconRef) -> i16;
  pub fn GetIconRef(
    vRefNum: i16,
    creator: u32,
    iconType: u32,
    theIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn GetIconRefFromFolder(
    vRefNum: i16,
    parentFolderID: i32,
    folderID: i32,
    attributes: i8,
    accessPrivileges: i8,
    theIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn GetIconRefFromFileInfo(
    inRef: *mut FSRef,
    inFileNameLength: usize,
    inFileName: *mut u16,
    inWhichInfo: u32,
    inCatalogInfo: *mut FSCatalogInfo,
    inUsageFlags: u32,
    outIconRef: *mut *mut OpaqueIconRef,
    outLabel: *mut i16,
  ) -> i32;
  pub fn GetIconRefFromTypeInfo(
    inCreator: u32,
    inType: u32,
    inExtension: *mut __CFString,
    inMIMEType: *mut __CFString,
    inUsageFlags: u32,
    outIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn GetIconRefFromIconFamilyPtr(
    inIconFamilyPtr: *mut IconFamilyResource,
    inSize: isize,
    outIconRef: *mut *mut OpaqueIconRef,
  ) -> i32;
  pub fn GetIconRefFromComponent(
    inComponent: *mut ComponentRecord,
    outIconRef: *mut *mut OpaqueIconRef,
  ) -> i32;
  pub fn RegisterIconRefFromIconFamily(
    creator: u32,
    iconType: u32,
    iconFamily: *mut *mut IconFamilyResource,
    theIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn RegisterIconRefFromFSRef(
    creator: u32,
    iconType: u32,
    iconFile: *mut FSRef,
    theIconRef: *mut *mut OpaqueIconRef,
  ) -> i32;
  pub fn UnregisterIconRef(creator: u32, iconType: u32) -> i16;
  pub fn UpdateIconRef(theIconRef: *mut OpaqueIconRef) -> i16;
  pub fn OverrideIconRef(oldIconRef: *mut OpaqueIconRef, newIconRef: *mut OpaqueIconRef) -> i16;
  pub fn RemoveIconRefOverride(theIconRef: *mut OpaqueIconRef) -> i16;
  pub fn CompositeIconRef(
    backgroundIconRef: *mut OpaqueIconRef,
    foregroundIconRef: *mut OpaqueIconRef,
    compositeIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn IsIconRefComposite(
    compositeIconRef: *mut OpaqueIconRef,
    backgroundIconRef: *mut *mut OpaqueIconRef,
    foregroundIconRef: *mut *mut OpaqueIconRef,
  ) -> i16;
  pub fn IsValidIconRef(theIconRef: *mut OpaqueIconRef) -> u8;
  pub fn IsDataAvailableInIconRef(inIconKind: u32, inIconRef: *mut OpaqueIconRef) -> u8;
  pub fn SetCustomIconsEnabled(vRefNum: i16, enableCustomIcons: u8) -> i16;
  pub fn GetCustomIconsEnabled(vRefNum: i16, customIconsEnabled: *mut u8) -> i16;
  pub fn ReadIconFromFSRef(ref_: *mut FSRef, iconFamily: *mut *mut *mut IconFamilyResource) -> i32;
  pub fn LSCopyDefaultApplicationURLForURL(
    inURL: *mut __CFURL,
    inRoleMask: LSRolesMask,
    outError: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn LSCopyDefaultApplicationURLForContentType(
    inContentType: *mut __CFString,
    inRoleMask: LSRolesMask,
    outError: *mut *mut __CFError,
  ) -> *mut __CFURL;
  pub fn LSCopyApplicationURLsForBundleIdentifier(
    inBundleIdentifier: *mut __CFString,
    outError: *mut *mut __CFError,
  ) -> *mut __CFArray;
  pub fn LSCopyApplicationURLsForURL(
    inURL: *mut __CFURL,
    inRoleMask: LSRolesMask,
  ) -> *mut __CFArray;
  pub fn LSCanURLAcceptURL(
    inItemURL: *mut __CFURL,
    inTargetURL: *mut __CFURL,
    inRoleMask: LSRolesMask,
    inFlags: LSAcceptanceFlags,
    outAcceptsItem: *mut u8,
  ) -> i32;
  pub fn LSRegisterURL(inURL: *mut __CFURL, inUpdate: u8) -> i32;
  pub fn LSCopyDefaultRoleHandlerForContentType(
    inContentType: *mut __CFString,
    inRole: LSRolesMask,
  ) -> *mut __CFString;
  pub fn LSCopyAllRoleHandlersForContentType(
    inContentType: *mut __CFString,
    inRole: LSRolesMask,
  ) -> *mut __CFArray;
  pub fn LSSetDefaultRoleHandlerForContentType(
    inContentType: *mut __CFString,
    inRole: LSRolesMask,
    inHandlerBundleID: *mut __CFString,
  ) -> i32;
  pub fn LSCopyDefaultHandlerForURLScheme(inURLScheme: *mut __CFString) -> *mut __CFString;
  pub fn LSCopyAllHandlersForURLScheme(inURLScheme: *mut __CFString) -> *mut __CFArray;
  pub fn LSSetDefaultHandlerForURLScheme(
    inURLScheme: *mut __CFString,
    inHandlerBundleID: *mut __CFString,
  ) -> i32;
  pub fn LSCopyItemInfoForURL(
    inURL: *mut __CFURL,
    inWhichInfo: LSRequestedInfo,
    outItemInfo: *mut LSItemInfoRecord,
  ) -> i32;
  pub fn LSCopyItemInfoForRef(
    inItemRef: *mut FSRef,
    inWhichInfo: LSRequestedInfo,
    outItemInfo: *mut LSItemInfoRecord,
  ) -> i32;
  pub fn LSGetExtensionInfo(
    inNameLen: usize,
    inNameBuffer: *mut u16,
    outExtStartIndex: *mut usize,
  ) -> i32;
  pub fn LSCopyDisplayNameForRef(inRef: *mut FSRef, outDisplayName: *mut *mut __CFString) -> i32;
  pub fn LSCopyDisplayNameForURL(inURL: *mut __CFURL, outDisplayName: *mut *mut __CFString) -> i32;
  pub fn LSSetExtensionHiddenForRef(inRef: *mut FSRef, inHide: u8) -> i32;
  pub fn LSSetExtensionHiddenForURL(inURL: *mut __CFURL, inHide: u8) -> i32;
  pub fn LSCopyKindStringForRef(inFSRef: *mut FSRef, outKindString: *mut *mut __CFString) -> i32;
  pub fn LSCopyKindStringForURL(inURL: *mut __CFURL, outKindString: *mut *mut __CFString) -> i32;
  pub fn LSCopyKindStringForTypeInfo(
    inType: u32,
    inCreator: u32,
    inExtension: *mut __CFString,
    outKindString: *mut *mut __CFString,
  ) -> i32;
  pub fn LSCopyKindStringForMIMEType(
    inMIMEType: *mut __CFString,
    outKindString: *mut *mut __CFString,
  ) -> i32;
  pub fn LSGetApplicationForItem(
    inItemRef: *mut FSRef,
    inRoleMask: LSRolesMask,
    outAppRef: *mut FSRef,
    outAppURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSGetApplicationForInfo(
    inType: u32,
    inCreator: u32,
    inExtension: *mut __CFString,
    inRoleMask: LSRolesMask,
    outAppRef: *mut FSRef,
    outAppURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSCopyApplicationForMIMEType(
    inMIMEType: *mut __CFString,
    inRoleMask: LSRolesMask,
    outAppURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSGetApplicationForURL(
    inURL: *mut __CFURL,
    inRoleMask: LSRolesMask,
    outAppRef: *mut FSRef,
    outAppURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSFindApplicationForInfo(
    inCreator: u32,
    inBundleID: *mut __CFString,
    inName: *mut __CFString,
    outAppRef: *mut FSRef,
    outAppURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSCanRefAcceptItem(
    inItemFSRef: *mut FSRef,
    inTargetRef: *mut FSRef,
    inRoleMask: LSRolesMask,
    inFlags: LSAcceptanceFlags,
    outAcceptsItem: *mut u8,
  ) -> i32;
  pub fn LSRegisterFSRef(inRef: *mut FSRef, inUpdate: u8) -> i32;
  pub fn LSCopyItemAttribute(
    inItem: *mut FSRef,
    inRoles: LSRolesMask,
    inAttributeName: *mut __CFString,
    outValue: *mut *mut c_void,
  ) -> i32;
  pub fn LSCopyItemAttributes(
    inItem: *mut FSRef,
    inRoles: LSRolesMask,
    inAttributeNames: *mut __CFArray,
    outValues: *mut *mut __CFDictionary,
  ) -> i32;
  pub fn LSSetItemAttribute(
    inItem: *mut FSRef,
    inRoles: LSRolesMask,
    inAttributeName: *mut __CFString,
    inValue: *mut c_void,
  ) -> i32;
  pub fn LSGetHandlerOptionsForContentType(inContentType: *mut __CFString) -> LSHandlerOptions;
  pub fn LSSetHandlerOptionsForContentType(
    inContentType: *mut __CFString,
    inOptions: LSHandlerOptions,
  ) -> i32;
  pub fn LSOpenCFURLRef(inURL: *mut __CFURL, outLaunchedURL: *mut *mut __CFURL) -> i32;
  pub fn LSOpenFromURLSpec(
    inLaunchSpec: *mut LSLaunchURLSpec,
    outLaunchedURL: *mut *mut __CFURL,
  ) -> i32;
  pub fn LSOpenFSRef(inRef: *mut FSRef, outLaunchedRef: *mut FSRef) -> i32;
  pub fn LSOpenFromRefSpec(inLaunchSpec: *mut LSLaunchFSRefSpec, outLaunchedRef: *mut FSRef)
    -> i32;
  pub fn LSOpenApplication(
    appParams: *mut LSApplicationParameters,
    outPSN: *mut ProcessSerialNumber,
  ) -> i32;
  pub fn LSOpenItemsWithRole(
    inItems: *mut FSRef,
    inItemCount: isize,
    inRole: LSRolesMask,
    inAEParam: *mut AEKeyDesc,
    inAppParams: *mut LSApplicationParameters,
    outPSNs: *mut ProcessSerialNumber,
    inMaxPSNCount: isize,
  ) -> i32;
  pub fn LSOpenURLsWithRole(
    inURLs: *mut __CFArray,
    inRole: LSRolesMask,
    inAEParam: *mut AEKeyDesc,
    inAppParams: *mut LSApplicationParameters,
    outPSNs: *mut ProcessSerialNumber,
    inMaxPSNCount: isize,
  ) -> i32;
  pub fn UTTypeCreatePreferredIdentifierForTag(
    inTagClass: *mut __CFString,
    inTag: *mut __CFString,
    inConformingToUTI: *mut __CFString,
  ) -> *mut __CFString;
  pub fn UTTypeCreateAllIdentifiersForTag(
    inTagClass: *mut __CFString,
    inTag: *mut __CFString,
    inConformingToUTI: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn UTTypeCopyPreferredTagWithClass(
    inUTI: *mut __CFString,
    inTagClass: *mut __CFString,
  ) -> *mut __CFString;
  pub fn UTTypeCopyAllTagsWithClass(
    inUTI: *mut __CFString,
    inTagClass: *mut __CFString,
  ) -> *mut __CFArray;
  pub fn UTTypeEqual(inUTI1: *mut __CFString, inUTI2: *mut __CFString) -> u8;
  pub fn UTTypeConformsTo(inUTI: *mut __CFString, inConformsToUTI: *mut __CFString) -> u8;
  pub fn UTTypeCopyDescription(inUTI: *mut __CFString) -> *mut __CFString;
  pub fn UTTypeIsDeclared(inUTI: *mut __CFString) -> u8;
  pub fn UTTypeIsDynamic(inUTI: *mut __CFString) -> u8;
  pub fn UTTypeCopyDeclaration(inUTI: *mut __CFString) -> *mut __CFDictionary;
  pub fn UTTypeCopyDeclaringBundleURL(inUTI: *mut __CFString) -> *mut __CFURL;
  pub fn UTCreateStringForOSType(inOSType: u32) -> *mut __CFString;
  pub fn UTGetOSTypeFromString(inString: *mut __CFString) -> u32;
}
