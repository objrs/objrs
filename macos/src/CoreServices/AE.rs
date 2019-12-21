use crate::message::mach_msg_header_t;
#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::CFStreamError;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
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
pub type DescType = u32;
pub type AEKeyword = u32;
#[repr(C)]
pub struct OpaqueAEDataStorageType {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AEDesc {
  pub descriptorType: u32,
  pub dataHandle: *mut *mut OpaqueAEDataStorageType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AEKeyDesc {
  pub descKey: u32,
  pub descContent: AEDesc,
}
pub type AEDescList = AEDesc;
pub type AERecord = AEDesc;
pub type AEAddressDesc = AEDesc;
pub type AppleEvent = AEDesc;
pub type AEReturnID = i16;
pub type AETransactionID = i32;
pub type AEEventClass = u32;
pub type AEEventID = u32;
pub type AEArrayType = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub union AEArrayData {
  pub kAEDataArray: [i16; 1],
  pub kAEPackedArray: [i8; 1],
  pub kAEHandleArray: [*mut *mut i8; 1],
  pub kAEDescArray: [AEDesc; 1],
  pub kAEKeyDescArray: [AEKeyDesc; 1],
}
pub type AESendPriority = i16;
pub type AESendMode = i32;
pub type AECoerceDescUPP = Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>;
pub type AECoercePtrUPP =
  Option<extern "C" fn(u32, *mut c_void, isize, u32, *mut c_void, *mut AEDesc) -> i16>;
pub type AECoercionHandlerUPP =
  Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>;
pub type AEDisposeExternalUPP = Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>;
pub type AEEventHandlerUPP = Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>;
pub type AEEventSource = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AERemoteProcessResolverContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
#[repr(C)]
pub struct AERemoteProcessResolver {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccntTokenRecord {
  pub tokenClass: u32,
  pub token: AEDesc,
}
pub type OSLAccessorUPP =
  Option<extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16>;
pub type OSLCompareUPP = Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>;
pub type OSLCountUPP = Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>;
pub type OSLDisposeTokenUPP = Option<extern "C" fn(*mut AEDesc) -> i16>;
pub type OSLGetMarkTokenUPP = Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>;
pub type OSLGetErrDescUPP = Option<extern "C" fn(*mut *mut AEDesc) -> i16>;
pub type OSLMarkUPP = Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>;
pub type OSLAdjustMarksUPP = Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextRange {
  pub fStart: i32,
  pub fEnd: i32,
  pub fHiliteStyle: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextRangeArray {
  pub fNumOfRanges: i16,
  pub fRange: [TextRange; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OffsetArray {
  pub fNumOfOffsets: i16,
  pub fOffset: [i32; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WritingCode {
  pub theScriptCode: i16,
  pub theLangCode: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IntlText {
  pub theScriptCode: i16,
  pub theLangCode: i16,
  pub theText: [i8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TScriptingSizeResource {
  pub scriptingSizeFlags: i16,
  pub minStackSize: u32,
  pub preferredStackSize: u32,
  pub maxStackSize: u32,
  pub minHeapSize: u32,
  pub preferredHeapSize: u32,
  pub maxHeapSize: u32,
}
pub type AEBuildErrorCode = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AEBuildError {
  pub fError: u32,
  pub fErrorPos: u32,
}
#[repr(C)]
pub struct OpaqueAEStreamRef {
  opaque: u32,
}
extern "C" {
  pub fn NewAECoerceDescUPP(
    userRoutine: Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
  ) -> Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>;
  pub fn NewAECoercePtrUPP(
    userRoutine: Option<
      extern "C" fn(u32, *mut c_void, isize, u32, *mut c_void, *mut AEDesc) -> i16,
    >,
  ) -> Option<extern "C" fn(u32, *mut c_void, isize, u32, *mut c_void, *mut AEDesc) -> i16>;
  pub fn DisposeAECoerceDescUPP(
    userUPP: Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
  ) -> ();
  pub fn DisposeAECoercePtrUPP(
    userUPP: Option<extern "C" fn(u32, *mut c_void, isize, u32, *mut c_void, *mut AEDesc) -> i16>,
  ) -> ();
  pub fn InvokeAECoerceDescUPP(
    fromDesc: *mut AEDesc,
    toType: u32,
    handlerRefcon: *mut c_void,
    toDesc: *mut AEDesc,
    userUPP: Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn InvokeAECoercePtrUPP(
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
    toType: u32,
    handlerRefcon: *mut c_void,
    result: *mut AEDesc,
    userUPP: Option<extern "C" fn(u32, *mut c_void, isize, u32, *mut c_void, *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn AEInstallCoercionHandler(
    fromType: u32,
    toType: u32,
    handler: Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
    handlerRefcon: *mut c_void,
    fromTypeIsDesc: u8,
    isSysHandler: u8,
  ) -> i16;
  pub fn AERemoveCoercionHandler(
    fromType: u32,
    toType: u32,
    handler: Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEGetCoercionHandler(
    fromType: u32,
    toType: u32,
    handler: *mut Option<extern "C" fn(*mut AEDesc, u32, *mut c_void, *mut AEDesc) -> i16>,
    handlerRefcon: *mut *mut c_void,
    fromTypeIsDesc: *mut u8,
    isSysHandler: u8,
  ) -> i16;
  pub fn AECoercePtr(
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
    toType: u32,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AECoerceDesc(theAEDesc: *mut AEDesc, toType: u32, result: *mut AEDesc) -> i16;
  pub fn AEInitializeDesc(desc: *mut AEDesc) -> ();
  pub fn AECreateDesc(
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AEDisposeDesc(theAEDesc: *mut AEDesc) -> i16;
  pub fn AEDuplicateDesc(theAEDesc: *mut AEDesc, result: *mut AEDesc) -> i16;
  pub fn AECreateDescFromExternalPtr(
    descriptorType: u32,
    dataPtr: *mut c_void,
    dataLength: isize,
    disposeCallback: Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>,
    disposeRefcon: *mut c_void,
    theDesc: *mut AEDesc,
  ) -> i32;
  pub fn AECompareDesc(desc1: *mut AEDesc, desc2: *mut AEDesc, resultP: *mut u8) -> i32;
  pub fn AECreateList(
    factoringPtr: *mut c_void,
    factoredSize: isize,
    isRecord: u8,
    resultList: *mut AEDesc,
  ) -> i16;
  pub fn AECountItems(theAEDescList: *mut AEDesc, theCount: *mut isize) -> i16;
  pub fn AEPutPtr(
    theAEDescList: *mut AEDesc,
    index: isize,
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
  ) -> i16;
  pub fn AEPutDesc(theAEDescList: *mut AEDesc, index: isize, theAEDesc: *mut AEDesc) -> i16;
  pub fn AEGetNthPtr(
    theAEDescList: *mut AEDesc,
    index: isize,
    desiredType: u32,
    theAEKeyword: *mut u32,
    typeCode: *mut u32,
    dataPtr: *mut c_void,
    maximumSize: isize,
    actualSize: *mut isize,
  ) -> i16;
  pub fn AEGetNthDesc(
    theAEDescList: *mut AEDesc,
    index: isize,
    desiredType: u32,
    theAEKeyword: *mut u32,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AESizeOfNthItem(
    theAEDescList: *mut AEDesc,
    index: isize,
    typeCode: *mut u32,
    dataSize: *mut isize,
  ) -> i16;
  pub fn AEGetArray(
    theAEDescList: *mut AEDesc,
    arrayType: i8,
    arrayPtr: *mut AEArrayData,
    maximumSize: isize,
    itemType: *mut u32,
    itemSize: *mut isize,
    itemCount: *mut isize,
  ) -> i16;
  pub fn AEPutArray(
    theAEDescList: *mut AEDesc,
    arrayType: i8,
    arrayPtr: *mut AEArrayData,
    itemType: u32,
    itemSize: isize,
    itemCount: isize,
  ) -> i16;
  pub fn AEDeleteItem(theAEDescList: *mut AEDesc, index: isize) -> i16;
  pub fn AECheckIsRecord(theDesc: *mut AEDesc) -> u8;
  pub fn AECreateAppleEvent(
    theAEEventClass: u32,
    theAEEventID: u32,
    target: *mut AEDesc,
    returnID: i16,
    transactionID: i32,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AEPutParamPtr(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
  ) -> i16;
  pub fn AEPutParamDesc(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    theAEDesc: *mut AEDesc,
  ) -> i16;
  pub fn AEGetParamPtr(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    desiredType: u32,
    actualType: *mut u32,
    dataPtr: *mut c_void,
    maximumSize: isize,
    actualSize: *mut isize,
  ) -> i16;
  pub fn AEGetParamDesc(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    desiredType: u32,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AESizeOfParam(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    typeCode: *mut u32,
    dataSize: *mut isize,
  ) -> i16;
  pub fn AEDeleteParam(theAppleEvent: *mut AEDesc, theAEKeyword: u32) -> i16;
  pub fn AEGetAttributePtr(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    desiredType: u32,
    typeCode: *mut u32,
    dataPtr: *mut c_void,
    maximumSize: isize,
    actualSize: *mut isize,
  ) -> i16;
  pub fn AEGetAttributeDesc(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    desiredType: u32,
    result: *mut AEDesc,
  ) -> i16;
  pub fn AESizeOfAttribute(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    typeCode: *mut u32,
    dataSize: *mut isize,
  ) -> i16;
  pub fn AEPutAttributePtr(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
  ) -> i16;
  pub fn AEPutAttributeDesc(
    theAppleEvent: *mut AEDesc,
    theAEKeyword: u32,
    theAEDesc: *mut AEDesc,
  ) -> i16;
  pub fn AESizeOfFlattenedDesc(theAEDesc: *mut AEDesc) -> isize;
  pub fn AEFlattenDesc(
    theAEDesc: *mut AEDesc,
    buffer: *mut i8,
    bufferSize: isize,
    actualSize: *mut isize,
  ) -> i32;
  pub fn AEUnflattenDesc(buffer: *mut c_void, result: *mut AEDesc) -> i32;
  pub fn AEGetDescData(theAEDesc: *mut AEDesc, dataPtr: *mut c_void, maximumSize: isize) -> i16;
  pub fn AEGetDescDataSize(theAEDesc: *mut AEDesc) -> isize;
  pub fn AEReplaceDescData(
    typeCode: u32,
    dataPtr: *mut c_void,
    dataSize: isize,
    theAEDesc: *mut AEDesc,
  ) -> i16;
  pub fn AEGetDescDataRange(
    dataDesc: *mut AEDesc,
    buffer: *mut c_void,
    offset: isize,
    length: isize,
  ) -> i32;
  pub fn NewAEDisposeExternalUPP(
    userRoutine: Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>,
  ) -> Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>;
  pub fn NewAEEventHandlerUPP(
    userRoutine: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
  ) -> Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>;
  pub fn DisposeAEDisposeExternalUPP(
    userUPP: Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>,
  ) -> ();
  pub fn DisposeAEEventHandlerUPP(
    userUPP: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
  ) -> ();
  pub fn InvokeAEDisposeExternalUPP(
    dataPtr: *mut c_void,
    dataLength: isize,
    refcon: *mut c_void,
    userUPP: Option<extern "C" fn(*mut c_void, isize, *mut c_void) -> ()>,
  ) -> ();
  pub fn InvokeAEEventHandlerUPP(
    theAppleEvent: *mut AEDesc,
    reply: *mut AEDesc,
    handlerRefcon: *mut c_void,
    userUPP: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
  ) -> i16;
  pub fn AEInstallEventHandler(
    theAEEventClass: u32,
    theAEEventID: u32,
    handler: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    handlerRefcon: *mut c_void,
    isSysHandler: u8,
  ) -> i16;
  pub fn AERemoveEventHandler(
    theAEEventClass: u32,
    theAEEventID: u32,
    handler: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEGetEventHandler(
    theAEEventClass: u32,
    theAEEventID: u32,
    handler: *mut Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    handlerRefcon: *mut *mut c_void,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEInstallSpecialHandler(
    functionClass: u32,
    handler: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    isSysHandler: u8,
  ) -> i16;
  pub fn AERemoveSpecialHandler(
    functionClass: u32,
    handler: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEGetSpecialHandler(
    functionClass: u32,
    handler: *mut Option<extern "C" fn(*mut AEDesc, *mut AEDesc, *mut c_void) -> i16>,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEManagerInfo(keyWord: u32, result: *mut isize) -> i16;
  pub fn AECreateRemoteProcessResolver(
    allocator: *mut __CFAllocator,
    url: *mut __CFURL,
  ) -> *mut AERemoteProcessResolver;
  pub fn AEDisposeRemoteProcessResolver(ref_: *mut AERemoteProcessResolver) -> ();
  pub fn AERemoteProcessResolverGetProcesses(
    ref_: *mut AERemoteProcessResolver,
    outError: *mut CFStreamError,
  ) -> *mut __CFArray;
  pub fn AERemoteProcessResolverScheduleWithRunLoop(
    ref_: *mut AERemoteProcessResolver,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
    callback: Option<extern "C" fn(*mut AERemoteProcessResolver, *mut c_void) -> ()>,
    ctx: *mut AERemoteProcessResolverContext,
  ) -> ();
  pub fn AEDeterminePermissionToAutomateTarget(
    target: *mut AEDesc,
    theAEEventClass: u32,
    theAEEventID: u32,
    askUserIfNeeded: u8,
  ) -> i32;
  pub fn CreateOffsetDescriptor(theOffset: isize, theDescriptor: *mut AEDesc) -> i16;
  pub fn CreateCompDescriptor(
    comparisonOperator: u32,
    operand1: *mut AEDesc,
    operand2: *mut AEDesc,
    disposeInputs: u8,
    theDescriptor: *mut AEDesc,
  ) -> i16;
  pub fn CreateLogicalDescriptor(
    theLogicalTerms: *mut AEDesc,
    theLogicOperator: u32,
    disposeInputs: u8,
    theDescriptor: *mut AEDesc,
  ) -> i16;
  pub fn CreateObjSpecifier(
    desiredClass: u32,
    theContainer: *mut AEDesc,
    keyForm: u32,
    keyData: *mut AEDesc,
    disposeInputs: u8,
    objSpecifier: *mut AEDesc,
  ) -> i16;
  pub fn CreateRangeDescriptor(
    rangeStart: *mut AEDesc,
    rangeStop: *mut AEDesc,
    disposeInputs: u8,
    theDescriptor: *mut AEDesc,
  ) -> i16;
  pub fn NewOSLAccessorUPP(
    userRoutine: Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
  ) -> Option<extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16>;
  pub fn NewOSLCompareUPP(
    userRoutine: Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>,
  ) -> Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>;
  pub fn NewOSLCountUPP(
    userRoutine: Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>,
  ) -> Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>;
  pub fn NewOSLDisposeTokenUPP(
    userRoutine: Option<extern "C" fn(*mut AEDesc) -> i16>,
  ) -> Option<extern "C" fn(*mut AEDesc) -> i16>;
  pub fn NewOSLGetMarkTokenUPP(
    userRoutine: Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>,
  ) -> Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>;
  pub fn NewOSLGetErrDescUPP(
    userRoutine: Option<extern "C" fn(*mut *mut AEDesc) -> i16>,
  ) -> Option<extern "C" fn(*mut *mut AEDesc) -> i16>;
  pub fn NewOSLMarkUPP(
    userRoutine: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>,
  ) -> Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>;
  pub fn NewOSLAdjustMarksUPP(
    userRoutine: Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>,
  ) -> Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>;
  pub fn DisposeOSLAccessorUPP(
    userUPP: Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
  ) -> ();
  pub fn DisposeOSLCompareUPP(
    userUPP: Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>,
  ) -> ();
  pub fn DisposeOSLCountUPP(
    userUPP: Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>,
  ) -> ();
  pub fn DisposeOSLDisposeTokenUPP(userUPP: Option<extern "C" fn(*mut AEDesc) -> i16>) -> ();
  pub fn DisposeOSLGetMarkTokenUPP(
    userUPP: Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>,
  ) -> ();
  pub fn DisposeOSLGetErrDescUPP(userUPP: Option<extern "C" fn(*mut *mut AEDesc) -> i16>) -> ();
  pub fn DisposeOSLMarkUPP(
    userUPP: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>,
  ) -> ();
  pub fn DisposeOSLAdjustMarksUPP(
    userUPP: Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>,
  ) -> ();
  pub fn InvokeOSLAccessorUPP(
    desiredClass: u32,
    container: *mut AEDesc,
    containerClass: u32,
    form: u32,
    selectionData: *mut AEDesc,
    value: *mut AEDesc,
    accessorRefcon: *mut c_void,
    userUPP: Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
  ) -> i16;
  pub fn InvokeOSLCompareUPP(
    oper: u32,
    obj1: *mut AEDesc,
    obj2: *mut AEDesc,
    result: *mut u8,
    userUPP: Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>,
  ) -> i16;
  pub fn InvokeOSLCountUPP(
    desiredType: u32,
    containerClass: u32,
    container: *mut AEDesc,
    result: *mut isize,
    userUPP: Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>,
  ) -> i16;
  pub fn InvokeOSLDisposeTokenUPP(
    unneededToken: *mut AEDesc,
    userUPP: Option<extern "C" fn(*mut AEDesc) -> i16>,
  ) -> i16;
  pub fn InvokeOSLGetMarkTokenUPP(
    dContainerToken: *mut AEDesc,
    containerClass: u32,
    result: *mut AEDesc,
    userUPP: Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn InvokeOSLGetErrDescUPP(
    appDescPtr: *mut *mut AEDesc,
    userUPP: Option<extern "C" fn(*mut *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn InvokeOSLMarkUPP(
    dToken: *mut AEDesc,
    markToken: *mut AEDesc,
    index: isize,
    userUPP: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>,
  ) -> i16;
  pub fn InvokeOSLAdjustMarksUPP(
    newStart: isize,
    newStop: isize,
    markToken: *mut AEDesc,
    userUPP: Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn AEObjectInit() -> i16;
  pub fn AESetObjectCallbacks(
    myCompareProc: Option<extern "C" fn(u32, *mut AEDesc, *mut AEDesc, *mut u8) -> i16>,
    myCountProc: Option<extern "C" fn(u32, u32, *mut AEDesc, *mut isize) -> i16>,
    myDisposeTokenProc: Option<extern "C" fn(*mut AEDesc) -> i16>,
    myGetMarkTokenProc: Option<extern "C" fn(*mut AEDesc, u32, *mut AEDesc) -> i16>,
    myMarkProc: Option<extern "C" fn(*mut AEDesc, *mut AEDesc, isize) -> i16>,
    myAdjustMarksProc: Option<extern "C" fn(isize, isize, *mut AEDesc) -> i16>,
    myGetErrDescProcPtr: Option<extern "C" fn(*mut *mut AEDesc) -> i16>,
  ) -> i16;
  pub fn AEResolve(objectSpecifier: *mut AEDesc, callbackFlags: i16, theToken: *mut AEDesc) -> i16;
  pub fn AEInstallObjectAccessor(
    desiredClass: u32,
    containerType: u32,
    theAccessor: Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
    accessorRefcon: *mut c_void,
    isSysHandler: u8,
  ) -> i16;
  pub fn AERemoveObjectAccessor(
    desiredClass: u32,
    containerType: u32,
    theAccessor: Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEGetObjectAccessor(
    desiredClass: u32,
    containerType: u32,
    accessor: *mut Option<
      extern "C" fn(u32, *mut AEDesc, u32, u32, *mut AEDesc, *mut AEDesc, *mut c_void) -> i16,
    >,
    accessorRefcon: *mut *mut c_void,
    isSysHandler: u8,
  ) -> i16;
  pub fn AEDisposeToken(theToken: *mut AEDesc) -> i16;
  pub fn AECallObjectAccessor(
    desiredClass: u32,
    containerToken: *mut AEDesc,
    containerClass: u32,
    keyForm: u32,
    keyData: *mut AEDesc,
    token: *mut AEDesc,
  ) -> i16;
  pub fn AEBuildDesc(dst: *mut AEDesc, error: *mut AEBuildError, src: *mut i8, ...) -> i32;
  pub fn AEBuildParameters(
    event: *mut AEDesc,
    error: *mut AEBuildError,
    format: *mut i8,
    ...
  ) -> i32;
  pub fn AEBuildAppleEvent(
    theClass: u32,
    theID: u32,
    addressType: u32,
    addressData: *mut c_void,
    addressLength: isize,
    returnID: i16,
    transactionID: i32,
    result: *mut AEDesc,
    error: *mut AEBuildError,
    paramsFmt: *mut i8,
    ...
  ) -> i32;
  pub fn AEPrintDescToHandle(desc: *mut AEDesc, result: *mut *mut *mut i8) -> i32;
  pub fn AEStreamOpen() -> *mut OpaqueAEStreamRef;
  pub fn AEStreamClose(ref_: *mut OpaqueAEStreamRef, desc: *mut AEDesc) -> i32;
  pub fn AEStreamOpenDesc(ref_: *mut OpaqueAEStreamRef, newType: u32) -> i32;
  pub fn AEStreamWriteData(ref_: *mut OpaqueAEStreamRef, data: *mut c_void, length: isize) -> i32;
  pub fn AEStreamCloseDesc(ref_: *mut OpaqueAEStreamRef) -> i32;
  pub fn AEStreamWriteDesc(
    ref_: *mut OpaqueAEStreamRef,
    newType: u32,
    data: *mut c_void,
    length: isize,
  ) -> i32;
  pub fn AEStreamWriteAEDesc(ref_: *mut OpaqueAEStreamRef, desc: *mut AEDesc) -> i32;
  pub fn AEStreamOpenList(ref_: *mut OpaqueAEStreamRef) -> i32;
  pub fn AEStreamCloseList(ref_: *mut OpaqueAEStreamRef) -> i32;
  pub fn AEStreamOpenRecord(ref_: *mut OpaqueAEStreamRef, newType: u32) -> i32;
  pub fn AEStreamSetRecordType(ref_: *mut OpaqueAEStreamRef, newType: u32) -> i32;
  pub fn AEStreamCloseRecord(ref_: *mut OpaqueAEStreamRef) -> i32;
  pub fn AEStreamWriteKeyDesc(
    ref_: *mut OpaqueAEStreamRef,
    key: u32,
    newType: u32,
    data: *mut c_void,
    length: isize,
  ) -> i32;
  pub fn AEStreamOpenKeyDesc(ref_: *mut OpaqueAEStreamRef, key: u32, newType: u32) -> i32;
  pub fn AEStreamWriteKey(ref_: *mut OpaqueAEStreamRef, key: u32) -> i32;
  pub fn AEStreamCreateEvent(
    clazz: u32,
    id: u32,
    targetType: u32,
    targetData: *mut c_void,
    targetLength: isize,
    returnID: i16,
    transactionID: i32,
  ) -> *mut OpaqueAEStreamRef;
  pub fn AEStreamOpenEvent(event: *mut AEDesc) -> *mut OpaqueAEStreamRef;
  pub fn AEStreamOptionalParam(ref_: *mut OpaqueAEStreamRef, key: u32) -> i32;
  pub fn AEGetRegisteredMachPort() -> u32;
  pub fn AEDecodeMessage(
    header: *mut mach_msg_header_t,
    event: *mut AEDesc,
    reply: *mut AEDesc,
  ) -> i32;
  pub fn AEProcessMessage(header: *mut mach_msg_header_t) -> i32;
  pub fn AESendMessage(
    event: *mut AEDesc,
    reply: *mut AEDesc,
    sendMode: i32,
    timeOutInTicks: isize,
  ) -> i32;
}
