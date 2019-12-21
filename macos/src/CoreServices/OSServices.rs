#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::FSRef;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFData;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFError;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFTree;
use crate::CoreFoundation::__CFURL;
use crate::CoreFoundation::__CFUUID;
use crate::CoreServices::CarbonCore::AliasRecord;
use crate::Security::AuthorizationOpaqueRef;
use crate::Security::OpaqueSecCertificateRef;
use crate::Security::OpaqueSecKeychainItemRef;
use crate::Security::OpaqueSecKeychainRef;
use crate::Security::OpaqueSecKeychainSearchRef;
use crate::Security::SecKeychainAttribute;
use crate::Security::SecKeychainAttributeList;
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
pub struct __CSIdentityAuthority {
  opaque: u32,
}
#[repr(C)]
pub struct __CSIdentity {
  opaque: u32,
}
#[repr(C)]
pub struct __CSIdentityQuery {
  opaque: u32,
}
pub type CSIdentityClass = isize;
pub type CSIdentityFlags = usize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSIdentityClientContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub statusUpdated:
    Option<extern "C" fn(*mut __CSIdentity, isize, *mut __CFError, *mut c_void) -> ()>,
}
pub type CSIdentityQueryFlags = usize;
pub type CSIdentityQueryStringComparisonMethod = isize;
pub type CSIdentityQueryEvent = isize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSIdentityQueryClientContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retainInfo: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub releaseInfo: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyInfoDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
  pub receiveEvent: Option<
    extern "C" fn(*mut __CSIdentityQuery, isize, *mut __CFArray, *mut __CFError, *mut c_void) -> (),
  >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IconFamilyElement {
  pub elementType: u32,
  pub elementSize: i32,
  pub elementData: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IconFamilyResource {
  pub resourceType: u32,
  pub resourceSize: i32,
  pub elements: [IconFamilyElement; 1],
}
#[repr(C)]
pub struct SleepQRec {
  opaque: u32,
}
pub type SleepQUPP = Option<extern "C" fn(isize, *mut SleepQRec) -> isize>;
pub type KCRef = *mut OpaqueSecKeychainRef;
pub type KCItemRef = *mut OpaqueSecKeychainItemRef;
pub type KCSearchRef = *mut OpaqueSecKeychainSearchRef;
pub type KCAttribute = SecKeychainAttribute;
pub type KCAttributeList = SecKeychainAttributeList;
pub type KCAttrType = u32;
pub type KCStatus = u32;
pub type KCEvent = u16;
pub type KCEventMask = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KCCallbackInfo {
  pub version: u32,
  pub item: *mut OpaqueSecKeychainItemRef,
  pub processID: [i32; 2],
  pub event: [i32; 4],
  pub keychain: *mut OpaqueSecKeychainRef,
}
pub type KCItemClass = u32;
pub type KCItemAttr = u32;
pub type KCAuthType = u32;
pub type KCProtocolType = u32;
pub type KCCertAddOptions = u32;
pub type KCVerifyStopOn = u16;
pub type KCCertSearchOptions = u32;
pub type KCCallbackUPP = Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum WSTypeID {
  eWSUnknownType = 0,
  eWSNullType = 1,
  eWSBooleanType = 2,
  eWSIntegerType = 3,
  eWSDoubleType = 4,
  eWSStringType = 5,
  eWSDateType = 6,
  eWSDataType = 7,
  eWSArrayType = 8,
  eWSDictionaryType = 9,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WSClientContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct OpaqueWSMethodInvocationRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueWSProtocolHandlerRef {
  opaque: u32,
}
extern "C" {
  pub fn CSIdentityAuthorityGetTypeID() -> usize;
  pub fn CSGetDefaultIdentityAuthority() -> *mut __CSIdentityAuthority;
  pub fn CSGetLocalIdentityAuthority() -> *mut __CSIdentityAuthority;
  pub fn CSGetManagedIdentityAuthority() -> *mut __CSIdentityAuthority;
  pub fn CSIdentityAuthorityCopyLocalizedName(
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CFString;
  pub fn CSIdentityGetTypeID() -> usize;
  pub fn CSIdentityCreate(
    allocator: *mut __CFAllocator,
    identityClass: isize,
    fullName: *mut __CFString,
    posixName: *mut __CFString,
    flags: usize,
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CSIdentity;
  pub fn CSIdentityCreateCopy(
    allocator: *mut __CFAllocator,
    identity: *mut __CSIdentity,
  ) -> *mut __CSIdentity;
  pub fn CSIdentityGetClass(identity: *mut __CSIdentity) -> isize;
  pub fn CSIdentityGetAuthority(identity: *mut __CSIdentity) -> *mut __CSIdentityAuthority;
  pub fn CSIdentityGetUUID(identity: *mut __CSIdentity) -> *mut __CFUUID;
  pub fn CSIdentityGetFullName(identity: *mut __CSIdentity) -> *mut __CFString;
  pub fn CSIdentityGetPosixID(identity: *mut __CSIdentity) -> u32;
  pub fn CSIdentityGetPosixName(identity: *mut __CSIdentity) -> *mut __CFString;
  pub fn CSIdentityGetEmailAddress(identity: *mut __CSIdentity) -> *mut __CFString;
  pub fn CSIdentityGetImageURL(identity: *mut __CSIdentity) -> *mut __CFURL;
  pub fn CSIdentityGetImageData(identity: *mut __CSIdentity) -> *mut __CFData;
  pub fn CSIdentityGetImageDataType(identity: *mut __CSIdentity) -> *mut __CFString;
  pub fn CSIdentityGetAliases(identity: *mut __CSIdentity) -> *mut __CFArray;
  pub fn CSIdentityIsMemberOfGroup(identity: *mut __CSIdentity, group: *mut __CSIdentity) -> u8;
  pub fn CSIdentityIsHidden(identity: *mut __CSIdentity) -> u8;
  pub fn CSIdentityCreatePersistentReference(
    allocator: *mut __CFAllocator,
    identity: *mut __CSIdentity,
  ) -> *mut __CFData;
  pub fn CSIdentityIsEnabled(user: *mut __CSIdentity) -> u8;
  pub fn CSIdentityAuthenticateUsingPassword(
    user: *mut __CSIdentity,
    password: *mut __CFString,
  ) -> u8;
  pub fn CSIdentityGetCertificate(user: *mut __CSIdentity) -> *mut OpaqueSecCertificateRef;
  pub fn CSIdentityCreateGroupMembershipQuery(
    allocator: *mut __CFAllocator,
    group: *mut __CSIdentity,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentitySetFullName(identity: *mut __CSIdentity, fullName: *mut __CFString) -> ();
  pub fn CSIdentitySetEmailAddress(
    identity: *mut __CSIdentity,
    emailAddress: *mut __CFString,
  ) -> ();
  pub fn CSIdentitySetImageURL(identity: *mut __CSIdentity, url: *mut __CFURL) -> ();
  pub fn CSIdentitySetImageData(
    identity: *mut __CSIdentity,
    imageData: *mut __CFData,
    imageDataType: *mut __CFString,
  ) -> ();
  pub fn CSIdentityAddAlias(identity: *mut __CSIdentity, alias: *mut __CFString) -> ();
  pub fn CSIdentityRemoveAlias(identity: *mut __CSIdentity, alias: *mut __CFString) -> ();
  pub fn CSIdentityAddMember(group: *mut __CSIdentity, member: *mut __CSIdentity) -> ();
  pub fn CSIdentityRemoveMember(group: *mut __CSIdentity, member: *mut __CSIdentity) -> ();
  pub fn CSIdentitySetIsEnabled(user: *mut __CSIdentity, isEnabled: u8) -> ();
  pub fn CSIdentitySetPassword(user: *mut __CSIdentity, password: *mut __CFString) -> ();
  pub fn CSIdentitySetCertificate(
    user: *mut __CSIdentity,
    certificate: *mut OpaqueSecCertificateRef,
  ) -> ();
  pub fn CSIdentityDelete(identity: *mut __CSIdentity) -> ();
  pub fn CSIdentityCommit(
    identity: *mut __CSIdentity,
    authorization: *mut AuthorizationOpaqueRef,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CSIdentityCommitAsynchronously(
    identity: *mut __CSIdentity,
    clientContext: *mut CSIdentityClientContext,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
    authorization: *mut AuthorizationOpaqueRef,
  ) -> u8;
  pub fn CSIdentityIsCommitting(identity: *mut __CSIdentity) -> u8;
  pub fn CSIdentityRemoveClient(identity: *mut __CSIdentity) -> ();
  pub fn CSIdentityQueryGetTypeID() -> usize;
  pub fn CSIdentityQueryCreate(
    allocator: *mut __CFAllocator,
    identityClass: isize,
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCreateForName(
    allocator: *mut __CFAllocator,
    name: *mut __CFString,
    comparisonMethod: isize,
    identityClass: isize,
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCreateForUUID(
    allocator: *mut __CFAllocator,
    uuid: *mut __CFUUID,
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCreateForPosixID(
    allocator: *mut __CFAllocator,
    posixID: u32,
    identityClass: isize,
    authority: *mut __CSIdentityAuthority,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCreateForPersistentReference(
    allocator: *mut __CFAllocator,
    referenceData: *mut __CFData,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCreateForCurrentUser(
    allocator: *mut __CFAllocator,
  ) -> *mut __CSIdentityQuery;
  pub fn CSIdentityQueryCopyResults(query: *mut __CSIdentityQuery) -> *mut __CFArray;
  pub fn CSIdentityQueryExecute(
    query: *mut __CSIdentityQuery,
    flags: usize,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn CSIdentityQueryExecuteAsynchronously(
    query: *mut __CSIdentityQuery,
    flags: usize,
    clientContext: *mut CSIdentityQueryClientContext,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> u8;
  pub fn CSIdentityQueryStop(query: *mut __CSIdentityQuery) -> ();
  pub fn NewSleepQUPP(
    userRoutine: Option<extern "C" fn(isize, *mut SleepQRec) -> isize>,
  ) -> Option<extern "C" fn(isize, *mut SleepQRec) -> isize>;
  pub fn DisposeSleepQUPP(userUPP: Option<extern "C" fn(isize, *mut SleepQRec) -> isize>) -> ();
  pub fn InvokeSleepQUPP(
    message: isize,
    qRecPtr: *mut SleepQRec,
    userUPP: Option<extern "C" fn(isize, *mut SleepQRec) -> isize>,
  ) -> isize;
  pub fn GetCPUSpeed() -> isize;
  pub fn SleepQInstall(qRecPtr: *mut SleepQRec) -> ();
  pub fn SleepQRemove(qRecPtr: *mut SleepQRec) -> ();
  pub fn MaximumProcessorSpeed() -> i16;
  pub fn MinimumProcessorSpeed() -> i16;
  pub fn CurrentProcessorSpeed() -> i16;
  pub fn BatteryCount() -> i16;
  pub fn UpdateSystemActivity(activity: u8) -> i16;
  pub fn KCGetKeychainManagerVersion(returnVers: *mut u32) -> i32;
  pub fn KCSetInteractionAllowed(state: u8) -> i32;
  pub fn KCIsInteractionAllowed() -> u8;
  pub fn KCMakeKCRefFromFSRef(
    keychainFSRef: *mut FSRef,
    keychain: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn KCMakeKCRefFromAlias(
    keychainAlias: *mut *mut AliasRecord,
    keychain: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn KCMakeAliasFromKCRef(
    keychain: *mut OpaqueSecKeychainRef,
    keychainAlias: *mut *mut *mut AliasRecord,
  ) -> i32;
  pub fn KCReleaseKeychain(keychain: *mut *mut OpaqueSecKeychainRef) -> i32;
  pub fn KCGetDefaultKeychain(keychain: *mut *mut OpaqueSecKeychainRef) -> i32;
  pub fn KCSetDefaultKeychain(keychain: *mut OpaqueSecKeychainRef) -> i32;
  pub fn KCGetStatus(keychain: *mut OpaqueSecKeychainRef, keychainStatus: *mut u32) -> i32;
  pub fn KCGetKeychain(
    item: *mut OpaqueSecKeychainItemRef,
    keychain: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn KCGetKeychainName(keychain: *mut OpaqueSecKeychainRef, keychainName: *mut u8) -> i32;
  pub fn KCCountKeychains() -> u16;
  pub fn KCGetIndKeychain(index: u16, keychain: *mut *mut OpaqueSecKeychainRef) -> i32;
  pub fn NewKCCallbackUPP(
    userRoutine: Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>,
  ) -> Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>;
  pub fn DisposeKCCallbackUPP(
    userUPP: Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>,
  ) -> ();
  pub fn InvokeKCCallbackUPP(
    keychainEvent: u16,
    info: *mut KCCallbackInfo,
    userContext: *mut c_void,
    userUPP: Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>,
  ) -> i32;
  pub fn KCFindAppleSharePassword(
    serverSignature: *mut [u8; 16],
    serverAddress: *mut u8,
    serverName: *mut u8,
    volumeName: *mut u8,
    accountName: *mut u8,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCFindInternetPassword(
    serverName: *mut u8,
    securityDomain: *mut u8,
    accountName: *mut u8,
    port: u16,
    protocol: u32,
    authType: u32,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCFindInternetPasswordWithPath(
    serverName: *mut u8,
    securityDomain: *mut u8,
    accountName: *mut u8,
    path: *mut u8,
    port: u16,
    protocol: u32,
    authType: u32,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCFindGenericPassword(
    serviceName: *mut u8,
    accountName: *mut u8,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCAddCallback(
    callbackProc: Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>,
    eventMask: u16,
    userContext: *mut c_void,
  ) -> i32;
  pub fn KCRemoveCallback(
    callbackProc: Option<extern "C" fn(u16, *mut KCCallbackInfo, *mut c_void) -> i32>,
  ) -> i32;
  pub fn KCNewItem(
    itemClass: u32,
    itemCreator: u32,
    length: u32,
    data: *mut c_void,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCSetAttribute(
    item: *mut OpaqueSecKeychainItemRef,
    attr: *mut SecKeychainAttribute,
  ) -> i32;
  pub fn KCGetAttribute(
    item: *mut OpaqueSecKeychainItemRef,
    attr: *mut SecKeychainAttribute,
    actualLength: *mut u32,
  ) -> i32;
  pub fn KCSetData(item: *mut OpaqueSecKeychainItemRef, length: u32, data: *mut c_void) -> i32;
  pub fn KCUpdateItem(item: *mut OpaqueSecKeychainItemRef) -> i32;
  pub fn KCReleaseItem(item: *mut *mut OpaqueSecKeychainItemRef) -> i32;
  pub fn KCCopyItem(
    item: *mut OpaqueSecKeychainItemRef,
    destKeychain: *mut OpaqueSecKeychainRef,
    copy: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCFindFirstItem(
    keychain: *mut OpaqueSecKeychainRef,
    attrList: *mut SecKeychainAttributeList,
    search: *mut *mut OpaqueSecKeychainSearchRef,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCFindNextItem(
    search: *mut OpaqueSecKeychainSearchRef,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn KCReleaseSearch(search: *mut *mut OpaqueSecKeychainSearchRef) -> i32;
  pub fn KCDeleteItem(item: *mut OpaqueSecKeychainItemRef) -> i32;
  pub fn KCGetData(
    item: *mut OpaqueSecKeychainItemRef,
    maxLength: u32,
    data: *mut c_void,
    actualLength: *mut u32,
  ) -> i32;
  pub fn KCLock(keychain: *mut OpaqueSecKeychainRef) -> i32;
  pub fn kcgetkeychainname(keychain: *mut OpaqueSecKeychainRef, keychainName: *mut i8) -> i32;
  pub fn kcfindapplesharepassword(
    serverSignature: *mut [u8; 16],
    serverAddress: *mut i8,
    serverName: *mut i8,
    volumeName: *mut i8,
    accountName: *mut i8,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn kcfindinternetpassword(
    serverName: *mut i8,
    securityDomain: *mut i8,
    accountName: *mut i8,
    port: u16,
    protocol: u32,
    authType: u32,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn kcfindinternetpasswordwithpath(
    serverName: *mut i8,
    securityDomain: *mut i8,
    accountName: *mut i8,
    path: *mut i8,
    port: u16,
    protocol: u32,
    authType: u32,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn kcfindgenericpassword(
    serviceName: *mut i8,
    accountName: *mut i8,
    maxLength: u32,
    passwordData: *mut c_void,
    actualLength: *mut u32,
    item: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn WSGetWSTypeIDFromCFType(ref_: *mut c_void) -> WSTypeID;
  pub fn WSGetCFTypeIDFromWSTypeID(typeID: WSTypeID) -> usize;
  pub fn WSMethodInvocationGetTypeID() -> usize;
  pub fn WSMethodInvocationCreate(
    url: *mut __CFURL,
    methodName: *mut __CFString,
    protocol: *mut __CFString,
  ) -> *mut OpaqueWSMethodInvocationRef;
  pub fn WSMethodInvocationCreateFromSerialization(
    contract: *mut __CFData,
  ) -> *mut OpaqueWSMethodInvocationRef;
  pub fn WSMethodInvocationCopySerialization(
    invocation: *mut OpaqueWSMethodInvocationRef,
  ) -> *mut __CFData;
  pub fn WSMethodInvocationSetParameters(
    invocation: *mut OpaqueWSMethodInvocationRef,
    parameters: *mut __CFDictionary,
    parameterOrder: *mut __CFArray,
  ) -> ();
  pub fn WSMethodInvocationCopyParameters(
    invocation: *mut OpaqueWSMethodInvocationRef,
    parameterOrder: *mut *mut __CFArray,
  ) -> *mut __CFDictionary;
  pub fn WSMethodInvocationSetProperty(
    invocation: *mut OpaqueWSMethodInvocationRef,
    propertyName: *mut __CFString,
    propertyValue: *mut c_void,
  ) -> ();
  pub fn WSMethodInvocationCopyProperty(
    invocation: *mut OpaqueWSMethodInvocationRef,
    propertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn WSMethodInvocationInvoke(
    invocation: *mut OpaqueWSMethodInvocationRef,
  ) -> *mut __CFDictionary;
  pub fn WSMethodInvocationSetCallBack(
    invocation: *mut OpaqueWSMethodInvocationRef,
    clientCB: Option<
      extern "C" fn(*mut OpaqueWSMethodInvocationRef, *mut c_void, *mut __CFDictionary) -> (),
    >,
    context: *mut WSClientContext,
  ) -> ();
  pub fn WSMethodInvocationScheduleWithRunLoop(
    invocation: *mut OpaqueWSMethodInvocationRef,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn WSMethodInvocationUnscheduleFromRunLoop(
    invocation: *mut OpaqueWSMethodInvocationRef,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> ();
  pub fn WSMethodResultIsFault(methodResult: *mut __CFDictionary) -> u8;
  pub fn WSMethodInvocationAddSerializationOverride(
    invocation: *mut OpaqueWSMethodInvocationRef,
    objType: usize,
    serializationProc: Option<
      extern "C" fn(*mut OpaqueWSMethodInvocationRef, *mut c_void, *mut c_void) -> *mut __CFString,
    >,
    context: *mut WSClientContext,
  ) -> ();
  pub fn WSMethodInvocationAddDeserializationOverride(
    invocation: *mut OpaqueWSMethodInvocationRef,
    typeNamespace: *mut __CFString,
    typeName: *mut __CFString,
    deserializationProc: Option<
      extern "C" fn(
        *mut OpaqueWSMethodInvocationRef,
        *mut __CFTree,
        *mut __CFTree,
        *mut c_void,
      ) -> *mut c_void,
    >,
    context: *mut WSClientContext,
  ) -> ();
  pub fn WSProtocolHandlerGetTypeID() -> usize;
  pub fn WSProtocolHandlerCreate(
    allocator: *mut __CFAllocator,
    protocol: *mut __CFString,
  ) -> *mut OpaqueWSProtocolHandlerRef;
  pub fn WSProtocolHandlerCopyRequestDictionary(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    data: *mut __CFData,
  ) -> *mut __CFDictionary;
  pub fn WSProtocolHandlerCopyReplyDictionary(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    methodName: *mut __CFString,
    data: *mut __CFData,
  ) -> *mut __CFDictionary;
  pub fn WSProtocolHandlerCopyReplyDocument(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    methodContext: *mut __CFDictionary,
    resultValue: *mut c_void,
  ) -> *mut __CFData;
  pub fn WSProtocolHandlerCopyFaultDocument(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    methodContext: *mut __CFDictionary,
    faultDict: *mut __CFDictionary,
  ) -> *mut __CFData;
  pub fn WSProtocolHandlerCopyRequestDocument(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    methodName: *mut __CFString,
    methodParams: *mut __CFDictionary,
    methodParamOrder: *mut __CFArray,
    methodExtras: *mut __CFDictionary,
  ) -> *mut __CFData;
  pub fn WSProtocolHandlerCopyProperty(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    propertyName: *mut __CFString,
  ) -> *mut c_void;
  pub fn WSProtocolHandlerSetProperty(
    ref_: *mut OpaqueWSProtocolHandlerRef,
    propertyName: *mut __CFString,
    propertyValue: *mut c_void,
  ) -> ();
  pub fn WSProtocolHandlerSetSerializationOverride(
    protocol: *mut OpaqueWSProtocolHandlerRef,
    objType: usize,
    serializationProc: Option<
      extern "C" fn(*mut OpaqueWSProtocolHandlerRef, *mut c_void, *mut c_void) -> *mut __CFString,
    >,
    context: *mut WSClientContext,
  ) -> ();
  pub fn WSProtocolHandlerSetDeserializationOverride(
    protocol: *mut OpaqueWSProtocolHandlerRef,
    typeNamespace: *mut __CFString,
    typeName: *mut __CFString,
    deserializationProc: Option<
      extern "C" fn(
        *mut OpaqueWSProtocolHandlerRef,
        *mut __CFTree,
        *mut __CFTree,
        *mut c_void,
      ) -> *mut c_void,
    >,
    context: *mut WSClientContext,
  ) -> ();
}
