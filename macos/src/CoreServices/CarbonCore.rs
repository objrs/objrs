use crate::acl::_acl;
use crate::hfs_unistr::HFSUniStr255;
#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::CFUUIDBytes;
use crate::CoreFoundation::FSRef;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFRunLoop;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::CoreFoundation::__CFUUID;
use crate::DiskArbitration::__DADisk;
use crate::MacTypes::wide;
use crate::MacTypes::Float80;
use crate::MacTypes::Point;
use crate::MacTypes::Rect;
use crate::MacTypes::UnsignedWide;
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
#[derive(Copy, Clone)]
pub struct CustomBadgeResource {
  pub version: i16,
  pub customBadgeResourceID: i16,
  pub customBadgeType: u32,
  pub customBadgeCreator: u32,
  pub windowBadgeType: u32,
  pub windowBadgeCreator: u32,
  pub overrideType: u32,
  pub overrideCreator: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RoutingResourceEntry {
  pub creator: u32,
  pub fileType: u32,
  pub targetFolder: u32,
  pub destinationFolder: u32,
  pub reservedField: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileInfo {
  pub fileType: u32,
  pub fileCreator: u32,
  pub finderFlags: u16,
  pub location: Point,
  pub reservedField: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FolderInfo {
  pub windowBounds: Rect,
  pub finderFlags: u16,
  pub location: Point,
  pub reservedField: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtendedFileInfo {
  pub reserved1: [i16; 4],
  pub extendedFinderFlags: u16,
  pub reserved2: i16,
  pub putAwayFolderID: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtendedFolderInfo {
  pub scrollPosition: Point,
  pub reserved1: i32,
  pub extendedFinderFlags: u16,
  pub reserved2: i16,
  pub putAwayFolderID: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FInfo {
  pub fdType: u32,
  pub fdCreator: u32,
  pub fdFlags: u16,
  pub fdLocation: Point,
  pub fdFldr: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FXInfo {
  pub fdIconID: i16,
  pub fdReserved: [i16; 3],
  pub fdScript: i8,
  pub fdXFlags: i8,
  pub fdComment: i16,
  pub fdPutAway: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DInfo {
  pub frRect: Rect,
  pub frFlags: u16,
  pub frLocation: Point,
  pub frView: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DXInfo {
  pub frScroll: Point,
  pub frOpenChain: i32,
  pub frScript: i8,
  pub frXFlags: i8,
  pub frComment: i16,
  pub frPutAway: i32,
}
pub type DateOrders = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OffPair {
  pub offFirst: i16,
  pub offSecond: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Intl0Rec {
  pub decimalPt: i8,
  pub thousSep: i8,
  pub listSep: i8,
  pub currSym1: i8,
  pub currSym2: i8,
  pub currSym3: i8,
  pub currFmt: u8,
  pub dateOrder: u8,
  pub shrtDateFmt: u8,
  pub dateSep: i8,
  pub timeCycle: u8,
  pub timeFmt: u8,
  pub mornStr: [i8; 4],
  pub eveStr: [i8; 4],
  pub timeSep: i8,
  pub time1Suff: i8,
  pub time2Suff: i8,
  pub time3Suff: i8,
  pub time4Suff: i8,
  pub time5Suff: i8,
  pub time6Suff: i8,
  pub time7Suff: i8,
  pub time8Suff: i8,
  pub metricSys: u8,
  pub intl0Vers: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Intl1Rec {
  pub days: [[u8; 16]; 7],
  pub months: [[u8; 16]; 12],
  pub suppressDay: u8,
  pub lngDateFmt: u8,
  pub dayLeading0: u8,
  pub abbrLen: u8,
  pub st0: [i8; 4],
  pub st1: [i8; 4],
  pub st2: [i8; 4],
  pub st3: [i8; 4],
  pub st4: [i8; 4],
  pub intl1Vers: i16,
  pub localRtn: [i16; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Itl1ExtRec {
  pub base: Intl1Rec,
  pub version: i16,
  pub format: i16,
  pub calendarCode: i16,
  pub extraDaysTableOffset: i32,
  pub extraDaysTableLength: i32,
  pub extraMonthsTableOffset: i32,
  pub extraMonthsTableLength: i32,
  pub abbrevDaysTableOffset: i32,
  pub abbrevDaysTableLength: i32,
  pub abbrevMonthsTableOffset: i32,
  pub abbrevMonthsTableLength: i32,
  pub extraSepsTableOffset: i32,
  pub extraSepsTableLength: i32,
  pub tables: [i16; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UntokenTable {
  pub len: i16,
  pub lastToken: i16,
  pub index: [i16; 256],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WideChar {
  pub b: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WideCharArr {
  pub size: i16,
  pub data: [WideChar; 10],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NumberParts {
  pub version: i16,
  pub data: [WideChar; 31],
  pub pePlus: WideCharArr,
  pub peMinus: WideCharArr,
  pub peMinusPlus: WideCharArr,
  pub altNumTable: WideCharArr,
  pub reserved: [i8; 20],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Itl4Rec {
  pub flags: i16,
  pub resourceType: i32,
  pub resourceNum: i16,
  pub version: i16,
  pub resHeader1: i32,
  pub resHeader2: i32,
  pub numTables: i16,
  pub mapOffset: i32,
  pub strOffset: i32,
  pub fetchOffset: i32,
  pub unTokenOffset: i32,
  pub defPartsOffset: i32,
  pub resOffset6: i32,
  pub resOffset7: i32,
  pub resOffset8: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NItl4Rec {
  pub flags: i16,
  pub resourceType: i32,
  pub resourceNum: i16,
  pub version: i16,
  pub format: i16,
  pub resHeader: i16,
  pub resHeader2: i32,
  pub numTables: i16,
  pub mapOffset: i32,
  pub strOffset: i32,
  pub fetchOffset: i32,
  pub unTokenOffset: i32,
  pub defPartsOffset: i32,
  pub whtSpListOffset: i32,
  pub resOffset7: i32,
  pub resOffset8: i32,
  pub resLength1: i16,
  pub resLength2: i16,
  pub resLength3: i16,
  pub unTokenLength: i16,
  pub defPartsLength: i16,
  pub whtSpListLength: i16,
  pub resLength7: i16,
  pub resLength8: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TableDirectoryRecord {
  pub tableSignature: u32,
  pub reserved: u32,
  pub tableStartOffset: u32,
  pub tableSize: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Itl5Record {
  pub versionNumber: i32,
  pub numberOfTables: u16,
  pub reserved: [u16; 3],
  pub tableDirectory: [TableDirectoryRecord; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RuleBasedTrslRecord {
  pub sourceType: i16,
  pub targetType: i16,
  pub formatNumber: i16,
  pub propertyFlag: i16,
  pub numberOfRules: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ItlcRecord {
  pub itlcSystem: i16,
  pub itlcReserved: i16,
  pub itlcFontForce: i8,
  pub itlcIntlForce: i8,
  pub itlcOldKybd: i8,
  pub itlcFlags: i8,
  pub itlcIconOffset: i16,
  pub itlcIconSide: i8,
  pub itlcIconRsvd: i8,
  pub itlcRegionCode: i16,
  pub itlcSysFlags: i16,
  pub itlcReserved4: [i8; 32],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ItlbRecord {
  pub itlbNumber: i16,
  pub itlbDate: i16,
  pub itlbSort: i16,
  pub itlbFlags: i16,
  pub itlbToken: i16,
  pub itlbEncoding: i16,
  pub itlbLang: i16,
  pub itlbNumRep: i8,
  pub itlbDateRep: i8,
  pub itlbKeys: i16,
  pub itlbIcon: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ItlbExtRecord {
  pub base: ItlbRecord,
  pub itlbLocalSize: i32,
  pub itlbMonoFond: i16,
  pub itlbMonoSize: i16,
  pub itlbPrefFond: i16,
  pub itlbPrefSize: i16,
  pub itlbSmallFond: i16,
  pub itlbSmallSize: i16,
  pub itlbSysFond: i16,
  pub itlbSysSize: i16,
  pub itlbAppFond: i16,
  pub itlbAppSize: i16,
  pub itlbHelpFond: i16,
  pub itlbHelpSize: i16,
  pub itlbValidStyles: u8,
  pub itlbAliasStyle: u8,
}
pub type TokenResults = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TokenRec {
  pub theToken: i16,
  pub position: *mut i8,
  pub length: isize,
  pub stringPosition: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TokenBlock {
  pub source: *mut i8,
  pub sourceLength: isize,
  pub tokenList: *mut i8,
  pub tokenLength: isize,
  pub tokenCount: isize,
  pub stringList: *mut i8,
  pub stringLength: isize,
  pub stringCount: isize,
  pub doString: u8,
  pub doAppend: u8,
  pub doAlphanumeric: u8,
  pub doNest: u8,
  pub leftDelims: [i16; 2],
  pub rightDelims: [i16; 2],
  pub leftComment: [i16; 4],
  pub rightComment: [i16; 4],
  pub escapeCode: i16,
  pub decimalCode: i16,
  pub itlResource: *mut *mut i8,
  pub reserved: [isize; 8],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UTCDateTime {
  pub highSeconds: u16,
  pub lowSeconds: u32,
  pub fraction: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LocalDateTime {
  pub highSeconds: u16,
  pub lowSeconds: u32,
  pub fraction: u16,
}
pub type TextEncodingBase = u32;
pub type TextEncodingVariant = u32;
pub type TextEncodingFormat = u32;
pub type TextEncoding = u32;
pub type TextEncodingNameSelector = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextEncodingRun {
  pub offset: usize,
  pub textEncoding: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScriptCodeRun {
  pub offset: usize,
  pub script: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECInfo {
  pub format: u16,
  pub tecVersion: u16,
  pub tecTextConverterFeatures: u32,
  pub tecUnicodeConverterFeatures: u32,
  pub tecTextCommonFeatures: u32,
  pub tecTextEncodingsFolderName: [u8; 32],
  pub tecExtensionFileName: [u8; 32],
  pub tecLowestTEFileVersion: u16,
  pub tecHighestTEFileVersion: u16,
}
pub type UCCharPropertyType = i32;
pub type UCCharPropertyValue = u32;
pub type ISAType = i8;
pub type RTAType = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RoutineRecord {
  pub procInfo: usize,
  pub reserved1: i8,
  pub ISA: i8,
  pub routineFlags: u16,
  pub procDescriptor: Option<extern "C" fn() -> isize>,
  pub reserved2: u32,
  pub selector: u32,
}
pub type RDFlagsType = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RoutineDescriptor {
  pub goMixedModeTrap: u16,
  pub version: i8,
  pub routineDescriptorFlags: u8,
  pub reserved1: u32,
  pub reserved2: u8,
  pub selectorInfo: u8,
  pub routineCount: u16,
  pub routineRecords: [RoutineRecord; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MixedModeStateRecord {
  pub state1: u32,
  pub state2: u32,
  pub state3: u32,
  pub state4: u32,
}
#[repr(C)]
pub struct OpaqueCollection {
  opaque: u32,
}
pub type CollectionTag = u32;
pub type CollectionFlattenUPP = Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>;
pub type CollectionExceptionUPP = Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianUInt32 {
  pub bigEndianValue: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianLong {
  pub bigEndianValue: isize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianUnsignedLong {
  pub bigEndianValue: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianShort {
  pub bigEndianValue: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianUnsignedShort {
  pub bigEndianValue: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianFixed {
  pub bigEndianValue: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianUnsignedFixed {
  pub bigEndianValue: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigEndianOSType {
  pub bigEndianValue: u32,
}
pub type SelectorFunctionUPP = Option<extern "C" fn(u32, *mut i32) -> i16>;
pub type ToggleResults = i16;
pub type LongDateField = i8;
pub type DateForm = i8;
pub type String2DateStatus = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DateCacheRecord {
  pub hidden: [i16; 256],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DateTimeRec {
  pub year: i16,
  pub month: i16,
  pub day: i16,
  pub hour: i16,
  pub minute: i16,
  pub second: i16,
  pub dayOfWeek: i16,
}
pub type LongDateTime = i64;
#[repr(C)]
#[derive(Copy, Clone)]
pub union LongDateCvt {
  pub c: i64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union LongDateRec {
  pub list: [i16; 14],
}
pub type DateDelta = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TogglePB {
  pub togFlags: isize,
  pub amChars: u32,
  pub pmChars: u32,
  pub reserved: [isize; 4],
}
pub type QTypes = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QElem {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub qData: [i16; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QHdr {
  pub qFlags: i16,
  pub qHead: *mut QElem,
  pub qTail: *mut QElem,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MachineLocation {
  pub latitude: i32,
  pub longitude: i32,
}
pub type DeferredTaskUPP = Option<extern "C" fn(isize) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeferredTask {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub dtFlags: i16,
  pub dtAddr: Option<extern "C" fn(isize) -> ()>,
  pub dtParam: isize,
  pub dtReserved: isize,
}
pub type FSVolumeRefNum = i16;
#[repr(C)]
pub struct __FSFileSecurity {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CatPositionRec {
  pub initialize: i32,
  pub priv_: [i16; 6],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSSpec {
  pub hidden: [u8; 70],
}
pub type FSSpecArrayPtr = *mut FSSpec;
#[repr(C)]
pub struct ParamBlockRec {
  opaque: u32,
}
pub type IOCompletionUPP = Option<extern "C" fn(*mut c_void) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSPermissionInfo {
  pub userID: u32,
  pub groupID: u32,
  pub reserved1: u8,
  pub userAccess: u8,
  pub mode: u16,
  pub fileSec: *mut __FSFileSecurity,
}
pub type FSCatalogInfoBitmap = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSCatalogInfo {
  pub nodeFlags: u16,
  pub volume: i16,
  pub parentDirID: u32,
  pub nodeID: u32,
  pub sharingFlags: u8,
  pub userPrivileges: u8,
  pub reserved1: u8,
  pub reserved2: u8,
  pub createDate: UTCDateTime,
  pub contentModDate: UTCDateTime,
  pub attributeModDate: UTCDateTime,
  pub accessDate: UTCDateTime,
  pub backupDate: UTCDateTime,
  pub permissions: FSPermissionInfo,
  pub finderInfo: [u8; 16],
  pub extFinderInfo: [u8; 16],
  pub dataLogicalSize: u64,
  pub dataPhysicalSize: u64,
  pub rsrcLogicalSize: u64,
  pub rsrcPhysicalSize: u64,
  pub valence: u32,
  pub textEncodingHint: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSRefParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub ioNamePtr: *mut u8,
  pub ioVRefNum: i16,
  pub reserved1: i16,
  pub reserved2: u8,
  pub reserved3: u8,
  pub ref_: *mut FSRef,
  pub whichInfo: u32,
  pub catInfo: *mut FSCatalogInfo,
  pub nameLength: usize,
  pub name: *mut u16,
  pub ioDirID: u32,
  pub spec: *mut FSSpec,
  pub parentRef: *mut FSRef,
  pub newRef: *mut FSRef,
  pub textEncodingHint: u32,
  pub outName: *mut HFSUniStr255,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSRefForkIOParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub parentRef: *mut FSRef,
  pub nameLength: usize,
  pub name: *mut u16,
  pub whichInfo: u32,
  pub catInfo: *mut FSCatalogInfo,
  pub forkNameLength: usize,
  pub forkName: *mut u16,
  pub permissions: i8,
  pub reserved1: u8,
  pub forkRefNum: i32,
  pub newRef: *mut FSRef,
}
#[repr(C)]
pub struct OpaqueFSIterator {
  opaque: u32,
}
pub type FSIteratorFlags = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSSearchParams {
  pub searchTime: i32,
  pub searchBits: u32,
  pub searchNameLength: usize,
  pub searchName: *mut u16,
  pub searchInfo1: *mut FSCatalogInfo,
  pub searchInfo2: *mut FSCatalogInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSCatalogBulkParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub containerChanged: u8,
  pub reserved: u8,
  pub iteratorFlags: u32,
  pub iterator: *mut OpaqueFSIterator,
  pub container: *mut FSRef,
  pub maximumItems: usize,
  pub actualItems: usize,
  pub whichInfo: u32,
  pub catalogInfo: *mut FSCatalogInfo,
  pub refs: *mut FSRef,
  pub specs: *mut FSSpec,
  pub names: *mut HFSUniStr255,
  pub searchParams: *mut FSSearchParams,
}
pub type FSAllocationFlags = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSForkIOParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub reserved1: *mut c_void,
  pub reserved2: i16,
  pub forkRefNum: i32,
  pub reserved3: u8,
  pub permissions: i8,
  pub ref_: *mut FSRef,
  pub buffer: *mut i8,
  pub requestCount: u32,
  pub actualCount: u32,
  pub positionMode: u16,
  pub positionOffset: i64,
  pub allocationFlags: u16,
  pub allocationAmount: u64,
  pub forkNameLength: usize,
  pub forkName: *mut u16,
  pub forkIterator: CatPositionRec,
  pub outForkName: *mut HFSUniStr255,
}
pub type FSForkInfoFlags = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSForkInfo {
  pub flags: u8,
  pub permissions: i8,
  pub volume: i16,
  pub reserved2: u32,
  pub nodeID: u32,
  pub forkID: u32,
  pub currentPosition: u64,
  pub logicalEOF: u64,
  pub physicalEOF: u64,
  pub process: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSForkCBInfoParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub desiredRefNum: i32,
  pub volumeRefNum: i16,
  pub iterator: i32,
  pub actualRefNum: i16,
  pub ref_: *mut FSRef,
  pub forkInfo: *mut FSForkInfo,
  pub forkName: *mut HFSUniStr255,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSRangeLockParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub forkRefNum: i32,
  pub requestCount: u64,
  pub positionMode: u16,
  pub positionOffset: i64,
  pub rangeStart: u64,
}
pub type FSVolumeInfoBitmap = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSVolumeInfo {
  pub createDate: UTCDateTime,
  pub modifyDate: UTCDateTime,
  pub backupDate: UTCDateTime,
  pub checkedDate: UTCDateTime,
  pub fileCount: u32,
  pub folderCount: u32,
  pub totalBytes: u64,
  pub freeBytes: u64,
  pub blockSize: u32,
  pub totalBlocks: u32,
  pub freeBlocks: u32,
  pub nextAllocation: u32,
  pub rsrcClumpSize: u32,
  pub dataClumpSize: u32,
  pub nextCatalogID: u32,
  pub finderInfo: [u8; 32],
  pub flags: u16,
  pub filesystemID: u16,
  pub signature: u16,
  pub driveNumber: u16,
  pub driverRefNum: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSVolumeInfoParam {
  pub qLink: *mut QElem,
  pub qType: i16,
  pub ioTrap: i16,
  pub ioCmdAddr: *mut i8,
  pub ioCompletion: Option<extern "C" fn(*mut c_void) -> ()>,
  pub ioResult: i16,
  pub ioNamePtr: *mut u8,
  pub ioVRefNum: i16,
  pub volumeIndex: u32,
  pub whichInfo: u32,
  pub volumeInfo: *mut FSVolumeInfo,
  pub volumeName: *mut HFSUniStr255,
  pub ref_: *mut FSRef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetVolParmsInfoBuffer {
  pub vMVersion: i16,
  pub vMAttrib: i32,
  pub vMLocalHand: *mut *mut i8,
  pub vMServerAdr: i32,
  pub vMVolumeGrade: i32,
  pub vMForeignPrivID: i16,
  pub vMExtendedAttributes: i32,
  pub vMDeviceID: *mut c_void,
  pub vMMaxNameLength: usize,
}
pub type VolumeType = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VolMountInfoHeader {
  pub length: i16,
  pub media: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VolumeMountInfoHeader {
  pub length: i16,
  pub media: u32,
  pub flags: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AFPVolMountInfo {
  pub length: i16,
  pub media: u32,
  pub flags: i16,
  pub nbpInterval: i8,
  pub nbpCount: i8,
  pub uamType: i16,
  pub zoneNameOffset: i16,
  pub serverNameOffset: i16,
  pub volNameOffset: i16,
  pub userNameOffset: i16,
  pub userPasswordOffset: i16,
  pub volPasswordOffset: i16,
  pub AFPData: [i8; 144],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AFPXVolMountInfo {
  pub length: i16,
  pub media: u32,
  pub flags: i16,
  pub nbpInterval: i8,
  pub nbpCount: i8,
  pub uamType: i16,
  pub zoneNameOffset: i16,
  pub serverNameOffset: i16,
  pub volNameOffset: i16,
  pub userNameOffset: i16,
  pub userPasswordOffset: i16,
  pub volPasswordOffset: i16,
  pub extendedFlags: i16,
  pub uamNameOffset: i16,
  pub alternateAddressOffset: i16,
  pub AFPData: [i8; 176],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AFPTagData {
  pub fLength: u8,
  pub fType: u8,
  pub fData: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AFPAlternateAddress {
  pub fVersion: u8,
  pub fAddressCount: u8,
  pub fAddressList: [u8; 1],
}
pub type FNMessage = u32;
#[repr(C)]
pub struct OpaqueFNSubscriptionRef {
  opaque: u32,
}
pub type FNSubscriptionUPP =
  Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>;
pub type FSMountStatus = u32;
pub type FSEjectStatus = u32;
pub type FSUnmountStatus = u32;
#[repr(C)]
pub struct OpaqueFSVolumeOperation {
  opaque: u32,
}
pub type FSVolumeMountUPP =
  Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>;
pub type FSVolumeUnmountUPP =
  Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>;
pub type FSVolumeEjectUPP =
  Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>;
#[repr(C)]
pub struct __FSFileOperation {
  opaque: u32,
}
pub type FSFileOperationStage = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSFileOperationClientContext {
  pub version: isize,
  pub info: *mut c_void,
  pub retain: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  pub release: Option<extern "C" fn(*mut c_void) -> ()>,
  pub copyDescription: Option<extern "C" fn(*mut c_void) -> *mut __CFString>,
}
pub type ResID = i16;
pub type ResAttributes = i16;
pub type ResFileAttributes = i16;
pub type ResourceCount = i16;
pub type ResourceIndex = i16;
pub type ResFileRefNum = i32;
pub type ResErrUPP = Option<extern "C" fn(i16) -> ()>;
pub type RsrcChainLocation = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentDescription {
  pub componentType: u32,
  pub componentSubType: u32,
  pub componentManufacturer: u32,
  pub componentFlags: u32,
  pub componentFlagsMask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ResourceSpec {
  pub resType: u32,
  pub resID: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentResource {
  pub cd: ComponentDescription,
  pub component: ResourceSpec,
  pub componentName: ResourceSpec,
  pub componentInfo: ResourceSpec,
  pub componentIcon: ResourceSpec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentPlatformInfo {
  pub componentFlags: i32,
  pub component: ResourceSpec,
  pub platformType: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentResourceExtension {
  pub componentVersion: i32,
  pub componentRegisterFlags: i32,
  pub componentIconFamily: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentPlatformInfoArray {
  pub count: i32,
  pub platformArray: [ComponentPlatformInfo; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtComponentResource {
  pub cd: ComponentDescription,
  pub component: ResourceSpec,
  pub componentName: ResourceSpec,
  pub componentInfo: ResourceSpec,
  pub componentIcon: ResourceSpec,
  pub componentVersion: i32,
  pub componentRegisterFlags: i32,
  pub componentIconFamily: i16,
  pub count: i32,
  pub platformArray: [ComponentPlatformInfo; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentAliasResource {
  pub cr: ComponentResource,
  pub aliasCD: ComponentDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentParameters {
  pub flags: u8,
  pub paramSize: u8,
  pub what: i16,
  pub padding: u32,
  pub params: [isize; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentRecord {
  pub data: [isize; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentInstanceRecord {
  pub data: [isize; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegisteredComponentRecord {
  pub data: [isize; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegisteredComponentInstanceRecord {
  pub data: [isize; 1],
}
pub type ComponentResult = i32;
pub type CSComponentsThreadMode = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentMPWorkFunctionHeaderRecord {
  pub headerSize: u32,
  pub recordSize: u32,
  pub workFlags: u32,
  pub processorCount: u16,
  pub unused: u8,
  pub isRunning: u8,
}
pub type ComponentMPWorkFunctionUPP =
  Option<extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32>;
pub type ComponentRoutineUPP = Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>;
pub type GetMissingComponentResourceUPP =
  Option<extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16>;
pub type ComponentFunctionUPP = Option<extern "C" fn() -> isize>;
#[repr(C)]
pub struct OpaqueMPProcessID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPTaskID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPQueueID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPSemaphoreID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPCriticalRegionID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPTimerID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPEventID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPAddressSpaceID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPNotificationID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPCoherenceID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPCpuID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPAreaID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPConsoleID {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueMPOpaqueID {
  opaque: u32,
}
pub type MPOpaqueIDClass = u32;
pub type MPTaskOptions = u32;
pub type TaskStorageIndex = usize;
pub type TaskStorageValue = *mut c_void;
pub type MPSemaphoreCount = usize;
pub type MPTaskWeight = u32;
pub type MPEventFlags = u32;
pub type MPExceptionKind = u32;
pub type MPTaskStateKind = u32;
pub type MPPageSizeClass = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPTaskInfoVersion2 {
  pub version: u32,
  pub name: u32,
  pub queueName: u32,
  pub runState: u16,
  pub lastCPU: u16,
  pub weight: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub cpuTime: UnsignedWide,
  pub schedTime: UnsignedWide,
  pub creationTime: UnsignedWide,
  pub codePageFaults: usize,
  pub dataPageFaults: usize,
  pub preemptions: usize,
  pub cpuID: *mut OpaqueMPCpuID,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPTaskInfo {
  pub version: u32,
  pub name: u32,
  pub queueName: u32,
  pub runState: u16,
  pub lastCPU: u16,
  pub weight: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub cpuTime: UnsignedWide,
  pub schedTime: UnsignedWide,
  pub creationTime: UnsignedWide,
  pub codePageFaults: usize,
  pub dataPageFaults: usize,
  pub preemptions: usize,
  pub cpuID: *mut OpaqueMPCpuID,
  pub blockedObject: *mut OpaqueMPOpaqueID,
  pub spaceID: *mut OpaqueMPAddressSpaceID,
  pub stackBase: *mut c_void,
  pub stackLimit: *mut c_void,
  pub stackCurr: *mut c_void,
}
pub type MPDebuggerLevel = u32;
pub type MPRemoteContext = u8;
pub type FSAliasInfoBitmap = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AliasRecord {
  pub hidden: [u8; 6],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FSAliasInfo {
  pub volumeCreateDate: UTCDateTime,
  pub targetCreateDate: UTCDateTime,
  pub fileType: u32,
  pub fileCreator: u32,
  pub parentDirID: u32,
  pub nodeID: u32,
  pub filesystemID: u16,
  pub signature: u16,
  pub volumeIsBootVolume: u8,
  pub volumeIsAutomounted: u8,
  pub volumeIsEjectable: u8,
  pub volumeHasPersistentFileIDs: u8,
  pub isDirectory: u8,
}
#[repr(C)]
pub struct OpaqueLocaleRef {
  opaque: u32,
}
pub type LocalePartMask = u32;
pub type LocaleOperationClass = u32;
pub type LocaleOperationVariant = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LocaleAndVariant {
  pub locale: *mut OpaqueLocaleRef,
  pub opVariant: u32,
}
pub type LocaleNameMask = u32;
pub type DebugComponentCallbackUPP = Option<extern "C" fn(i32, u32, *mut u8) -> ()>;
pub type DebugAssertOutputHandlerUPP = Option<
  extern "C" fn(u32, u32, *mut i8, *mut i8, *mut i8, *mut i8, isize, *mut c_void, *mut u8) -> (),
>;
#[repr(C)]
pub struct OpaqueAreaID {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MachineInformationPowerPC {
  pub CTR: UnsignedWide,
  pub LR: UnsignedWide,
  pub PC: UnsignedWide,
  pub CRRegister: usize,
  pub XER: usize,
  pub MSR: usize,
  pub MQ: usize,
  pub ExceptKind: usize,
  pub DSISR: usize,
  pub DAR: UnsignedWide,
  pub Reserved: UnsignedWide,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegisterInformationPowerPC {
  pub R0: UnsignedWide,
  pub R1: UnsignedWide,
  pub R2: UnsignedWide,
  pub R3: UnsignedWide,
  pub R4: UnsignedWide,
  pub R5: UnsignedWide,
  pub R6: UnsignedWide,
  pub R7: UnsignedWide,
  pub R8: UnsignedWide,
  pub R9: UnsignedWide,
  pub R10: UnsignedWide,
  pub R11: UnsignedWide,
  pub R12: UnsignedWide,
  pub R13: UnsignedWide,
  pub R14: UnsignedWide,
  pub R15: UnsignedWide,
  pub R16: UnsignedWide,
  pub R17: UnsignedWide,
  pub R18: UnsignedWide,
  pub R19: UnsignedWide,
  pub R20: UnsignedWide,
  pub R21: UnsignedWide,
  pub R22: UnsignedWide,
  pub R23: UnsignedWide,
  pub R24: UnsignedWide,
  pub R25: UnsignedWide,
  pub R26: UnsignedWide,
  pub R27: UnsignedWide,
  pub R28: UnsignedWide,
  pub R29: UnsignedWide,
  pub R30: UnsignedWide,
  pub R31: UnsignedWide,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPUInformationPowerPC {
  pub Registers: [UnsignedWide; 32],
  pub FPSCR: usize,
  pub Reserved: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector128 {
  pub l: [usize; 4],
  pub s: [u16; 8],
  pub c: [u8; 16],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorInformationPowerPC {
  pub Registers: [Vector128; 32],
  pub VSCR: Vector128,
  pub VRsave: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryExceptionInformation {
  pub theArea: *mut OpaqueAreaID,
  pub theAddress: *mut c_void,
  pub theError: i32,
  pub theReference: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ExceptionInfo {
  pub memoryInfo: *mut MemoryExceptionInformation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExceptionInformationPowerPC {
  pub theKind: usize,
  pub machineState: *mut MachineInformationPowerPC,
  pub registerImage: *mut RegisterInformationPowerPC,
  pub FPUImage: *mut FPUInformationPowerPC,
  pub info: ExceptionInfo,
  pub vectorImage: *mut VectorInformationPowerPC,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Vector128Intel {
  pub s: (),
  pub si: (),
  pub sd: (),
  pub c: [u8; 16],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MachineInformationIntel64 {
  pub CS: usize,
  pub FS: usize,
  pub GS: usize,
  pub RFLAGS: usize,
  pub RIP: usize,
  pub ExceptTrap: usize,
  pub ExceptErr: usize,
  pub ExceptAddr: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegisterInformationIntel64 {
  pub RAX: usize,
  pub RBX: usize,
  pub RCX: usize,
  pub RDX: usize,
  pub RDI: usize,
  pub RSI: usize,
  pub RBP: usize,
  pub RSP: usize,
  pub R8: usize,
  pub R9: usize,
  pub R10: usize,
  pub R11: usize,
  pub R12: usize,
  pub R13: usize,
  pub R14: usize,
  pub R15: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FPUInformationIntel64 {
  pub Registers: [[u8; 10]; 8],
  pub Control: u16,
  pub Status: u16,
  pub Tag: u16,
  pub Opcode: u16,
  pub IP: u32,
  pub DP: u32,
  pub DS: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VectorInformationIntel64 {
  pub Registers: [Vector128Intel; 16],
}
pub type MachineInformation = MachineInformationIntel64;
pub type RegisterInformation = RegisterInformationIntel64;
pub type FPUInformation = FPUInformationIntel64;
pub type VectorInformation = VectorInformationIntel64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExceptionInformation {
  pub theKind: usize,
  pub machineState: *mut MachineInformationIntel64,
  pub registerImage: *mut RegisterInformationIntel64,
  pub FPUImage: *mut FPUInformationIntel64,
  pub info: ExceptionInfo,
  pub vectorImage: *mut VectorInformationIntel64,
}
pub type ExceptionHandlerUPP = Option<extern "C" fn(*mut ExceptionInformation) -> i32>;
pub type ExceptionHandlerTPP = Option<extern "C" fn(*mut ExceptionInformation) -> i32>;
pub type ExceptionHandler = Option<extern "C" fn(*mut ExceptionInformation) -> i32>;
pub type Nanoseconds = UnsignedWide;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NumFormatString {
  pub fLength: u8,
  pub fVersion: u8,
  pub data: [i8; 254],
}
pub type NumFormatStringRec = NumFormatString;
pub type FormatClass = i8;
pub type FormatResultType = i8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FVector {
  pub start: i16,
  pub length: i16,
}
pub type UCKeyOutput = u16;
pub type UCKeyCharSeq = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyStateRecord {
  pub stateZeroCharData: u16,
  pub stateZeroNextState: u16,
  pub stateEntryCount: u16,
  pub stateEntryFormat: u16,
  pub stateEntryData: [u32; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyStateEntryTerminal {
  pub curState: u16,
  pub charData: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyStateEntryRange {
  pub curStateStart: u16,
  pub curStateRange: u8,
  pub deltaMultiplier: u8,
  pub charData: u16,
  pub nextState: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyboardTypeHeader {
  pub keyboardTypeFirst: u32,
  pub keyboardTypeLast: u32,
  pub keyModifiersToTableNumOffset: u32,
  pub keyToCharTableIndexOffset: u32,
  pub keyStateRecordsIndexOffset: u32,
  pub keyStateTerminatorsOffset: u32,
  pub keySequenceDataIndexOffset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyboardLayout {
  pub keyLayoutHeaderFormat: u16,
  pub keyLayoutDataVersion: u16,
  pub keyLayoutFeatureInfoOffset: u32,
  pub keyboardTypeCount: u32,
  pub keyboardTypeList: [UCKeyboardTypeHeader; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyLayoutFeatureInfo {
  pub keyLayoutFeatureInfoFormat: u16,
  pub reserved: u16,
  pub maxOutputStringLength: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyModifiersToTableNum {
  pub keyModifiersToTableNumFormat: u16,
  pub defaultTableNum: u16,
  pub modifiersCount: u32,
  pub tableNum: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyToCharTableIndex {
  pub keyToCharTableIndexFormat: u16,
  pub keyToCharTableSize: u16,
  pub keyToCharTableCount: u32,
  pub keyToCharTableOffsets: [u32; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyStateRecordsIndex {
  pub keyStateRecordsIndexFormat: u16,
  pub keyStateRecordCount: u16,
  pub keyStateRecordOffsets: [u32; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeyStateTerminators {
  pub keyStateTerminatorsFormat: u16,
  pub keyStateTerminatorCount: u16,
  pub keyStateTerminators: [u16; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UCKeySequenceDataIndex {
  pub keySequenceDataIndexFormat: u16,
  pub charSequenceCount: u16,
  pub charSequenceOffsets: [u16; 1],
}
#[repr(C)]
pub struct OpaqueCollatorRef {
  opaque: u32,
}
pub type UCCollateOptions = u32;
pub type UCCollationValue = u32;
#[repr(C)]
pub struct OpaqueUCTypeSelectRef {
  opaque: u32,
}
pub type UCTypeSelectCompareResult = i32;
pub type UCTSWalkDirection = u16;
pub type UCTypeSelectOptions = u16;
pub type IndexToUCStringUPP =
  Option<extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8>;
#[repr(C)]
pub struct OpaqueTextBreakLocatorRef {
  opaque: u32,
}
pub type UCTextBreakType = u32;
pub type UCTextBreakOptions = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct decimal {
  pub sgn: i8,
  pub unused: i8,
  pub exp: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct decform {
  pub style: i8,
  pub unused: i8,
  pub digits: i16,
}
pub type TECPluginSignature = u32;
pub type TECPluginVersion = u32;
#[repr(C)]
pub struct OpaqueTECObjectRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueTECSnifferObjectRef {
  opaque: u32,
}
pub type TECPluginSig = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECConversionInfo {
  pub sourceEncoding: u32,
  pub destinationEncoding: u32,
  pub reserved1: u16,
  pub reserved2: u16,
}
pub type TECInternetNameUsageMask = u32;
#[repr(C)]
pub struct OpaqueTextToUnicodeInfo {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueUnicodeToTextInfo {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueUnicodeToTextRunInfo {
  opaque: u32,
}
pub type ConstTextToUnicodeInfo = *mut OpaqueTextToUnicodeInfo;
pub type ConstUnicodeToTextInfo = *mut OpaqueUnicodeToTextInfo;
pub type UnicodeMapVersion = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UnicodeMapping {
  pub unicodeEncoding: u32,
  pub otherEncoding: u32,
  pub mappingVersion: i32,
}
pub type UnicodeToTextFallbackUPP = Option<
  extern "C" fn(
    *mut u16,
    usize,
    *mut usize,
    *mut u8,
    usize,
    *mut usize,
    *mut c_void,
    *mut UnicodeMapping,
  ) -> i32,
>;
pub type ThreadState = u16;
pub type ThreadStyle = u32;
pub type ThreadOptions = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SchedulerInfoRec {
  pub InfoRecSize: u32,
  pub CurrentThreadID: usize,
  pub SuggestedThreadID: usize,
  pub InterruptedCoopThreadID: usize,
}
pub type ThreadEntryUPP = Option<extern "C" fn(*mut c_void) -> *mut c_void>;
pub type ThreadSchedulerUPP = Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
pub type ThreadSwitchUPP = Option<extern "C" fn(usize, *mut c_void) -> ()>;
pub type ThreadTerminationUPP = Option<extern "C" fn(usize, *mut c_void) -> ()>;
pub type DebuggerNewThreadUPP = Option<extern "C" fn(usize) -> ()>;
pub type DebuggerDisposeThreadUPP = Option<extern "C" fn(usize) -> ()>;
pub type DebuggerThreadSchedulerUPP = Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
pub type ThreadEntryTPP = Option<extern "C" fn(*mut c_void) -> *mut c_void>;
pub type ThreadSchedulerTPP = Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
pub type ThreadSwitchTPP = Option<extern "C" fn(usize, *mut c_void) -> ()>;
pub type ThreadTerminationTPP = Option<extern "C" fn(usize, *mut c_void) -> ()>;
pub type DebuggerNewThreadTPP = Option<extern "C" fn(usize) -> ()>;
pub type DebuggerDisposeThreadTPP = Option<extern "C" fn(usize) -> ()>;
pub type DebuggerThreadSchedulerTPP = Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
pub type FolderDescFlags = u32;
pub type FolderClass = u32;
pub type FolderType = u32;
pub type FolderLocation = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FolderDesc {
  pub descSize: isize,
  pub foldType: u32,
  pub flags: u32,
  pub foldClass: u32,
  pub foldLocation: u32,
  pub badgeSignature: u32,
  pub badgeType: u32,
  pub reserved: u32,
  pub name: [u8; 64],
}
pub type RoutingFlags = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FolderRouting {
  pub descSize: isize,
  pub fileType: u32,
  pub routeFromFolder: u32,
  pub routeToFolder: u32,
  pub flags: u32,
}
pub type FolderManagerNotificationUPP = Option<extern "C" fn(u32, *mut c_void, *mut c_void) -> i32>;
#[repr(C)]
pub struct TMTask {
  opaque: u32,
}
pub type TimerUPP = Option<extern "C" fn(*mut TMTask) -> ()>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPQueueInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub queueName: u32,
  pub nWaiting: usize,
  pub waitingTaskID: *mut OpaqueMPTaskID,
  pub nMessages: usize,
  pub nReserved: usize,
  pub p1: *mut c_void,
  pub p2: *mut c_void,
  pub p3: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPSemaphoreInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub semaphoreName: u32,
  pub nWaiting: usize,
  pub waitingTaskID: *mut OpaqueMPTaskID,
  pub maximum: usize,
  pub count: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPEventInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub eventName: u32,
  pub nWaiting: usize,
  pub waitingTaskID: *mut OpaqueMPTaskID,
  pub events: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPCriticalRegionInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub regionName: u32,
  pub nWaiting: usize,
  pub waitingTaskID: *mut OpaqueMPTaskID,
  pub owningTask: *mut OpaqueMPTaskID,
  pub count: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPNotificationInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub notificationName: u32,
  pub queueID: *mut OpaqueMPQueueID,
  pub p1: *mut c_void,
  pub p2: *mut c_void,
  pub p3: *mut c_void,
  pub eventID: *mut OpaqueMPEventID,
  pub events: u32,
  pub semaphoreID: *mut OpaqueMPSemaphoreID,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPAddressSpaceInfo {
  pub version: u32,
  pub processID: *mut OpaqueMPProcessID,
  pub groupID: *mut OpaqueMPCoherenceID,
  pub nTasks: usize,
  pub vsid: [u32; 16],
}
pub type AVLVisitStage = u16;
pub type AVLOrder = u16;
pub type AVLNodeType = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVLTreeStruct {
  pub signature: u32,
  pub privateStuff: [usize; 8],
}
pub type AVLCompareItemsUPP =
  Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, *mut c_void, u16) -> i32>;
pub type AVLItemSizeUPP = Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> usize>;
pub type AVLDisposeItemUPP = Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> ()>;
pub type AVLWalkUPP =
  Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, u16, u16, u32, i32, *mut c_void) -> i16>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFContainerHeader {
  pub tag1: u32,
  pub tag2: u32,
  pub architecture: u32,
  pub formatVersion: u32,
  pub dateTimeStamp: u32,
  pub oldDefVersion: u32,
  pub oldImpVersion: u32,
  pub currentVersion: u32,
  pub sectionCount: u16,
  pub instSectionCount: u16,
  pub reservedA: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFSectionHeader {
  pub nameOffset: i32,
  pub defaultAddress: u32,
  pub totalLength: u32,
  pub unpackedLength: u32,
  pub containerLength: u32,
  pub containerOffset: u32,
  pub sectionKind: u8,
  pub shareKind: u8,
  pub alignment: u8,
  pub reservedA: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFLoaderInfoHeader {
  pub mainSection: i32,
  pub mainOffset: u32,
  pub initSection: i32,
  pub initOffset: u32,
  pub termSection: i32,
  pub termOffset: u32,
  pub importedLibraryCount: u32,
  pub totalImportedSymbolCount: u32,
  pub relocSectionCount: u32,
  pub relocInstrOffset: u32,
  pub loaderStringsOffset: u32,
  pub exportHashOffset: u32,
  pub exportHashTablePower: u32,
  pub exportedSymbolCount: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFImportedLibrary {
  pub nameOffset: u32,
  pub oldImpVersion: u32,
  pub currentVersion: u32,
  pub importedSymbolCount: u32,
  pub firstImportedSymbol: u32,
  pub options: u8,
  pub reservedA: u8,
  pub reservedB: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFImportedSymbol {
  pub classAndName: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFExportedSymbolHashSlot {
  pub countAndStart: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFSplitHashWord {
  pub nameLength: u16,
  pub hashValue: u16,
}
#[repr(C)]
pub struct PEFExportedSymbolKey {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFExportedSymbol {
  pub classAndName: u32,
  pub symbolValue: u32,
  pub sectionIndex: i16,
}
pub type PEFRelocChunk = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PEFLoaderRelocationHeader {
  pub sectionIndex: u16,
  pub reservedA: u16,
  pub relocCount: u32,
  pub firstRelocOffset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XLibContainerHeader {
  pub tag1: u32,
  pub tag2: u32,
  pub currentFormat: u32,
  pub containerStringsOffset: u32,
  pub exportHashOffset: u32,
  pub exportKeyOffset: u32,
  pub exportSymbolOffset: u32,
  pub exportNamesOffset: u32,
  pub exportHashTablePower: u32,
  pub exportedSymbolCount: u32,
  pub fragNameOffset: u32,
  pub fragNameLength: u32,
  pub dylibPathOffset: u32,
  pub dylibPathLength: u32,
  pub cpuFamily: u32,
  pub cpuModel: u32,
  pub dateTimeStamp: u32,
  pub currentVersion: u32,
  pub oldDefVersion: u32,
  pub oldImpVersion: u32,
}
pub type XLibExportedSymbolHashSlot = PEFExportedSymbolHashSlot;
pub type XLibExportedSymbolKey = PEFExportedSymbolKey;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XLibExportedSymbol {
  pub classAndName: u32,
  pub bpOffset: u32,
}
pub type HFSCatalogNodeID = u32;
pub type MarkerIdType = i16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ChunkHeader {
  pub ckID: u32,
  pub ckSize: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContainerChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub formType: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatVersionChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub timestamp: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommonChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub numChannels: i16,
  pub numSampleFrames: u32,
  pub sampleSize: i16,
  pub sampleRate: Float80,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtCommonChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub numChannels: i16,
  pub numSampleFrames: u32,
  pub sampleSize: i16,
  pub sampleRate: Float80,
  pub compressionType: u32,
  pub compressionName: [i8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SoundDataChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub offset: u32,
  pub blockSize: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Marker {
  pub id: i16,
  pub position: u32,
  pub markerName: [u8; 256],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MarkerChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub numMarkers: u16,
  pub Markers: [Marker; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AIFFLoop {
  pub playMode: i16,
  pub beginLoop: i16,
  pub endLoop: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstrumentChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub baseFrequency: u8,
  pub detune: u8,
  pub lowFrequency: u8,
  pub highFrequency: u8,
  pub lowVelocity: u8,
  pub highVelocity: u8,
  pub gain: i16,
  pub sustainLoop: AIFFLoop,
  pub releaseLoop: AIFFLoop,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIDataChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub MIDIdata: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioRecordingChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub AESChannelStatus: [u8; 24],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationSpecificChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub applicationSignature: u32,
  pub data: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Comment {
  pub timeStamp: u32,
  pub marker: i16,
  pub count: u16,
  pub text: [i8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommentsChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub numComments: u16,
  pub comments: [Comment; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextChunk {
  pub ckID: u32,
  pub ckSize: i32,
  pub text: [i8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextEncodingRec {
  pub base: u32,
  pub variant: u32,
  pub format: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECEncodingsListRec {
  pub count: u32,
  pub encodings: TextEncodingRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECSubTextEncodingRec {
  pub offset: u32,
  pub searchEncoding: TextEncodingRec,
  pub count: u32,
  pub subEncodings: TextEncodingRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECSubTextEncodingsRec {
  pub count: u32,
  pub subTextEncodingRec: TECSubTextEncodingRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECEncodingPairRec {
  pub source: TextEncodingRec,
  pub dest: TextEncodingRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECEncodingPairs {
  pub encodingPair: TECEncodingPairRec,
  pub flags: u32,
  pub speed: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECEncodingPairsRec {
  pub count: u32,
  pub encodingPairs: TECEncodingPairs,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECLocaleListToEncodingListRec {
  pub offset: u32,
  pub count: u32,
  pub locales: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECLocaleToEncodingsListRec {
  pub count: u32,
  pub localeListToEncodingList: TECLocaleListToEncodingListRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECInternetNameRec {
  pub offset: u32,
  pub searchEncoding: TextEncodingRec,
  pub encodingNameLength: u8,
  pub encodingName: [u8; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECInternetNamesRec {
  pub count: u32,
  pub InternetNames: TECInternetNameRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECBufferContextRec {
  pub textInputBuffer: *mut u8,
  pub textInputBufferEnd: *mut u8,
  pub textOutputBuffer: *mut u8,
  pub textOutputBufferEnd: *mut u8,
  pub encodingInputBuffer: *mut TextEncodingRun,
  pub encodingInputBufferEnd: *mut TextEncodingRun,
  pub encodingOutputBuffer: *mut TextEncodingRun,
  pub encodingOutputBufferEnd: *mut TextEncodingRun,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECPluginStateRec {
  pub state1: u8,
  pub state2: u8,
  pub state3: u8,
  pub state4: u8,
  pub longState1: u32,
  pub longState2: u32,
  pub longState3: u32,
  pub longState4: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECConverterContextRec {
  pub pluginRec: *mut i8,
  pub sourceEncoding: u32,
  pub destEncoding: u32,
  pub reserved1: u32,
  pub reserved2: u32,
  pub bufferContext: TECBufferContextRec,
  pub contextRefCon: *mut c_void,
  pub conversionProc: Option<extern "C" fn() -> isize>,
  pub flushProc: Option<extern "C" fn() -> isize>,
  pub clearContextInfoProc: Option<extern "C" fn() -> isize>,
  pub options1: u32,
  pub options2: u32,
  pub pluginState: TECPluginStateRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECSnifferContextRec {
  pub pluginRec: *mut i8,
  pub encoding: u32,
  pub maxErrors: usize,
  pub maxFeatures: usize,
  pub textInputBuffer: *mut u8,
  pub textInputBufferEnd: *mut u8,
  pub numFeatures: usize,
  pub numErrors: usize,
  pub contextRefCon: *mut c_void,
  pub sniffProc: Option<extern "C" fn() -> isize>,
  pub clearContextInfoProc: Option<extern "C" fn() -> isize>,
  pub pluginState: TECPluginStateRec,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TECPluginDispatchTable {
  pub version: u32,
  pub compatibleVersion: u32,
  pub PluginID: u32,
  pub PluginNewEncodingConverter: Option<
    extern "C" fn(*mut *mut OpaqueTECObjectRef, *mut TECConverterContextRec, u32, u32) -> i32,
  >,
  pub PluginClearContextInfo:
    Option<extern "C" fn(*mut OpaqueTECObjectRef, *mut TECConverterContextRec) -> i32>,
  pub PluginConvertTextEncoding:
    Option<extern "C" fn(*mut OpaqueTECObjectRef, *mut TECConverterContextRec) -> i32>,
  pub PluginFlushConversion:
    Option<extern "C" fn(*mut OpaqueTECObjectRef, *mut TECConverterContextRec) -> i32>,
  pub PluginDisposeEncodingConverter:
    Option<extern "C" fn(*mut OpaqueTECObjectRef, *mut TECConverterContextRec) -> i32>,
  pub PluginNewEncodingSniffer: Option<
    extern "C" fn(*mut *mut OpaqueTECSnifferObjectRef, *mut TECSnifferContextRec, u32) -> i32,
  >,
  pub PluginClearSnifferContextInfo:
    Option<extern "C" fn(*mut OpaqueTECSnifferObjectRef, *mut TECSnifferContextRec) -> i32>,
  pub PluginSniffTextEncoding:
    Option<extern "C" fn(*mut OpaqueTECSnifferObjectRef, *mut TECSnifferContextRec) -> i32>,
  pub PluginDisposeEncodingSniffer:
    Option<extern "C" fn(*mut OpaqueTECSnifferObjectRef, *mut TECSnifferContextRec) -> i32>,
  pub PluginGetCountAvailableTextEncodings:
    Option<extern "C" fn(*mut u32, usize, *mut usize) -> i32>,
  pub PluginGetCountAvailableTextEncodingPairs:
    Option<extern "C" fn(*mut TECConversionInfo, usize, *mut usize) -> i32>,
  pub PluginGetCountDestinationTextEncodings:
    Option<extern "C" fn(u32, *mut u32, usize, *mut usize) -> i32>,
  pub PluginGetCountSubTextEncodings:
    Option<extern "C" fn(u32, *mut u32, usize, *mut usize) -> i32>,
  pub PluginGetCountAvailableSniffers: Option<extern "C" fn(*mut u32, usize, *mut usize) -> i32>,
  pub PluginGetCountWebTextEncodings: Option<extern "C" fn(*mut u32, usize, *mut usize) -> i32>,
  pub PluginGetCountMailTextEncodings: Option<extern "C" fn(*mut u32, usize, *mut usize) -> i32>,
  pub PluginGetTextEncodingInternetName: Option<extern "C" fn(u32, [u8; 256]) -> i32>,
  pub PluginGetTextEncodingFromInternetName: Option<extern "C" fn(*mut u32, *mut u8) -> i32>,
}
extern "C" {
  pub fn FixRatio(numer: i16, denom: i16) -> i32;
  pub fn FixMul(a: i32, b: i32) -> i32;
  pub fn FixRound(x: i32) -> i16;
  pub fn Fix2Frac(x: i32) -> i32;
  pub fn Fix2Long(x: i32) -> i32;
  pub fn Long2Fix(x: i32) -> i32;
  pub fn Frac2Fix(x: i32) -> i32;
  pub fn FracMul(x: i32, y: i32) -> i32;
  pub fn FixDiv(x: i32, y: i32) -> i32;
  pub fn FracDiv(x: i32, y: i32) -> i32;
  pub fn FracSqrt(x: i32) -> i32;
  pub fn FracSin(x: i32) -> i32;
  pub fn FracCos(x: i32) -> i32;
  pub fn FixATan2(x: i32, y: i32) -> i32;
  pub fn Frac2X(x: i32) -> f64;
  pub fn Fix2X(x: i32) -> f64;
  pub fn X2Fix(x: f64) -> i32;
  pub fn X2Frac(x: f64) -> i32;
  pub fn WideCompare(target: *mut wide, source: *mut wide) -> i16;
  pub fn WideAdd(target: *mut wide, source: *mut wide) -> *mut wide;
  pub fn WideSubtract(target: *mut wide, source: *mut wide) -> *mut wide;
  pub fn WideNegate(target: *mut wide) -> *mut wide;
  pub fn WideShift(target: *mut wide, shift: i32) -> *mut wide;
  pub fn WideSquareRoot(source: *mut wide) -> u32;
  pub fn WideMultiply(multiplicand: i32, multiplier: i32, target: *mut wide) -> *mut wide;
  pub fn WideDivide(dividend: *mut wide, divisor: i32, remainder: *mut i32) -> i32;
  pub fn WideWideDivide(dividend: *mut wide, divisor: i32, remainder: *mut i32) -> *mut wide;
  pub fn WideBitShift(target: *mut wide, shift: i32) -> *mut wide;
  pub fn UnsignedFixedMulDiv(value: u32, multiplier: u32, divisor: u32) -> u32;
  pub fn GetScriptManagerVariable(selector: i16) -> isize;
  pub fn SetScriptManagerVariable(selector: i16, param: isize) -> i16;
  pub fn SysError(errorCode: i16) -> ();
  pub fn CreateTextEncoding(encodingBase: u32, encodingVariant: u32, encodingFormat: u32) -> u32;
  pub fn GetTextEncodingBase(encoding: u32) -> u32;
  pub fn GetTextEncodingVariant(encoding: u32) -> u32;
  pub fn GetTextEncodingFormat(encoding: u32) -> u32;
  pub fn ResolveDefaultTextEncoding(encoding: u32) -> u32;
  pub fn GetTextEncodingName(
    iEncoding: u32,
    iNamePartSelector: u32,
    iPreferredRegion: i16,
    iPreferredEncoding: u32,
    iOutputBufLen: usize,
    oNameLength: *mut usize,
    oActualRegion: *mut i16,
    oActualEncoding: *mut u32,
    oEncodingName: *mut u8,
  ) -> i32;
  pub fn TECGetInfo(tecInfo: *mut *mut *mut TECInfo) -> i32;
  pub fn UpgradeScriptInfoToTextEncoding(
    iTextScriptID: i16,
    iTextLanguageID: i16,
    iRegionID: i16,
    iTextFontname: *mut u8,
    oEncoding: *mut u32,
  ) -> i32;
  pub fn RevertTextEncodingToScriptInfo(
    iEncoding: u32,
    oTextScriptID: *mut i16,
    oTextLanguageID: *mut i16,
    oTextFontname: [u8; 256],
  ) -> i32;
  pub fn GetTextEncodingFromScriptInfo(
    iTextScriptID: i16,
    iTextLanguageID: i16,
    iTextRegionID: i16,
    oEncoding: *mut u32,
  ) -> i32;
  pub fn GetScriptInfoFromTextEncoding(
    iEncoding: u32,
    oTextScriptID: *mut i16,
    oTextLanguageID: *mut i16,
  ) -> i32;
  pub fn NearestMacTextEncodings(
    generalEncoding: u32,
    bestMacEncoding: *mut u32,
    alternateMacEncoding: *mut u32,
  ) -> i32;
  pub fn UCGetCharProperty(
    charPtr: *mut u16,
    textLength: usize,
    propType: i32,
    propValue: *mut u32,
  ) -> i32;
  pub fn UCIsSurrogateHighCharacter(character: u16) -> u8;
  pub fn UCIsSurrogateLowCharacter(character: u16) -> u8;
  pub fn UCGetUnicodeScalarValueForSurrogatePair(surrogateHigh: u16, surrogateLow: u16) -> u32;
  pub fn NewCollectionFlattenUPP(
    userRoutine: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
  ) -> Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>;
  pub fn NewCollectionExceptionUPP(
    userRoutine: Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>,
  ) -> Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>;
  pub fn DisposeCollectionFlattenUPP(
    userUPP: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
  ) -> ();
  pub fn DisposeCollectionExceptionUPP(
    userUPP: Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>,
  ) -> ();
  pub fn InvokeCollectionFlattenUPP(
    size: i32,
    data: *mut c_void,
    refCon: *mut c_void,
    userUPP: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
  ) -> i16;
  pub fn InvokeCollectionExceptionUPP(
    c: *mut OpaqueCollection,
    status: i16,
    userUPP: Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>,
  ) -> i16;
  pub fn NewCollection() -> *mut OpaqueCollection;
  pub fn DisposeCollection(c: *mut OpaqueCollection) -> ();
  pub fn CloneCollection(c: *mut OpaqueCollection) -> *mut OpaqueCollection;
  pub fn CountCollectionOwners(c: *mut OpaqueCollection) -> i32;
  pub fn RetainCollection(c: *mut OpaqueCollection) -> i32;
  pub fn ReleaseCollection(c: *mut OpaqueCollection) -> i32;
  pub fn GetCollectionRetainCount(c: *mut OpaqueCollection) -> usize;
  pub fn CopyCollection(
    srcCollection: *mut OpaqueCollection,
    dstCollection: *mut OpaqueCollection,
  ) -> *mut OpaqueCollection;
  pub fn GetCollectionDefaultAttributes(c: *mut OpaqueCollection) -> i32;
  pub fn SetCollectionDefaultAttributes(
    c: *mut OpaqueCollection,
    whichAttributes: i32,
    newAttributes: i32,
  ) -> ();
  pub fn CountCollectionItems(c: *mut OpaqueCollection) -> i32;
  pub fn AddCollectionItem(
    c: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    itemSize: i32,
    itemData: *mut c_void,
  ) -> i16;
  pub fn GetCollectionItem(
    c: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    itemSize: *mut i32,
    itemData: *mut c_void,
  ) -> i16;
  pub fn RemoveCollectionItem(c: *mut OpaqueCollection, tag: u32, id: i32) -> i16;
  pub fn SetCollectionItemInfo(
    c: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    whichAttributes: i32,
    newAttributes: i32,
  ) -> i16;
  pub fn GetCollectionItemInfo(
    c: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    itemIndex: *mut i32,
    itemSize: *mut i32,
    attributes: *mut i32,
  ) -> i16;
  pub fn ReplaceIndexedCollectionItem(
    c: *mut OpaqueCollection,
    itemIndex: i32,
    itemSize: i32,
    itemData: *mut c_void,
  ) -> i16;
  pub fn GetIndexedCollectionItem(
    c: *mut OpaqueCollection,
    itemIndex: i32,
    itemSize: *mut i32,
    itemData: *mut c_void,
  ) -> i16;
  pub fn RemoveIndexedCollectionItem(c: *mut OpaqueCollection, itemIndex: i32) -> i16;
  pub fn SetIndexedCollectionItemInfo(
    c: *mut OpaqueCollection,
    itemIndex: i32,
    whichAttributes: i32,
    newAttributes: i32,
  ) -> i16;
  pub fn GetIndexedCollectionItemInfo(
    c: *mut OpaqueCollection,
    itemIndex: i32,
    tag: *mut u32,
    id: *mut i32,
    itemSize: *mut i32,
    attributes: *mut i32,
  ) -> i16;
  pub fn CollectionTagExists(c: *mut OpaqueCollection, tag: u32) -> u8;
  pub fn CountCollectionTags(c: *mut OpaqueCollection) -> i32;
  pub fn GetIndexedCollectionTag(c: *mut OpaqueCollection, tagIndex: i32, tag: *mut u32) -> i16;
  pub fn CountTaggedCollectionItems(c: *mut OpaqueCollection, tag: u32) -> i32;
  pub fn GetTaggedCollectionItem(
    c: *mut OpaqueCollection,
    tag: u32,
    whichItem: i32,
    itemSize: *mut i32,
    itemData: *mut c_void,
  ) -> i16;
  pub fn GetTaggedCollectionItemInfo(
    c: *mut OpaqueCollection,
    tag: u32,
    whichItem: i32,
    id: *mut i32,
    itemIndex: *mut i32,
    itemSize: *mut i32,
    attributes: *mut i32,
  ) -> i16;
  pub fn PurgeCollection(
    c: *mut OpaqueCollection,
    whichAttributes: i32,
    matchingAttributes: i32,
  ) -> ();
  pub fn PurgeCollectionTag(c: *mut OpaqueCollection, tag: u32) -> ();
  pub fn EmptyCollection(c: *mut OpaqueCollection) -> ();
  pub fn FlattenCollection(
    c: *mut OpaqueCollection,
    flattenProc: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
    refCon: *mut c_void,
  ) -> i16;
  pub fn FlattenPartialCollection(
    c: *mut OpaqueCollection,
    flattenProc: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
    refCon: *mut c_void,
    whichAttributes: i32,
    matchingAttributes: i32,
  ) -> i16;
  pub fn UnflattenCollection(
    c: *mut OpaqueCollection,
    flattenProc: Option<extern "C" fn(i32, *mut c_void, *mut c_void) -> i16>,
    refCon: *mut c_void,
  ) -> i16;
  pub fn GetCollectionExceptionProc(
    c: *mut OpaqueCollection,
  ) -> Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>;
  pub fn SetCollectionExceptionProc(
    c: *mut OpaqueCollection,
    exceptionProc: Option<extern "C" fn(*mut OpaqueCollection, i16) -> i16>,
  ) -> ();
  pub fn GetNewCollection(collectionID: i16) -> *mut OpaqueCollection;
  pub fn AddCollectionItemHdl(
    aCollection: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    itemData: *mut *mut i8,
  ) -> i16;
  pub fn GetCollectionItemHdl(
    aCollection: *mut OpaqueCollection,
    tag: u32,
    id: i32,
    itemData: *mut *mut i8,
  ) -> i16;
  pub fn ReplaceIndexedCollectionItemHdl(
    aCollection: *mut OpaqueCollection,
    itemIndex: i32,
    itemData: *mut *mut i8,
  ) -> i16;
  pub fn GetIndexedCollectionItemHdl(
    aCollection: *mut OpaqueCollection,
    itemIndex: i32,
    itemData: *mut *mut i8,
  ) -> i16;
  pub fn FlattenCollectionToHdl(aCollection: *mut OpaqueCollection, flattened: *mut *mut i8)
    -> i16;
  pub fn UnflattenCollectionFromHdl(
    aCollection: *mut OpaqueCollection,
    flattened: *mut *mut i8,
  ) -> i16;
  pub fn CoreEndianInstallFlipper(
    dataDomain: u32,
    dataType: u32,
    proc_: Option<extern "C" fn(u32, u32, i16, *mut c_void, usize, u8, *mut c_void) -> i32>,
    refcon: *mut c_void,
  ) -> i32;
  pub fn CoreEndianGetFlipper(
    dataDomain: u32,
    dataType: u32,
    proc_: *mut Option<extern "C" fn(u32, u32, i16, *mut c_void, usize, u8, *mut c_void) -> i32>,
    refcon: *mut *mut c_void,
  ) -> i32;
  pub fn CoreEndianFlipData(
    dataDomain: u32,
    dataType: u32,
    id: i16,
    data: *mut c_void,
    dataLen: usize,
    currentlyNative: u8,
  ) -> i32;
  pub fn Gestalt(selector: u32, response: *mut i32) -> i16;
  pub fn NewGestaltValue(selector: u32, newValue: i32) -> i16;
  pub fn ReplaceGestaltValue(selector: u32, replacementValue: i32) -> i16;
  pub fn SetGestaltValue(selector: u32, newValue: i32) -> i16;
  pub fn DeleteGestaltValue(selector: u32) -> i16;
  pub fn NewSelectorFunctionUPP(
    userRoutine: Option<extern "C" fn(u32, *mut i32) -> i16>,
  ) -> Option<extern "C" fn(u32, *mut i32) -> i16>;
  pub fn DisposeSelectorFunctionUPP(userUPP: Option<extern "C" fn(u32, *mut i32) -> i16>) -> ();
  pub fn InvokeSelectorFunctionUPP(
    selector: u32,
    response: *mut i32,
    userUPP: Option<extern "C" fn(u32, *mut i32) -> i16>,
  ) -> i16;
  pub fn MemError() -> i16;
  pub fn LMGetMemErr() -> i16;
  pub fn LMSetMemErr(value: i16) -> ();
  pub fn NewHandle(byteCount: isize) -> *mut *mut i8;
  pub fn NewHandleClear(byteCount: isize) -> *mut *mut i8;
  pub fn RecoverHandle(p: *mut i8) -> *mut *mut i8;
  pub fn NewPtr(byteCount: isize) -> *mut i8;
  pub fn NewPtrClear(byteCount: isize) -> *mut i8;
  pub fn NewEmptyHandle() -> *mut *mut i8;
  pub fn HLock(h: *mut *mut i8) -> ();
  pub fn HLockHi(h: *mut *mut i8) -> ();
  pub fn HUnlock(h: *mut *mut i8) -> ();
  pub fn TempNewHandle(logicalSize: isize, resultCode: *mut i16) -> *mut *mut i8;
  pub fn DisposePtr(p: *mut i8) -> ();
  pub fn GetPtrSize(p: *mut i8) -> isize;
  pub fn SetPtrSize(p: *mut i8, newSize: isize) -> ();
  pub fn DisposeHandle(h: *mut *mut i8) -> ();
  pub fn SetHandleSize(h: *mut *mut i8, newSize: isize) -> ();
  pub fn GetHandleSize(h: *mut *mut i8) -> isize;
  pub fn ReallocateHandle(h: *mut *mut i8, byteCount: isize) -> ();
  pub fn EmptyHandle(h: *mut *mut i8) -> ();
  pub fn HSetRBit(h: *mut *mut i8) -> ();
  pub fn HClrRBit(h: *mut *mut i8) -> ();
  pub fn HGetState(h: *mut *mut i8) -> i8;
  pub fn HSetState(h: *mut *mut i8, flags: i8) -> ();
  pub fn HandToHand(theHndl: *mut *mut *mut i8) -> i16;
  pub fn PtrToXHand(srcPtr: *mut c_void, dstHndl: *mut *mut i8, size: isize) -> i16;
  pub fn PtrToHand(srcPtr: *mut c_void, dstHndl: *mut *mut *mut i8, size: isize) -> i16;
  pub fn HandAndHand(hand1: *mut *mut i8, hand2: *mut *mut i8) -> i16;
  pub fn PtrAndHand(ptr1: *mut c_void, hand2: *mut *mut i8, size: isize) -> i16;
  pub fn IsHeapValid() -> u8;
  pub fn IsHandleValid(h: *mut *mut i8) -> u8;
  pub fn IsPointerValid(p: *mut i8) -> u8;
  pub fn S64Max() -> i64;
  pub fn S64Min() -> i64;
  pub fn S64Add(left: i64, right: i64) -> i64;
  pub fn S64Subtract(left: i64, right: i64) -> i64;
  pub fn S64Negate(value: i64) -> i64;
  pub fn S64Multiply(left: i64, right: i64) -> i64;
  pub fn S64Mod(dividend: i64, divisor: i64) -> i64;
  pub fn S64Divide(dividend: i64, divisor: i64, remainder: *mut i64) -> i64;
  pub fn S64Div(dividend: i64, divisor: i64) -> i64;
  pub fn S64Set(value: i32) -> i64;
  pub fn S64SetU(value: u32) -> i64;
  pub fn S32Set(value: i64) -> i32;
  pub fn S64And(left: i64, right: i64) -> u8;
  pub fn S64Or(left: i64, right: i64) -> u8;
  pub fn S64Eor(left: i64, right: i64) -> u8;
  pub fn S64Not(value: i64) -> u8;
  pub fn S64Compare(left: i64, right: i64) -> i32;
  pub fn S64BitwiseAnd(left: i64, right: i64) -> i64;
  pub fn S64BitwiseOr(left: i64, right: i64) -> i64;
  pub fn S64BitwiseEor(left: i64, right: i64) -> i64;
  pub fn S64BitwiseNot(value: i64) -> i64;
  pub fn S64ShiftRight(value: i64, shift: u32) -> i64;
  pub fn S64ShiftLeft(value: i64, shift: u32) -> i64;
  pub fn SInt64ToLongDouble(value: i64) -> ();
  pub fn LongDoubleToSInt64(value: ()) -> i64;
  pub fn U64Max() -> u64;
  pub fn U64Add(left: u64, right: u64) -> u64;
  pub fn U64Subtract(left: u64, right: u64) -> u64;
  pub fn U64Multiply(left: u64, right: u64) -> u64;
  pub fn U64Mod(dividend: u64, divisor: u64) -> u64;
  pub fn U64Divide(dividend: u64, divisor: u64, remainder: *mut u64) -> u64;
  pub fn U64Div(dividend: u64, divisor: u64) -> u64;
  pub fn U64Set(value: i32) -> u64;
  pub fn U64SetU(value: u32) -> u64;
  pub fn U32SetU(value: u64) -> u32;
  pub fn U64And(left: u64, right: u64) -> u8;
  pub fn U64Or(left: u64, right: u64) -> u8;
  pub fn U64Eor(left: u64, right: u64) -> u8;
  pub fn U64Not(value: u64) -> u8;
  pub fn U64Compare(left: u64, right: u64) -> i32;
  pub fn U64BitwiseAnd(left: u64, right: u64) -> u64;
  pub fn U64BitwiseOr(left: u64, right: u64) -> u64;
  pub fn U64BitwiseEor(left: u64, right: u64) -> u64;
  pub fn U64BitwiseNot(value: u64) -> u64;
  pub fn U64ShiftRight(value: u64, shift: u32) -> u64;
  pub fn U64ShiftLeft(value: u64, shift: u32) -> u64;
  pub fn UInt64ToLongDouble(value: u64) -> ();
  pub fn LongDoubleToUInt64(value: ()) -> u64;
  pub fn UInt64ToSInt64(value: u64) -> i64;
  pub fn SInt64ToUInt64(value: i64) -> u64;
  pub fn SInt64ToWide(s: i64) -> wide;
  pub fn WideToSInt64(w: wide) -> i64;
  pub fn UInt64ToUnsignedWide(u: u64) -> UnsignedWide;
  pub fn UnsignedWideToUInt64(uw: UnsignedWide) -> u64;
  pub fn CSBackupSetItemExcluded(item: *mut __CFURL, exclude: u8, excludeByPath: u8) -> i32;
  pub fn CSBackupIsItemExcluded(item: *mut __CFURL, excludeByPath: *mut u8) -> u8;
  pub fn CSDiskSpaceStartRecovery(
    volumeURL: *mut __CFURL,
    bytesNeeded: u64,
    options: i32,
    outOperationUUID: *mut *mut __CFUUID,
    callbackQueue: *mut NSObject,
    callback: (),
  ) -> ();
  pub fn CSDiskSpaceCancelRecovery(operationUUID: *mut __CFUUID) -> ();
  pub fn CSDiskSpaceGetRecoveryEstimate(volumeURL: *mut __CFURL) -> u64;
  pub fn UCConvertUTCDateTimeToCFAbsoluteTime(iUTCDate: *mut UTCDateTime, oCFTime: *mut f64)
    -> i32;
  pub fn UCConvertSecondsToCFAbsoluteTime(iSeconds: u32, oCFTime: *mut f64) -> i32;
  pub fn UCConvertLongDateTimeToCFAbsoluteTime(iLongTime: i64, oCFTime: *mut f64) -> i32;
  pub fn UCConvertCFAbsoluteTimeToUTCDateTime(iCFTime: f64, oUTCDate: *mut UTCDateTime) -> i32;
  pub fn UCConvertCFAbsoluteTimeToSeconds(iCFTime: f64, oSeconds: *mut u32) -> i32;
  pub fn UCConvertCFAbsoluteTimeToLongDateTime(iCFTime: f64, oLongDate: *mut i64) -> i32;
  pub fn IsMetric() -> u8;
  pub fn Delay(numTicks: usize, finalTicks: *mut usize) -> ();
  pub fn Enqueue(qElement: *mut QElem, qHeader: *mut QHdr) -> ();
  pub fn Dequeue(qElement: *mut QElem, qHeader: *mut QHdr) -> i16;
  pub fn ReadLocation(loc: *mut MachineLocation) -> ();
  pub fn TickCount() -> u32;
  pub fn CSCopyUserName(useShortName: u8) -> *mut __CFString;
  pub fn CSCopyMachineName() -> *mut __CFString;
  pub fn NewDeferredTaskUPP(
    userRoutine: Option<extern "C" fn(isize) -> ()>,
  ) -> Option<extern "C" fn(isize) -> ()>;
  pub fn DisposeDeferredTaskUPP(userUPP: Option<extern "C" fn(isize) -> ()>) -> ();
  pub fn InvokeDeferredTaskUPP(dtParam: isize, userUPP: Option<extern "C" fn(isize) -> ()>) -> ();
  pub fn NewIOCompletionUPP(
    userRoutine: Option<extern "C" fn(*mut c_void) -> ()>,
  ) -> Option<extern "C" fn(*mut c_void) -> ()>;
  pub fn DisposeIOCompletionUPP(userUPP: Option<extern "C" fn(*mut c_void) -> ()>) -> ();
  pub fn InvokeIOCompletionUPP(
    paramBlock: *mut c_void,
    userUPP: Option<extern "C" fn(*mut c_void) -> ()>,
  ) -> ();
  pub fn FSMakeFSRefUnicode(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    textEncodingHint: u32,
    newRef: *mut FSRef,
  ) -> i16;
  pub fn PBMakeFSRefUnicodeSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBMakeFSRefUnicodeAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSCompareFSRefs(ref1: *mut FSRef, ref2: *mut FSRef) -> i16;
  pub fn PBCompareFSRefsSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBCompareFSRefsAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSCreateFileUnicode(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    newRef: *mut FSRef,
    newSpec: *mut FSSpec,
  ) -> i16;
  pub fn PBCreateFileUnicodeSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBCreateFileUnicodeAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSCreateDirectoryUnicode(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    newRef: *mut FSRef,
    newSpec: *mut FSSpec,
    newDirID: *mut u32,
  ) -> i16;
  pub fn PBCreateDirectoryUnicodeSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBCreateDirectoryUnicodeAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSDeleteObject(ref_: *mut FSRef) -> i16;
  pub fn PBDeleteObjectSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBDeleteObjectAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSUnlinkObject(ref_: *mut FSRef) -> i16;
  pub fn PBUnlinkObjectSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBUnlinkObjectAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSMoveObject(ref_: *mut FSRef, destDirectory: *mut FSRef, newRef: *mut FSRef) -> i16;
  pub fn PBMoveObjectSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBMoveObjectAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSExchangeObjects(ref_: *mut FSRef, destRef: *mut FSRef) -> i16;
  pub fn PBExchangeObjectsSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBExchangeObjectsAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSReplaceObject(
    originalObject: *mut FSRef,
    replacementObject: *mut FSRef,
    newName: *mut __CFString,
    temporaryName: *mut __CFString,
    temporaryDirectory: *mut FSRef,
    flags: u32,
    resultObject: *mut FSRef,
  ) -> i32;
  pub fn FSPathReplaceObject(
    originalObjectPath: *mut i8,
    replacementObjectPath: *mut i8,
    newName: *mut __CFString,
    temporaryName: *mut __CFString,
    temporaryDirectoryPath: *mut i8,
    flags: u32,
  ) -> i32;
  pub fn FSGetTemporaryDirectoryForReplaceObject(
    originalObject: *mut FSRef,
    temporaryDirectory: *mut FSRef,
    flags: u32,
  ) -> i32;
  pub fn FSPathGetTemporaryDirectoryForReplaceObject(
    originalObjectPath: *mut i8,
    temporaryDirectoryPath: *mut i8,
    maxPathSize: u32,
    flags: u32,
  ) -> i32;
  pub fn FSRenameUnicode(
    ref_: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    textEncodingHint: u32,
    newRef: *mut FSRef,
  ) -> i16;
  pub fn PBRenameUnicodeSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBRenameUnicodeAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSGetCatalogInfo(
    ref_: *mut FSRef,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    outName: *mut HFSUniStr255,
    fsSpec: *mut FSSpec,
    parentRef: *mut FSRef,
  ) -> i16;
  pub fn PBGetCatalogInfoSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBGetCatalogInfoAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSSetCatalogInfo(ref_: *mut FSRef, whichInfo: u32, catalogInfo: *mut FSCatalogInfo)
    -> i16;
  pub fn PBSetCatalogInfoSync(paramBlock: *mut FSRefParam) -> i16;
  pub fn PBSetCatalogInfoAsync(paramBlock: *mut FSRefParam) -> ();
  pub fn FSOpenIterator(
    container: *mut FSRef,
    iteratorFlags: u32,
    iterator: *mut *mut OpaqueFSIterator,
  ) -> i16;
  pub fn PBOpenIteratorSync(paramBlock: *mut FSCatalogBulkParam) -> i16;
  pub fn PBOpenIteratorAsync(paramBlock: *mut FSCatalogBulkParam) -> ();
  pub fn FSCloseIterator(iterator: *mut OpaqueFSIterator) -> i16;
  pub fn PBCloseIteratorSync(paramBlock: *mut FSCatalogBulkParam) -> i16;
  pub fn PBCloseIteratorAsync(paramBlock: *mut FSCatalogBulkParam) -> ();
  pub fn FSGetCatalogInfoBulk(
    iterator: *mut OpaqueFSIterator,
    maximumObjects: usize,
    actualObjects: *mut usize,
    containerChanged: *mut u8,
    whichInfo: u32,
    catalogInfos: *mut FSCatalogInfo,
    refs: *mut FSRef,
    specs: *mut FSSpec,
    names: *mut HFSUniStr255,
  ) -> i16;
  pub fn PBGetCatalogInfoBulkSync(paramBlock: *mut FSCatalogBulkParam) -> i16;
  pub fn PBGetCatalogInfoBulkAsync(paramBlock: *mut FSCatalogBulkParam) -> ();
  pub fn FSCatalogSearch(
    iterator: *mut OpaqueFSIterator,
    searchCriteria: *mut FSSearchParams,
    maximumObjects: usize,
    actualObjects: *mut usize,
    containerChanged: *mut u8,
    whichInfo: u32,
    catalogInfos: *mut FSCatalogInfo,
    refs: *mut FSRef,
    specs: *mut FSSpec,
    names: *mut HFSUniStr255,
  ) -> i16;
  pub fn PBCatalogSearchSync(paramBlock: *mut FSCatalogBulkParam) -> i16;
  pub fn PBCatalogSearchAsync(paramBlock: *mut FSCatalogBulkParam) -> ();
  pub fn FSCreateFileAndOpenForkUnicode(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    forkNameLength: usize,
    forkName: *mut u16,
    permissions: i8,
    forkRefNum: *mut i32,
    newRef: *mut FSRef,
  ) -> i32;
  pub fn PBCreateFileAndOpenForkUnicodeSync(paramBlock: *mut FSRefForkIOParam) -> i32;
  pub fn PBCreateFileAndOpenForkUnicodeAsync(paramBlock: *mut FSRefForkIOParam) -> ();
  pub fn FSCreateFork(ref_: *mut FSRef, forkNameLength: usize, forkName: *mut u16) -> i16;
  pub fn PBCreateForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBCreateForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSDeleteFork(ref_: *mut FSRef, forkNameLength: usize, forkName: *mut u16) -> i16;
  pub fn PBDeleteForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBDeleteForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSIterateForks(
    ref_: *mut FSRef,
    forkIterator: *mut CatPositionRec,
    forkName: *mut HFSUniStr255,
    forkSize: *mut i64,
    forkPhysicalSize: *mut u64,
  ) -> i16;
  pub fn PBIterateForksSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBIterateForksAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSOpenFork(
    ref_: *mut FSRef,
    forkNameLength: usize,
    forkName: *mut u16,
    permissions: i8,
    forkRefNum: *mut i32,
  ) -> i16;
  pub fn PBOpenForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBOpenForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSReadFork(
    forkRefNum: i32,
    positionMode: u16,
    positionOffset: i64,
    requestCount: usize,
    buffer: *mut c_void,
    actualCount: *mut usize,
  ) -> i16;
  pub fn PBReadForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBReadForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSWriteFork(
    forkRefNum: i32,
    positionMode: u16,
    positionOffset: i64,
    requestCount: usize,
    buffer: *mut c_void,
    actualCount: *mut usize,
  ) -> i16;
  pub fn PBWriteForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBWriteForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSGetForkPosition(forkRefNum: i32, position: *mut i64) -> i16;
  pub fn PBGetForkPositionSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBGetForkPositionAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSSetForkPosition(forkRefNum: i32, positionMode: u16, positionOffset: i64) -> i16;
  pub fn PBSetForkPositionSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBSetForkPositionAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSGetForkSize(forkRefNum: i32, forkSize: *mut i64) -> i16;
  pub fn PBGetForkSizeSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBGetForkSizeAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSSetForkSize(forkRefNum: i32, positionMode: u16, positionOffset: i64) -> i16;
  pub fn PBSetForkSizeSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBSetForkSizeAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSAllocateFork(
    forkRefNum: i32,
    flags: u16,
    positionMode: u16,
    positionOffset: i64,
    requestCount: u64,
    actualCount: *mut u64,
  ) -> i16;
  pub fn PBAllocateForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBAllocateForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSFlushFork(forkRefNum: i32) -> i16;
  pub fn PBFlushForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBFlushForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSCloseFork(forkRefNum: i32) -> i16;
  pub fn PBCloseForkSync(paramBlock: *mut FSForkIOParam) -> i16;
  pub fn PBCloseForkAsync(paramBlock: *mut FSForkIOParam) -> ();
  pub fn FSGetForkCBInfo(
    desiredRefNum: i32,
    volume: i16,
    iterator: *mut i16,
    actualRefNum: *mut i32,
    forkInfo: *mut FSForkInfo,
    ref_: *mut FSRef,
    outForkName: *mut HFSUniStr255,
  ) -> i16;
  pub fn PBGetForkCBInfoSync(paramBlock: *mut FSForkCBInfoParam) -> i16;
  pub fn PBGetForkCBInfoAsync(paramBlock: *mut FSForkCBInfoParam) -> ();
  pub fn FSLockRange(
    forkRefNum: i32,
    positionMode: u16,
    positionOffset: i64,
    requestCount: u64,
    rangeStart: *mut u64,
  ) -> i32;
  pub fn PBXLockRangeSync(paramBlock: *mut FSRangeLockParam) -> i32;
  pub fn PBXLockRangeAsync(paramBlock: *mut FSRangeLockParam) -> i32;
  pub fn FSUnlockRange(
    forkRefNum: i32,
    positionMode: u16,
    positionOffset: i64,
    requestCount: u64,
    rangeStart: *mut u64,
  ) -> i32;
  pub fn PBXUnlockRangeSync(paramBlock: *mut FSRangeLockParam) -> i32;
  pub fn PBXUnlockRangeAsync(paramBlock: *mut FSRangeLockParam) -> i32;
  pub fn FSGetVolumeInfo(
    volume: i16,
    volumeIndex: usize,
    actualVolume: *mut i16,
    whichInfo: u32,
    info: *mut FSVolumeInfo,
    volumeName: *mut HFSUniStr255,
    rootDirectory: *mut FSRef,
  ) -> i16;
  pub fn PBGetVolumeInfoSync(paramBlock: *mut FSVolumeInfoParam) -> i16;
  pub fn PBGetVolumeInfoAsync(paramBlock: *mut FSVolumeInfoParam) -> ();
  pub fn FSSetVolumeInfo(volume: i16, whichInfo: u32, info: *mut FSVolumeInfo) -> i16;
  pub fn PBSetVolumeInfoSync(paramBlock: *mut FSVolumeInfoParam) -> i16;
  pub fn PBSetVolumeInfoAsync(paramBlock: *mut FSVolumeInfoParam) -> ();
  pub fn FSGetDataForkName(dataForkName: *mut HFSUniStr255) -> i16;
  pub fn FSGetResourceForkName(resourceForkName: *mut HFSUniStr255) -> i16;
  pub fn FSRefMakePath(ref_: *mut FSRef, path: *mut u8, pathBufferSize: u32) -> i32;
  pub fn FSPathMakeRef(path: *mut u8, ref_: *mut FSRef, isDirectory: *mut u8) -> i32;
  pub fn FSPathMakeRefWithOptions(
    path: *mut u8,
    options: u32,
    ref_: *mut FSRef,
    isDirectory: *mut u8,
  ) -> i32;
  pub fn FSIsFSRefValid(ref_: *mut FSRef) -> u8;
  pub fn FNNotify(ref_: *mut FSRef, message: u32, flags: u32) -> i32;
  pub fn FNNotifyByPath(path: *mut u8, message: u32, flags: u32) -> i32;
  pub fn FNNotifyAll(message: u32, flags: u32) -> i32;
  pub fn NewFNSubscriptionUPP(
    userRoutine: Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>,
  ) -> Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>;
  pub fn DisposeFNSubscriptionUPP(
    userUPP: Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>,
  ) -> ();
  pub fn InvokeFNSubscriptionUPP(
    message: u32,
    flags: u32,
    refcon: *mut c_void,
    subscription: *mut OpaqueFNSubscriptionRef,
    userUPP: Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>,
  ) -> ();
  pub fn FNSubscribe(
    directoryRef: *mut FSRef,
    callback: Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>,
    refcon: *mut c_void,
    flags: u32,
    subscription: *mut *mut OpaqueFNSubscriptionRef,
  ) -> i32;
  pub fn FNSubscribeByPath(
    directoryPath: *mut u8,
    callback: Option<extern "C" fn(u32, u32, *mut c_void, *mut OpaqueFNSubscriptionRef) -> ()>,
    refcon: *mut c_void,
    flags: u32,
    subscription: *mut *mut OpaqueFNSubscriptionRef,
  ) -> i32;
  pub fn FNUnsubscribe(subscription: *mut OpaqueFNSubscriptionRef) -> i32;
  pub fn FNGetDirectoryForSubscription(
    subscription: *mut OpaqueFNSubscriptionRef,
    ref_: *mut FSRef,
  ) -> i32;
  pub fn NewFSVolumeMountUPP(
    userRoutine: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>,
  ) -> Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>;
  pub fn NewFSVolumeUnmountUPP(
    userRoutine: Option<
      extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> (),
    >,
  ) -> Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>;
  pub fn NewFSVolumeEjectUPP(
    userRoutine: Option<
      extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> (),
    >,
  ) -> Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>;
  pub fn DisposeFSVolumeMountUPP(
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>,
  ) -> ();
  pub fn DisposeFSVolumeUnmountUPP(
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
  ) -> ();
  pub fn DisposeFSVolumeEjectUPP(
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
  ) -> ();
  pub fn InvokeFSVolumeMountUPP(
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    err: i32,
    mountedVolumeRefNum: i16,
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>,
  ) -> ();
  pub fn InvokeFSVolumeUnmountUPP(
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    err: i32,
    volumeRefNum: i16,
    dissenter: i32,
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
  ) -> ();
  pub fn InvokeFSVolumeEjectUPP(
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    err: i32,
    volumeRefNum: i16,
    dissenter: i32,
    userUPP: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
  ) -> ();
  pub fn FSCreateVolumeOperation(volumeOp: *mut *mut OpaqueFSVolumeOperation) -> i32;
  pub fn FSDisposeVolumeOperation(volumeOp: *mut OpaqueFSVolumeOperation) -> i32;
  pub fn FSMountLocalVolumeSync(
    diskID: *mut __CFString,
    mountDir: *mut __CFURL,
    mountedVolumeRefNum: *mut i16,
    flags: u32,
  ) -> i32;
  pub fn FSMountLocalVolumeAsync(
    diskID: *mut __CFString,
    mountDir: *mut __CFURL,
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    flags: u32,
    callback: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>,
    runloop: *mut __CFRunLoop,
    runloopMode: *mut __CFString,
  ) -> i32;
  pub fn FSMountServerVolumeSync(
    url: *mut __CFURL,
    mountDir: *mut __CFURL,
    user: *mut __CFString,
    password: *mut __CFString,
    mountedVolumeRefNum: *mut i16,
    flags: u32,
  ) -> i32;
  pub fn FSMountServerVolumeAsync(
    url: *mut __CFURL,
    mountDir: *mut __CFURL,
    user: *mut __CFString,
    password: *mut __CFString,
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    flags: u32,
    callback: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16) -> ()>,
    runloop: *mut __CFRunLoop,
    runloopMode: *mut __CFString,
  ) -> i32;
  pub fn FSGetAsyncMountStatus(
    volumeOp: *mut OpaqueFSVolumeOperation,
    status: *mut u32,
    volumeOpStatus: *mut i32,
    mountedVolumeRefNum: *mut i16,
    clientData: *mut *mut c_void,
  ) -> i32;
  pub fn FSUnmountVolumeSync(vRefNum: i16, flags: u32, dissenter: *mut i32) -> i32;
  pub fn FSUnmountVolumeAsync(
    vRefNum: i16,
    flags: u32,
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    callback: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
    runloop: *mut __CFRunLoop,
    runloopMode: *mut __CFString,
  ) -> i32;
  pub fn FSGetAsyncUnmountStatus(
    volumeOp: *mut OpaqueFSVolumeOperation,
    status: *mut u32,
    volumeOpStatus: *mut i32,
    volumeRefNum: *mut i16,
    dissenter: *mut i32,
    clientData: *mut *mut c_void,
  ) -> i32;
  pub fn FSCancelVolumeOperation(volumeOp: *mut OpaqueFSVolumeOperation) -> i32;
  pub fn FSEjectVolumeSync(vRefNum: i16, flags: u32, dissenter: *mut i32) -> i32;
  pub fn FSEjectVolumeAsync(
    vRefNum: i16,
    flags: u32,
    volumeOp: *mut OpaqueFSVolumeOperation,
    clientData: *mut c_void,
    callback: Option<extern "C" fn(*mut OpaqueFSVolumeOperation, *mut c_void, i32, i16, i32) -> ()>,
    runloop: *mut __CFRunLoop,
    runloopMode: *mut __CFString,
  ) -> i32;
  pub fn FSGetAsyncEjectStatus(
    volumeOp: *mut OpaqueFSVolumeOperation,
    status: *mut u32,
    volumeOpStatus: *mut i32,
    volumeRefNum: *mut i16,
    dissenter: *mut i32,
    clientData: *mut *mut c_void,
  ) -> i32;
  pub fn FSCopyDiskIDForVolume(vRefNum: i16, diskID: *mut *mut __CFString) -> i32;
  pub fn FSCopyURLForVolume(vRefNum: i16, url: *mut *mut __CFURL) -> i32;
  pub fn FSGetVolumeForDiskID(diskID: *mut __CFString, vRefNum: *mut i16) -> i32;
  pub fn FSCopyDADiskForVolume(vRefNum: i16, disk: *mut *mut __DADisk) -> i32;
  pub fn FSGetVolumeForDADisk(disk: *mut __DADisk, vRefNum: *mut i16) -> i32;
  pub fn FSCopyObjectSync(
    source: *mut FSRef,
    destDir: *mut FSRef,
    destName: *mut __CFString,
    target: *mut FSRef,
    options: u32,
  ) -> i32;
  pub fn FSMoveObjectSync(
    source: *mut FSRef,
    destDir: *mut FSRef,
    destName: *mut __CFString,
    target: *mut FSRef,
    options: u32,
  ) -> i32;
  pub fn FSMoveObjectToTrashSync(source: *mut FSRef, target: *mut FSRef, options: u32) -> i32;
  pub fn FSPathCopyObjectSync(
    sourcePath: *mut i8,
    destDirPath: *mut i8,
    destName: *mut __CFString,
    targetPath: *mut *mut i8,
    options: u32,
  ) -> i32;
  pub fn FSPathMoveObjectSync(
    sourcePath: *mut i8,
    destDirPath: *mut i8,
    destName: *mut __CFString,
    targetPath: *mut *mut i8,
    options: u32,
  ) -> i32;
  pub fn FSPathMoveObjectToTrashSync(
    sourcePath: *mut i8,
    targetPath: *mut *mut i8,
    options: u32,
  ) -> i32;
  pub fn FSFileOperationGetTypeID() -> usize;
  pub fn FSFileOperationCreate(alloc: *mut __CFAllocator) -> *mut __FSFileOperation;
  pub fn FSFileOperationScheduleWithRunLoop(
    fileOp: *mut __FSFileOperation,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> i32;
  pub fn FSFileOperationUnscheduleFromRunLoop(
    fileOp: *mut __FSFileOperation,
    runLoop: *mut __CFRunLoop,
    runLoopMode: *mut __CFString,
  ) -> i32;
  pub fn FSCopyObjectAsync(
    fileOp: *mut __FSFileOperation,
    source: *mut FSRef,
    destDir: *mut FSRef,
    destName: *mut __CFString,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut FSRef,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSMoveObjectAsync(
    fileOp: *mut __FSFileOperation,
    source: *mut FSRef,
    destDir: *mut FSRef,
    destName: *mut __CFString,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut FSRef,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSMoveObjectToTrashAsync(
    fileOp: *mut __FSFileOperation,
    source: *mut FSRef,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut FSRef,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSPathCopyObjectAsync(
    fileOp: *mut __FSFileOperation,
    sourcePath: *mut i8,
    destDirPath: *mut i8,
    destName: *mut __CFString,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut i8,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSPathMoveObjectAsync(
    fileOp: *mut __FSFileOperation,
    sourcePath: *mut i8,
    destDirPath: *mut i8,
    destName: *mut __CFString,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut i8,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSPathMoveObjectToTrashAsync(
    fileOp: *mut __FSFileOperation,
    sourcePath: *mut i8,
    flags: u32,
    callback: Option<
      extern "C" fn(
        *mut __FSFileOperation,
        *mut i8,
        u32,
        i32,
        *mut __CFDictionary,
        *mut c_void,
      ) -> (),
    >,
    statusChangeInterval: f64,
    clientContext: *mut FSFileOperationClientContext,
  ) -> i32;
  pub fn FSFileOperationCancel(fileOp: *mut __FSFileOperation) -> i32;
  pub fn FSFileOperationCopyStatus(
    fileOp: *mut __FSFileOperation,
    currentItem: *mut FSRef,
    stage: *mut u32,
    error: *mut i32,
    statusDictionary: *mut *mut __CFDictionary,
    info: *mut *mut c_void,
  ) -> i32;
  pub fn FSPathFileOperationCopyStatus(
    fileOp: *mut __FSFileOperation,
    currentItem: *mut *mut i8,
    stage: *mut u32,
    error: *mut i32,
    statusDictionary: *mut *mut __CFDictionary,
    info: *mut *mut c_void,
  ) -> i32;
  pub fn FSCreateStringFromHFSUniStr(
    alloc: *mut __CFAllocator,
    uniStr: *mut HFSUniStr255,
  ) -> *mut __CFString;
  pub fn FSGetHFSUniStrFromString(theString: *mut __CFString, uniStr: *mut HFSUniStr255) -> i32;
  pub fn FSFileSecurityGetTypeID() -> usize;
  pub fn FSFileSecurityCreate(alloc: *mut __CFAllocator) -> *mut __FSFileSecurity;
  pub fn FSFileSecurityCreateWithFSPermissionInfo(
    alloc: *mut __CFAllocator,
    permissions: *mut FSPermissionInfo,
  ) -> *mut __FSFileSecurity;
  pub fn FSFileSecurityRefCreateCopy(
    alloc: *mut __CFAllocator,
    fileSec: *mut __FSFileSecurity,
  ) -> *mut __FSFileSecurity;
  pub fn FSFileSecurityGetOwnerUUID(fileSec: *mut __FSFileSecurity, owner: *mut CFUUIDBytes)
    -> i32;
  pub fn FSFileSecuritySetOwnerUUID(fileSec: *mut __FSFileSecurity, owner: *mut CFUUIDBytes)
    -> i32;
  pub fn FSFileSecurityGetGroupUUID(fileSec: *mut __FSFileSecurity, group: *mut CFUUIDBytes)
    -> i32;
  pub fn FSFileSecuritySetGroupUUID(fileSec: *mut __FSFileSecurity, group: *mut CFUUIDBytes)
    -> i32;
  pub fn FSFileSecurityCopyAccessControlList(
    fileSec: *mut __FSFileSecurity,
    accessControlList: *mut *mut _acl,
  ) -> i32;
  pub fn FSFileSecuritySetAccessControlList(
    fileSec: *mut __FSFileSecurity,
    accessControlList: *mut _acl,
  ) -> i32;
  pub fn FSFileSecurityGetOwner(fileSec: *mut __FSFileSecurity, owner: *mut u32) -> i32;
  pub fn FSFileSecuritySetOwner(fileSec: *mut __FSFileSecurity, owner: u32) -> i32;
  pub fn FSFileSecurityGetGroup(fileSec: *mut __FSFileSecurity, group: *mut u32) -> i32;
  pub fn FSFileSecuritySetGroup(fileSec: *mut __FSFileSecurity, group: u32) -> i32;
  pub fn FSFileSecurityGetMode(fileSec: *mut __FSFileSecurity, mode: *mut u16) -> i32;
  pub fn FSFileSecuritySetMode(fileSec: *mut __FSFileSecurity, mode: u16) -> i32;
  pub fn FSGetVolumeParms(
    volume: i16,
    buffer: *mut GetVolParmsInfoBuffer,
    bufferSize: usize,
  ) -> i32;
  pub fn FSGetVolumeMountInfoSize(volume: i16, size: *mut usize) -> i32;
  pub fn FSGetVolumeMountInfo(
    volume: i16,
    buffer: *mut u8,
    bufferSize: usize,
    actualSize: *mut usize,
  ) -> i32;
  pub fn FSVolumeMount(buffer: *mut u8, mountedVolume: *mut i16) -> i32;
  pub fn FSFlushVolume(vRefNum: i16) -> i32;
  pub fn PBFlushVolumeSync(paramBlock: *mut FSRefParam) -> i32;
  pub fn PBFlushVolumeAsync(paramBlock: *mut FSRefParam) -> i32;
  pub fn PBFSCopyFileSync(paramBlock: *mut FSRefParam) -> i32;
  pub fn PBFSCopyFileAsync(paramBlock: *mut FSRefParam) -> i32;
  pub fn FSResolveNodeID(volume: i16, nodeID: u32, newRef: *mut FSRef) -> i32;
  pub fn PBFSResolveNodeIDSync(paramBlock: *mut FSRefParam) -> i32;
  pub fn PBFSResolveNodeIDAsync(paramBlock: *mut FSRefParam) -> i32;
  pub fn NewResErrUPP(
    userRoutine: Option<extern "C" fn(i16) -> ()>,
  ) -> Option<extern "C" fn(i16) -> ()>;
  pub fn DisposeResErrUPP(userUPP: Option<extern "C" fn(i16) -> ()>) -> ();
  pub fn InvokeResErrUPP(thErr: i16, userUPP: Option<extern "C" fn(i16) -> ()>) -> ();
  pub fn CloseResFile(refNum: i32) -> ();
  pub fn ResError() -> i16;
  pub fn CurResFile() -> i32;
  pub fn HomeResFile(theResource: *mut *mut i8) -> i32;
  pub fn UseResFile(refNum: i32) -> ();
  pub fn CountTypes() -> i16;
  pub fn Count1Types() -> i16;
  pub fn GetIndType(theType: *mut u32, itemIndex: i16) -> ();
  pub fn Get1IndType(theType: *mut u32, itemIndex: i16) -> ();
  pub fn SetResLoad(load: u8) -> ();
  pub fn CountResources(theType: u32) -> i16;
  pub fn Count1Resources(theType: u32) -> i16;
  pub fn GetIndResource(theType: u32, itemIndex: i16) -> *mut *mut i8;
  pub fn Get1IndResource(theType: u32, itemIndex: i16) -> *mut *mut i8;
  pub fn GetResource(theType: u32, theID: i16) -> *mut *mut i8;
  pub fn Get1Resource(theType: u32, theID: i16) -> *mut *mut i8;
  pub fn GetNamedResource(theType: u32, name: *mut u8) -> *mut *mut i8;
  pub fn Get1NamedResource(theType: u32, name: *mut u8) -> *mut *mut i8;
  pub fn LoadResource(theResource: *mut *mut i8) -> ();
  pub fn ReleaseResource(theResource: *mut *mut i8) -> ();
  pub fn DetachResource(theResource: *mut *mut i8) -> ();
  pub fn UniqueID(theType: u32) -> i16;
  pub fn Unique1ID(theType: u32) -> i16;
  pub fn GetResAttrs(theResource: *mut *mut i8) -> i16;
  pub fn GetResInfo(
    theResource: *mut *mut i8,
    theID: *mut i16,
    theType: *mut u32,
    name: [u8; 256],
  ) -> ();
  pub fn SetResInfo(theResource: *mut *mut i8, theID: i16, name: *mut u8) -> ();
  pub fn AddResource(theData: *mut *mut i8, theType: u32, theID: i16, name: *mut u8) -> ();
  pub fn GetResourceSizeOnDisk(theResource: *mut *mut i8) -> isize;
  pub fn GetMaxResourceSize(theResource: *mut *mut i8) -> isize;
  pub fn SetResAttrs(theResource: *mut *mut i8, attrs: i16) -> ();
  pub fn ChangedResource(theResource: *mut *mut i8) -> ();
  pub fn RemoveResource(theResource: *mut *mut i8) -> ();
  pub fn UpdateResFile(refNum: i32) -> ();
  pub fn WriteResource(theResource: *mut *mut i8) -> ();
  pub fn SetResPurge(install: u8) -> ();
  pub fn GetResFileAttrs(refNum: i32) -> i16;
  pub fn SetResFileAttrs(refNum: i32, attrs: i16) -> ();
  pub fn ReadPartialResource(
    theResource: *mut *mut i8,
    offset: isize,
    buffer: *mut c_void,
    count: isize,
  ) -> ();
  pub fn WritePartialResource(
    theResource: *mut *mut i8,
    offset: isize,
    buffer: *mut c_void,
    count: isize,
  ) -> ();
  pub fn SetResourceSize(theResource: *mut *mut i8, newSize: isize) -> ();
  pub fn GetNextFOND(fondHandle: *mut *mut i8) -> *mut *mut i8;
  pub fn InsertResourceFile(refNum: i32, where_: i16) -> i16;
  pub fn DetachResourceFile(refNum: i32) -> i16;
  pub fn GetTopResourceFile(refNum: *mut i32) -> i16;
  pub fn GetNextResourceFile(curRefNum: i32, nextRefNum: *mut i32) -> i16;
  pub fn FSOpenResFile(ref_: *mut FSRef, permission: i8) -> i32;
  pub fn FSCreateResFile(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    newRef: *mut FSRef,
    newSpec: *mut FSSpec,
  ) -> ();
  pub fn FSResourceFileAlreadyOpen(
    resourceFileRef: *mut FSRef,
    inChain: *mut u8,
    refNum: *mut i32,
  ) -> u8;
  pub fn FSOpenOrphanResFile(ref_: *mut FSRef, permission: i8, refNum: *mut i32) -> i16;
  pub fn FSCreateResourceFile(
    parentRef: *mut FSRef,
    nameLength: usize,
    name: *mut u16,
    whichInfo: u32,
    catalogInfo: *mut FSCatalogInfo,
    forkNameLength: usize,
    forkName: *mut u16,
    newRef: *mut FSRef,
    newSpec: *mut FSSpec,
  ) -> i16;
  pub fn FSCreateResourceFork(
    ref_: *mut FSRef,
    forkNameLength: usize,
    forkName: *mut u16,
    flags: u32,
  ) -> i16;
  pub fn FSOpenResourceFile(
    ref_: *mut FSRef,
    forkNameLength: usize,
    forkName: *mut u16,
    permissions: i8,
    refNum: *mut i32,
  ) -> i16;
  pub fn CSSetComponentsThreadMode(mode: u32) -> ();
  pub fn CSGetComponentsThreadMode() -> u32;
  pub fn NewComponentFunctionUPP(
    userRoutine: Option<extern "C" fn() -> isize>,
    procInfo: usize,
  ) -> Option<extern "C" fn() -> isize>;
  pub fn DisposeComponentFunctionUPP(userUPP: Option<extern "C" fn() -> isize>) -> ();
  pub fn RegisterComponent(
    cd: *mut ComponentDescription,
    componentEntryPoint: Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>,
    global: i16,
    componentName: *mut *mut i8,
    componentInfo: *mut *mut i8,
    componentIcon: *mut *mut i8,
  ) -> *mut ComponentRecord;
  pub fn RegisterComponentResource(
    cr: *mut *mut ComponentResource,
    global: i16,
  ) -> *mut ComponentRecord;
  pub fn UnregisterComponent(aComponent: *mut ComponentRecord) -> i16;
  pub fn FindNextComponent(
    aComponent: *mut ComponentRecord,
    looking: *mut ComponentDescription,
  ) -> *mut ComponentRecord;
  pub fn CountComponents(looking: *mut ComponentDescription) -> isize;
  pub fn GetComponentInfo(
    aComponent: *mut ComponentRecord,
    cd: *mut ComponentDescription,
    componentName: *mut *mut i8,
    componentInfo: *mut *mut i8,
    componentIcon: *mut *mut i8,
  ) -> i16;
  pub fn GetComponentListModSeed() -> i32;
  pub fn GetComponentTypeModSeed(componentType: u32) -> i32;
  pub fn OpenAComponent(
    aComponent: *mut ComponentRecord,
    ci: *mut *mut ComponentInstanceRecord,
  ) -> i16;
  pub fn OpenComponent(aComponent: *mut ComponentRecord) -> *mut ComponentInstanceRecord;
  pub fn CloseComponent(aComponentInstance: *mut ComponentInstanceRecord) -> i16;
  pub fn GetComponentInstanceError(aComponentInstance: *mut ComponentInstanceRecord) -> i16;
  pub fn ResolveComponentAlias(aComponent: *mut ComponentRecord) -> *mut ComponentRecord;
  pub fn GetComponentPublicResource(
    aComponent: *mut ComponentRecord,
    resourceType: u32,
    resourceID: i16,
    theResource: *mut *mut *mut i8,
  ) -> i16;
  pub fn GetComponentPublicResourceList(
    resourceType: u32,
    resourceID: i16,
    flags: i32,
    cd: *mut ComponentDescription,
    missingProc: Option<
      extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16,
    >,
    refCon: *mut c_void,
    atomContainerPtr: *mut c_void,
  ) -> i16;
  pub fn GetComponentPublicIndString(
    aComponent: *mut ComponentRecord,
    theString: [u8; 256],
    strListID: i16,
    index: i16,
  ) -> i16;
  pub fn SetComponentInstanceError(
    aComponentInstance: *mut ComponentInstanceRecord,
    theError: i16,
  ) -> ();
  pub fn GetComponentRefcon(aComponent: *mut ComponentRecord) -> isize;
  pub fn SetComponentRefcon(aComponent: *mut ComponentRecord, theRefcon: isize) -> ();
  pub fn OpenComponentResFile(aComponent: *mut ComponentRecord) -> i32;
  pub fn OpenAComponentResFile(aComponent: *mut ComponentRecord, resRef: *mut i32) -> i16;
  pub fn CloseComponentResFile(refnum: i32) -> i16;
  pub fn GetComponentResource(
    aComponent: *mut ComponentRecord,
    resType: u32,
    resID: i16,
    theResource: *mut *mut *mut i8,
  ) -> i16;
  pub fn GetComponentIndString(
    aComponent: *mut ComponentRecord,
    theString: [u8; 256],
    strListID: i16,
    index: i16,
  ) -> i16;
  pub fn GetComponentInstanceStorage(
    aComponentInstance: *mut ComponentInstanceRecord,
  ) -> *mut *mut i8;
  pub fn SetComponentInstanceStorage(
    aComponentInstance: *mut ComponentInstanceRecord,
    theStorage: *mut *mut i8,
  ) -> ();
  pub fn CountComponentInstances(aComponent: *mut ComponentRecord) -> isize;
  pub fn CallComponentFunction(
    params: *mut ComponentParameters,
    func: Option<extern "C" fn() -> isize>,
  ) -> i32;
  pub fn CallComponentFunctionWithStorage(
    storage: *mut *mut i8,
    params: *mut ComponentParameters,
    func: Option<extern "C" fn() -> isize>,
  ) -> i32;
  pub fn CallComponentFunctionWithStorageProcInfo(
    storage: *mut *mut i8,
    params: *mut ComponentParameters,
    func: Option<extern "C" fn() -> isize>,
    funcProcInfo: usize,
  ) -> i32;
  pub fn DelegateComponentCall(
    originalParams: *mut ComponentParameters,
    ci: *mut ComponentInstanceRecord,
  ) -> i32;
  pub fn SetDefaultComponent(aComponent: *mut ComponentRecord, flags: i16) -> i16;
  pub fn OpenDefaultComponent(
    componentType: u32,
    componentSubType: u32,
  ) -> *mut ComponentInstanceRecord;
  pub fn OpenADefaultComponent(
    componentType: u32,
    componentSubType: u32,
    ci: *mut *mut ComponentInstanceRecord,
  ) -> i16;
  pub fn CaptureComponent(
    capturedComponent: *mut ComponentRecord,
    capturingComponent: *mut ComponentRecord,
  ) -> *mut ComponentRecord;
  pub fn UncaptureComponent(aComponent: *mut ComponentRecord) -> i16;
  pub fn RegisterComponentResourceFile(resRefNum: i16, global: i16) -> i32;
  pub fn RegisterComponentFileRef(ref_: *mut FSRef, global: i16) -> i16;
  pub fn RegisterComponentFileRefEntries(
    ref_: *mut FSRef,
    global: i16,
    toRegister: *mut ComponentDescription,
    registerCount: u32,
  ) -> i16;
  pub fn CallComponentOpen(
    ci: *mut ComponentInstanceRecord,
    self_: *mut ComponentInstanceRecord,
  ) -> i32;
  pub fn CallComponentClose(
    ci: *mut ComponentInstanceRecord,
    self_: *mut ComponentInstanceRecord,
  ) -> i32;
  pub fn CallComponentCanDo(ci: *mut ComponentInstanceRecord, ftnNumber: i16) -> i32;
  pub fn CallComponentVersion(ci: *mut ComponentInstanceRecord) -> i32;
  pub fn CallComponentRegister(ci: *mut ComponentInstanceRecord) -> i32;
  pub fn CallComponentTarget(
    ci: *mut ComponentInstanceRecord,
    target: *mut ComponentInstanceRecord,
  ) -> i32;
  pub fn CallComponentUnregister(ci: *mut ComponentInstanceRecord) -> i32;
  pub fn CallComponentGetMPWorkFunction(
    ci: *mut ComponentInstanceRecord,
    workFunction: *mut Option<
      extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32,
    >,
    refCon: *mut *mut c_void,
  ) -> i32;
  pub fn CallComponentGetPublicResource(
    ci: *mut ComponentInstanceRecord,
    resourceType: u32,
    resourceID: i16,
    resource: *mut *mut *mut i8,
  ) -> i32;
  pub fn CallComponentDispatch(cp: *mut ComponentParameters) -> i32;
  pub fn NewComponentMPWorkFunctionUPP(
    userRoutine: Option<
      extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32,
    >,
  ) -> Option<extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32>;
  pub fn NewComponentRoutineUPP(
    userRoutine: Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>,
  ) -> Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>;
  pub fn NewGetMissingComponentResourceUPP(
    userRoutine: Option<
      extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16,
    >,
  ) -> Option<extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16>;
  pub fn DisposeComponentMPWorkFunctionUPP(
    userUPP: Option<extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32>,
  ) -> ();
  pub fn DisposeComponentRoutineUPP(
    userUPP: Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>,
  ) -> ();
  pub fn DisposeGetMissingComponentResourceUPP(
    userUPP: Option<
      extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16,
    >,
  ) -> ();
  pub fn InvokeComponentMPWorkFunctionUPP(
    globalRefCon: *mut c_void,
    header: *mut ComponentMPWorkFunctionHeaderRecord,
    userUPP: Option<extern "C" fn(*mut c_void, *mut ComponentMPWorkFunctionHeaderRecord) -> i32>,
  ) -> i32;
  pub fn InvokeComponentRoutineUPP(
    cp: *mut ComponentParameters,
    componentStorage: *mut *mut i8,
    userUPP: Option<extern "C" fn(*mut ComponentParameters, *mut *mut i8) -> i32>,
  ) -> i32;
  pub fn InvokeGetMissingComponentResourceUPP(
    c: *mut ComponentRecord,
    resType: u32,
    resID: i16,
    refCon: *mut c_void,
    resource: *mut *mut *mut i8,
    userUPP: Option<
      extern "C" fn(*mut ComponentRecord, u32, i16, *mut c_void, *mut *mut *mut i8) -> i16,
    >,
  ) -> i16;
  pub fn MPProcessors() -> usize;
  pub fn MPProcessorsScheduled() -> usize;
  pub fn MPCreateTask(
    entryPoint: Option<extern "C" fn(*mut c_void) -> i32>,
    parameter: *mut c_void,
    stackSize: usize,
    notifyQueue: *mut OpaqueMPQueueID,
    terminationParameter1: *mut c_void,
    terminationParameter2: *mut c_void,
    options: u32,
    task: *mut *mut OpaqueMPTaskID,
  ) -> i32;
  pub fn MPTerminateTask(task: *mut OpaqueMPTaskID, terminationStatus: i32) -> i32;
  pub fn MPSetTaskWeight(task: *mut OpaqueMPTaskID, weight: u32) -> i32;
  pub fn MPTaskIsPreemptive(taskID: *mut OpaqueMPTaskID) -> u8;
  pub fn MPExit(status: i32) -> ();
  pub fn MPYield() -> ();
  pub fn MPCurrentTaskID() -> *mut OpaqueMPTaskID;
  pub fn MPSetTaskType(task: *mut OpaqueMPTaskID, taskType: u32) -> i32;
  pub fn MPAllocateTaskStorageIndex(taskIndex: *mut usize) -> i32;
  pub fn MPDeallocateTaskStorageIndex(taskIndex: usize) -> i32;
  pub fn MPSetTaskStorageValue(taskIndex: usize, value: *mut c_void) -> i32;
  pub fn MPGetTaskStorageValue(taskIndex: usize) -> *mut c_void;
  pub fn MPCreateQueue(queue: *mut *mut OpaqueMPQueueID) -> i32;
  pub fn MPDeleteQueue(queue: *mut OpaqueMPQueueID) -> i32;
  pub fn MPNotifyQueue(
    queue: *mut OpaqueMPQueueID,
    param1: *mut c_void,
    param2: *mut c_void,
    param3: *mut c_void,
  ) -> i32;
  pub fn MPWaitOnQueue(
    queue: *mut OpaqueMPQueueID,
    param1: *mut *mut c_void,
    param2: *mut *mut c_void,
    param3: *mut *mut c_void,
    timeout: i32,
  ) -> i32;
  pub fn MPSetQueueReserve(queue: *mut OpaqueMPQueueID, count: usize) -> i32;
  pub fn MPCreateSemaphore(
    maximumValue: usize,
    initialValue: usize,
    semaphore: *mut *mut OpaqueMPSemaphoreID,
  ) -> i32;
  pub fn MPDeleteSemaphore(semaphore: *mut OpaqueMPSemaphoreID) -> i32;
  pub fn MPSignalSemaphore(semaphore: *mut OpaqueMPSemaphoreID) -> i32;
  pub fn MPWaitOnSemaphore(semaphore: *mut OpaqueMPSemaphoreID, timeout: i32) -> i32;
  pub fn MPCreateCriticalRegion(criticalRegion: *mut *mut OpaqueMPCriticalRegionID) -> i32;
  pub fn MPDeleteCriticalRegion(criticalRegion: *mut OpaqueMPCriticalRegionID) -> i32;
  pub fn MPEnterCriticalRegion(criticalRegion: *mut OpaqueMPCriticalRegionID, timeout: i32) -> i32;
  pub fn MPExitCriticalRegion(criticalRegion: *mut OpaqueMPCriticalRegionID) -> i32;
  pub fn MPCreateEvent(event: *mut *mut OpaqueMPEventID) -> i32;
  pub fn MPDeleteEvent(event: *mut OpaqueMPEventID) -> i32;
  pub fn MPSetEvent(event: *mut OpaqueMPEventID, flags: u32) -> i32;
  pub fn MPWaitForEvent(event: *mut OpaqueMPEventID, flags: *mut u32, timeout: i32) -> i32;
  pub fn MPCreateNotification(notificationID: *mut *mut OpaqueMPNotificationID) -> i32;
  pub fn MPDeleteNotification(notificationID: *mut OpaqueMPNotificationID) -> i32;
  pub fn MPModifyNotification(
    notificationID: *mut OpaqueMPNotificationID,
    anID: *mut OpaqueMPOpaqueID,
    notifyParam1: *mut c_void,
    notifyParam2: *mut c_void,
    notifyParam3: *mut c_void,
  ) -> i32;
  pub fn MPModifyNotificationParameters(
    notificationID: *mut OpaqueMPNotificationID,
    kind: u32,
    notifyParam1: *mut c_void,
    notifyParam2: *mut c_void,
    notifyParam3: *mut c_void,
  ) -> i32;
  pub fn MPCauseNotification(notificationID: *mut OpaqueMPNotificationID) -> i32;
  pub fn MPDelayUntil(expirationTime: *mut UnsignedWide) -> i32;
  pub fn MPCreateTimer(timerID: *mut *mut OpaqueMPTimerID) -> i32;
  pub fn MPDeleteTimer(timerID: *mut OpaqueMPTimerID) -> i32;
  pub fn MPSetTimerNotify(
    timerID: *mut OpaqueMPTimerID,
    anID: *mut OpaqueMPOpaqueID,
    notifyParam1: *mut c_void,
    notifyParam2: *mut c_void,
    notifyParam3: *mut c_void,
  ) -> i32;
  pub fn MPArmTimer(
    timerID: *mut OpaqueMPTimerID,
    expirationTime: *mut UnsignedWide,
    options: u32,
  ) -> i32;
  pub fn MPCancelTimer(timerID: *mut OpaqueMPTimerID, timeRemaining: *mut UnsignedWide) -> i32;
  pub fn MPAllocateAligned(size: usize, alignment: u8, options: u32) -> *mut c_void;
  pub fn MPAllocate(size: usize) -> *mut c_void;
  pub fn MPFree(object: *mut c_void) -> ();
  pub fn MPGetAllocatedBlockSize(object: *mut c_void) -> usize;
  pub fn MPBlockCopy(source: *mut c_void, destination: *mut c_void, size: usize) -> ();
  pub fn MPBlockClear(address: *mut c_void, size: usize) -> ();
  pub fn MPSetExceptionHandler(task: *mut OpaqueMPTaskID, exceptionQ: *mut OpaqueMPQueueID) -> i32;
  pub fn MPDisposeTaskException(task: *mut OpaqueMPTaskID, action: u32) -> i32;
  pub fn MPExtractTaskState(task: *mut OpaqueMPTaskID, kind: u32, info: *mut c_void) -> i32;
  pub fn MPSetTaskState(task: *mut OpaqueMPTaskID, kind: u32, info: *mut c_void) -> i32;
  pub fn MPThrowException(task: *mut OpaqueMPTaskID, kind: u32) -> i32;
  pub fn MPRegisterDebugger(queue: *mut OpaqueMPQueueID, level: u32) -> i32;
  pub fn MPUnregisterDebugger(queue: *mut OpaqueMPQueueID) -> i32;
  pub fn MPRemoteCall(
    remoteProc: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    parameter: *mut c_void,
    context: u8,
  ) -> *mut c_void;
  pub fn MPRemoteCallCFM(
    remoteProc: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    parameter: *mut c_void,
    context: u8,
  ) -> *mut c_void;
  pub fn _MPIsFullyInitialized() -> u8;
  pub fn _MPLibraryVersion(
    versionCString: *mut *mut i8,
    major: *mut u32,
    minor: *mut u32,
    release: *mut u32,
    revision: *mut u32,
  ) -> ();
  pub fn _MPLibraryIsCompatible(
    versionCString: *mut i8,
    major: u32,
    minor: u32,
    release: u32,
    revision: u32,
  ) -> u8;
  pub fn FSNewAlias(
    fromFile: *mut FSRef,
    target: *mut FSRef,
    inAlias: *mut *mut *mut AliasRecord,
  ) -> i16;
  pub fn FSNewAliasMinimal(target: *mut FSRef, inAlias: *mut *mut *mut AliasRecord) -> i16;
  pub fn FSIsAliasFile(fileRef: *mut FSRef, aliasFileFlag: *mut u8, folderFlag: *mut u8) -> i16;
  pub fn FSResolveAliasWithMountFlags(
    fromFile: *mut FSRef,
    inAlias: *mut *mut AliasRecord,
    target: *mut FSRef,
    wasChanged: *mut u8,
    mountFlags: usize,
  ) -> i16;
  pub fn FSResolveAlias(
    fromFile: *mut FSRef,
    alias: *mut *mut AliasRecord,
    target: *mut FSRef,
    wasChanged: *mut u8,
  ) -> i16;
  pub fn FSResolveAliasFileWithMountFlags(
    theRef: *mut FSRef,
    resolveAliasChains: u8,
    targetIsFolder: *mut u8,
    wasAliased: *mut u8,
    mountFlags: usize,
  ) -> i16;
  pub fn FSResolveAliasFile(
    theRef: *mut FSRef,
    resolveAliasChains: u8,
    targetIsFolder: *mut u8,
    wasAliased: *mut u8,
  ) -> i16;
  pub fn FSFollowFinderAlias(
    fromFile: *mut FSRef,
    alias: *mut *mut AliasRecord,
    logon: u8,
    target: *mut FSRef,
    wasChanged: *mut u8,
  ) -> i16;
  pub fn FSUpdateAlias(
    fromFile: *mut FSRef,
    target: *mut FSRef,
    alias: *mut *mut AliasRecord,
    wasChanged: *mut u8,
  ) -> i16;
  pub fn FSNewAliasUnicode(
    fromFile: *mut FSRef,
    targetParentRef: *mut FSRef,
    targetNameLength: usize,
    targetName: *mut u16,
    inAlias: *mut *mut *mut AliasRecord,
    isDirectory: *mut u8,
  ) -> i16;
  pub fn FSNewAliasMinimalUnicode(
    targetParentRef: *mut FSRef,
    targetNameLength: usize,
    targetName: *mut u16,
    inAlias: *mut *mut *mut AliasRecord,
    isDirectory: *mut u8,
  ) -> i16;
  pub fn FSNewAliasFromPath(
    fromFilePath: *mut i8,
    targetPath: *mut i8,
    flags: u32,
    inAlias: *mut *mut *mut AliasRecord,
    isDirectory: *mut u8,
  ) -> i32;
  pub fn FSMatchAliasBulk(
    fromFile: *mut FSRef,
    rulesMask: usize,
    inAlias: *mut *mut AliasRecord,
    aliasCount: *mut i16,
    aliasList: *mut FSRef,
    needsUpdate: *mut u8,
    aliasFilter: Option<extern "C" fn(*mut FSRef, *mut u8, *mut i8) -> u8>,
    yourDataPtr: *mut c_void,
  ) -> i32;
  pub fn FSCopyAliasInfo(
    inAlias: *mut *mut AliasRecord,
    targetName: *mut HFSUniStr255,
    volumeName: *mut HFSUniStr255,
    pathString: *mut *mut __CFString,
    whichInfo: *mut u32,
    info: *mut FSAliasInfo,
  ) -> i32;
  pub fn GetAliasSize(alias: *mut *mut AliasRecord) -> isize;
  pub fn GetAliasUserType(alias: *mut *mut AliasRecord) -> u32;
  pub fn SetAliasUserType(alias: *mut *mut AliasRecord, userType: u32) -> ();
  pub fn GetAliasSizeFromPtr(alias: *mut AliasRecord) -> isize;
  pub fn GetAliasUserTypeFromPtr(alias: *mut AliasRecord) -> u32;
  pub fn SetAliasUserTypeWithPtr(alias: *mut AliasRecord, userType: u32) -> ();
  pub fn LocaleRefFromLangOrRegionCode(
    lang: i16,
    region: i16,
    locale: *mut *mut OpaqueLocaleRef,
  ) -> i32;
  pub fn LocaleRefFromLocaleString(localeString: *mut i8, locale: *mut *mut OpaqueLocaleRef)
    -> i32;
  pub fn LocaleRefGetPartString(
    locale: *mut OpaqueLocaleRef,
    partMask: u32,
    maxStringLen: usize,
    partString: *mut i8,
  ) -> i32;
  pub fn LocaleStringToLangAndRegionCodes(
    localeString: *mut i8,
    lang: *mut i16,
    region: *mut i16,
  ) -> i32;
  pub fn LocaleOperationCountLocales(opClass: u32, localeCount: *mut usize) -> i32;
  pub fn LocaleOperationGetLocales(
    opClass: u32,
    maxLocaleCount: usize,
    actualLocaleCount: *mut usize,
    localeVariantList: *mut LocaleAndVariant,
  ) -> i32;
  pub fn LocaleGetName(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    nameMask: u32,
    displayLocale: *mut OpaqueLocaleRef,
    maxNameLen: usize,
    actualNameLen: *mut usize,
    displayName: *mut u16,
  ) -> i32;
  pub fn LocaleCountNames(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    nameMask: u32,
    nameCount: *mut usize,
  ) -> i32;
  pub fn LocaleGetIndName(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    nameMask: u32,
    nameIndex: usize,
    maxNameLen: usize,
    actualNameLen: *mut usize,
    displayName: *mut u16,
    displayLocale: *mut *mut OpaqueLocaleRef,
  ) -> i32;
  pub fn LocaleOperationGetName(
    opClass: u32,
    displayLocale: *mut OpaqueLocaleRef,
    maxNameLen: usize,
    actualNameLen: *mut usize,
    displayName: *mut u16,
  ) -> i32;
  pub fn LocaleOperationCountNames(opClass: u32, nameCount: *mut usize) -> i32;
  pub fn LocaleOperationGetIndName(
    opClass: u32,
    nameIndex: usize,
    maxNameLen: usize,
    actualNameLen: *mut usize,
    displayName: *mut u16,
    displayLocale: *mut *mut OpaqueLocaleRef,
  ) -> i32;
  pub fn DebugAssert(
    componentSignature: u32,
    options: u32,
    assertionString: *mut i8,
    exceptionLabelString: *mut i8,
    errorString: *mut i8,
    fileName: *mut i8,
    lineNumber: isize,
    value: *mut c_void,
  ) -> ();
  pub fn TaskLevel() -> u32;
  pub fn NewDebugComponent(
    componentSignature: u32,
    componentName: *mut u8,
    componentCallback: Option<extern "C" fn(i32, u32, *mut u8) -> ()>,
  ) -> i32;
  pub fn NewDebugOption(
    componentSignature: u32,
    optionSelectorNum: i32,
    optionName: *mut u8,
  ) -> i32;
  pub fn DisposeDebugComponent(componentSignature: u32) -> i32;
  pub fn GetDebugComponentInfo(
    itemIndex: u32,
    componentSignature: *mut u32,
    componentName: [u8; 256],
  ) -> i32;
  pub fn GetDebugOptionInfo(
    itemIndex: u32,
    componentSignature: u32,
    optionSelectorNum: *mut i32,
    optionName: [u8; 256],
    optionSetting: *mut u8,
  ) -> i32;
  pub fn SetDebugOptionValue(
    componentSignature: u32,
    optionSelectorNum: i32,
    newOptionSetting: u8,
  ) -> i32;
  pub fn InstallDebugAssertOutputHandler(
    handler: Option<
      extern "C" fn(
        u32,
        u32,
        *mut i8,
        *mut i8,
        *mut i8,
        *mut i8,
        isize,
        *mut c_void,
        *mut u8,
      ) -> (),
    >,
  ) -> ();
  pub fn GetMacOSStatusErrorString(err: i32) -> *mut i8;
  pub fn GetMacOSStatusCommentString(err: i32) -> *mut i8;
  pub fn NewDebugComponentCallbackUPP(
    userRoutine: Option<extern "C" fn(i32, u32, *mut u8) -> ()>,
  ) -> Option<extern "C" fn(i32, u32, *mut u8) -> ()>;
  pub fn NewDebugAssertOutputHandlerUPP(
    userRoutine: Option<
      extern "C" fn(
        u32,
        u32,
        *mut i8,
        *mut i8,
        *mut i8,
        *mut i8,
        isize,
        *mut c_void,
        *mut u8,
      ) -> (),
    >,
  ) -> Option<
    extern "C" fn(u32, u32, *mut i8, *mut i8, *mut i8, *mut i8, isize, *mut c_void, *mut u8) -> (),
  >;
  pub fn DisposeDebugComponentCallbackUPP(
    userUPP: Option<extern "C" fn(i32, u32, *mut u8) -> ()>,
  ) -> ();
  pub fn DisposeDebugAssertOutputHandlerUPP(
    userUPP: Option<
      extern "C" fn(
        u32,
        u32,
        *mut i8,
        *mut i8,
        *mut i8,
        *mut i8,
        isize,
        *mut c_void,
        *mut u8,
      ) -> (),
    >,
  ) -> ();
  pub fn InvokeDebugComponentCallbackUPP(
    optionSelectorNum: i32,
    command: u32,
    optionSetting: *mut u8,
    userUPP: Option<extern "C" fn(i32, u32, *mut u8) -> ()>,
  ) -> ();
  pub fn InvokeDebugAssertOutputHandlerUPP(
    componentSignature: u32,
    options: u32,
    assertionString: *mut i8,
    exceptionLabelString: *mut i8,
    errorString: *mut i8,
    fileName: *mut i8,
    lineNumber: isize,
    value: *mut c_void,
    outputMsg: *mut u8,
    userUPP: Option<
      extern "C" fn(
        u32,
        u32,
        *mut i8,
        *mut i8,
        *mut i8,
        *mut i8,
        isize,
        *mut c_void,
        *mut u8,
      ) -> (),
    >,
  ) -> ();
  pub fn PLstrcmp(str1: *mut u8, str2: *mut u8) -> i16;
  pub fn PLstrncmp(str1: *mut u8, str2: *mut u8, num: i16) -> i16;
  pub fn PLstrcpy(dest: *mut u8, source: *mut u8) -> *mut u8;
  pub fn PLstrncpy(dest: *mut u8, source: *mut u8, num: i16) -> *mut u8;
  pub fn PLstrcat(str: *mut u8, append: *mut u8) -> *mut u8;
  pub fn PLstrncat(str1: *mut u8, append: *mut u8, num: i16) -> *mut u8;
  pub fn PLstrchr(str1: *mut u8, ch1: i16) -> *mut i8;
  pub fn PLstrrchr(str1: *mut u8, ch1: i16) -> *mut i8;
  pub fn PLstrpbrk(str1: *mut u8, charSet: *mut u8) -> *mut i8;
  pub fn PLstrspn(str1: *mut u8, charSet: *mut u8) -> i16;
  pub fn PLstrstr(str1: *mut u8, searchStr: *mut u8) -> *mut i8;
  pub fn PLstrlen(str: *mut u8) -> i16;
  pub fn PLpos(str1: *mut u8, searchStr: *mut u8) -> i16;
  pub fn CompareAndSwap(oldValue: u32, newValue: u32, address: *mut u32) -> u8;
  pub fn TestAndClear(bit: u32, address: *mut u8) -> u8;
  pub fn TestAndSet(bit: u32, address: *mut u8) -> u8;
  pub fn IncrementAtomic8(address: *mut i8) -> i8;
  pub fn DecrementAtomic8(address: *mut i8) -> i8;
  pub fn AddAtomic8(amount: i32, address: *mut i8) -> i8;
  pub fn BitAndAtomic8(mask: u32, address: *mut u8) -> u8;
  pub fn BitOrAtomic8(mask: u32, address: *mut u8) -> u8;
  pub fn BitXorAtomic8(mask: u32, address: *mut u8) -> u8;
  pub fn IncrementAtomic16(address: *mut i16) -> i16;
  pub fn DecrementAtomic16(address: *mut i16) -> i16;
  pub fn AddAtomic16(amount: i32, address: *mut i16) -> i16;
  pub fn BitAndAtomic16(mask: u32, address: *mut u16) -> u16;
  pub fn BitOrAtomic16(mask: u32, address: *mut u16) -> u16;
  pub fn BitXorAtomic16(mask: u32, address: *mut u16) -> u16;
  pub fn IncrementAtomic(address: *mut i32) -> i32;
  pub fn DecrementAtomic(address: *mut i32) -> i32;
  pub fn AddAtomic(amount: i32, address: *mut i32) -> i32;
  pub fn BitAndAtomic(mask: u32, address: *mut u32) -> u32;
  pub fn BitOrAtomic(mask: u32, address: *mut u32) -> u32;
  pub fn BitXorAtomic(mask: u32, address: *mut u32) -> u32;
  pub fn NewExceptionHandlerUPP(
    userRoutine: Option<extern "C" fn(*mut ExceptionInformation) -> i32>,
  ) -> Option<extern "C" fn(*mut ExceptionInformation) -> i32>;
  pub fn DisposeExceptionHandlerUPP(
    userUPP: Option<extern "C" fn(*mut ExceptionInformation) -> i32>,
  ) -> ();
  pub fn InvokeExceptionHandlerUPP(
    theException: *mut ExceptionInformation,
    userUPP: Option<extern "C" fn(*mut ExceptionInformation) -> i32>,
  ) -> i32;
  pub fn InstallExceptionHandler(
    theHandler: Option<extern "C" fn(*mut ExceptionInformation) -> i32>,
  ) -> Option<extern "C" fn(*mut ExceptionInformation) -> i32>;
  pub fn UpTime() -> UnsignedWide;
  pub fn AbsoluteToNanoseconds(absoluteTime: UnsignedWide) -> UnsignedWide;
  pub fn AbsoluteToDuration(absoluteTime: UnsignedWide) -> i32;
  pub fn NanosecondsToAbsolute(nanoseconds: UnsignedWide) -> UnsignedWide;
  pub fn DurationToAbsolute(duration: i32) -> UnsignedWide;
  pub fn AddAbsoluteToAbsolute(
    absoluteTime1: UnsignedWide,
    absoluteTime2: UnsignedWide,
  ) -> UnsignedWide;
  pub fn SubAbsoluteFromAbsolute(
    leftAbsoluteTime: UnsignedWide,
    rightAbsoluteTime: UnsignedWide,
  ) -> UnsignedWide;
  pub fn AddNanosecondsToAbsolute(
    nanoseconds: UnsignedWide,
    absoluteTime: UnsignedWide,
  ) -> UnsignedWide;
  pub fn AddDurationToAbsolute(duration: i32, absoluteTime: UnsignedWide) -> UnsignedWide;
  pub fn SubNanosecondsFromAbsolute(
    nanoseconds: UnsignedWide,
    absoluteTime: UnsignedWide,
  ) -> UnsignedWide;
  pub fn SubDurationFromAbsolute(duration: i32, absoluteTime: UnsignedWide) -> UnsignedWide;
  pub fn AbsoluteDeltaToNanoseconds(
    leftAbsoluteTime: UnsignedWide,
    rightAbsoluteTime: UnsignedWide,
  ) -> UnsignedWide;
  pub fn AbsoluteDeltaToDuration(
    leftAbsoluteTime: UnsignedWide,
    rightAbsoluteTime: UnsignedWide,
  ) -> i32;
  pub fn DurationToNanoseconds(theDuration: i32) -> UnsignedWide;
  pub fn NanosecondsToDuration(theNanoseconds: UnsignedWide) -> i32;
  pub fn numtostring(theNum: isize, theString: *mut i8) -> ();
  pub fn Munger(
    h: *mut *mut i8,
    offset: isize,
    ptr1: *mut c_void,
    len1: isize,
    ptr2: *mut c_void,
    len2: isize,
  ) -> isize;
  pub fn BitTst(bytePtr: *mut c_void, bitNum: isize) -> u8;
  pub fn BitSet(bytePtr: *mut c_void, bitNum: isize) -> ();
  pub fn BitClr(bytePtr: *mut c_void, bitNum: isize) -> ();
  pub fn BitAnd(value1: isize, value2: isize) -> isize;
  pub fn BitOr(value1: isize, value2: isize) -> isize;
  pub fn BitXor(value1: isize, value2: isize) -> isize;
  pub fn BitNot(value: isize) -> isize;
  pub fn BitShift(value: isize, count: i16) -> isize;
  pub fn NewIndexToUCStringUPP(
    userRoutine: Option<
      extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8,
    >,
  ) -> Option<extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8>;
  pub fn DisposeIndexToUCStringUPP(
    userUPP: Option<
      extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8,
    >,
  ) -> ();
  pub fn InvokeIndexToUCStringUPP(
    index: u32,
    listDataPtr: *mut c_void,
    refcon: *mut c_void,
    outString: *mut *mut __CFString,
    tsOptions: *mut u16,
    userUPP: Option<
      extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8,
    >,
  ) -> u8;
  pub fn UCKeyTranslate(
    keyLayoutPtr: *mut UCKeyboardLayout,
    virtualKeyCode: u16,
    keyAction: u16,
    modifierKeyState: u32,
    keyboardType: u32,
    keyTranslateOptions: u32,
    deadKeyState: *mut u32,
    maxStringLength: usize,
    actualStringLength: *mut usize,
    unicodeString: *mut u16,
  ) -> i32;
  pub fn UCCreateCollator(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    options: u32,
    collatorRef: *mut *mut OpaqueCollatorRef,
  ) -> i32;
  pub fn UCGetCollationKey(
    collatorRef: *mut OpaqueCollatorRef,
    textPtr: *mut u16,
    textLength: usize,
    maxKeySize: usize,
    actualKeySize: *mut usize,
    collationKey: *mut u32,
  ) -> i32;
  pub fn UCCompareCollationKeys(
    key1Ptr: *mut u32,
    key1Length: usize,
    key2Ptr: *mut u32,
    key2Length: usize,
    equivalent: *mut u8,
    order: *mut i32,
  ) -> i32;
  pub fn UCCompareText(
    collatorRef: *mut OpaqueCollatorRef,
    text1Ptr: *mut u16,
    text1Length: usize,
    text2Ptr: *mut u16,
    text2Length: usize,
    equivalent: *mut u8,
    order: *mut i32,
  ) -> i32;
  pub fn UCDisposeCollator(collatorRef: *mut *mut OpaqueCollatorRef) -> i32;
  pub fn UCCompareTextDefault(
    options: u32,
    text1Ptr: *mut u16,
    text1Length: usize,
    text2Ptr: *mut u16,
    text2Length: usize,
    equivalent: *mut u8,
    order: *mut i32,
  ) -> i32;
  pub fn UCCompareTextNoLocale(
    options: u32,
    text1Ptr: *mut u16,
    text1Length: usize,
    text2Ptr: *mut u16,
    text2Length: usize,
    equivalent: *mut u8,
    order: *mut i32,
  ) -> i32;
  pub fn UCCreateTextBreakLocator(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    breakTypes: u32,
    breakRef: *mut *mut OpaqueTextBreakLocatorRef,
  ) -> i32;
  pub fn UCFindTextBreak(
    breakRef: *mut OpaqueTextBreakLocatorRef,
    breakType: u32,
    options: u32,
    textPtr: *mut u16,
    textLength: usize,
    startOffset: usize,
    breakOffset: *mut usize,
  ) -> i32;
  pub fn UCDisposeTextBreakLocator(breakRef: *mut *mut OpaqueTextBreakLocatorRef) -> i32;
  pub fn UCTypeSelectCreateSelector(
    locale: *mut OpaqueLocaleRef,
    opVariant: u32,
    options: u32,
    newSelector: *mut *mut OpaqueUCTypeSelectRef,
  ) -> i32;
  pub fn UCTypeSelectFlushSelectorData(ref_: *mut OpaqueUCTypeSelectRef) -> i32;
  pub fn UCTypeSelectReleaseSelector(ref_: *mut *mut OpaqueUCTypeSelectRef) -> i32;
  pub fn UCTypeSelectWouldResetBuffer(
    inRef: *mut OpaqueUCTypeSelectRef,
    inText: *mut __CFString,
    inEventTime: f64,
  ) -> u8;
  pub fn UCTypeSelectAddKeyToSelector(
    inRef: *mut OpaqueUCTypeSelectRef,
    inText: *mut __CFString,
    inEventTime: f64,
    updateFlag: *mut u8,
  ) -> i32;
  pub fn UCTypeSelectCompare(
    ref_: *mut OpaqueUCTypeSelectRef,
    inText: *mut __CFString,
    result: *mut i32,
  ) -> i32;
  pub fn UCTypeSelectFindItem(
    ref_: *mut OpaqueUCTypeSelectRef,
    listSize: u32,
    listDataPtr: *mut c_void,
    refcon: *mut c_void,
    userUPP: Option<
      extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8,
    >,
    closestItem: *mut u32,
  ) -> i32;
  pub fn UCTypeSelectWalkList(
    ref_: *mut OpaqueUCTypeSelectRef,
    currSelect: *mut __CFString,
    direction: u16,
    listSize: u32,
    listDataPtr: *mut c_void,
    refcon: *mut c_void,
    userUPP: Option<
      extern "C" fn(u32, *mut c_void, *mut c_void, *mut *mut __CFString, *mut u16) -> u8,
    >,
    closestItem: *mut u32,
  ) -> i32;
  pub fn compound(rate: f64, periods: f64) -> f64;
  pub fn annuity(rate: f64, periods: f64) -> f64;
  pub fn randomx(x: *mut f64) -> f64;
  pub fn relation(x: f64, y: f64) -> i16;
  pub fn num2dec(f: *mut decform, x: f64, d: *mut decimal) -> ();
  pub fn dec2num(d: *mut decimal) -> f64;
  pub fn dec2str(f: *mut decform, d: *mut decimal, s: *mut i8) -> ();
  pub fn str2dec(s: *mut i8, ix: *mut i16, d: *mut decimal, vp: *mut i16) -> ();
  pub fn dec2f(d: *mut decimal) -> f32;
  pub fn dec2s(d: *mut decimal) -> i16;
  pub fn dec2l(d: *mut decimal) -> isize;
  pub fn relationl(x: (), y: ()) -> i16;
  pub fn num2decl(f: *mut decform, x: (), d: *mut decimal) -> ();
  pub fn dec2numl(d: *mut decimal) -> ();
  pub fn x80tod(x80: *mut Float80) -> f64;
  pub fn dtox80(x: *mut f64, x80: *mut Float80) -> ();
  pub fn x80told(x80: *mut Float80, x: *mut c_void) -> ();
  pub fn ldtox80(x: *mut c_void, x80: *mut Float80) -> ();
  pub fn TECCountAvailableTextEncodings(numberEncodings: *mut usize) -> i32;
  pub fn TECGetAvailableTextEncodings(
    availableEncodings: *mut u32,
    maxAvailableEncodings: usize,
    actualAvailableEncodings: *mut usize,
  ) -> i32;
  pub fn TECCountDirectTextEncodingConversions(numberOfEncodings: *mut usize) -> i32;
  pub fn TECGetDirectTextEncodingConversions(
    availableConversions: *mut TECConversionInfo,
    maxAvailableConversions: usize,
    actualAvailableConversions: *mut usize,
  ) -> i32;
  pub fn TECCountDestinationTextEncodings(inputEncoding: u32, numberOfEncodings: *mut usize)
    -> i32;
  pub fn TECGetDestinationTextEncodings(
    inputEncoding: u32,
    destinationEncodings: *mut u32,
    maxDestinationEncodings: usize,
    actualDestinationEncodings: *mut usize,
  ) -> i32;
  pub fn TECGetTextEncodingInternetName(textEncoding: u32, encodingName: [u8; 256]) -> i32;
  pub fn TECGetTextEncodingFromInternetName(textEncoding: *mut u32, encodingName: *mut u8) -> i32;
  pub fn TECCreateConverter(
    newEncodingConverter: *mut *mut OpaqueTECObjectRef,
    inputEncoding: u32,
    outputEncoding: u32,
  ) -> i32;
  pub fn TECCreateConverterFromPath(
    newEncodingConverter: *mut *mut OpaqueTECObjectRef,
    inPath: *mut u32,
    inEncodings: usize,
  ) -> i32;
  pub fn TECDisposeConverter(newEncodingConverter: *mut OpaqueTECObjectRef) -> i32;
  pub fn TECClearConverterContextInfo(encodingConverter: *mut OpaqueTECObjectRef) -> i32;
  pub fn TECConvertText(
    encodingConverter: *mut OpaqueTECObjectRef,
    inputBuffer: *mut u8,
    inputBufferLength: usize,
    actualInputLength: *mut usize,
    outputBuffer: *mut u8,
    outputBufferLength: usize,
    actualOutputLength: *mut usize,
  ) -> i32;
  pub fn TECFlushText(
    encodingConverter: *mut OpaqueTECObjectRef,
    outputBuffer: *mut u8,
    outputBufferLength: usize,
    actualOutputLength: *mut usize,
  ) -> i32;
  pub fn TECCountSubTextEncodings(inputEncoding: u32, numberOfEncodings: *mut usize) -> i32;
  pub fn TECGetSubTextEncodings(
    inputEncoding: u32,
    subEncodings: *mut u32,
    maxSubEncodings: usize,
    actualSubEncodings: *mut usize,
  ) -> i32;
  pub fn TECGetEncodingList(
    encodingConverter: *mut OpaqueTECObjectRef,
    numEncodings: *mut usize,
    encodingList: *mut *mut *mut i8,
  ) -> i32;
  pub fn TECCreateOneToManyConverter(
    newEncodingConverter: *mut *mut OpaqueTECObjectRef,
    inputEncoding: u32,
    numOutputEncodings: usize,
    outputEncodings: *mut u32,
  ) -> i32;
  pub fn TECConvertTextToMultipleEncodings(
    encodingConverter: *mut OpaqueTECObjectRef,
    inputBuffer: *mut u8,
    inputBufferLength: usize,
    actualInputLength: *mut usize,
    outputBuffer: *mut u8,
    outputBufferLength: usize,
    actualOutputLength: *mut usize,
    outEncodingsBuffer: *mut TextEncodingRun,
    maxOutEncodingRuns: usize,
    actualOutEncodingRuns: *mut usize,
  ) -> i32;
  pub fn TECFlushMultipleEncodings(
    encodingConverter: *mut OpaqueTECObjectRef,
    outputBuffer: *mut u8,
    outputBufferLength: usize,
    actualOutputLength: *mut usize,
    outEncodingsBuffer: *mut TextEncodingRun,
    maxOutEncodingRuns: usize,
    actualOutEncodingRuns: *mut usize,
  ) -> i32;
  pub fn TECCountWebTextEncodings(locale: i16, numberEncodings: *mut usize) -> i32;
  pub fn TECGetWebTextEncodings(
    locale: i16,
    availableEncodings: *mut u32,
    maxAvailableEncodings: usize,
    actualAvailableEncodings: *mut usize,
  ) -> i32;
  pub fn TECCountMailTextEncodings(locale: i16, numberEncodings: *mut usize) -> i32;
  pub fn TECGetMailTextEncodings(
    locale: i16,
    availableEncodings: *mut u32,
    maxAvailableEncodings: usize,
    actualAvailableEncodings: *mut usize,
  ) -> i32;
  pub fn TECCountAvailableSniffers(numberOfEncodings: *mut usize) -> i32;
  pub fn TECGetAvailableSniffers(
    availableSniffers: *mut u32,
    maxAvailableSniffers: usize,
    actualAvailableSniffers: *mut usize,
  ) -> i32;
  pub fn TECCreateSniffer(
    encodingSniffer: *mut *mut OpaqueTECSnifferObjectRef,
    testEncodings: *mut u32,
    numTextEncodings: usize,
  ) -> i32;
  pub fn TECSniffTextEncoding(
    encodingSniffer: *mut OpaqueTECSnifferObjectRef,
    inputBuffer: *mut u8,
    inputBufferLength: usize,
    testEncodings: *mut u32,
    numTextEncodings: usize,
    numErrsArray: *mut usize,
    maxErrs: usize,
    numFeaturesArray: *mut usize,
    maxFeatures: usize,
  ) -> i32;
  pub fn TECDisposeSniffer(encodingSniffer: *mut OpaqueTECSnifferObjectRef) -> i32;
  pub fn TECClearSnifferContextInfo(encodingSniffer: *mut OpaqueTECSnifferObjectRef) -> i32;
  pub fn TECSetBasicOptions(encodingConverter: *mut OpaqueTECObjectRef, controlFlags: u32) -> i32;
  pub fn TECCopyTextEncodingInternetNameAndMIB(
    textEncoding: u32,
    usage: u32,
    encodingNamePtr: *mut *mut __CFString,
    mibEnumPtr: *mut i32,
  ) -> i32;
  pub fn TECGetTextEncodingFromInternetNameOrMIB(
    textEncodingPtr: *mut u32,
    usage: u32,
    encodingName: *mut __CFString,
    mibEnum: i32,
  ) -> i32;
  pub fn NewUnicodeToTextFallbackUPP(
    userRoutine: Option<
      extern "C" fn(
        *mut u16,
        usize,
        *mut usize,
        *mut u8,
        usize,
        *mut usize,
        *mut c_void,
        *mut UnicodeMapping,
      ) -> i32,
    >,
  ) -> Option<
    extern "C" fn(
      *mut u16,
      usize,
      *mut usize,
      *mut u8,
      usize,
      *mut usize,
      *mut c_void,
      *mut UnicodeMapping,
    ) -> i32,
  >;
  pub fn DisposeUnicodeToTextFallbackUPP(
    userUPP: Option<
      extern "C" fn(
        *mut u16,
        usize,
        *mut usize,
        *mut u8,
        usize,
        *mut usize,
        *mut c_void,
        *mut UnicodeMapping,
      ) -> i32,
    >,
  ) -> ();
  pub fn InvokeUnicodeToTextFallbackUPP(
    iSrcUniStr: *mut u16,
    iSrcUniStrLen: usize,
    oSrcConvLen: *mut usize,
    oDestStr: *mut u8,
    iDestStrLen: usize,
    oDestConvLen: *mut usize,
    iInfoPtr: *mut c_void,
    iUnicodeMappingPtr: *mut UnicodeMapping,
    userUPP: Option<
      extern "C" fn(
        *mut u16,
        usize,
        *mut usize,
        *mut u8,
        usize,
        *mut usize,
        *mut c_void,
        *mut UnicodeMapping,
      ) -> i32,
    >,
  ) -> i32;
  pub fn CreateTextToUnicodeInfo(
    iUnicodeMapping: *mut UnicodeMapping,
    oTextToUnicodeInfo: *mut *mut OpaqueTextToUnicodeInfo,
  ) -> i32;
  pub fn CreateTextToUnicodeInfoByEncoding(
    iEncoding: u32,
    oTextToUnicodeInfo: *mut *mut OpaqueTextToUnicodeInfo,
  ) -> i32;
  pub fn CreateUnicodeToTextInfo(
    iUnicodeMapping: *mut UnicodeMapping,
    oUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextInfo,
  ) -> i32;
  pub fn CreateUnicodeToTextInfoByEncoding(
    iEncoding: u32,
    oUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextInfo,
  ) -> i32;
  pub fn CreateUnicodeToTextRunInfo(
    iNumberOfMappings: usize,
    iUnicodeMappings: *mut UnicodeMapping,
    oUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextRunInfo,
  ) -> i32;
  pub fn CreateUnicodeToTextRunInfoByEncoding(
    iNumberOfEncodings: usize,
    iEncodings: *mut u32,
    oUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextRunInfo,
  ) -> i32;
  pub fn CreateUnicodeToTextRunInfoByScriptCode(
    iNumberOfScriptCodes: usize,
    iScripts: *mut i16,
    oUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextRunInfo,
  ) -> i32;
  pub fn ChangeTextToUnicodeInfo(
    ioTextToUnicodeInfo: *mut OpaqueTextToUnicodeInfo,
    iUnicodeMapping: *mut UnicodeMapping,
  ) -> i32;
  pub fn ChangeUnicodeToTextInfo(
    ioUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo,
    iUnicodeMapping: *mut UnicodeMapping,
  ) -> i32;
  pub fn DisposeTextToUnicodeInfo(ioTextToUnicodeInfo: *mut *mut OpaqueTextToUnicodeInfo) -> i32;
  pub fn DisposeUnicodeToTextInfo(ioUnicodeToTextInfo: *mut *mut OpaqueUnicodeToTextInfo) -> i32;
  pub fn DisposeUnicodeToTextRunInfo(
    ioUnicodeToTextRunInfo: *mut *mut OpaqueUnicodeToTextRunInfo,
  ) -> i32;
  pub fn ConvertFromTextToUnicode(
    iTextToUnicodeInfo: *mut OpaqueTextToUnicodeInfo,
    iSourceLen: usize,
    iSourceStr: *mut c_void,
    iControlFlags: u32,
    iOffsetCount: usize,
    iOffsetArray: *mut usize,
    oOffsetCount: *mut usize,
    oOffsetArray: *mut usize,
    iOutputBufLen: usize,
    oSourceRead: *mut usize,
    oUnicodeLen: *mut usize,
    oUnicodeStr: *mut u16,
  ) -> i32;
  pub fn ConvertFromUnicodeToText(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo,
    iUnicodeLen: usize,
    iUnicodeStr: *mut u16,
    iControlFlags: u32,
    iOffsetCount: usize,
    iOffsetArray: *mut usize,
    oOffsetCount: *mut usize,
    oOffsetArray: *mut usize,
    iOutputBufLen: usize,
    oInputRead: *mut usize,
    oOutputLen: *mut usize,
    oOutputStr: *mut c_void,
  ) -> i32;
  pub fn ConvertFromUnicodeToTextRun(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextRunInfo,
    iUnicodeLen: usize,
    iUnicodeStr: *mut u16,
    iControlFlags: u32,
    iOffsetCount: usize,
    iOffsetArray: *mut usize,
    oOffsetCount: *mut usize,
    oOffsetArray: *mut usize,
    iOutputBufLen: usize,
    oInputRead: *mut usize,
    oOutputLen: *mut usize,
    oOutputStr: *mut c_void,
    iEncodingRunBufLen: usize,
    oEncodingRunOutLen: *mut usize,
    oEncodingRuns: *mut TextEncodingRun,
  ) -> i32;
  pub fn ConvertFromUnicodeToScriptCodeRun(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextRunInfo,
    iUnicodeLen: usize,
    iUnicodeStr: *mut u16,
    iControlFlags: u32,
    iOffsetCount: usize,
    iOffsetArray: *mut usize,
    oOffsetCount: *mut usize,
    oOffsetArray: *mut usize,
    iOutputBufLen: usize,
    oInputRead: *mut usize,
    oOutputLen: *mut usize,
    oOutputStr: *mut c_void,
    iScriptRunBufLen: usize,
    oScriptRunOutLen: *mut usize,
    oScriptCodeRuns: *mut ScriptCodeRun,
  ) -> i32;
  pub fn TruncateForTextToUnicode(
    iTextToUnicodeInfo: *mut OpaqueTextToUnicodeInfo,
    iSourceLen: usize,
    iSourceStr: *mut c_void,
    iMaxLen: usize,
    oTruncatedLen: *mut usize,
  ) -> i32;
  pub fn TruncateForUnicodeToText(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo,
    iSourceLen: usize,
    iSourceStr: *mut u16,
    iControlFlags: u32,
    iMaxLen: usize,
    oTruncatedLen: *mut usize,
  ) -> i32;
  pub fn ConvertFromPStringToUnicode(
    iTextToUnicodeInfo: *mut OpaqueTextToUnicodeInfo,
    iPascalStr: *mut u8,
    iOutputBufLen: usize,
    oUnicodeLen: *mut usize,
    oUnicodeStr: *mut u16,
  ) -> i32;
  pub fn ConvertFromUnicodeToPString(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo,
    iUnicodeLen: usize,
    iUnicodeStr: *mut u16,
    oPascalStr: [u8; 256],
  ) -> i32;
  pub fn CountUnicodeMappings(
    iFilter: u32,
    iFindMapping: *mut UnicodeMapping,
    oActualCount: *mut usize,
  ) -> i32;
  pub fn QueryUnicodeMappings(
    iFilter: u32,
    iFindMapping: *mut UnicodeMapping,
    iMaxCount: usize,
    oActualCount: *mut usize,
    oReturnedMappings: *mut UnicodeMapping,
  ) -> i32;
  pub fn SetFallbackUnicodeToText(
    iUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo,
    iFallback: Option<
      extern "C" fn(
        *mut u16,
        usize,
        *mut usize,
        *mut u8,
        usize,
        *mut usize,
        *mut c_void,
        *mut UnicodeMapping,
      ) -> i32,
    >,
    iControlFlags: u32,
    iInfoPtr: *mut c_void,
  ) -> i32;
  pub fn SetFallbackUnicodeToTextRun(
    iUnicodeToTextRunInfo: *mut OpaqueUnicodeToTextRunInfo,
    iFallback: Option<
      extern "C" fn(
        *mut u16,
        usize,
        *mut usize,
        *mut u8,
        usize,
        *mut usize,
        *mut c_void,
        *mut UnicodeMapping,
      ) -> i32,
    >,
    iControlFlags: u32,
    iInfoPtr: *mut c_void,
  ) -> i32;
  pub fn ResetTextToUnicodeInfo(ioTextToUnicodeInfo: *mut OpaqueTextToUnicodeInfo) -> i32;
  pub fn ResetUnicodeToTextInfo(ioUnicodeToTextInfo: *mut OpaqueUnicodeToTextInfo) -> i32;
  pub fn ResetUnicodeToTextRunInfo(ioUnicodeToTextRunInfo: *mut OpaqueUnicodeToTextRunInfo) -> i32;
  pub fn NewThreadEntryUPP(
    userRoutine: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  ) -> Option<extern "C" fn(*mut c_void) -> *mut c_void>;
  pub fn NewThreadSchedulerUPP(
    userRoutine: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
  pub fn NewThreadSwitchUPP(
    userRoutine: Option<extern "C" fn(usize, *mut c_void) -> ()>,
  ) -> Option<extern "C" fn(usize, *mut c_void) -> ()>;
  pub fn NewThreadTerminationUPP(
    userRoutine: Option<extern "C" fn(usize, *mut c_void) -> ()>,
  ) -> Option<extern "C" fn(usize, *mut c_void) -> ()>;
  pub fn NewDebuggerNewThreadUPP(
    userRoutine: Option<extern "C" fn(usize) -> ()>,
  ) -> Option<extern "C" fn(usize) -> ()>;
  pub fn NewDebuggerDisposeThreadUPP(
    userRoutine: Option<extern "C" fn(usize) -> ()>,
  ) -> Option<extern "C" fn(usize) -> ()>;
  pub fn NewDebuggerThreadSchedulerUPP(
    userRoutine: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>;
  pub fn DisposeThreadEntryUPP(userUPP: Option<extern "C" fn(*mut c_void) -> *mut c_void>) -> ();
  pub fn DisposeThreadSchedulerUPP(
    userUPP: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> ();
  pub fn DisposeThreadSwitchUPP(userUPP: Option<extern "C" fn(usize, *mut c_void) -> ()>) -> ();
  pub fn DisposeThreadTerminationUPP(
    userUPP: Option<extern "C" fn(usize, *mut c_void) -> ()>,
  ) -> ();
  pub fn DisposeDebuggerNewThreadUPP(userUPP: Option<extern "C" fn(usize) -> ()>) -> ();
  pub fn DisposeDebuggerDisposeThreadUPP(userUPP: Option<extern "C" fn(usize) -> ()>) -> ();
  pub fn DisposeDebuggerThreadSchedulerUPP(
    userUPP: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> ();
  pub fn InvokeThreadEntryUPP(
    threadParam: *mut c_void,
    userUPP: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
  ) -> *mut c_void;
  pub fn InvokeThreadSchedulerUPP(
    schedulerInfo: *mut SchedulerInfoRec,
    userUPP: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> usize;
  pub fn InvokeThreadSwitchUPP(
    threadBeingSwitched: usize,
    switchProcParam: *mut c_void,
    userUPP: Option<extern "C" fn(usize, *mut c_void) -> ()>,
  ) -> ();
  pub fn InvokeThreadTerminationUPP(
    threadTerminated: usize,
    terminationProcParam: *mut c_void,
    userUPP: Option<extern "C" fn(usize, *mut c_void) -> ()>,
  ) -> ();
  pub fn InvokeDebuggerNewThreadUPP(
    threadCreated: usize,
    userUPP: Option<extern "C" fn(usize) -> ()>,
  ) -> ();
  pub fn InvokeDebuggerDisposeThreadUPP(
    threadDeleted: usize,
    userUPP: Option<extern "C" fn(usize) -> ()>,
  ) -> ();
  pub fn InvokeDebuggerThreadSchedulerUPP(
    schedulerInfo: *mut SchedulerInfoRec,
    userUPP: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> usize;
  pub fn NewThread(
    threadStyle: u32,
    threadEntry: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    threadParam: *mut c_void,
    stackSize: isize,
    options: u32,
    threadResult: *mut *mut c_void,
    threadMade: *mut usize,
  ) -> i16;
  pub fn SetThreadScheduler(
    threadScheduler: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> i16;
  pub fn SetThreadSwitcher(
    thread: usize,
    threadSwitcher: Option<extern "C" fn(usize, *mut c_void) -> ()>,
    switchProcParam: *mut c_void,
    inOrOut: u8,
  ) -> i16;
  pub fn SetThreadTerminator(
    thread: usize,
    threadTerminator: Option<extern "C" fn(usize, *mut c_void) -> ()>,
    terminationProcParam: *mut c_void,
  ) -> i16;
  pub fn SetDebuggerNotificationProcs(
    notifyNewThread: Option<extern "C" fn(usize) -> ()>,
    notifyDisposeThread: Option<extern "C" fn(usize) -> ()>,
    notifyThreadScheduler: Option<extern "C" fn(*mut SchedulerInfoRec) -> usize>,
  ) -> i16;
  pub fn CreateThreadPool(threadStyle: u32, numToCreate: i16, stackSize: isize) -> i16;
  pub fn GetDefaultThreadStackSize(threadStyle: u32, stackSize: *mut isize) -> i16;
  pub fn ThreadCurrentStackSpace(thread: usize, freeStack: *mut usize) -> i16;
  pub fn DisposeThread(threadToDump: usize, threadResult: *mut c_void, recycleThread: u8) -> i16;
  pub fn YieldToThread(suggestedThread: usize) -> i16;
  pub fn YieldToAnyThread() -> i16;
  pub fn GetCurrentThread(currentThreadID: *mut usize) -> i16;
  pub fn GetThreadState(threadToGet: usize, threadState: *mut u16) -> i16;
  pub fn SetThreadState(threadToSet: usize, newState: u16, suggestedThread: usize) -> i16;
  pub fn SetThreadStateEndCritical(
    threadToSet: usize,
    newState: u16,
    suggestedThread: usize,
  ) -> i16;
  pub fn ThreadBeginCritical() -> i16;
  pub fn ThreadEndCritical() -> i16;
  pub fn GetThreadCurrentTaskRef(threadTRef: *mut *mut c_void) -> i16;
  pub fn GetThreadStateGivenTaskRef(
    threadTRef: *mut c_void,
    threadToGet: usize,
    threadState: *mut u16,
  ) -> i16;
  pub fn SetThreadReadyGivenTaskRef(threadTRef: *mut c_void, threadToSet: usize) -> i16;
  pub fn FindFolder(
    vRefNum: i16,
    folderType: u32,
    createFolder: u8,
    foundVRefNum: *mut i16,
    foundDirID: *mut i32,
  ) -> i16;
  pub fn ReleaseFolder(vRefNum: i16, folderType: u32) -> i16;
  pub fn FSFindFolder(vRefNum: i16, folderType: u32, createFolder: u8, foundRef: *mut FSRef)
    -> i16;
  pub fn AddFolderDescriptor(
    foldType: u32,
    flags: u32,
    foldClass: u32,
    foldLocation: u32,
    badgeSignature: u32,
    badgeType: u32,
    name: *mut u8,
    replaceFlag: u8,
  ) -> i16;
  pub fn GetFolderTypes(
    requestedTypeCount: u32,
    totalTypeCount: *mut u32,
    theTypes: *mut u32,
  ) -> i16;
  pub fn RemoveFolderDescriptor(foldType: u32) -> i16;
  pub fn GetFolderNameUnicode(
    vRefNum: i16,
    foldType: u32,
    foundVRefNum: *mut i16,
    name: *mut HFSUniStr255,
  ) -> i32;
  pub fn InvalidateFolderDescriptorCache(vRefNum: i16, dirID: i32) -> i16;
  pub fn IdentifyFolder(vRefNum: i16, dirID: i32, foldType: *mut u32) -> i16;
  pub fn FSDetermineIfRefIsEnclosedByFolder(
    domainOrVRefNum: i16,
    folderType: u32,
    inRef: *mut FSRef,
    outResult: *mut u8,
  ) -> i16;
  pub fn DetermineIfPathIsEnclosedByFolder(
    domainOrVRefNum: i16,
    folderType: u32,
    utf8Path: *mut u8,
    pathIsRealPath: u8,
    outResult: *mut u8,
  ) -> i16;
  pub fn NewFolderManagerNotificationUPP(
    userRoutine: Option<extern "C" fn(u32, *mut c_void, *mut c_void) -> i32>,
  ) -> Option<extern "C" fn(u32, *mut c_void, *mut c_void) -> i32>;
  pub fn DisposeFolderManagerNotificationUPP(
    userUPP: Option<extern "C" fn(u32, *mut c_void, *mut c_void) -> i32>,
  ) -> ();
  pub fn InvokeFolderManagerNotificationUPP(
    message: u32,
    arg: *mut c_void,
    userRefCon: *mut c_void,
    userUPP: Option<extern "C" fn(u32, *mut c_void, *mut c_void) -> i32>,
  ) -> i32;
  pub fn Microseconds(microTickCount: *mut UnsignedWide) -> ();
  pub fn InsTime(tmTaskPtr: *mut QElem) -> ();
  pub fn InsXTime(tmTaskPtr: *mut QElem) -> ();
  pub fn PrimeTime(tmTaskPtr: *mut QElem, count: isize) -> ();
  pub fn RmvTime(tmTaskPtr: *mut QElem) -> ();
  pub fn InstallTimeTask(tmTaskPtr: *mut QElem) -> i16;
  pub fn InstallXTimeTask(tmTaskPtr: *mut QElem) -> i16;
  pub fn PrimeTimeTask(tmTaskPtr: *mut QElem, count: isize) -> i16;
  pub fn RemoveTimeTask(tmTaskPtr: *mut QElem) -> i16;
  pub fn NewTimerUPP(
    userRoutine: Option<extern "C" fn(*mut TMTask) -> ()>,
  ) -> Option<extern "C" fn(*mut TMTask) -> ()>;
  pub fn DisposeTimerUPP(userUPP: Option<extern "C" fn(*mut TMTask) -> ()>) -> ();
  pub fn InvokeTimerUPP(
    tmTaskPtr: *mut TMTask,
    userUPP: Option<extern "C" fn(*mut TMTask) -> ()>,
  ) -> ();
  pub fn MPGetNextCpuID(
    owningCoherenceID: *mut OpaqueMPCoherenceID,
    cpuID: *mut *mut OpaqueMPCpuID,
  ) -> i32;
  pub fn MPGetNextTaskID(
    owningProcessID: *mut OpaqueMPProcessID,
    taskID: *mut *mut OpaqueMPTaskID,
  ) -> i32;
  pub fn LMGetBootDrive() -> i16;
  pub fn LMSetBootDrive(value: i16) -> ();
  pub fn LMGetApFontID() -> i16;
  pub fn LMSetApFontID(value: i16) -> ();
  pub fn LMGetSysMap() -> i16;
  pub fn LMSetSysMap(value: i16) -> ();
  pub fn LMGetResLoad() -> u8;
  pub fn LMSetResLoad(value: u8) -> ();
  pub fn LMGetResErr() -> i16;
  pub fn LMSetResErr(value: i16) -> ();
  pub fn LMGetTmpResLoad() -> u8;
  pub fn LMSetTmpResLoad(value: u8) -> ();
  pub fn LMGetIntlSpec() -> *mut i8;
  pub fn LMSetIntlSpec(value: *mut i8) -> ();
  pub fn LMSetSysFontFam(value: i16) -> ();
  pub fn LMGetSysFontSize() -> i16;
  pub fn LMSetSysFontSize(value: i16) -> ();
  pub fn NewAVLCompareItemsUPP(
    userRoutine: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, *mut c_void, u16) -> i32>,
  ) -> Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, *mut c_void, u16) -> i32>;
  pub fn NewAVLItemSizeUPP(
    userRoutine: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> usize>,
  ) -> Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> usize>;
  pub fn NewAVLDisposeItemUPP(
    userRoutine: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> ()>,
  ) -> Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> ()>;
  pub fn NewAVLWalkUPP(
    userRoutine: Option<
      extern "C" fn(*mut AVLTreeStruct, *mut c_void, u16, u16, u32, i32, *mut c_void) -> i16,
    >,
  ) -> Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, u16, u16, u32, i32, *mut c_void) -> i16>;
  pub fn DisposeAVLCompareItemsUPP(
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, *mut c_void, u16) -> i32>,
  ) -> ();
  pub fn DisposeAVLItemSizeUPP(
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> usize>,
  ) -> ();
  pub fn DisposeAVLDisposeItemUPP(
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> ()>,
  ) -> ();
  pub fn DisposeAVLWalkUPP(
    userUPP: Option<
      extern "C" fn(*mut AVLTreeStruct, *mut c_void, u16, u16, u32, i32, *mut c_void) -> i16,
    >,
  ) -> ();
  pub fn InvokeAVLCompareItemsUPP(
    tree: *mut AVLTreeStruct,
    i1: *mut c_void,
    i2: *mut c_void,
    nd_typ: u16,
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void, *mut c_void, u16) -> i32>,
  ) -> i32;
  pub fn InvokeAVLItemSizeUPP(
    tree: *mut AVLTreeStruct,
    itemPtr: *mut c_void,
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> usize>,
  ) -> usize;
  pub fn InvokeAVLDisposeItemUPP(
    tree: *mut AVLTreeStruct,
    dataP: *mut c_void,
    userUPP: Option<extern "C" fn(*mut AVLTreeStruct, *mut c_void) -> ()>,
  ) -> ();
  pub fn InvokeAVLWalkUPP(
    tree: *mut AVLTreeStruct,
    dataPtr: *mut c_void,
    visitStage: u16,
    node: u16,
    level: u32,
    balance: i32,
    refCon: *mut c_void,
    userUPP: Option<
      extern "C" fn(*mut AVLTreeStruct, *mut c_void, u16, u16, u32, i32, *mut c_void) -> i16,
    >,
  ) -> i16;
}
