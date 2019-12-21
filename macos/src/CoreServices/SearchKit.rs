#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::CFRange;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFData;
use crate::CoreFoundation::__CFDictionary;
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
pub type SKDocumentRef = *mut c_void;
#[repr(C)]
pub struct __SKIndex {
  opaque: u32,
}
#[repr(C)]
pub struct __SKIndexDocumentIterator {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SKIndexType {
  kSKIndexUnknown = 0,
  kSKIndexInverted = 1,
  kSKIndexVector = 2,
  kSKIndexInvertedVector = 3,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SKDocumentIndexState {
  kSKDocumentStateNotIndexed = 0,
  kSKDocumentStateIndexed = 1,
  kSKDocumentStateAddPending = 2,
  kSKDocumentStateDeletePending = 3,
}
pub type SKDocumentID = isize;
#[repr(C)]
pub struct __SKSearch {
  opaque: u32,
}
pub type SKSearchOptions = u32;
#[repr(C)]
pub struct __SKSearchGroup {
  opaque: u32,
}
#[repr(C)]
pub struct __SKSearchResults {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SKSearchType {
  kSKSearchRanked = 0,
  kSKSearchBooleanRanked = 1,
  kSKSearchRequiredRanked = 2,
  kSKSearchPrefixRanked = 3,
}
#[repr(C)]
pub struct __SKSummary {
  opaque: u32,
}
extern "C" {
  pub fn SKDocumentGetTypeID() -> usize;
  pub fn SKDocumentCreateWithURL(inURL: *mut __CFURL) -> *mut c_void;
  pub fn SKDocumentCopyURL(inDocument: *mut c_void) -> *mut __CFURL;
  pub fn SKDocumentCreate(
    inScheme: *mut __CFString,
    inParent: *mut c_void,
    inName: *mut __CFString,
  ) -> *mut c_void;
  pub fn SKDocumentGetSchemeName(inDocument: *mut c_void) -> *mut __CFString;
  pub fn SKDocumentGetName(inDocument: *mut c_void) -> *mut __CFString;
  pub fn SKDocumentGetParent(inDocument: *mut c_void) -> *mut c_void;
  pub fn SKIndexGetTypeID() -> usize;
  pub fn SKIndexDocumentIteratorGetTypeID() -> usize;
  pub fn SKIndexCreateWithURL(
    inURL: *mut __CFURL,
    inIndexName: *mut __CFString,
    inIndexType: SKIndexType,
    inAnalysisProperties: *mut __CFDictionary,
  ) -> *mut __SKIndex;
  pub fn SKIndexOpenWithURL(
    inURL: *mut __CFURL,
    inIndexName: *mut __CFString,
    inWriteAccess: u8,
  ) -> *mut __SKIndex;
  pub fn SKIndexCreateWithMutableData(
    inData: *mut __CFData,
    inIndexName: *mut __CFString,
    inIndexType: SKIndexType,
    inAnalysisProperties: *mut __CFDictionary,
  ) -> *mut __SKIndex;
  pub fn SKIndexOpenWithData(inData: *mut __CFData, inIndexName: *mut __CFString)
    -> *mut __SKIndex;
  pub fn SKIndexOpenWithMutableData(
    inData: *mut __CFData,
    inIndexName: *mut __CFString,
  ) -> *mut __SKIndex;
  pub fn SKIndexFlush(inIndex: *mut __SKIndex) -> u8;
  pub fn SKIndexSetMaximumBytesBeforeFlush(inIndex: *mut __SKIndex, inBytesForUpdate: isize) -> ();
  pub fn SKIndexGetMaximumBytesBeforeFlush(inIndex: *mut __SKIndex) -> isize;
  pub fn SKIndexCompact(inIndex: *mut __SKIndex) -> u8;
  pub fn SKIndexGetIndexType(inIndex: *mut __SKIndex) -> SKIndexType;
  pub fn SKIndexGetAnalysisProperties(inIndex: *mut __SKIndex) -> *mut __CFDictionary;
  pub fn SKIndexGetDocumentCount(inIndex: *mut __SKIndex) -> isize;
  pub fn SKIndexClose(inIndex: *mut __SKIndex) -> ();
  pub fn SKIndexAddDocumentWithText(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
    inDocumentText: *mut __CFString,
    inCanReplace: u8,
  ) -> u8;
  pub fn SKIndexAddDocument(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
    inMIMETypeHint: *mut __CFString,
    inCanReplace: u8,
  ) -> u8;
  pub fn SKIndexRemoveDocument(inIndex: *mut __SKIndex, inDocument: *mut c_void) -> u8;
  pub fn SKIndexCopyDocumentProperties(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
  ) -> *mut __CFDictionary;
  pub fn SKIndexSetDocumentProperties(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
    inProperties: *mut __CFDictionary,
  ) -> ();
  pub fn SKIndexGetDocumentState(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
  ) -> SKDocumentIndexState;
  pub fn SKIndexGetDocumentID(inIndex: *mut __SKIndex, inDocument: *mut c_void) -> isize;
  pub fn SKIndexCopyDocumentForDocumentID(
    inIndex: *mut __SKIndex,
    inDocumentID: isize,
  ) -> *mut c_void;
  pub fn SKIndexRenameDocument(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
    inNewName: *mut __CFString,
  ) -> u8;
  pub fn SKIndexMoveDocument(
    inIndex: *mut __SKIndex,
    inDocument: *mut c_void,
    inNewParent: *mut c_void,
  ) -> u8;
  pub fn SKIndexDocumentIteratorCreate(
    inIndex: *mut __SKIndex,
    inParentDocument: *mut c_void,
  ) -> *mut __SKIndexDocumentIterator;
  pub fn SKIndexDocumentIteratorCopyNext(inIterator: *mut __SKIndexDocumentIterator)
    -> *mut c_void;
  pub fn SKIndexGetMaximumDocumentID(inIndex: *mut __SKIndex) -> isize;
  pub fn SKIndexGetDocumentTermCount(inIndex: *mut __SKIndex, inDocumentID: isize) -> isize;
  pub fn SKIndexCopyTermIDArrayForDocumentID(
    inIndex: *mut __SKIndex,
    inDocumentID: isize,
  ) -> *mut __CFArray;
  pub fn SKIndexGetDocumentTermFrequency(
    inIndex: *mut __SKIndex,
    inDocumentID: isize,
    inTermID: isize,
  ) -> isize;
  pub fn SKIndexGetMaximumTermID(inIndex: *mut __SKIndex) -> isize;
  pub fn SKIndexGetTermDocumentCount(inIndex: *mut __SKIndex, inTermID: isize) -> isize;
  pub fn SKIndexCopyDocumentIDArrayForTermID(
    inIndex: *mut __SKIndex,
    inTermID: isize,
  ) -> *mut __CFArray;
  pub fn SKIndexCopyTermStringForTermID(
    inIndex: *mut __SKIndex,
    inTermID: isize,
  ) -> *mut __CFString;
  pub fn SKIndexGetTermIDForTermString(
    inIndex: *mut __SKIndex,
    inTermString: *mut __CFString,
  ) -> isize;
  pub fn SKLoadDefaultExtractorPlugIns() -> ();
  pub fn SKSearchGetTypeID() -> usize;
  pub fn SKSearchCreate(
    inIndex: *mut __SKIndex,
    inQuery: *mut __CFString,
    inSearchOptions: u32,
  ) -> *mut __SKSearch;
  pub fn SKSearchCancel(inSearch: *mut __SKSearch) -> ();
  pub fn SKSearchFindMatches(
    inSearch: *mut __SKSearch,
    inMaximumCount: isize,
    outDocumentIDsArray: *mut isize,
    outScoresArray: *mut f32,
    maximumTime: f64,
    outFoundCount: *mut isize,
  ) -> u8;
  pub fn SKIndexCopyInfoForDocumentIDs(
    inIndex: *mut __SKIndex,
    inCount: isize,
    inDocumentIDsArray: *mut isize,
    outNamesArray: *mut *mut __CFString,
    outParentIDsArray: *mut isize,
  ) -> ();
  pub fn SKIndexCopyDocumentRefsForDocumentIDs(
    inIndex: *mut __SKIndex,
    inCount: isize,
    inDocumentIDsArray: *mut isize,
    outDocumentRefsArray: *mut *mut c_void,
  ) -> ();
  pub fn SKIndexCopyDocumentURLsForDocumentIDs(
    inIndex: *mut __SKIndex,
    inCount: isize,
    inDocumentIDsArray: *mut isize,
    outDocumentURLsArray: *mut *mut __CFURL,
  ) -> ();
  pub fn SKSearchGroupGetTypeID() -> usize;
  pub fn SKSearchResultsGetTypeID() -> usize;
  pub fn SKSearchGroupCreate(inArrayOfInIndexes: *mut __CFArray) -> *mut __SKSearchGroup;
  pub fn SKSearchGroupCopyIndexes(inSearchGroup: *mut __SKSearchGroup) -> *mut __CFArray;
  pub fn SKSearchResultsCreateWithQuery(
    inSearchGroup: *mut __SKSearchGroup,
    inQuery: *mut __CFString,
    inSearchType: SKSearchType,
    inMaxFoundDocuments: isize,
    inContext: *mut c_void,
    inFilterCallBack: Option<extern "C" fn(*mut __SKIndex, *mut c_void, *mut c_void) -> u8>,
  ) -> *mut __SKSearchResults;
  pub fn SKSearchResultsCreateWithDocuments(
    inSearchGroup: *mut __SKSearchGroup,
    inExampleDocuments: *mut __CFArray,
    inMaxFoundDocuments: isize,
    inContext: *mut c_void,
    inFilterCallBack: Option<extern "C" fn(*mut __SKIndex, *mut c_void, *mut c_void) -> u8>,
  ) -> *mut __SKSearchResults;
  pub fn SKSearchResultsGetCount(inSearchResults: *mut __SKSearchResults) -> isize;
  pub fn SKSearchResultsGetInfoInRange(
    inSearchResults: *mut __SKSearchResults,
    inRange: CFRange,
    outDocumentsArray: *mut *mut c_void,
    outIndexesArray: *mut *mut __SKIndex,
    outScoresArray: *mut f32,
  ) -> isize;
  pub fn SKSearchResultsCopyMatchingTerms(
    inSearchResults: *mut __SKSearchResults,
    inItem: isize,
  ) -> *mut __CFArray;
  pub fn SKSummaryGetTypeID() -> usize;
  pub fn SKSummaryCreateWithString(inString: *mut __CFString) -> *mut __SKSummary;
  pub fn SKSummaryGetSentenceCount(summary: *mut __SKSummary) -> isize;
  pub fn SKSummaryGetParagraphCount(summary: *mut __SKSummary) -> isize;
  pub fn SKSummaryCopySentenceAtIndex(summary: *mut __SKSummary, i: isize) -> *mut __CFString;
  pub fn SKSummaryCopyParagraphAtIndex(summary: *mut __SKSummary, i: isize) -> *mut __CFString;
  pub fn SKSummaryCopySentenceSummaryString(
    summary: *mut __SKSummary,
    numSentences: isize,
  ) -> *mut __CFString;
  pub fn SKSummaryCopyParagraphSummaryString(
    summary: *mut __SKSummary,
    numParagraphs: isize,
  ) -> *mut __CFString;
  pub fn SKSummaryGetSentenceSummaryInfo(
    summary: *mut __SKSummary,
    numSentencesInSummary: isize,
    outRankOrderOfSentences: *mut isize,
    outSentenceIndexOfSentences: *mut isize,
    outParagraphIndexOfSentences: *mut isize,
  ) -> isize;
  pub fn SKSummaryGetParagraphSummaryInfo(
    summary: *mut __SKSummary,
    numParagraphsInSummary: isize,
    outRankOrderOfParagraphs: *mut isize,
    outParagraphIndexOfParagraphs: *mut isize,
  ) -> isize;
}
