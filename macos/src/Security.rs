use crate::message::audit_token_t;
#[allow(unused_imports)]
use crate::objc::*;
use crate::CoreFoundation::__CFAllocator;
use crate::CoreFoundation::__CFArray;
use crate::CoreFoundation::__CFBundle;
use crate::CoreFoundation::__CFData;
use crate::CoreFoundation::__CFDate;
use crate::CoreFoundation::__CFDictionary;
use crate::CoreFoundation::__CFError;
use crate::CoreFoundation::__CFReadStream;
use crate::CoreFoundation::__CFString;
use crate::CoreFoundation::__CFURL;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
use crate::stdio::FILE;
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
pub struct OpaqueSecCertificateRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecIdentityRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecKeyRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecPolicyRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecAccessControlRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecKeychainRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecKeychainItemRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecKeychainSearchRef {
  opaque: u32,
}
pub type SecKeychainAttrType = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttribute {
  pub tag: u32,
  pub length: u32,
  pub data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttributeList {
  pub count: u32,
  pub attr: *mut SecKeychainAttribute,
}
pub type SecKeychainStatus = u32;
#[repr(C)]
pub struct OpaqueSecTrustedApplicationRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecAccessRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecTrustRef {
  opaque: u32,
}
#[repr(C)]
pub struct OpaqueSecPasswordRef {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainAttributeInfo {
  pub count: u32,
  pub tag: *mut u32,
  pub format: *mut u32,
}
pub type sint64 = i64;
pub type uint64 = u64;
pub type sint32 = i32;
pub type sint16 = i16;
pub type sint8 = i8;
pub type uint32 = u32;
pub type uint16 = u16;
pub type uint8 = u8;
pub type CSSM_INTPTR = isize;
pub type CSSM_SIZE = usize;
pub type CSSM_HANDLE = isize;
pub type CSSM_LONG_HANDLE = u64;
pub type CSSM_MODULE_HANDLE = isize;
pub type CSSM_CC_HANDLE = u64;
pub type CSSM_CSP_HANDLE = isize;
pub type CSSM_TP_HANDLE = isize;
pub type CSSM_AC_HANDLE = isize;
pub type CSSM_CL_HANDLE = isize;
pub type CSSM_DL_HANDLE = isize;
pub type CSSM_DB_HANDLE = isize;
pub type CSSM_BOOL = i32;
pub type CSSM_RETURN = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_data {
  pub Length: usize,
  pub Data: *mut u8,
}
pub type CSSM_DATA = cssm_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_guid {
  pub Data1: u32,
  pub Data2: u16,
  pub Data3: u16,
  pub Data4: [u8; 8],
}
pub type CSSM_GUID = cssm_guid;
pub type CSSM_BITMASK = u32;
pub type CSSM_KEY_HIERARCHY = u32;
pub type CSSM_PVC_MODE = u32;
pub type CSSM_PRIVILEGE_SCOPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_version {
  pub Major: u32,
  pub Minor: u32,
}
pub type CSSM_VERSION = cssm_version;
pub type CSSM_SERVICE_MASK = u32;
pub type CSSM_SERVICE_TYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_subservice_uid {
  pub Guid: CSSM_GUID,
  pub Version: CSSM_VERSION,
  pub SubserviceId: u32,
  pub SubserviceType: u32,
}
pub type CSSM_SUBSERVICE_UID = cssm_subservice_uid;
pub type CSSM_MODULE_EVENT = u32;
pub type CSSM_ATTACH_FLAGS = u32;
pub type CSSM_PRIVILEGE = u64;
pub type CSSM_USEE_TAG = u64;
pub type CSSM_NET_ADDRESS_TYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_net_address {
  pub AddressType: u32,
  pub Address: CSSM_DATA,
}
pub type CSSM_NET_ADDRESS = cssm_net_address;
pub type CSSM_NET_PROTOCOL = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_crypto_data {
  pub Param: CSSM_DATA,
  pub Callback: Option<extern "C" fn(*mut cssm_data, *mut c_void) -> i32>,
  pub CallerCtx: *mut c_void,
}
pub type CSSM_CRYPTO_DATA = cssm_crypto_data;
pub type CSSM_WORDID_TYPE = i32;
pub type CSSM_LIST_ELEMENT_TYPE = u32;
pub type CSSM_LIST_TYPE = u32;
#[repr(C)]
pub struct cssm_list_element {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_list {
  pub ListType: u32,
  pub Head: *mut cssm_list_element,
  pub Tail: *mut cssm_list_element,
}
pub type CSSM_LIST = cssm_list;
pub type CSSM_LIST_ELEMENT = cssm_list_element;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_TUPLE {
  pub Issuer: CSSM_LIST,
  pub Subject: CSSM_LIST,
  pub Delegate: i32,
  pub AuthorizationTag: CSSM_LIST,
  pub ValidityPeriod: CSSM_LIST,
}
pub type CSSM_TUPLE_PTR = *mut CSSM_TUPLE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tuplegroup {
  pub NumberOfTuples: u32,
  pub Tuples: CSSM_TUPLE_PTR,
}
pub type CSSM_TUPLEGROUP = cssm_tuplegroup;
pub type CSSM_SAMPLE_TYPE = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_sample {
  pub TypedSample: CSSM_LIST,
  pub Verifier: *mut CSSM_SUBSERVICE_UID,
}
pub type CSSM_SAMPLE = cssm_sample;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_samplegroup {
  pub NumberOfSamples: u32,
  pub Samples: *mut CSSM_SAMPLE,
}
pub type CSSM_SAMPLEGROUP = cssm_samplegroup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_memory_funcs {
  pub malloc_func: Option<extern "C" fn(usize, *mut c_void) -> *mut c_void>,
  pub free_func: Option<extern "C" fn(*mut c_void, *mut c_void) -> ()>,
  pub realloc_func: Option<extern "C" fn(*mut c_void, usize, *mut c_void) -> *mut c_void>,
  pub calloc_func: Option<extern "C" fn(u32, usize, *mut c_void) -> *mut c_void>,
  pub AllocRef: *mut c_void,
}
pub type CSSM_MEMORY_FUNCS = cssm_memory_funcs;
pub type CSSM_API_MEMORY_FUNCS = CSSM_MEMORY_FUNCS;
pub type CSSM_CERT_TYPE = u32;
pub type CSSM_CERT_ENCODING = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_encoded_cert {
  pub CertType: u32,
  pub CertEncoding: u32,
  pub CertBlob: CSSM_DATA,
}
pub type CSSM_ENCODED_CERT = cssm_encoded_cert;
pub type CSSM_CERT_PARSE_FORMAT = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_parsed_cert {
  pub CertType: u32,
  pub ParsedCertFormat: u32,
  pub ParsedCert: *mut c_void,
}
pub type CSSM_PARSED_CERT = cssm_parsed_cert;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_cert_pair {
  pub EncodedCert: CSSM_ENCODED_CERT,
  pub ParsedCert: CSSM_PARSED_CERT,
}
pub type CSSM_CERT_PAIR = cssm_cert_pair;
pub type CSSM_CERTGROUP_TYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_certgroup {
  pub CertType: u32,
  pub CertEncoding: u32,
  pub NumCerts: u32,
  pub CertGroupType: u32,
  pub Reserved: *mut c_void,
}
pub type CSSM_CERTGROUP = cssm_certgroup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_base_certs {
  pub TPHandle: isize,
  pub CLHandle: isize,
  pub Certs: CSSM_CERTGROUP,
}
pub type CSSM_BASE_CERTS = cssm_base_certs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_access_credentials {
  pub EntryTag: [i8; 68],
  pub BaseCerts: CSSM_BASE_CERTS,
  pub Samples: CSSM_SAMPLEGROUP,
  pub Callback: Option<
    extern "C" fn(
      *mut CSSM_LIST,
      *mut cssm_samplegroup,
      *mut c_void,
      *mut CSSM_MEMORY_FUNCS,
    ) -> i32,
  >,
  pub CallerCtx: *mut c_void,
}
pub type CSSM_ACCESS_CREDENTIALS = cssm_access_credentials;
pub type CSSM_ACL_SUBJECT_TYPE = i32;
pub type CSSM_ACL_AUTHORIZATION_TAG = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_authorizationgroup {
  pub NumberOfAuthTags: u32,
  pub AuthTags: *mut i32,
}
pub type CSSM_AUTHORIZATIONGROUP = cssm_authorizationgroup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_validity_period {
  pub StartDate: CSSM_DATA,
  pub EndDate: CSSM_DATA,
}
pub type CSSM_ACL_VALIDITY_PERIOD = cssm_acl_validity_period;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_entry_prototype {
  pub TypedSubject: CSSM_LIST,
  pub Delegate: i32,
  pub Authorization: CSSM_AUTHORIZATIONGROUP,
  pub TimeRange: CSSM_ACL_VALIDITY_PERIOD,
  pub EntryTag: [i8; 68],
}
pub type CSSM_ACL_ENTRY_PROTOTYPE = cssm_acl_entry_prototype;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_owner_prototype {
  pub TypedSubject: CSSM_LIST,
  pub Delegate: i32,
}
pub type CSSM_ACL_OWNER_PROTOTYPE = cssm_acl_owner_prototype;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_entry_input {
  pub Prototype: CSSM_ACL_ENTRY_PROTOTYPE,
  pub Callback: Option<
    extern "C" fn(*mut CSSM_LIST, *mut cssm_list, *mut c_void, *mut CSSM_MEMORY_FUNCS) -> i32,
  >,
  pub CallerContext: *mut c_void,
}
pub type CSSM_ACL_ENTRY_INPUT = cssm_acl_entry_input;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_resource_control_context {
  pub AccessCred: *mut cssm_access_credentials,
  pub InitialAclEntry: CSSM_ACL_ENTRY_INPUT,
}
pub type CSSM_RESOURCE_CONTROL_CONTEXT = cssm_resource_control_context;
pub type CSSM_ACL_HANDLE = isize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_entry_info {
  pub EntryPublicInfo: CSSM_ACL_ENTRY_PROTOTYPE,
  pub EntryHandle: isize,
}
pub type CSSM_ACL_ENTRY_INFO = cssm_acl_entry_info;
pub type CSSM_ACL_EDIT_MODE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_edit {
  pub EditMode: u32,
  pub OldEntryHandle: isize,
  pub NewEntry: *mut CSSM_ACL_ENTRY_INPUT,
}
pub type CSSM_ACL_EDIT = cssm_acl_edit;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_func_name_addr {
  pub Name: [i8; 68],
  pub Address: Option<extern "C" fn() -> ()>,
}
pub type CSSM_FUNC_NAME_ADDR = cssm_func_name_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_date {
  pub Year: [u8; 4],
  pub Month: [u8; 2],
  pub Day: [u8; 2],
}
pub type CSSM_DATE = cssm_date;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_range {
  pub Min: u32,
  pub Max: u32,
}
pub type CSSM_RANGE = cssm_range;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_query_size_data {
  pub SizeInputBlock: u32,
  pub SizeOutputBlock: u32,
}
pub type CSSM_QUERY_SIZE_DATA = cssm_query_size_data;
pub type CSSM_HEADERVERSION = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_key_size {
  pub LogicalKeySizeInBits: u32,
  pub EffectiveKeySizeInBits: u32,
}
pub type CSSM_KEY_SIZE = cssm_key_size;
pub type CSSM_KEYBLOB_TYPE = u32;
pub type CSSM_KEYBLOB_FORMAT = u32;
pub type CSSM_KEYCLASS = u32;
pub type CSSM_KEYATTR_FLAGS = u32;
pub type CSSM_KEYUSE = u32;
pub type CSSM_ALGORITHMS = u32;
pub type CSSM_ENCRYPT_MODE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_keyheader {
  pub HeaderVersion: u32,
  pub CspId: CSSM_GUID,
  pub BlobType: u32,
  pub Format: u32,
  pub AlgorithmId: u32,
  pub KeyClass: u32,
  pub LogicalKeySizeInBits: u32,
  pub KeyAttr: u32,
  pub KeyUsage: u32,
  pub StartDate: CSSM_DATE,
  pub EndDate: CSSM_DATE,
  pub WrapAlgorithmId: u32,
  pub WrapMode: u32,
  pub Reserved: u32,
}
pub type CSSM_KEYHEADER = cssm_keyheader;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_key {
  pub KeyHeader: CSSM_KEYHEADER,
  pub KeyData: CSSM_DATA,
}
pub type CSSM_KEY = cssm_key;
pub type CSSM_WRAP_KEY = CSSM_KEY;
pub type CSSM_CSPTYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_dl_db_handle {
  pub DLHandle: isize,
  pub DBHandle: isize,
}
pub type CSSM_DL_DB_HANDLE = cssm_dl_db_handle;
pub type CSSM_CONTEXT_TYPE = u32;
pub type CSSM_ATTRIBUTE_TYPE = u32;
pub type CSSM_PADDING = u32;
pub type CSSM_KEY_TYPE = u32;
#[repr(C)]
pub struct cssm_kr_profile {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_context_attribute_value {
  pub String: *mut i8,
  pub Uint32: u32,
  pub AccessCredentials: *mut cssm_access_credentials,
  pub Key: *mut cssm_key,
  pub Data: *mut cssm_data,
  pub Padding: u32,
  pub Date: *mut cssm_date,
  pub Range: *mut cssm_range,
  pub CryptoData: *mut cssm_crypto_data,
  pub Version: *mut cssm_version,
  pub DLDBHandle: *mut cssm_dl_db_handle,
  pub KRProfile: *mut cssm_kr_profile,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_context_attribute {
  pub AttributeType: u32,
  pub AttributeLength: u32,
  pub Attribute: cssm_context_attribute_value,
}
pub type CSSM_CONTEXT_ATTRIBUTE = cssm_context_attribute;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_context {
  pub ContextType: u32,
  pub AlgorithmType: u32,
  pub NumberOfAttributes: u32,
  pub ContextAttributes: *mut cssm_context_attribute,
  pub CSPHandle: isize,
  pub Privileged: i32,
  pub EncryptionProhibited: u32,
  pub WorkFactor: u32,
  pub Reserved: u32,
}
pub type CSSM_CONTEXT = cssm_context;
pub type CSSM_SC_FLAGS = u32;
pub type CSSM_CSP_READER_FLAGS = u32;
pub type CSSM_CSP_FLAGS = u32;
pub type CSSM_PKCS_OAEP_MGF = u32;
pub type CSSM_PKCS_OAEP_PSOURCE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_pkcs1_oaep_params {
  pub HashAlgorithm: u32,
  pub HashParams: CSSM_DATA,
  pub MGF: u32,
  pub MGFParams: CSSM_DATA,
  pub PSource: u32,
  pub PSourceParams: CSSM_DATA,
}
pub type CSSM_PKCS1_OAEP_PARAMS = cssm_pkcs1_oaep_params;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_csp_operational_statistics {
  pub UserAuthenticated: i32,
  pub DeviceFlags: u32,
  pub TokenMaxSessionCount: u32,
  pub TokenOpenedSessionCount: u32,
  pub TokenMaxRWSessionCount: u32,
  pub TokenOpenedRWSessionCount: u32,
  pub TokenTotalPublicMem: u32,
  pub TokenFreePublicMem: u32,
  pub TokenTotalPrivateMem: u32,
  pub TokenFreePrivateMem: u32,
}
pub type CSSM_CSP_OPERATIONAL_STATISTICS = cssm_csp_operational_statistics;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_pkcs5_pbkdf1_params {
  pub Passphrase: CSSM_DATA,
  pub InitVector: CSSM_DATA,
}
pub type CSSM_PKCS5_PBKDF1_PARAMS = cssm_pkcs5_pbkdf1_params;
pub type CSSM_PKCS5_PBKDF2_PRF = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_pkcs5_pbkdf2_params {
  pub Passphrase: CSSM_DATA,
  pub PseudoRandomFunction: u32,
}
pub type CSSM_PKCS5_PBKDF2_PARAMS = cssm_pkcs5_pbkdf2_params;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_kea_derive_params {
  pub Rb: CSSM_DATA,
  pub Yb: CSSM_DATA,
}
pub type CSSM_KEA_DERIVE_PARAMS = cssm_kea_derive_params;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_authority_id {
  pub AuthorityCert: *mut CSSM_DATA,
  pub AuthorityLocation: *mut cssm_net_address,
}
pub type CSSM_TP_AUTHORITY_ID = cssm_tp_authority_id;
pub type CSSM_TP_AUTHORITY_REQUEST_TYPE = u32;
pub type CSSM_OID = CSSM_DATA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_field {
  pub FieldOid: CSSM_DATA,
  pub FieldValue: CSSM_DATA,
}
pub type CSSM_FIELD = cssm_field;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_policyinfo {
  pub NumberOfPolicyIds: u32,
  pub PolicyIds: *mut cssm_field,
  pub PolicyControl: *mut c_void,
}
pub type CSSM_TP_POLICYINFO = cssm_tp_policyinfo;
pub type CSSM_TP_SERVICES = u32;
pub type CSSM_TP_ACTION = u32;
pub type CSSM_TP_STOP_ON = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_dl_db_list {
  pub NumHandles: u32,
  pub DLDBHandle: *mut cssm_dl_db_handle,
}
pub type CSSM_DL_DB_LIST = cssm_dl_db_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_callerauth_context {
  pub Policy: CSSM_TP_POLICYINFO,
  pub VerifyTime: *mut i8,
  pub VerificationAbortOn: u32,
  pub CallbackWithVerifiedCert: Option<extern "C" fn(isize, *mut c_void, *mut cssm_data) -> i32>,
  pub NumberOfAnchorCerts: u32,
  pub AnchorCerts: *mut cssm_data,
  pub DBList: *mut cssm_dl_db_list,
  pub CallerCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CALLERAUTH_CONTEXT = cssm_tp_callerauth_context;
pub type CSSM_CRL_PARSE_FORMAT = u32;
pub type CSSM_CRL_TYPE = u32;
pub type CSSM_CRL_ENCODING = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_encoded_crl {
  pub CrlType: u32,
  pub CrlEncoding: u32,
  pub CrlBlob: CSSM_DATA,
}
pub type CSSM_ENCODED_CRL = cssm_encoded_crl;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_parsed_crl {
  pub CrlType: u32,
  pub ParsedCrlFormat: u32,
  pub ParsedCrl: *mut c_void,
}
pub type CSSM_PARSED_CRL = cssm_parsed_crl;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_crl_pair {
  pub EncodedCrl: CSSM_ENCODED_CRL,
  pub ParsedCrl: CSSM_PARSED_CRL,
}
pub type CSSM_CRL_PAIR = cssm_crl_pair;
pub type CSSM_CRLGROUP_TYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_crlgroup {
  pub CrlType: u32,
  pub CrlEncoding: u32,
  pub NumberOfCrls: u32,
  pub CrlGroupType: u32,
}
pub type CSSM_CRLGROUP = cssm_crlgroup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_fieldgroup {
  pub NumberOfFields: i32,
  pub Fields: *mut cssm_field,
}
pub type CSSM_FIELDGROUP = cssm_fieldgroup;
pub type CSSM_EVIDENCE_FORM = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_evidence {
  pub EvidenceForm: u32,
  pub Evidence: *mut c_void,
}
pub type CSSM_EVIDENCE = cssm_evidence;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_verify_context {
  pub Action: u32,
  pub ActionData: CSSM_DATA,
  pub Crls: CSSM_CRLGROUP,
  pub Cred: *mut cssm_tp_callerauth_context,
}
pub type CSSM_TP_VERIFY_CONTEXT = cssm_tp_verify_context;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_verify_context_result {
  pub NumberOfEvidences: u32,
  pub Evidence: *mut cssm_evidence,
}
pub type CSSM_TP_VERIFY_CONTEXT_RESULT = cssm_tp_verify_context_result;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_request_set {
  pub NumberOfRequests: u32,
  pub Requests: *mut c_void,
}
pub type CSSM_TP_REQUEST_SET = cssm_tp_request_set;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_result_set {
  pub NumberOfResults: u32,
  pub Results: *mut c_void,
}
pub type CSSM_TP_RESULT_SET = cssm_tp_result_set;
pub type CSSM_TP_CONFIRM_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_confirm_response {
  pub NumberOfResponses: u32,
  pub Responses: *mut u32,
}
pub type CSSM_TP_CONFIRM_RESPONSE = cssm_tp_confirm_response;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certissue_input {
  pub CSPSubserviceUid: CSSM_SUBSERVICE_UID,
  pub CLHandle: isize,
  pub NumberOfTemplateFields: u32,
  pub SubjectCertFields: *mut cssm_field,
  pub MoreServiceRequests: u32,
  pub NumberOfServiceControls: u32,
  pub ServiceControls: *mut cssm_field,
  pub UserCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CERTISSUE_INPUT = cssm_tp_certissue_input;
pub type CSSM_TP_CERTISSUE_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certissue_output {
  pub IssueStatus: u32,
  pub CertGroup: *mut cssm_certgroup,
  pub PerformedServiceRequests: u32,
}
pub type CSSM_TP_CERTISSUE_OUTPUT = cssm_tp_certissue_output;
pub type CSSM_TP_CERTCHANGE_ACTION = u32;
pub type CSSM_TP_CERTCHANGE_REASON = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certchange_input {
  pub Action: u32,
  pub Reason: u32,
  pub CLHandle: isize,
  pub Cert: *mut cssm_data,
  pub ChangeInfo: *mut cssm_field,
  pub StartTime: *mut i8,
  pub CallerCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CERTCHANGE_INPUT = cssm_tp_certchange_input;
pub type CSSM_TP_CERTCHANGE_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certchange_output {
  pub ActionStatus: u32,
  pub RevokeInfo: CSSM_FIELD,
}
pub type CSSM_TP_CERTCHANGE_OUTPUT = cssm_tp_certchange_output;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certverify_input {
  pub CLHandle: isize,
  pub Cert: *mut cssm_data,
  pub VerifyContext: *mut cssm_tp_verify_context,
}
pub type CSSM_TP_CERTVERIFY_INPUT = cssm_tp_certverify_input;
pub type CSSM_TP_CERTVERIFY_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certverify_output {
  pub VerifyStatus: u32,
  pub NumberOfEvidence: u32,
  pub Evidence: *mut cssm_evidence,
}
pub type CSSM_TP_CERTVERIFY_OUTPUT = cssm_tp_certverify_output;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certnotarize_input {
  pub CLHandle: isize,
  pub NumberOfFields: u32,
  pub MoreFields: *mut cssm_field,
  pub SignScope: *mut cssm_field,
  pub ScopeSize: u32,
  pub MoreServiceRequests: u32,
  pub NumberOfServiceControls: u32,
  pub ServiceControls: *mut cssm_field,
  pub UserCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CERTNOTARIZE_INPUT = cssm_tp_certnotarize_input;
pub type CSSM_TP_CERTNOTARIZE_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certnotarize_output {
  pub NotarizeStatus: u32,
  pub NotarizedCertGroup: *mut cssm_certgroup,
  pub PerformedServiceRequests: u32,
}
pub type CSSM_TP_CERTNOTARIZE_OUTPUT = cssm_tp_certnotarize_output;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certreclaim_input {
  pub CLHandle: isize,
  pub NumberOfSelectionFields: u32,
  pub SelectionFields: *mut cssm_field,
  pub UserCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CERTRECLAIM_INPUT = cssm_tp_certreclaim_input;
pub type CSSM_TP_CERTRECLAIM_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_certreclaim_output {
  pub ReclaimStatus: u32,
  pub ReclaimedCertGroup: *mut cssm_certgroup,
  pub KeyCacheHandle: u64,
}
pub type CSSM_TP_CERTRECLAIM_OUTPUT = cssm_tp_certreclaim_output;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_crlissue_input {
  pub CLHandle: isize,
  pub CrlIdentifier: u32,
  pub CrlThisTime: *mut i8,
  pub PolicyIdentifier: *mut cssm_field,
  pub CallerCredentials: *mut cssm_access_credentials,
}
pub type CSSM_TP_CRLISSUE_INPUT = cssm_tp_crlissue_input;
pub type CSSM_TP_CRLISSUE_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_tp_crlissue_output {
  pub IssueStatus: u32,
  pub Crl: *mut cssm_encoded_crl,
  pub CrlNextTime: *mut i8,
}
pub type CSSM_TP_CRLISSUE_OUTPUT = cssm_tp_crlissue_output;
pub type CSSM_TP_FORM_TYPE = u32;
pub type CSSM_CL_TEMPLATE_TYPE = u32;
pub type CSSM_CERT_BUNDLE_TYPE = u32;
pub type CSSM_CERT_BUNDLE_ENCODING = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_cert_bundle_header {
  pub BundleType: u32,
  pub BundleEncoding: u32,
}
pub type CSSM_CERT_BUNDLE_HEADER = cssm_cert_bundle_header;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_cert_bundle {
  pub BundleHeader: CSSM_CERT_BUNDLE_HEADER,
  pub Bundle: CSSM_DATA,
}
pub type CSSM_CERT_BUNDLE = cssm_cert_bundle;
pub type CSSM_DB_ATTRIBUTE_NAME_FORMAT = u32;
pub type CSSM_DB_ATTRIBUTE_FORMAT = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_db_attribute_label {
  pub AttributeName: *mut i8,
  pub AttributeOID: CSSM_DATA,
  pub AttributeID: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_attribute_info {
  pub AttributeNameFormat: u32,
  pub Label: cssm_db_attribute_label,
  pub AttributeFormat: u32,
}
pub type CSSM_DB_ATTRIBUTE_INFO = cssm_db_attribute_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_attribute_data {
  pub Info: CSSM_DB_ATTRIBUTE_INFO,
  pub NumberOfValues: u32,
  pub Value: *mut cssm_data,
}
pub type CSSM_DB_ATTRIBUTE_DATA = cssm_db_attribute_data;
pub type CSSM_DB_RECORDTYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_record_attribute_info {
  pub DataRecordType: u32,
  pub NumberOfAttributes: u32,
  pub AttributeInfo: *mut cssm_db_attribute_info,
}
pub type CSSM_DB_RECORD_ATTRIBUTE_INFO = cssm_db_record_attribute_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_record_attribute_data {
  pub DataRecordType: u32,
  pub SemanticInformation: u32,
  pub NumberOfAttributes: u32,
  pub AttributeData: *mut cssm_db_attribute_data,
}
pub type CSSM_DB_RECORD_ATTRIBUTE_DATA = cssm_db_record_attribute_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_parsing_module_info {
  pub RecordType: u32,
  pub ModuleSubserviceUid: CSSM_SUBSERVICE_UID,
}
pub type CSSM_DB_PARSING_MODULE_INFO = cssm_db_parsing_module_info;
pub type CSSM_DB_INDEX_TYPE = u32;
pub type CSSM_DB_INDEXED_DATA_LOCATION = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_index_info {
  pub IndexType: u32,
  pub IndexedDataLocation: u32,
  pub Info: CSSM_DB_ATTRIBUTE_INFO,
}
pub type CSSM_DB_INDEX_INFO = cssm_db_index_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_unique_record {
  pub RecordLocator: CSSM_DB_INDEX_INFO,
  pub RecordIdentifier: CSSM_DATA,
}
pub type CSSM_DB_UNIQUE_RECORD = cssm_db_unique_record;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_record_index_info {
  pub DataRecordType: u32,
  pub NumberOfIndexes: u32,
  pub IndexInfo: *mut cssm_db_index_info,
}
pub type CSSM_DB_RECORD_INDEX_INFO = cssm_db_record_index_info;
pub type CSSM_DB_ACCESS_TYPE = u32;
pub type CSSM_DB_MODIFY_MODE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_dbinfo {
  pub NumberOfRecordTypes: u32,
  pub DefaultParsingModules: *mut cssm_db_parsing_module_info,
  pub RecordAttributeNames: *mut cssm_db_record_attribute_info,
  pub RecordIndexes: *mut cssm_db_record_index_info,
  pub IsLocal: i32,
  pub AccessPath: *mut i8,
  pub Reserved: *mut c_void,
}
pub type CSSM_DBINFO = cssm_dbinfo;
pub type CSSM_DB_OPERATOR = u32;
pub type CSSM_DB_CONJUNCTIVE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_selection_predicate {
  pub DbOperator: u32,
  pub Attribute: CSSM_DB_ATTRIBUTE_DATA,
}
pub type CSSM_SELECTION_PREDICATE = cssm_selection_predicate;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_query_limits {
  pub TimeLimit: u32,
  pub SizeLimit: u32,
}
pub type CSSM_QUERY_LIMITS = cssm_query_limits;
pub type CSSM_QUERY_FLAGS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_query {
  pub RecordType: u32,
  pub Conjunctive: u32,
  pub NumSelectionPredicates: u32,
  pub SelectionPredicate: *mut cssm_selection_predicate,
  pub QueryLimits: CSSM_QUERY_LIMITS,
  pub QueryFlags: u32,
}
pub type CSSM_QUERY = cssm_query;
pub type CSSM_DLTYPE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_dl_pkcs11_attributes {
  pub DeviceAccessFlags: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_name_list {
  pub NumStrings: u32,
  pub String: *mut *mut i8,
}
pub type CSSM_NAME_LIST = cssm_name_list;
pub type CSSM_DB_RETRIEVAL_MODES = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_schema_attribute_info {
  pub AttributeId: u32,
  pub AttributeName: *mut i8,
  pub AttributeNameID: CSSM_DATA,
  pub DataType: u32,
}
pub type CSSM_DB_SCHEMA_ATTRIBUTE_INFO = cssm_db_schema_attribute_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_db_schema_index_info {
  pub AttributeId: u32,
  pub IndexId: u32,
  pub IndexType: u32,
  pub IndexedDataLocation: u32,
}
pub type CSSM_DB_SCHEMA_INDEX_INFO = cssm_db_schema_index_info;
pub type CSSM_BER_TAG = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_algorithm_identifier {
  pub algorithm: CSSM_DATA,
  pub parameters: CSSM_DATA,
}
pub type CSSM_X509_ALGORITHM_IDENTIFIER = cssm_x509_algorithm_identifier;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_type_value_pair {
  pub type_: CSSM_DATA,
  pub valueType: u8,
  pub value: CSSM_DATA,
}
pub type CSSM_X509_TYPE_VALUE_PAIR = cssm_x509_type_value_pair;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_rdn {
  pub numberOfPairs: u32,
  pub AttributeTypeAndValue: *mut cssm_x509_type_value_pair,
}
pub type CSSM_X509_RDN = cssm_x509_rdn;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_name {
  pub numberOfRDNs: u32,
  pub RelativeDistinguishedName: *mut cssm_x509_rdn,
}
pub type CSSM_X509_NAME = cssm_x509_name;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_subject_public_key_info {
  pub algorithm: CSSM_X509_ALGORITHM_IDENTIFIER,
  pub subjectPublicKey: CSSM_DATA,
}
pub type CSSM_X509_SUBJECT_PUBLIC_KEY_INFO = cssm_x509_subject_public_key_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_time {
  pub timeType: u8,
  pub time: CSSM_DATA,
}
pub type CSSM_X509_TIME = cssm_x509_time;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x509_validity {
  pub notBefore: CSSM_X509_TIME,
  pub notAfter: CSSM_X509_TIME,
}
pub type CSSM_X509_VALIDITY = x509_validity;
pub type CSSM_X509_OPTION = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509ext_basicConstraints {
  pub cA: i32,
  pub pathLenConstraintPresent: i32,
  pub pathLenConstraint: u32,
}
pub type CSSM_X509EXT_BASICCONSTRAINTS = cssm_x509ext_basicConstraints;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CSSM_X509EXT_DATA_FORMAT {
  CSSM_X509_DATAFORMAT_ENCODED = 0,
  CSSM_X509_DATAFORMAT_PARSED = 1,
  CSSM_X509_DATAFORMAT_PAIR = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_extensionTagAndValue {
  pub type_: u8,
  pub value: CSSM_DATA,
}
pub type CSSM_X509EXT_TAGandVALUE = cssm_x509_extensionTagAndValue;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509ext_pair {
  pub tagAndValue: CSSM_X509EXT_TAGandVALUE,
  pub parsedValue: *mut c_void,
}
pub type CSSM_X509EXT_PAIR = cssm_x509ext_pair;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cssm_x509ext_value {
  pub tagAndValue: *mut CSSM_X509EXT_TAGandVALUE,
  pub parsedValue: *mut c_void,
  pub valuePair: *mut CSSM_X509EXT_PAIR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_extension {
  pub extnId: CSSM_DATA,
  pub critical: i32,
  pub format: CSSM_X509EXT_DATA_FORMAT,
  pub value: cssm_x509ext_value,
  pub BERvalue: CSSM_DATA,
}
pub type CSSM_X509_EXTENSION = cssm_x509_extension;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_extensions {
  pub numberOfExtensions: u32,
  pub extensions: *mut cssm_x509_extension,
}
pub type CSSM_X509_EXTENSIONS = cssm_x509_extensions;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_tbs_certificate {
  pub version: CSSM_DATA,
  pub serialNumber: CSSM_DATA,
  pub signature: CSSM_X509_ALGORITHM_IDENTIFIER,
  pub issuer: CSSM_X509_NAME,
  pub validity: CSSM_X509_VALIDITY,
  pub subject: CSSM_X509_NAME,
  pub subjectPublicKeyInfo: CSSM_X509_SUBJECT_PUBLIC_KEY_INFO,
  pub issuerUniqueIdentifier: CSSM_DATA,
  pub subjectUniqueIdentifier: CSSM_DATA,
  pub extensions: CSSM_X509_EXTENSIONS,
}
pub type CSSM_X509_TBS_CERTIFICATE = cssm_x509_tbs_certificate;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_signature {
  pub algorithmIdentifier: CSSM_X509_ALGORITHM_IDENTIFIER,
  pub encrypted: CSSM_DATA,
}
pub type CSSM_X509_SIGNATURE = cssm_x509_signature;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_signed_certificate {
  pub certificate: CSSM_X509_TBS_CERTIFICATE,
  pub signature: CSSM_X509_SIGNATURE,
}
pub type CSSM_X509_SIGNED_CERTIFICATE = cssm_x509_signed_certificate;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509ext_policyQualifierInfo {
  pub policyQualifierId: CSSM_DATA,
  pub value: CSSM_DATA,
}
pub type CSSM_X509EXT_POLICYQUALIFIERINFO = cssm_x509ext_policyQualifierInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509ext_policyQualifiers {
  pub numberOfPolicyQualifiers: u32,
  pub policyQualifier: *mut CSSM_X509EXT_POLICYQUALIFIERINFO,
}
pub type CSSM_X509EXT_POLICYQUALIFIERS = cssm_x509ext_policyQualifiers;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509ext_policyInfo {
  pub policyIdentifier: CSSM_DATA,
  pub policyQualifiers: CSSM_X509EXT_POLICYQUALIFIERS,
}
pub type CSSM_X509EXT_POLICYINFO = cssm_x509ext_policyInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_revoked_cert_entry {
  pub certificateSerialNumber: CSSM_DATA,
  pub revocationDate: CSSM_X509_TIME,
  pub extensions: CSSM_X509_EXTENSIONS,
}
pub type CSSM_X509_REVOKED_CERT_ENTRY = cssm_x509_revoked_cert_entry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_revoked_cert_list {
  pub numberOfRevokedCertEntries: u32,
  pub revokedCertEntry: *mut cssm_x509_revoked_cert_entry,
}
pub type CSSM_X509_REVOKED_CERT_LIST = cssm_x509_revoked_cert_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_tbs_certlist {
  pub version: CSSM_DATA,
  pub signature: CSSM_X509_ALGORITHM_IDENTIFIER,
  pub issuer: CSSM_X509_NAME,
  pub thisUpdate: CSSM_X509_TIME,
  pub nextUpdate: CSSM_X509_TIME,
  pub revokedCertificates: *mut cssm_x509_revoked_cert_list,
  pub extensions: CSSM_X509_EXTENSIONS,
}
pub type CSSM_X509_TBS_CERTLIST = cssm_x509_tbs_certlist;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_x509_signed_crl {
  pub tbsCertList: CSSM_X509_TBS_CERTLIST,
  pub signature: CSSM_X509_SIGNATURE,
}
pub type CSSM_X509_SIGNED_CRL = cssm_x509_signed_crl;
bitflags! { # [ repr ( C ) ] pub struct SecKeyUsage : u32 { const kSecKeyUsageUnspecified = 0 ; const kSecKeyUsageDigitalSignature = 1 ; const kSecKeyUsageNonRepudiation = 2 ; const kSecKeyUsageKeyEncipherment = 4 ; const kSecKeyUsageDataEncipherment = 8 ; const kSecKeyUsageKeyAgreement = 16 ; const kSecKeyUsageKeyCertSign = 32 ; const kSecKeyUsageCRLSign = 64 ; const kSecKeyUsageEncipherOnly = 128 ; const kSecKeyUsageDecipherOnly = 256 ; const kSecKeyUsageCritical = 2147483648 ; const kSecKeyUsageAll = 2147483647 ; } }
bitflags! { # [ repr ( C ) ] pub struct SecAccessControlCreateFlags : usize { const kSecAccessControlUserPresence = 1 ; const kSecAccessControlBiometryAny = 2 ; const kSecAccessControlBiometryCurrentSet = 8 ; const kSecAccessControlDevicePasscode = 16 ; const kSecAccessControlOr = 16384 ; const kSecAccessControlAnd = 32768 ; const kSecAccessControlPrivateKeyUsage = 1073741824 ; const kSecAccessControlApplicationPassword = 2147483648 ; } }
pub type SecAccessOwnerType = u32;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecCredentialType {
  kSecCredentialTypeDefault = 0,
  kSecCredentialTypeWithUI = 1,
  kSecCredentialTypeNoUI = 2,
}
bitflags! { # [ repr ( C ) ] pub struct SecPadding : u32 { const kSecPaddingNone = 0 ; const kSecPaddingPKCS1 = 1 ; const kSecPaddingOAEP = 2 ; const kSecPaddingSigRaw = 16384 ; const kSecPaddingPKCS1MD2 = 32768 ; const kSecPaddingPKCS1MD5 = 32769 ; const kSecPaddingPKCS1SHA1 = 32770 ; const kSecPaddingPKCS1SHA224 = 32771 ; const kSecPaddingPKCS1SHA256 = 32772 ; const kSecPaddingPKCS1SHA384 = 32773 ; const kSecPaddingPKCS1SHA512 = 32774 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecKeySizes {
  kSecDefaultKeySize = 0,
  kSec3DES192 = 192,
  kSecAES128 = 128,
  kSecAES256 = 256,
  kSecp384r1 = 384,
  kSecp521r1 = 521,
  kSecRSAMin = 1024,
  kSecRSAMax = 4096,
}
pub type SecKeyAlgorithm = *mut __CFString;
pub type SecKeyKeyExchangeParameter = *mut __CFString;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum SecKeyOperationType {
  kSecKeyOperationTypeSign = 0,
  kSecKeyOperationTypeVerify = 1,
  kSecKeyOperationTypeEncrypt = 2,
  kSecKeyOperationTypeDecrypt = 3,
  kSecKeyOperationTypeKeyExchange = 4,
}
#[repr(C)]
pub struct __SecRandom {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CE_GeneralNameType {
  GNT_OtherName = 0,
  GNT_RFC822Name = 1,
  GNT_DNSName = 2,
  GNT_X400Address = 3,
  GNT_DirectoryName = 4,
  GNT_EdiPartyName = 5,
  GNT_URI = 6,
  GNT_IPAddress = 7,
  GNT_RegisteredID = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_OtherName {
  pub typeId: CSSM_DATA,
  pub value: CSSM_DATA,
}
pub type CE_OtherName = __CE_OtherName;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_GeneralName {
  pub nameType: CE_GeneralNameType,
  pub berEncoded: i32,
  pub name: CSSM_DATA,
}
pub type CE_GeneralName = __CE_GeneralName;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_GeneralNames {
  pub numNames: u32,
  pub generalName: *mut CE_GeneralName,
}
pub type CE_GeneralNames = __CE_GeneralNames;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_AuthorityKeyID {
  pub keyIdentifierPresent: i32,
  pub keyIdentifier: CSSM_DATA,
  pub generalNamesPresent: i32,
  pub generalNames: *mut CE_GeneralNames,
  pub serialNumberPresent: i32,
  pub serialNumber: CSSM_DATA,
}
pub type CE_AuthorityKeyID = __CE_AuthorityKeyID;
pub type CE_SubjectKeyID = CSSM_DATA;
pub type CE_KeyUsage = u16;
pub type CE_CrlReason = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_ExtendedKeyUsage {
  pub numPurposes: u32,
  pub purposes: *mut CSSM_DATA,
}
pub type CE_ExtendedKeyUsage = __CE_ExtendedKeyUsage;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_BasicConstraints {
  pub cA: i32,
  pub pathLenConstraintPresent: i32,
  pub pathLenConstraint: u32,
}
pub type CE_BasicConstraints = __CE_BasicConstraints;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_PolicyQualifierInfo {
  pub policyQualifierId: CSSM_DATA,
  pub qualifier: CSSM_DATA,
}
pub type CE_PolicyQualifierInfo = __CE_PolicyQualifierInfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_PolicyInformation {
  pub certPolicyId: CSSM_DATA,
  pub numPolicyQualifiers: u32,
  pub policyQualifiers: *mut CE_PolicyQualifierInfo,
}
pub type CE_PolicyInformation = __CE_PolicyInformation;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_CertPolicies {
  pub numPolicies: u32,
  pub policies: *mut CE_PolicyInformation,
}
pub type CE_CertPolicies = __CE_CertPolicies;
pub type CE_NetscapeCertType = u16;
pub type CE_CrlDistReasonFlags = u8;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CE_CrlDistributionPointNameType {
  CE_CDNT_FullName = 0,
  CE_CDNT_NameRelativeToCrlIssuer = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_DistributionPointName {
  pub nameType: CE_CrlDistributionPointNameType,
}
pub type CE_DistributionPointName = __CE_DistributionPointName;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_CRLDistributionPoint {
  pub distPointName: *mut CE_DistributionPointName,
  pub reasonsPresent: i32,
  pub reasons: u8,
  pub crlIssuer: *mut CE_GeneralNames,
}
pub type CE_CRLDistributionPoint = __CE_CRLDistributionPoint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_CRLDistPointsSyntax {
  pub numDistPoints: u32,
  pub distPoints: *mut CE_CRLDistributionPoint,
}
pub type CE_CRLDistPointsSyntax = __CE_CRLDistPointsSyntax;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_AccessDescription {
  pub accessMethod: CSSM_DATA,
  pub accessLocation: CE_GeneralName,
}
pub type CE_AccessDescription = __CE_AccessDescription;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_AuthorityInfoAccess {
  pub numAccessDescriptions: u32,
  pub accessDescriptions: *mut CE_AccessDescription,
}
pub type CE_AuthorityInfoAccess = __CE_AuthorityInfoAccess;
pub type CE_NameRegistrationAuthorities = CE_GeneralNames;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_SemanticsInformation {
  pub semanticsIdentifier: *mut CSSM_DATA,
  pub nameRegistrationAuthorities: *mut CE_GeneralNames,
}
pub type CE_SemanticsInformation = __CE_SemanticsInformation;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_QC_Statement {
  pub statementId: CSSM_DATA,
  pub semanticsInfo: *mut CE_SemanticsInformation,
  pub otherInfo: *mut CSSM_DATA,
}
pub type CE_QC_Statement = __CE_QC_Statement;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_QC_Statements {
  pub numQCStatements: u32,
  pub qcStatements: *mut CE_QC_Statement,
}
pub type CE_QC_Statements = __CE_QC_Statements;
pub type CE_CrlNumber = u32;
pub type CE_DeltaCrl = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_IssuingDistributionPoint {
  pub distPointName: *mut CE_DistributionPointName,
  pub onlyUserCertsPresent: i32,
  pub onlyUserCerts: i32,
  pub onlyCACertsPresent: i32,
  pub onlyCACerts: i32,
  pub onlySomeReasonsPresent: i32,
  pub onlySomeReasons: u8,
  pub indirectCrlPresent: i32,
  pub indirectCrl: i32,
}
pub type CE_IssuingDistributionPoint = __CE_IssuingDistributionPoint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_GeneralSubtree {
  pub base: *mut CE_GeneralNames,
  pub minimum: u32,
  pub maximumPresent: i32,
  pub maximum: u32,
}
pub type CE_GeneralSubtree = __CE_GeneralSubtree;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_GeneralSubtrees {
  pub numSubtrees: u32,
  pub subtrees: *mut CE_GeneralSubtree,
}
pub type CE_GeneralSubtrees = __CE_GeneralSubtrees;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_NameConstraints {
  pub permitted: *mut CE_GeneralSubtrees,
  pub excluded: *mut CE_GeneralSubtrees,
}
pub type CE_NameConstraints = __CE_NameConstraints;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_PolicyMapping {
  pub issuerDomainPolicy: CSSM_DATA,
  pub subjectDomainPolicy: CSSM_DATA,
}
pub type CE_PolicyMapping = __CE_PolicyMapping;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_PolicyMappings {
  pub numPolicyMappings: u32,
  pub policyMappings: *mut CE_PolicyMapping,
}
pub type CE_PolicyMappings = __CE_PolicyMappings;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_PolicyConstraints {
  pub requireExplicitPolicyPresent: i32,
  pub requireExplicitPolicy: u32,
  pub inhibitPolicyMappingPresent: i32,
  pub inhibitPolicyMapping: u32,
}
pub type CE_PolicyConstraints = __CE_PolicyConstraints;
pub type CE_InhibitAnyPolicy = u32;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CE_DataType {
  DT_AuthorityKeyID = 0,
  DT_SubjectKeyID = 1,
  DT_KeyUsage = 2,
  DT_SubjectAltName = 3,
  DT_IssuerAltName = 4,
  DT_ExtendedKeyUsage = 5,
  DT_BasicConstraints = 6,
  DT_CertPolicies = 7,
  DT_NetscapeCertType = 8,
  DT_CrlNumber = 9,
  DT_DeltaCrl = 10,
  DT_CrlReason = 11,
  DT_CrlDistributionPoints = 12,
  DT_IssuingDistributionPoint = 13,
  DT_AuthorityInfoAccess = 14,
  DT_Other = 15,
  DT_QC_Statements = 16,
  DT_NameConstraints = 17,
  DT_PolicyMappings = 18,
  DT_PolicyConstraints = 19,
  DT_InhibitAnyPolicy = 20,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CE_Data {
  pub authorityKeyID: CE_AuthorityKeyID,
  pub subjectKeyID: CSSM_DATA,
  pub keyUsage: u16,
  pub subjectAltName: CE_GeneralNames,
  pub issuerAltName: CE_GeneralNames,
  pub extendedKeyUsage: CE_ExtendedKeyUsage,
  pub basicConstraints: CE_BasicConstraints,
  pub certPolicies: CE_CertPolicies,
  pub netscapeCertType: u16,
  pub crlNumber: u32,
  pub deltaCrl: u32,
  pub crlReason: u32,
  pub crlDistPoints: CE_CRLDistPointsSyntax,
  pub issuingDistPoint: CE_IssuingDistributionPoint,
  pub authorityInfoAccess: CE_AuthorityInfoAccess,
  pub qualifiedCertStatements: CE_QC_Statements,
  pub nameConstraints: CE_NameConstraints,
  pub policyMappings: CE_PolicyMappings,
  pub policyConstraints: CE_PolicyConstraints,
  pub inhibitAnyPolicy: u32,
  pub rawData: CSSM_DATA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __CE_DataAndType {
  pub type_: CE_DataType,
  pub extension: CE_Data,
  pub critical: i32,
}
pub type CE_DataAndType = __CE_DataAndType;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_process_subject_selector {
  pub version: u16,
  pub mask: u16,
  pub uid: u32,
  pub gid: u32,
}
pub type CSSM_ACL_PROCESS_SUBJECT_SELECTOR = cssm_acl_process_subject_selector;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_acl_keychain_prompt_selector {
  pub version: u16,
  pub flags: u16,
}
pub type CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR = cssm_acl_keychain_prompt_selector;
pub type CSSM_ACL_PREAUTH_TRACKING_STATE = u32;
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum cssm_appledl_open_parameters_mask {
  kCSSM_APPLEDL_MASK_MODE = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_appledl_open_parameters {
  pub length: u32,
  pub version: u32,
  pub autoCommit: i32,
  pub mask: u32,
  pub mode: u16,
}
pub type CSSM_APPLEDL_OPEN_PARAMETERS = cssm_appledl_open_parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_applecspdl_db_settings_parameters {
  pub idleTimeout: u32,
  pub lockOnSleep: u8,
}
pub type CSSM_APPLECSPDL_DB_SETTINGS_PARAMETERS = cssm_applecspdl_db_settings_parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_applecspdl_db_is_locked_parameters {
  pub isLocked: u8,
}
pub type CSSM_APPLECSPDL_DB_IS_LOCKED_PARAMETERS = cssm_applecspdl_db_is_locked_parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_applecspdl_db_change_password_parameters {
  pub accessCredentials: *mut CSSM_ACCESS_CREDENTIALS,
}
pub type CSSM_APPLECSPDL_DB_CHANGE_PASSWORD_PARAMETERS =
  cssm_applecspdl_db_change_password_parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_NAME_OID {
  pub string: *mut i8,
  pub oid: *mut CSSM_DATA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_CERT_REQUEST {
  pub cspHand: isize,
  pub clHand: isize,
  pub serialNumber: u32,
  pub numSubjectNames: u32,
  pub subjectNames: *mut CSSM_APPLE_TP_NAME_OID,
  pub numIssuerNames: u32,
  pub issuerNames: *mut CSSM_APPLE_TP_NAME_OID,
  pub issuerNameX509: *mut cssm_x509_name,
  pub certPublicKey: *mut CSSM_KEY,
  pub issuerPrivateKey: *mut CSSM_KEY,
  pub signatureAlg: u32,
  pub signatureOid: CSSM_DATA,
  pub notBefore: u32,
  pub notAfter: u32,
  pub numExtensions: u32,
  pub extensions: *mut CE_DataAndType,
  pub challengeString: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_SSL_OPTIONS {
  pub Version: u32,
  pub ServerNameLen: u32,
  pub ServerName: *mut i8,
  pub Flags: u32,
}
pub type CSSM_APPLE_TP_CRL_OPT_FLAGS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_CRL_OPTIONS {
  pub Version: u32,
  pub CrlFlags: u32,
  pub crlStore: *mut cssm_dl_db_handle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_SMIME_OPTIONS {
  pub Version: u32,
  pub IntendedUsage: u16,
  pub SenderEmailLen: u32,
  pub SenderEmail: *mut i8,
}
pub type CSSM_APPLE_TP_ACTION_FLAGS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_TP_ACTION_DATA {
  pub Version: u32,
  pub ActionFlags: u32,
}
pub type CSSM_TP_APPLE_CERT_STATUS = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_TP_APPLE_EVIDENCE_INFO {
  pub StatusBits: u32,
  pub NumStatusCodes: u32,
  pub StatusCodes: *mut i32,
  pub Index: u32,
  pub DlDbHandle: CSSM_DL_DB_HANDLE,
  pub UniqueRecord: *mut cssm_db_unique_record,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_TP_APPLE_EVIDENCE_HEADER {
  pub Version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSSM_APPLE_CL_CSR_REQUEST {
  pub subjectNameX509: *mut cssm_x509_name,
  pub signatureAlg: u32,
  pub signatureOid: CSSM_DATA,
  pub cspHand: isize,
  pub subjectPublicKey: *mut CSSM_KEY,
  pub subjectPrivateKey: *mut CSSM_KEY,
  pub challengeString: *mut i8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainSettings {
  pub version: u32,
  pub lockOnSleep: u8,
  pub useLockInterval: u8,
  pub lockInterval: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecAuthenticationType {
  kSecAuthenticationTypeNTLM = 1835824238,
  kSecAuthenticationTypeMSN = 1634628461,
  kSecAuthenticationTypeDPA = 1633775716,
  kSecAuthenticationTypeRPA = 1633775730,
  kSecAuthenticationTypeHTTPBasic = 1886680168,
  kSecAuthenticationTypeHTTPDigest = 1685353576,
  kSecAuthenticationTypeHTMLForm = 1836216166,
  kSecAuthenticationTypeDefault = 1953261156,
  kSecAuthenticationTypeAny = 0,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecProtocolType {
  kSecProtocolTypeFTP = 1718906912,
  kSecProtocolTypeFTPAccount = 1718906977,
  kSecProtocolTypeHTTP = 1752462448,
  kSecProtocolTypeIRC = 1769104160,
  kSecProtocolTypeNNTP = 1852732528,
  kSecProtocolTypePOP3 = 1886351411,
  kSecProtocolTypeSMTP = 1936553072,
  kSecProtocolTypeSOCKS = 1936685088,
  kSecProtocolTypeIMAP = 1768776048,
  kSecProtocolTypeLDAP = 1818517872,
  kSecProtocolTypeAppleTalk = 1635019883,
  kSecProtocolTypeAFP = 1634103328,
  kSecProtocolTypeTelnet = 1952803950,
  kSecProtocolTypeSSH = 1936943136,
  kSecProtocolTypeFTPS = 1718906995,
  kSecProtocolTypeHTTPS = 1752461427,
  kSecProtocolTypeHTTPProxy = 1752461432,
  kSecProtocolTypeHTTPSProxy = 1752462200,
  kSecProtocolTypeFTPProxy = 1718907000,
  kSecProtocolTypeCIFS = 1667851891,
  kSecProtocolTypeSMB = 1936548384,
  kSecProtocolTypeRTSP = 1920234352,
  kSecProtocolTypeRTSPProxy = 1920234360,
  kSecProtocolTypeDAAP = 1684103536,
  kSecProtocolTypeEPPC = 1701867619,
  kSecProtocolTypeIPP = 1768976416,
  kSecProtocolTypeNNTPS = 1853124723,
  kSecProtocolTypeLDAPS = 1818521715,
  kSecProtocolTypeTelnetS = 1952803955,
  kSecProtocolTypeIMAPS = 1768779891,
  kSecProtocolTypeIRCS = 1769104243,
  kSecProtocolTypePOP3S = 1886351475,
  kSecProtocolTypeCVSpserver = 1668707184,
  kSecProtocolTypeSVN = 1937141280,
  kSecProtocolTypeAny = 0,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecKeychainEvent {
  kSecLockEvent = 1,
  kSecUnlockEvent = 2,
  kSecAddEvent = 3,
  kSecDeleteEvent = 4,
  kSecUpdateEvent = 5,
  kSecPasswordChangedEvent = 6,
  kSecDefaultChangedEvent = 9,
  kSecDataAccessEvent = 10,
  kSecKeychainListChangedEvent = 11,
  kSecTrustSettingsChangedEvent = 12,
}
bitflags! { # [ repr ( C ) ] pub struct SecKeychainEventMask : u32 { const kSecLockEventMask = 2 ; const kSecUnlockEventMask = 4 ; const kSecAddEventMask = 8 ; const kSecDeleteEventMask = 16 ; const kSecUpdateEventMask = 32 ; const kSecPasswordChangedEventMask = 64 ; const kSecDefaultChangedEventMask = 512 ; const kSecDataAccessEventMask = 1024 ; const kSecKeychainListChangedMask = 2048 ; const kSecTrustSettingsChangedEventMask = 4096 ; const kSecEveryEventMask = 4294967295 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeychainCallbackInfo {
  pub version: u32,
  pub item: *mut OpaqueSecKeychainItemRef,
  pub keychain: *mut OpaqueSecKeychainRef,
  pub pid: i32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SecPreferencesDomain {
  kSecPreferencesDomainUser = 0,
  kSecPreferencesDomainSystem = 1,
  kSecPreferencesDomainCommon = 2,
  kSecPreferencesDomainDynamic = 3,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecExternalFormat {
  kSecFormatUnknown = 0,
  kSecFormatOpenSSL = 1,
  kSecFormatSSH = 2,
  kSecFormatBSAFE = 3,
  kSecFormatRawKey = 4,
  kSecFormatWrappedPKCS8 = 5,
  kSecFormatWrappedOpenSSL = 6,
  kSecFormatWrappedSSH = 7,
  kSecFormatWrappedLSH = 8,
  kSecFormatX509Cert = 9,
  kSecFormatPEMSequence = 10,
  kSecFormatPKCS7 = 11,
  kSecFormatPKCS12 = 12,
  kSecFormatNetscapeCertSequence = 13,
  kSecFormatSSHv2 = 14,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecExternalItemType {
  kSecItemTypeUnknown = 0,
  kSecItemTypePrivateKey = 1,
  kSecItemTypePublicKey = 2,
  kSecItemTypeSessionKey = 3,
  kSecItemTypeCertificate = 4,
  kSecItemTypeAggregate = 5,
}
bitflags! { # [ repr ( C ) ] pub struct SecItemImportExportFlags : u32 { const kSecItemPemArmour = 1 ; } }
bitflags! { # [ repr ( C ) ] pub struct SecKeyImportExportFlags : u32 { const kSecKeyImportOnlyOne = 1 ; const kSecKeySecurePassphrase = 2 ; const kSecKeyNoAccessControl = 4 ; } }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecKeyImportExportParameters {
  pub version: u32,
  pub flags: SecKeyImportExportFlags,
  pub passphrase: *mut c_void,
  pub alertTitle: *mut __CFString,
  pub alertPrompt: *mut __CFString,
  pub accessRef: *mut OpaqueSecAccessRef,
  pub keyUsage: u32,
  pub keyAttributes: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecItemImportExportKeyParameters {
  pub version: u32,
  pub flags: SecKeyImportExportFlags,
  pub passphrase: *mut c_void,
  pub alertTitle: *mut __CFString,
  pub alertPrompt: *mut __CFString,
  pub accessRef: *mut OpaqueSecAccessRef,
  pub keyUsage: *mut __CFArray,
  pub keyAttributes: *mut __CFArray,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecTrustResultType {
  kSecTrustResultInvalid = 0,
  kSecTrustResultProceed = 1,
  kSecTrustResultConfirm = 2,
  kSecTrustResultDeny = 3,
  kSecTrustResultUnspecified = 4,
  kSecTrustResultRecoverableTrustFailure = 5,
  kSecTrustResultFatalTrustFailure = 6,
  kSecTrustResultOtherError = 7,
}
#[repr(C)]
pub struct __SecTrust {
  opaque: u32,
}
pub type SecTrustUserSetting = SecTrustResultType;
bitflags! { # [ repr ( C ) ] pub struct SecTrustOptionFlags : u32 { const kSecTrustOptionAllowExpired = 1 ; const kSecTrustOptionLeafIsCA = 2 ; const kSecTrustOptionFetchIssuerFromNet = 4 ; const kSecTrustOptionAllowExpiredRoot = 8 ; const kSecTrustOptionRequireRevPerCert = 16 ; const kSecTrustOptionUseTrustSettings = 32 ; const kSecTrustOptionImplicitAnchors = 64 ; } }
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_objectProto {}
pub type SSLCipherSuite = u32;
#[repr(C)]
pub struct SSLContext {
  opaque: u32,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLProtocol {
  kSSLProtocolUnknown = 0,
  kSSLProtocol3 = 2,
  kTLSProtocol1 = 4,
  kTLSProtocol11 = 7,
  kTLSProtocol12 = 8,
  kDTLSProtocol1 = 9,
  kTLSProtocol13 = 10,
  kTLSProtocolMaxSupported = 999,
  kSSLProtocol2 = 1,
  kSSLProtocol3Only = 3,
  kTLSProtocol1Only = 5,
  kSSLProtocolAll = 6,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLSessionOption {
  kSSLSessionOptionBreakOnServerAuth = 0,
  kSSLSessionOptionBreakOnCertRequested = 1,
  kSSLSessionOptionBreakOnClientAuth = 2,
  kSSLSessionOptionFalseStart = 3,
  kSSLSessionOptionSendOneByteRecord = 4,
  kSSLSessionOptionAllowServerIdentityChange = 5,
  kSSLSessionOptionFallback = 6,
  kSSLSessionOptionBreakOnClientHello = 7,
  kSSLSessionOptionAllowRenegotiation = 8,
  kSSLSessionOptionEnableSessionTickets = 9,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLSessionState {
  kSSLIdle = 0,
  kSSLHandshake = 1,
  kSSLConnected = 2,
  kSSLClosed = 3,
  kSSLAborted = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLClientCertificateState {
  kSSLClientCertNone = 0,
  kSSLClientCertRequested = 1,
  kSSLClientCertSent = 2,
  kSSLClientCertRejected = 3,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLCiphersuiteGroup {
  kSSLCiphersuiteGroupDefault = 0,
  kSSLCiphersuiteGroupCompatibility = 1,
  kSSLCiphersuiteGroupLegacy = 2,
  kSSLCiphersuiteGroupATS = 3,
  kSSLCiphersuiteGroupATSCompatibility = 4,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLProtocolSide {
  kSSLServerSide = 0,
  kSSLClientSide = 1,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLConnectionType {
  kSSLStreamType = 0,
  kSSLDatagramType = 1,
}
#[repr(i32)]
#[derive(Copy, Clone)]
pub enum SSLAuthenticate {
  kNeverAuthenticate = 0,
  kAlwaysAuthenticate = 1,
  kTryAuthenticate = 2,
}
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_trustProto {}
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_identityProto {}
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_certificateProto {}
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_protocol_metadataProto {}
#[objrs(protocol)]
#[link(name = "Security", kind = "framework")]
pub trait OS_sec_protocol_optionsProto {}
bitflags! { # [ repr ( C ) ] pub struct AuthorizationFlags : u32 { const kAuthorizationFlagDefaults = 0 ; const kAuthorizationFlagInteractionAllowed = 1 ; const kAuthorizationFlagExtendRights = 2 ; const kAuthorizationFlagPartialRights = 4 ; const kAuthorizationFlagDestroyRights = 8 ; const kAuthorizationFlagPreAuthorize = 16 ; const kAuthorizationFlagNoData = 1048576 ; } }
#[repr(C)]
pub struct AuthorizationOpaqueRef {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuthorizationItem {
  pub name: *mut i8,
  pub valueLength: usize,
  pub value: *mut c_void,
  pub flags: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuthorizationItemSet {
  pub count: u32,
  pub items: *mut AuthorizationItem,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuthorizationExternalForm {
  pub bytes: [i8; 32],
}
pub type AuthorizationRights = AuthorizationItemSet;
pub type AuthorizationEnvironment = AuthorizationItemSet;
pub type SecuritySessionId = u32;
bitflags! { # [ repr ( C ) ] pub struct SessionAttributeBits : u32 { const sessionIsRoot = 1 ; const sessionHasGraphicAccess = 16 ; const sessionHasTTY = 32 ; const sessionIsRemote = 4096 ; } }
bitflags! { # [ repr ( C ) ] pub struct SessionCreationFlags : u32 { const sessionKeepCurrentBootstrap = 32768 ; } }
pub type CSSM_MANAGER_EVENT_TYPES = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_manager_event_notification {
  pub DestinationModuleManagerType: u32,
  pub SourceModuleManagerType: u32,
  pub Event: u32,
  pub EventId: u32,
  pub EventData: CSSM_DATA,
}
pub type CSSM_MANAGER_EVENT_NOTIFICATION = cssm_manager_event_notification;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_ac_funcs {
  pub AuthCompute: Option<
    extern "C" fn(
      isize,
      *mut CSSM_TUPLEGROUP,
      *mut CSSM_TUPLEGROUP,
      u32,
      *mut CSSM_LIST,
      *mut CSSM_LIST,
      *mut CSSM_LIST,
      *mut cssm_tuplegroup,
    ) -> i32,
  >,
  pub PassThrough: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      u64,
      *mut CSSM_DL_DB_LIST,
      u32,
      *mut c_void,
      *mut *mut c_void,
    ) -> i32,
  >,
}
pub type CSSM_SPI_AC_FUNCS = cssm_spi_ac_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_cl_funcs {
  pub CertCreateTemplate: Option<extern "C" fn(isize, u32, *mut CSSM_FIELD, *mut cssm_data) -> i32>,
  pub CertGetAllTemplateFields:
    Option<extern "C" fn(isize, *mut CSSM_DATA, *mut u32, *mut *mut cssm_field) -> i32>,
  pub CertSign:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, *mut CSSM_FIELD, u32, *mut cssm_data) -> i32>,
  pub CertVerify:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, *mut CSSM_DATA, *mut CSSM_FIELD, u32) -> i32>,
  pub CertVerifyWithKey: Option<extern "C" fn(isize, u64, *mut CSSM_DATA) -> i32>,
  pub CertGetFirstFieldValue: Option<
    extern "C" fn(
      isize,
      *mut CSSM_DATA,
      *mut CSSM_DATA,
      *mut isize,
      *mut u32,
      *mut *mut cssm_data,
    ) -> i32,
  >,
  pub CertGetNextFieldValue: Option<extern "C" fn(isize, isize, *mut *mut cssm_data) -> i32>,
  pub CertAbortQuery: Option<extern "C" fn(isize, isize) -> i32>,
  pub CertGetKeyInfo: Option<extern "C" fn(isize, *mut CSSM_DATA, *mut *mut cssm_key) -> i32>,
  pub CertGetAllFields:
    Option<extern "C" fn(isize, *mut CSSM_DATA, *mut u32, *mut *mut cssm_field) -> i32>,
  pub FreeFields: Option<extern "C" fn(isize, u32, *mut *mut cssm_field) -> i32>,
  pub FreeFieldValue: Option<extern "C" fn(isize, *mut CSSM_DATA, *mut cssm_data) -> i32>,
  pub CertCache: Option<extern "C" fn(isize, *mut CSSM_DATA, *mut isize) -> i32>,
  pub CertGetFirstCachedFieldValue: Option<
    extern "C" fn(isize, isize, *mut CSSM_DATA, *mut isize, *mut u32, *mut *mut cssm_data) -> i32,
  >,
  pub CertGetNextCachedFieldValue: Option<extern "C" fn(isize, isize, *mut *mut cssm_data) -> i32>,
  pub CertAbortCache: Option<extern "C" fn(isize, isize) -> i32>,
  pub CertGroupToSignedBundle: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CERTGROUP,
      *mut CSSM_CERT_BUNDLE_HEADER,
      *mut cssm_data,
    ) -> i32,
  >,
  pub CertGroupFromVerifiedBundle: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CERT_BUNDLE,
      *mut CSSM_DATA,
      *mut *mut cssm_certgroup,
    ) -> i32,
  >,
  pub CertDescribeFormat: Option<extern "C" fn(isize, *mut u32, *mut *mut CSSM_DATA) -> i32>,
  pub CrlCreateTemplate: Option<extern "C" fn(isize, u32, *mut CSSM_FIELD, *mut cssm_data) -> i32>,
  pub CrlSetFields:
    Option<extern "C" fn(isize, u32, *mut CSSM_FIELD, *mut CSSM_DATA, *mut cssm_data) -> i32>,
  pub CrlAddCert: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_DATA,
      u32,
      *mut CSSM_FIELD,
      *mut CSSM_DATA,
      *mut cssm_data,
    ) -> i32,
  >,
  pub CrlRemoveCert:
    Option<extern "C" fn(isize, *mut CSSM_DATA, *mut CSSM_DATA, *mut cssm_data) -> i32>,
  pub CrlSign:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, *mut CSSM_FIELD, u32, *mut cssm_data) -> i32>,
  pub CrlVerify:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, *mut CSSM_DATA, *mut CSSM_FIELD, u32) -> i32>,
  pub CrlVerifyWithKey: Option<extern "C" fn(isize, u64, *mut CSSM_DATA) -> i32>,
  pub IsCertInCrl: Option<extern "C" fn(isize, *mut CSSM_DATA, *mut CSSM_DATA, *mut i32) -> i32>,
  pub CrlGetFirstFieldValue: Option<
    extern "C" fn(
      isize,
      *mut CSSM_DATA,
      *mut CSSM_DATA,
      *mut isize,
      *mut u32,
      *mut *mut cssm_data,
    ) -> i32,
  >,
  pub CrlGetNextFieldValue: Option<extern "C" fn(isize, isize, *mut *mut cssm_data) -> i32>,
  pub CrlAbortQuery: Option<extern "C" fn(isize, isize) -> i32>,
  pub CrlGetAllFields:
    Option<extern "C" fn(isize, *mut CSSM_DATA, *mut u32, *mut *mut cssm_field) -> i32>,
  pub CrlCache: Option<extern "C" fn(isize, *mut CSSM_DATA, *mut isize) -> i32>,
  pub IsCertInCachedCrl:
    Option<extern "C" fn(isize, *mut CSSM_DATA, isize, *mut i32, *mut cssm_data) -> i32>,
  pub CrlGetFirstCachedFieldValue: Option<
    extern "C" fn(
      isize,
      isize,
      *mut CSSM_DATA,
      *mut CSSM_DATA,
      *mut isize,
      *mut u32,
      *mut *mut cssm_data,
    ) -> i32,
  >,
  pub CrlGetNextCachedFieldValue: Option<extern "C" fn(isize, isize, *mut *mut cssm_data) -> i32>,
  pub CrlGetAllCachedRecordFields:
    Option<extern "C" fn(isize, isize, *mut CSSM_DATA, *mut u32, *mut *mut cssm_field) -> i32>,
  pub CrlAbortCache: Option<extern "C" fn(isize, isize) -> i32>,
  pub CrlDescribeFormat: Option<extern "C" fn(isize, *mut u32, *mut *mut CSSM_DATA) -> i32>,
  pub PassThrough: Option<extern "C" fn(isize, u64, u32, *mut c_void, *mut *mut c_void) -> i32>,
}
pub type CSSM_SPI_CL_FUNCS = cssm_spi_cl_funcs;
pub type CSSM_CONTEXT_EVENT = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_module_funcs {
  pub ServiceType: u32,
  pub NumberOfServiceFuncs: u32,
  pub ServiceFuncs: *mut Option<extern "C" fn() -> ()>,
}
pub type CSSM_MODULE_FUNCS = cssm_module_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_upcalls {
  pub malloc_func: Option<extern "C" fn(isize, usize) -> *mut c_void>,
  pub free_func: Option<extern "C" fn(isize, *mut c_void) -> ()>,
  pub realloc_func: Option<extern "C" fn(isize, *mut c_void, usize) -> *mut c_void>,
  pub calloc_func: Option<extern "C" fn(isize, usize, usize) -> *mut c_void>,
  pub CcToHandle_func: Option<extern "C" fn(u64, *mut isize) -> i32>,
  pub GetModuleInfo_func: Option<
    extern "C" fn(
      isize,
      *mut cssm_guid,
      *mut cssm_version,
      *mut u32,
      *mut u32,
      *mut u32,
      *mut u32,
      *mut CSSM_MEMORY_FUNCS,
      *mut cssm_func_name_addr,
      u32,
    ) -> i32,
  >,
}
pub type CSSM_UPCALLS = cssm_upcalls;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_csp_funcs {
  pub EventNotify: Option<extern "C" fn(isize, u32, u64, *mut CSSM_CONTEXT) -> i32>,
  pub QuerySize: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, i32, u32, *mut cssm_query_size_data) -> i32,
  >,
  pub SignData: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_DATA, u32, u32, *mut cssm_data) -> i32,
  >,
  pub SignDataInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT) -> i32>,
  pub SignDataUpdate: Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32) -> i32>,
  pub SignDataFinal: Option<extern "C" fn(isize, u64, *mut cssm_data) -> i32>,
  pub VerifyData: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_DATA, u32, u32, *mut CSSM_DATA) -> i32,
  >,
  pub VerifyDataInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT) -> i32>,
  pub VerifyDataUpdate: Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32) -> i32>,
  pub VerifyDataFinal: Option<extern "C" fn(isize, u64, *mut CSSM_DATA) -> i32>,
  pub DigestData: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_DATA, u32, *mut cssm_data) -> i32,
  >,
  pub DigestDataInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT) -> i32>,
  pub DigestDataUpdate: Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32) -> i32>,
  pub DigestDataClone: Option<extern "C" fn(isize, u64, u64) -> i32>,
  pub DigestDataFinal: Option<extern "C" fn(isize, u64, *mut cssm_data) -> i32>,
  pub GenerateMac: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_DATA, u32, *mut cssm_data) -> i32,
  >,
  pub GenerateMacInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT) -> i32>,
  pub GenerateMacUpdate: Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32) -> i32>,
  pub GenerateMacFinal: Option<extern "C" fn(isize, u64, *mut cssm_data) -> i32>,
  pub VerifyMac: Option<
    extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_DATA, u32, *mut CSSM_DATA) -> i32,
  >,
  pub VerifyMacInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT) -> i32>,
  pub VerifyMacUpdate: Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32) -> i32>,
  pub VerifyMacFinal: Option<extern "C" fn(isize, u64, *mut CSSM_DATA) -> i32>,
  pub EncryptData: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      u32,
      *mut cssm_data,
      u32,
      *mut usize,
      *mut cssm_data,
      u64,
    ) -> i32,
  >,
  pub EncryptDataInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT, u64) -> i32>,
  pub EncryptDataUpdate:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32, *mut cssm_data, u32, *mut usize) -> i32>,
  pub EncryptDataFinal: Option<extern "C" fn(isize, u64, *mut cssm_data) -> i32>,
  pub DecryptData: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      u32,
      *mut cssm_data,
      u32,
      *mut usize,
      *mut cssm_data,
      u64,
    ) -> i32,
  >,
  pub DecryptDataInit: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT, u64) -> i32>,
  pub DecryptDataUpdate:
    Option<extern "C" fn(isize, u64, *mut CSSM_DATA, u32, *mut cssm_data, u32, *mut usize) -> i32>,
  pub DecryptDataFinal: Option<extern "C" fn(isize, u64, *mut cssm_data) -> i32>,
  pub QueryKeySizeInBits:
    Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut CSSM_KEY, *mut cssm_key_size) -> i32>,
  pub GenerateKey: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      u32,
      u32,
      *mut CSSM_DATA,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      *mut cssm_key,
      u64,
    ) -> i32,
  >,
  pub GenerateKeyPair: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      u32,
      u32,
      *mut CSSM_DATA,
      *mut cssm_key,
      u32,
      u32,
      *mut CSSM_DATA,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      *mut cssm_key,
      u64,
    ) -> i32,
  >,
  pub GenerateRandom: Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT, *mut cssm_data) -> i32>,
  pub GenerateAlgorithmParams: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      u32,
      *mut cssm_data,
      *mut u32,
      *mut *mut cssm_context_attribute,
    ) -> i32,
  >,
  pub WrapKey: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut CSSM_KEY,
      *mut CSSM_DATA,
      *mut CSSM_KEY,
      u64,
    ) -> i32,
  >,
  pub UnwrapKey: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_KEY,
      *mut CSSM_KEY,
      u32,
      u32,
      *mut CSSM_DATA,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      *mut cssm_key,
      *mut cssm_data,
      u64,
    ) -> i32,
  >,
  pub DeriveKey: Option<
    extern "C" fn(
      isize,
      u64,
      *mut CSSM_CONTEXT,
      *mut cssm_data,
      u32,
      u32,
      *mut CSSM_DATA,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      *mut cssm_key,
    ) -> i32,
  >,
  pub FreeKey:
    Option<extern "C" fn(isize, *mut CSSM_ACCESS_CREDENTIALS, *mut cssm_key, i32) -> i32>,
  pub PassThrough:
    Option<extern "C" fn(isize, u64, *mut CSSM_CONTEXT, u32, *mut c_void, *mut *mut c_void) -> i32>,
  pub Login:
    Option<extern "C" fn(isize, *mut CSSM_ACCESS_CREDENTIALS, *mut CSSM_DATA, *mut c_void) -> i32>,
  pub Logout: Option<extern "C" fn(isize) -> i32>,
  pub ChangeLoginAcl:
    Option<extern "C" fn(isize, *mut CSSM_ACCESS_CREDENTIALS, *mut CSSM_ACL_EDIT) -> i32>,
  pub ObtainPrivateKeyFromPublicKey:
    Option<extern "C" fn(isize, *mut CSSM_KEY, *mut cssm_key) -> i32>,
  pub RetrieveUniqueId: Option<extern "C" fn(isize, *mut cssm_data) -> i32>,
  pub RetrieveCounter: Option<extern "C" fn(isize, *mut cssm_data) -> i32>,
  pub VerifyDevice: Option<extern "C" fn(isize, *mut CSSM_DATA) -> i32>,
  pub GetTimeValue: Option<extern "C" fn(isize, u32, *mut CSSM_DATA) -> i32>,
  pub GetOperationalStatistics:
    Option<extern "C" fn(isize, *mut CSSM_CSP_OPERATIONAL_STATISTICS) -> i32>,
  pub GetLoginAcl:
    Option<extern "C" fn(isize, *mut [i8; 68], *mut u32, *mut *mut cssm_acl_entry_info) -> i32>,
  pub GetKeyAcl: Option<
    extern "C" fn(
      isize,
      *mut CSSM_KEY,
      *mut [i8; 68],
      *mut u32,
      *mut *mut cssm_acl_entry_info,
    ) -> i32,
  >,
  pub ChangeKeyAcl: Option<
    extern "C" fn(isize, *mut CSSM_ACCESS_CREDENTIALS, *mut CSSM_ACL_EDIT, *mut CSSM_KEY) -> i32,
  >,
  pub GetKeyOwner:
    Option<extern "C" fn(isize, *mut CSSM_KEY, *mut cssm_acl_owner_prototype) -> i32>,
  pub ChangeKeyOwner: Option<
    extern "C" fn(
      isize,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut CSSM_KEY,
      *mut CSSM_ACL_OWNER_PROTOTYPE,
    ) -> i32,
  >,
  pub GetLoginOwner: Option<extern "C" fn(isize, *mut cssm_acl_owner_prototype) -> i32>,
  pub ChangeLoginOwner: Option<
    extern "C" fn(isize, *mut CSSM_ACCESS_CREDENTIALS, *mut CSSM_ACL_OWNER_PROTOTYPE) -> i32,
  >,
}
pub type CSSM_SPI_CSP_FUNCS = cssm_spi_csp_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_dl_funcs {
  pub DbOpen: Option<
    extern "C" fn(
      isize,
      *mut i8,
      *mut CSSM_NET_ADDRESS,
      u32,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut c_void,
      *mut isize,
    ) -> i32,
  >,
  pub DbClose: Option<extern "C" fn(CSSM_DL_DB_HANDLE) -> i32>,
  pub DbCreate: Option<
    extern "C" fn(
      isize,
      *mut i8,
      *mut CSSM_NET_ADDRESS,
      *mut CSSM_DBINFO,
      u32,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      *mut c_void,
      *mut isize,
    ) -> i32,
  >,
  pub DbDelete: Option<
    extern "C" fn(isize, *mut i8, *mut CSSM_NET_ADDRESS, *mut CSSM_ACCESS_CREDENTIALS) -> i32,
  >,
  pub CreateRelation: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut i8,
      u32,
      *mut CSSM_DB_SCHEMA_ATTRIBUTE_INFO,
      u32,
      *mut CSSM_DB_SCHEMA_INDEX_INFO,
    ) -> i32,
  >,
  pub DestroyRelation: Option<extern "C" fn(CSSM_DL_DB_HANDLE, u32) -> i32>,
  pub Authenticate:
    Option<extern "C" fn(CSSM_DL_DB_HANDLE, u32, *mut CSSM_ACCESS_CREDENTIALS) -> i32>,
  pub GetDbAcl: Option<
    extern "C" fn(CSSM_DL_DB_HANDLE, *mut [i8; 68], *mut u32, *mut *mut cssm_acl_entry_info) -> i32,
  >,
  pub ChangeDbAcl: Option<
    extern "C" fn(CSSM_DL_DB_HANDLE, *mut CSSM_ACCESS_CREDENTIALS, *mut CSSM_ACL_EDIT) -> i32,
  >,
  pub GetDbOwner: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut cssm_acl_owner_prototype) -> i32>,
  pub ChangeDbOwner: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut CSSM_ACL_OWNER_PROTOTYPE,
    ) -> i32,
  >,
  pub GetDbNames: Option<extern "C" fn(isize, *mut *mut cssm_name_list) -> i32>,
  pub GetDbNameFromHandle: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut *mut i8) -> i32>,
  pub FreeNameList: Option<extern "C" fn(isize, *mut cssm_name_list) -> i32>,
  pub DataInsert: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
      *mut CSSM_DATA,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataDelete: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut CSSM_DB_UNIQUE_RECORD) -> i32>,
  pub DataModify: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut cssm_db_unique_record,
      *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
      *mut CSSM_DATA,
      u32,
    ) -> i32,
  >,
  pub DataGetFirst: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      *mut CSSM_QUERY,
      *mut isize,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataGetNext: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      isize,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataAbortQuery: Option<extern "C" fn(CSSM_DL_DB_HANDLE, isize) -> i32>,
  pub DataGetFromUniqueRecordId: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      *mut CSSM_DB_UNIQUE_RECORD,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
    ) -> i32,
  >,
  pub FreeUniqueRecord: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut cssm_db_unique_record) -> i32>,
  pub PassThrough:
    Option<extern "C" fn(CSSM_DL_DB_HANDLE, u32, *mut c_void, *mut *mut c_void) -> i32>,
}
pub type CSSM_SPI_DL_FUNCS = cssm_spi_dl_funcs;
pub type CSSM_KRSP_HANDLE = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_kr_name {
  pub Type: u8,
  pub Length: u8,
  pub Name: *mut i8,
}
pub type CSSM_KR_NAME = cssm_kr_name;
pub type CSSM_KR_PROFILE = cssm_kr_profile;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_kr_wrappedproductinfo {
  pub StandardVersion: CSSM_VERSION,
  pub StandardDescription: [i8; 68],
  pub ProductVersion: CSSM_VERSION,
  pub ProductDescription: [i8; 68],
  pub ProductVendor: [i8; 68],
  pub ProductFlags: u32,
}
pub type CSSM_KR_WRAPPEDPRODUCT_INFO = cssm_kr_wrappedproductinfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_krsubservice {
  pub SubServiceId: u32,
  pub Description: *mut i8,
  pub WrappedProduct: CSSM_KR_WRAPPEDPRODUCT_INFO,
}
pub type CSSM_KRSUBSERVICE = cssm_krsubservice;
pub type CSSM_KR_POLICY_TYPE = u32;
pub type CSSM_KR_POLICY_FLAGS = u32;
#[repr(C)]
pub struct kr_policy_list_item {
  opaque: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_kr_policy_list_item {
  pub next: *mut kr_policy_list_item,
  pub AlgorithmId: u32,
  pub Mode: u32,
  pub MaxKeyLength: u32,
  pub MaxRounds: u32,
  pub WorkFactor: u8,
  pub PolicyFlags: u32,
  pub AlgClass: u32,
}
pub type CSSM_KR_POLICY_LIST_ITEM = cssm_kr_policy_list_item;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_kr_policy_info {
  pub krbNotAllowed: i32,
  pub numberOfEntries: u32,
  pub policyEntry: *mut CSSM_KR_POLICY_LIST_ITEM,
}
pub type CSSM_KR_POLICY_INFO = cssm_kr_policy_info;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_kr_funcs {
  pub RegistrationRequest: Option<
    extern "C" fn(
      u32,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      *mut CSSM_ACCESS_CREDENTIALS,
      u32,
      *mut i32,
      *mut isize,
    ) -> i32,
  >,
  pub RegistrationRetrieve:
    Option<extern "C" fn(u32, isize, *mut i32, *mut cssm_kr_profile) -> i32>,
  pub GenerateRecoveryFields: Option<
    extern "C" fn(
      u32,
      u64,
      *mut CSSM_CONTEXT,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      u32,
      *mut cssm_data,
    ) -> i32,
  >,
  pub ProcessRecoveryFields: Option<
    extern "C" fn(
      u32,
      u64,
      *mut CSSM_CONTEXT,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      u32,
      *mut CSSM_DATA,
    ) -> i32,
  >,
  pub RecoveryRequest: Option<
    extern "C" fn(
      u32,
      u64,
      *mut CSSM_CONTEXT,
      *mut CSSM_DATA,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut i32,
      *mut isize,
    ) -> i32,
  >,
  pub RecoveryRetrieve: Option<extern "C" fn(u32, isize, *mut i32, *mut isize, *mut u32) -> i32>,
  pub GetRecoveredObject: Option<
    extern "C" fn(
      u32,
      isize,
      u32,
      isize,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
      u32,
      *mut cssm_key,
      *mut cssm_data,
    ) -> i32,
  >,
  pub RecoveryRequestAbort: Option<extern "C" fn(u32, isize) -> i32>,
  pub PassThrough: Option<
    extern "C" fn(
      u32,
      u64,
      *mut CSSM_CONTEXT,
      u64,
      *mut CSSM_CONTEXT,
      u32,
      *mut c_void,
      *mut *mut c_void,
    ) -> i32,
  >,
}
pub type CSSM_SPI_KR_FUNCS = cssm_spi_kr_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_spi_tp_funcs {
  pub SubmitCredRequest: Option<
    extern "C" fn(
      isize,
      *mut CSSM_TP_AUTHORITY_ID,
      u32,
      *mut CSSM_TP_REQUEST_SET,
      *mut CSSM_TP_CALLERAUTH_CONTEXT,
      *mut i32,
      *mut cssm_data,
    ) -> i32,
  >,
  pub RetrieveCredResult: Option<
    extern "C" fn(
      isize,
      *mut CSSM_DATA,
      *mut CSSM_TP_CALLERAUTH_CONTEXT,
      *mut i32,
      *mut i32,
      *mut *mut cssm_tp_result_set,
    ) -> i32,
  >,
  pub ConfirmCredResult: Option<
    extern "C" fn(
      isize,
      *mut CSSM_DATA,
      *mut CSSM_TP_CALLERAUTH_CONTEXT,
      *mut CSSM_TP_CONFIRM_RESPONSE,
      *mut CSSM_TP_AUTHORITY_ID,
    ) -> i32,
  >,
  pub ReceiveConfirmation: Option<
    extern "C" fn(isize, *mut CSSM_DATA, *mut *mut cssm_tp_confirm_response, *mut i32) -> i32,
  >,
  pub CertReclaimKey: Option<
    extern "C" fn(
      isize,
      *mut CSSM_CERTGROUP,
      u32,
      u64,
      isize,
      *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    ) -> i32,
  >,
  pub CertReclaimAbort: Option<extern "C" fn(isize, u64) -> i32>,
  pub FormRequest:
    Option<extern "C" fn(isize, *mut CSSM_TP_AUTHORITY_ID, u32, *mut cssm_data) -> i32>,
  pub FormSubmit: Option<
    extern "C" fn(
      isize,
      u32,
      *mut CSSM_DATA,
      *mut CSSM_TP_AUTHORITY_ID,
      *mut CSSM_TP_AUTHORITY_ID,
      *mut cssm_access_credentials,
    ) -> i32,
  >,
  pub CertGroupVerify: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
    ) -> i32,
  >,
  pub CertCreateTemplate:
    Option<extern "C" fn(isize, isize, u32, *mut CSSM_FIELD, *mut cssm_data) -> i32>,
  pub CertGetAllTemplateFields:
    Option<extern "C" fn(isize, isize, *mut CSSM_DATA, *mut u32, *mut *mut cssm_field) -> i32>,
  pub CertSign: Option<
    extern "C" fn(
      isize,
      isize,
      u64,
      *mut CSSM_DATA,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
      *mut cssm_data,
    ) -> i32,
  >,
  pub CrlVerify: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_ENCODED_CRL,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
    ) -> i32,
  >,
  pub CrlCreateTemplate:
    Option<extern "C" fn(isize, isize, u32, *mut CSSM_FIELD, *mut cssm_data) -> i32>,
  pub CertRevoke: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_DATA,
      *mut CSSM_CERTGROUP,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
      u32,
      *mut cssm_data,
    ) -> i32,
  >,
  pub CertRemoveFromCrlTemplate: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_DATA,
      *mut CSSM_CERTGROUP,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
      *mut cssm_data,
    ) -> i32,
  >,
  pub CrlSign: Option<
    extern "C" fn(
      isize,
      isize,
      u64,
      *mut CSSM_ENCODED_CRL,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
      *mut cssm_data,
    ) -> i32,
  >,
  pub ApplyCrlToDb: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_ENCODED_CRL,
      *mut CSSM_CERTGROUP,
      *mut CSSM_TP_VERIFY_CONTEXT,
      *mut cssm_tp_verify_context_result,
    ) -> i32,
  >,
  pub CertGroupConstruct: Option<
    extern "C" fn(
      isize,
      isize,
      isize,
      *mut CSSM_DL_DB_LIST,
      *mut c_void,
      *mut CSSM_CERTGROUP,
      *mut *mut cssm_certgroup,
    ) -> i32,
  >,
  pub CertGroupPrune: Option<
    extern "C" fn(
      isize,
      isize,
      *mut CSSM_DL_DB_LIST,
      *mut CSSM_CERTGROUP,
      *mut *mut cssm_certgroup,
    ) -> i32,
  >,
  pub CertGroupToTupleGroup:
    Option<extern "C" fn(isize, isize, *mut CSSM_CERTGROUP, *mut *mut cssm_tuplegroup) -> i32>,
  pub TupleGroupToCertGroup:
    Option<extern "C" fn(isize, isize, *mut CSSM_TUPLEGROUP, *mut *mut cssm_certgroup) -> i32>,
  pub PassThrough: Option<
    extern "C" fn(
      isize,
      isize,
      u64,
      *mut CSSM_DL_DB_LIST,
      u32,
      *mut c_void,
      *mut *mut c_void,
    ) -> i32,
  >,
}
pub type CSSM_SPI_TP_FUNCS = cssm_spi_tp_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_state_funcs {
  pub cssm_GetAttachFunctions:
    Option<extern "C" fn(isize, u32, *mut *mut c_void, *mut cssm_guid, *mut i32) -> i32>,
  pub cssm_ReleaseAttachFunctions: Option<extern "C" fn(isize) -> i32>,
  pub cssm_GetAppMemoryFunctions: Option<extern "C" fn(isize, *mut cssm_upcalls) -> i32>,
  pub cssm_IsFuncCallValid: Option<
    extern "C" fn(
      isize,
      Option<extern "C" fn() -> ()>,
      Option<extern "C" fn() -> ()>,
      u64,
      *mut u64,
      u32,
      *mut i32,
    ) -> i32,
  >,
  pub cssm_DeregisterManagerServices: Option<extern "C" fn(*mut CSSM_GUID) -> i32>,
  pub cssm_DeliverModuleManagerEvent:
    Option<extern "C" fn(*mut CSSM_MANAGER_EVENT_NOTIFICATION) -> i32>,
}
pub type CSSM_STATE_FUNCS = cssm_state_funcs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cssm_manager_registration_info {
  pub Initialize: Option<extern "C" fn(u32, u32) -> i32>,
  pub Terminate: Option<extern "C" fn() -> i32>,
  pub RegisterDispatchTable: Option<extern "C" fn(*mut cssm_state_funcs) -> i32>,
  pub DeregisterDispatchTable: Option<extern "C" fn() -> i32>,
  pub EventNotifyManager: Option<extern "C" fn(*mut CSSM_MANAGER_EVENT_NOTIFICATION) -> i32>,
  pub RefreshFunctionTable: Option<extern "C" fn(*mut cssm_func_name_addr, u32) -> i32>,
}
pub type CSSM_MANAGER_REGISTRATION_INFO = cssm_manager_registration_info;
pub type MDS_HANDLE = isize;
pub type MDS_DB_HANDLE = CSSM_DL_DB_HANDLE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mds_funcs {
  pub DbOpen: Option<
    extern "C" fn(
      isize,
      *mut i8,
      *mut CSSM_NET_ADDRESS,
      u32,
      *mut CSSM_ACCESS_CREDENTIALS,
      *mut c_void,
      *mut isize,
    ) -> i32,
  >,
  pub DbClose: Option<extern "C" fn(CSSM_DL_DB_HANDLE) -> i32>,
  pub GetDbNames: Option<extern "C" fn(isize, *mut *mut cssm_name_list) -> i32>,
  pub GetDbNameFromHandle: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut *mut i8) -> i32>,
  pub FreeNameList: Option<extern "C" fn(isize, *mut cssm_name_list) -> i32>,
  pub DataInsert: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
      *mut CSSM_DATA,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataDelete: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut CSSM_DB_UNIQUE_RECORD) -> i32>,
  pub DataModify: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut cssm_db_unique_record,
      *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
      *mut CSSM_DATA,
      u32,
    ) -> i32,
  >,
  pub DataGetFirst: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      *mut CSSM_QUERY,
      *mut isize,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataGetNext: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      isize,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
      *mut *mut cssm_db_unique_record,
    ) -> i32,
  >,
  pub DataAbortQuery: Option<extern "C" fn(CSSM_DL_DB_HANDLE, isize) -> i32>,
  pub DataGetFromUniqueRecordId: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      *mut CSSM_DB_UNIQUE_RECORD,
      *mut cssm_db_record_attribute_data,
      *mut cssm_data,
    ) -> i32,
  >,
  pub FreeUniqueRecord: Option<extern "C" fn(CSSM_DL_DB_HANDLE, *mut cssm_db_unique_record) -> i32>,
  pub CreateRelation: Option<
    extern "C" fn(
      CSSM_DL_DB_HANDLE,
      u32,
      *mut i8,
      u32,
      *mut CSSM_DB_SCHEMA_ATTRIBUTE_INFO,
      u32,
      *mut CSSM_DB_SCHEMA_INDEX_INFO,
    ) -> i32,
  >,
  pub DestroyRelation: Option<extern "C" fn(CSSM_DL_DB_HANDLE, u32) -> i32>,
}
pub type MDS_FUNCS = mds_funcs;
pub type SecAsn1Item = CSSM_DATA;
pub type SecAsn1Oid = CSSM_DATA;
pub type SecAsn1AlgId = CSSM_X509_ALGORITHM_IDENTIFIER;
pub type SecAsn1PubKeyInfo = CSSM_X509_SUBJECT_PUBLIC_KEY_INFO;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecAsn1Template_struct {
  pub kind: u32,
  pub offset: u32,
  pub sub: *mut c_void,
  pub size: u32,
}
pub type SecAsn1Template = SecAsn1Template_struct;
bitflags! { # [ repr ( C ) ] pub struct SecKeychainPromptSelector : u16 { const kSecKeychainPromptRequirePassphase = 1 ; const kSecKeychainPromptUnsigned = 16 ; const kSecKeychainPromptUnsignedAct = 32 ; const kSecKeychainPromptInvalid = 64 ; const kSecKeychainPromptInvalidAct = 128 ; } }
#[repr(C)]
pub struct OpaqueSecIdentitySearchRef {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecItemClass {
  kSecInternetPasswordItemClass = 1768842612,
  kSecGenericPasswordItemClass = 1734700656,
  kSecAppleSharePasswordItemClass = 1634953328,
  kSecCertificateItemClass = 2147487744,
  kSecPublicKeyItemClass = 15,
  kSecPrivateKeyItemClass = 16,
  kSecSymmetricKeyItemClass = 17,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecItemAttr {
  kSecCreationDateItemAttr = 1667522932,
  kSecModDateItemAttr = 1835295092,
  kSecDescriptionItemAttr = 1684370275,
  kSecCommentItemAttr = 1768123764,
  kSecCreatorItemAttr = 1668445298,
  kSecTypeItemAttr = 1954115685,
  kSecScriptCodeItemAttr = 1935897200,
  kSecLabelItemAttr = 1818321516,
  kSecInvisibleItemAttr = 1768846953,
  kSecNegativeItemAttr = 1852139361,
  kSecCustomIconItemAttr = 1668641641,
  kSecAccountItemAttr = 1633903476,
  kSecServiceItemAttr = 1937138533,
  kSecGenericItemAttr = 1734700641,
  kSecSecurityDomainItemAttr = 1935961454,
  kSecServerItemAttr = 1936881266,
  kSecAuthenticationTypeItemAttr = 1635023216,
  kSecPortItemAttr = 1886351988,
  kSecPathItemAttr = 1885434984,
  kSecVolumeItemAttr = 1986817381,
  kSecAddressItemAttr = 1633969266,
  kSecSignatureItemAttr = 1936943463,
  kSecProtocolItemAttr = 1886675820,
  kSecCertificateType = 1668577648,
  kSecCertificateEncoding = 1667591779,
  kSecCrlType = 1668445296,
  kSecCrlEncoding = 1668443747,
  kSecAlias = 1634494835,
}
#[repr(C)]
pub struct OpaquePolicySearchRef {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct SecTrustSettingsKeyUsage : u32 { const kSecTrustSettingsKeyUseSignature = 1 ; const kSecTrustSettingsKeyUseEnDecryptData = 2 ; const kSecTrustSettingsKeyUseEnDecryptKey = 4 ; const kSecTrustSettingsKeyUseSignCert = 8 ; const kSecTrustSettingsKeyUseSignRevocation = 16 ; const kSecTrustSettingsKeyUseKeyExchange = 32 ; const kSecTrustSettingsKeyUseAny = 4294967295 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecTrustSettingsResult {
  kSecTrustSettingsResultInvalid = 0,
  kSecTrustSettingsResultTrustRoot = 1,
  kSecTrustSettingsResultTrustAsRoot = 2,
  kSecTrustSettingsResultDeny = 3,
  kSecTrustSettingsResultUnspecified = 4,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecTrustSettingsDomain {
  kSecTrustSettingsDomainUser = 0,
  kSecTrustSettingsDomainAdmin = 1,
  kSecTrustSettingsDomainSystem = 2,
}
#[repr(C)]
pub struct __SecCode {
  opaque: u32,
}
#[repr(C)]
pub struct __SecRequirement {
  opaque: u32,
}
pub type SecGuestRef = u32;
bitflags! { # [ repr ( C ) ] pub struct SecCSFlags : u32 { const kSecCSDefaultFlags = 0 ; const kSecCSConsiderExpiration = 2147483648 ; const kSecCSEnforceRevocationChecks = 1073741824 ; const kSecCSNoNetworkAccess = 536870912 ; const kSecCSReportProgress = 268435456 ; const kSecCSCheckTrustedAnchors = 134217728 ; const kSecCSQuickCheck = 67108864 ; } }
bitflags! { # [ repr ( C ) ] pub struct SecCodeSignatureFlags : u32 { const kSecCodeSignatureHost = 1 ; const kSecCodeSignatureAdhoc = 2 ; const kSecCodeSignatureForceHard = 256 ; const kSecCodeSignatureForceKill = 512 ; const kSecCodeSignatureForceExpiration = 1024 ; const kSecCodeSignatureRestrict = 2048 ; const kSecCodeSignatureEnforcement = 4096 ; const kSecCodeSignatureLibraryValidation = 8192 ; const kSecCodeSignatureRuntime = 65536 ; } }
bitflags! { # [ repr ( C ) ] pub struct SecCodeStatus : u32 { const kSecCodeStatusValid = 1 ; const kSecCodeStatusHard = 256 ; const kSecCodeStatusKill = 512 ; const kSecCodeStatusDebugged = 268435456 ; const kSecCodeStatusPlatform = 67108864 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecRequirementType {
  kSecHostRequirementType = 1,
  kSecGuestRequirementType = 2,
  kSecDesignatedRequirementType = 3,
  kSecLibraryRequirementType = 4,
  kSecPluginRequirementType = 5,
  kSecInvalidRequirementType = 6,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SecCSDigestAlgorithm {
  kSecCodeSignatureNoHash = 0,
  kSecCodeSignatureHashSHA1 = 1,
  kSecCodeSignatureHashSHA256 = 2,
  kSecCodeSignatureHashSHA256Truncated = 3,
  kSecCodeSignatureHashSHA384 = 4,
  kSecCodeSignatureHashSHA512 = 5,
}
#[repr(C)]
pub struct __SecTask {
  opaque: u32,
}
#[repr(C)]
pub struct _CMSDecoder {
  opaque: u32,
}
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CMSSignerStatus {
  kCMSSignerUnsigned = 0,
  kCMSSignerValid = 1,
  kCMSSignerNeedsDetachedContent = 2,
  kCMSSignerInvalidSignature = 3,
  kCMSSignerInvalidCert = 4,
  kCMSSignerInvalidIndex = 5,
}
#[repr(C)]
pub struct _CMSEncoder {
  opaque: u32,
}
bitflags! { # [ repr ( C ) ] pub struct CMSSignedAttributes : u32 { const kCMSAttrNone = 0 ; const kCMSAttrSmimeCapabilities = 1 ; const kCMSAttrSmimeEncryptionKeyPrefs = 2 ; const kCMSAttrSmimeMSEncryptionKeyPrefs = 4 ; const kCMSAttrSigningTime = 8 ; const kCMSAttrAppleCodesigningHashAgility = 16 ; const kCMSAttrAppleCodesigningHashAgilityV2 = 32 ; const kCMSAttrAppleExpirationTime = 64 ; } }
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CMSCertificateChainMode {
  kCMSCertificateNone = 0,
  kCMSCertificateSignerOnly = 1,
  kCMSCertificateChain = 2,
  kCMSCertificateChainWithRoot = 3,
}
pub type SecTransformRef = *mut c_void;
pub type SecGroupTransformRef = *mut c_void;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum SecTransformMetaAttributeType {
  kSecTransformMetaAttributeValue = 0,
  kSecTransformMetaAttributeName = 1,
  kSecTransformMetaAttributeRef = 2,
  kSecTransformMetaAttributeRequired = 3,
  kSecTransformMetaAttributeRequiresOutboundConnection = 4,
  kSecTransformMetaAttributeDeferred = 5,
  kSecTransformMetaAttributeStream = 6,
  kSecTransformMetaAttributeCanCycle = 7,
  kSecTransformMetaAttributeExternalize = 8,
  kSecTransformMetaAttributeHasOutboundConnections = 9,
  kSecTransformMetaAttributeHasInboundConnection = 10,
}
pub type SecTransformAttributeRef = *mut c_void;
pub type SecTransformStringOrAttributeRef = *mut c_void;
#[repr(C)]
pub struct OpaqueSecTransformImplementation {
  opaque: u32,
}
pub type DERByte = u8;
pub type DERSize = usize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DERItem {
  pub data: *mut u8,
  pub length: usize,
}
#[cfg(feature = "RK_Security")]
#[link(name = "Security", kind = "framework")]
extern "C" {
  pub fn SecCopyErrorMessageString(status: i32, reserved: *mut c_void) -> *mut __CFString;
  pub fn SecCertificateGetTypeID() -> usize;
  pub fn SecCertificateCreateWithData(
    allocator: *mut __CFAllocator,
    data: *mut __CFData,
  ) -> *mut OpaqueSecCertificateRef;
  pub fn SecCertificateCopyData(certificate: *mut OpaqueSecCertificateRef) -> *mut __CFData;
  pub fn SecCertificateCopySubjectSummary(
    certificate: *mut OpaqueSecCertificateRef,
  ) -> *mut __CFString;
  pub fn SecCertificateCopyCommonName(
    certificate: *mut OpaqueSecCertificateRef,
    commonName: *mut *mut __CFString,
  ) -> i32;
  pub fn SecCertificateCopyEmailAddresses(
    certificate: *mut OpaqueSecCertificateRef,
    emailAddresses: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecCertificateCopyNormalizedIssuerSequence(
    certificate: *mut OpaqueSecCertificateRef,
  ) -> *mut __CFData;
  pub fn SecCertificateCopyNormalizedSubjectSequence(
    certificate: *mut OpaqueSecCertificateRef,
  ) -> *mut __CFData;
  pub fn SecCertificateCopyKey(certificate: *mut OpaqueSecCertificateRef) -> *mut OpaqueSecKeyRef;
  pub fn SecCertificateCopyPublicKey(
    certificate: *mut OpaqueSecCertificateRef,
    key: *mut *mut OpaqueSecKeyRef,
  ) -> i32;
  pub fn SecCertificateCopySerialNumberData(
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecCertificateCopySerialNumber(
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecCertificateCreateFromData(
    data: *mut CSSM_DATA,
    type_: u32,
    encoding: u32,
    certificate: *mut *mut OpaqueSecCertificateRef,
  ) -> i32;
  pub fn SecCertificateAddToKeychain(
    certificate: *mut OpaqueSecCertificateRef,
    keychain: *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn SecCertificateGetData(
    certificate: *mut OpaqueSecCertificateRef,
    data: *mut cssm_data,
  ) -> i32;
  pub fn SecCertificateGetType(
    certificate: *mut OpaqueSecCertificateRef,
    certificateType: *mut u32,
  ) -> i32;
  pub fn SecCertificateGetSubject(
    certificate: *mut OpaqueSecCertificateRef,
    subject: *mut *mut CSSM_X509_NAME,
  ) -> i32;
  pub fn SecCertificateGetIssuer(
    certificate: *mut OpaqueSecCertificateRef,
    issuer: *mut *mut CSSM_X509_NAME,
  ) -> i32;
  pub fn SecCertificateGetCLHandle(
    certificate: *mut OpaqueSecCertificateRef,
    clHandle: *mut isize,
  ) -> i32;
  pub fn SecCertificateGetAlgorithmID(
    certificate: *mut OpaqueSecCertificateRef,
    algid: *mut *mut CSSM_X509_ALGORITHM_IDENTIFIER,
  ) -> i32;
  pub fn SecCertificateCopyPreference(
    name: *mut __CFString,
    keyUsage: u32,
    certificate: *mut *mut OpaqueSecCertificateRef,
  ) -> i32;
  pub fn SecCertificateCopyPreferred(
    name: *mut __CFString,
    keyUsage: *mut __CFArray,
  ) -> *mut OpaqueSecCertificateRef;
  pub fn SecCertificateSetPreference(
    certificate: *mut OpaqueSecCertificateRef,
    name: *mut __CFString,
    keyUsage: u32,
    date: *mut __CFDate,
  ) -> i32;
  pub fn SecCertificateSetPreferred(
    certificate: *mut OpaqueSecCertificateRef,
    name: *mut __CFString,
    keyUsage: *mut __CFArray,
  ) -> i32;
  pub fn SecCertificateCopyValues(
    certificate: *mut OpaqueSecCertificateRef,
    keys: *mut __CFArray,
    error: *mut *mut __CFError,
  ) -> *mut __CFDictionary;
  pub fn SecCertificateCopyLongDescription(
    alloc: *mut __CFAllocator,
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFString;
  pub fn SecCertificateCopyShortDescription(
    alloc: *mut __CFAllocator,
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFString;
  pub fn SecCertificateCopyNormalizedIssuerContent(
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecCertificateCopyNormalizedSubjectContent(
    certificate: *mut OpaqueSecCertificateRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecIdentityGetTypeID() -> usize;
  pub fn SecIdentityCreateWithCertificate(
    keychainOrArray: *mut c_void,
    certificateRef: *mut OpaqueSecCertificateRef,
    identityRef: *mut *mut OpaqueSecIdentityRef,
  ) -> i32;
  pub fn SecIdentityCopyCertificate(
    identityRef: *mut OpaqueSecIdentityRef,
    certificateRef: *mut *mut OpaqueSecCertificateRef,
  ) -> i32;
  pub fn SecIdentityCopyPrivateKey(
    identityRef: *mut OpaqueSecIdentityRef,
    privateKeyRef: *mut *mut OpaqueSecKeyRef,
  ) -> i32;
  pub fn SecIdentityCopyPreference(
    name: *mut __CFString,
    keyUsage: u32,
    validIssuers: *mut __CFArray,
    identity: *mut *mut OpaqueSecIdentityRef,
  ) -> i32;
  pub fn SecIdentityCopyPreferred(
    name: *mut __CFString,
    keyUsage: *mut __CFArray,
    validIssuers: *mut __CFArray,
  ) -> *mut OpaqueSecIdentityRef;
  pub fn SecIdentitySetPreference(
    identity: *mut OpaqueSecIdentityRef,
    name: *mut __CFString,
    keyUsage: u32,
  ) -> i32;
  pub fn SecIdentitySetPreferred(
    identity: *mut OpaqueSecIdentityRef,
    name: *mut __CFString,
    keyUsage: *mut __CFArray,
  ) -> i32;
  pub fn SecIdentityCopySystemIdentity(
    domain: *mut __CFString,
    idRef: *mut *mut OpaqueSecIdentityRef,
    actualDomain: *mut *mut __CFString,
  ) -> i32;
  pub fn SecIdentitySetSystemIdentity(
    domain: *mut __CFString,
    idRef: *mut OpaqueSecIdentityRef,
  ) -> i32;
  pub fn SecAccessControlGetTypeID() -> usize;
  pub fn SecAccessControlCreateWithFlags(
    allocator: *mut __CFAllocator,
    protection: *mut c_void,
    flags: SecAccessControlCreateFlags,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecAccessControlRef;
  pub fn SecItemCopyMatching(query: *mut __CFDictionary, result: *mut *mut c_void) -> i32;
  pub fn SecItemAdd(attributes: *mut __CFDictionary, result: *mut *mut c_void) -> i32;
  pub fn SecItemUpdate(query: *mut __CFDictionary, attributesToUpdate: *mut __CFDictionary) -> i32;
  pub fn SecItemDelete(query: *mut __CFDictionary) -> i32;
  pub fn SecAccessGetTypeID() -> usize;
  pub fn SecAccessCreate(
    descriptor: *mut __CFString,
    trustedlist: *mut __CFArray,
    accessRef: *mut *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecAccessCreateFromOwnerAndACL(
    owner: *mut CSSM_ACL_OWNER_PROTOTYPE,
    aclCount: u32,
    acls: *mut CSSM_ACL_ENTRY_INFO,
    accessRef: *mut *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecAccessCreateWithOwnerAndACL(
    userId: u32,
    groupId: u32,
    ownerType: u32,
    acls: *mut __CFArray,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecAccessRef;
  pub fn SecAccessGetOwnerAndACL(
    accessRef: *mut OpaqueSecAccessRef,
    owner: *mut *mut cssm_acl_owner_prototype,
    aclCount: *mut u32,
    acls: *mut *mut cssm_acl_entry_info,
  ) -> i32;
  pub fn SecAccessCopyOwnerAndACL(
    accessRef: *mut OpaqueSecAccessRef,
    userId: *mut u32,
    groupId: *mut u32,
    ownerType: *mut u32,
    aclList: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecAccessCopyACLList(
    accessRef: *mut OpaqueSecAccessRef,
    aclList: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecAccessCopySelectedACLList(
    accessRef: *mut OpaqueSecAccessRef,
    action: i32,
    aclList: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecAccessCopyMatchingACLList(
    accessRef: *mut OpaqueSecAccessRef,
    authorizationTag: *mut c_void,
  ) -> *mut __CFArray;
  pub fn SecKeyGetTypeID() -> usize;
  pub fn SecKeyCreatePair(
    keychainRef: *mut OpaqueSecKeychainRef,
    algorithm: u32,
    keySizeInBits: u32,
    contextHandle: u64,
    publicKeyUsage: u32,
    publicKeyAttr: u32,
    privateKeyUsage: u32,
    privateKeyAttr: u32,
    initialAccess: *mut OpaqueSecAccessRef,
    publicKey: *mut *mut OpaqueSecKeyRef,
    privateKey: *mut *mut OpaqueSecKeyRef,
  ) -> i32;
  pub fn SecKeyGenerate(
    keychainRef: *mut OpaqueSecKeychainRef,
    algorithm: u32,
    keySizeInBits: u32,
    contextHandle: u64,
    keyUsage: u32,
    keyAttr: u32,
    initialAccess: *mut OpaqueSecAccessRef,
    keyRef: *mut *mut OpaqueSecKeyRef,
  ) -> i32;
  pub fn SecKeyGetCSSMKey(key: *mut OpaqueSecKeyRef, cssmKey: *mut *mut CSSM_KEY) -> i32;
  pub fn SecKeyGetCSPHandle(keyRef: *mut OpaqueSecKeyRef, cspHandle: *mut isize) -> i32;
  pub fn SecKeyGetCredentials(
    keyRef: *mut OpaqueSecKeyRef,
    operation: i32,
    credentialType: SecCredentialType,
    outCredentials: *mut *mut CSSM_ACCESS_CREDENTIALS,
  ) -> i32;
  pub fn SecKeyGenerateSymmetric(
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyCreateFromData(
    parameters: *mut __CFDictionary,
    keyData: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyGeneratePairAsync(
    parameters: *mut __CFDictionary,
    deliveryQueue: *mut NSObject,
    result: (),
  ) -> ();
  pub fn SecKeyDeriveFromPassword(
    password: *mut __CFString,
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyWrapSymmetric(
    keyToWrap: *mut OpaqueSecKeyRef,
    wrappingKey: *mut OpaqueSecKeyRef,
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyUnwrapSymmetric(
    keyToUnwrap: *mut *mut __CFData,
    unwrappingKey: *mut OpaqueSecKeyRef,
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyGeneratePair(
    parameters: *mut __CFDictionary,
    publicKey: *mut *mut OpaqueSecKeyRef,
    privateKey: *mut *mut OpaqueSecKeyRef,
  ) -> i32;
  pub fn SecKeyCreateRandomKey(
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyCreateWithData(
    keyData: *mut __CFData,
    attributes: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyGetBlockSize(key: *mut OpaqueSecKeyRef) -> usize;
  pub fn SecKeyCopyExternalRepresentation(
    key: *mut OpaqueSecKeyRef,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyCopyAttributes(key: *mut OpaqueSecKeyRef) -> *mut __CFDictionary;
  pub fn SecKeyCopyPublicKey(key: *mut OpaqueSecKeyRef) -> *mut OpaqueSecKeyRef;
  pub fn SecKeyCreateSignature(
    key: *mut OpaqueSecKeyRef,
    algorithm: *mut __CFString,
    dataToSign: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyVerifySignature(
    key: *mut OpaqueSecKeyRef,
    algorithm: *mut __CFString,
    signedData: *mut __CFData,
    signature: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn SecKeyCreateEncryptedData(
    key: *mut OpaqueSecKeyRef,
    algorithm: *mut __CFString,
    plaintext: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyCreateDecryptedData(
    key: *mut OpaqueSecKeyRef,
    algorithm: *mut __CFString,
    ciphertext: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyCopyKeyExchangeResult(
    privateKey: *mut OpaqueSecKeyRef,
    algorithm: *mut __CFString,
    publicKey: *mut OpaqueSecKeyRef,
    parameters: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut __CFData;
  pub fn SecKeyIsAlgorithmSupported(
    key: *mut OpaqueSecKeyRef,
    operation: SecKeyOperationType,
    algorithm: *mut __CFString,
  ) -> u8;
  pub fn SecPolicyGetTypeID() -> usize;
  pub fn SecPolicyCopyProperties(policyRef: *mut OpaqueSecPolicyRef) -> *mut __CFDictionary;
  pub fn SecPolicyCreateBasicX509() -> *mut OpaqueSecPolicyRef;
  pub fn SecPolicyCreateSSL(server: u8, hostname: *mut __CFString) -> *mut OpaqueSecPolicyRef;
  pub fn SecPolicyCreateRevocation(revocationFlags: usize) -> *mut OpaqueSecPolicyRef;
  pub fn SecPolicyCreateWithProperties(
    policyIdentifier: *mut c_void,
    properties: *mut __CFDictionary,
  ) -> *mut OpaqueSecPolicyRef;
  pub fn SecPolicyCreateWithOID(policyOID: *mut c_void) -> *mut OpaqueSecPolicyRef;
  pub fn SecPolicyGetOID(policyRef: *mut OpaqueSecPolicyRef, oid: *mut CSSM_DATA) -> i32;
  pub fn SecPolicyGetValue(policyRef: *mut OpaqueSecPolicyRef, value: *mut CSSM_DATA) -> i32;
  pub fn SecPolicySetValue(policyRef: *mut OpaqueSecPolicyRef, value: *mut CSSM_DATA) -> i32;
  pub fn SecPolicySetProperties(
    policyRef: *mut OpaqueSecPolicyRef,
    properties: *mut __CFDictionary,
  ) -> i32;
  pub fn SecPolicyGetTPHandle(policyRef: *mut OpaqueSecPolicyRef, tpHandle: *mut isize) -> i32;
  pub fn SecRandomCopyBytes(rnd: *mut __SecRandom, count: usize, bytes: *mut c_void) -> i32;
  pub fn cssmPerror(how: *mut i8, error: i32) -> ();
  pub fn cssmOidToAlg(oid: *mut CSSM_DATA, alg: *mut u32) -> bool;
  pub fn cssmAlgToOid(algId: u32) -> *mut CSSM_DATA;
  pub fn SecKeychainGetTypeID() -> usize;
  pub fn SecKeychainGetVersion(returnVers: *mut u32) -> i32;
  pub fn SecKeychainOpen(pathName: *mut i8, keychain: *mut *mut OpaqueSecKeychainRef) -> i32;
  pub fn SecKeychainCreate(
    pathName: *mut i8,
    passwordLength: u32,
    password: *mut c_void,
    promptUser: u8,
    initialAccess: *mut OpaqueSecAccessRef,
    keychain: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn SecKeychainDelete(keychainOrArray: *mut OpaqueSecKeychainRef) -> i32;
  pub fn SecKeychainSetSettings(
    keychain: *mut OpaqueSecKeychainRef,
    newSettings: *mut SecKeychainSettings,
  ) -> i32;
  pub fn SecKeychainCopySettings(
    keychain: *mut OpaqueSecKeychainRef,
    outSettings: *mut SecKeychainSettings,
  ) -> i32;
  pub fn SecKeychainUnlock(
    keychain: *mut OpaqueSecKeychainRef,
    passwordLength: u32,
    password: *mut c_void,
    usePassword: u8,
  ) -> i32;
  pub fn SecKeychainLock(keychain: *mut OpaqueSecKeychainRef) -> i32;
  pub fn SecKeychainLockAll() -> i32;
  pub fn SecKeychainCopyDefault(keychain: *mut *mut OpaqueSecKeychainRef) -> i32;
  pub fn SecKeychainSetDefault(keychain: *mut OpaqueSecKeychainRef) -> i32;
  pub fn SecKeychainCopySearchList(searchList: *mut *mut __CFArray) -> i32;
  pub fn SecKeychainSetSearchList(searchList: *mut __CFArray) -> i32;
  pub fn SecKeychainCopyDomainDefault(
    domain: SecPreferencesDomain,
    keychain: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn SecKeychainSetDomainDefault(
    domain: SecPreferencesDomain,
    keychain: *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn SecKeychainCopyDomainSearchList(
    domain: SecPreferencesDomain,
    searchList: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecKeychainSetDomainSearchList(
    domain: SecPreferencesDomain,
    searchList: *mut __CFArray,
  ) -> i32;
  pub fn SecKeychainSetPreferenceDomain(domain: SecPreferencesDomain) -> i32;
  pub fn SecKeychainGetPreferenceDomain(domain: *mut SecPreferencesDomain) -> i32;
  pub fn SecKeychainGetStatus(keychain: *mut OpaqueSecKeychainRef, keychainStatus: *mut u32)
    -> i32;
  pub fn SecKeychainGetPath(
    keychain: *mut OpaqueSecKeychainRef,
    ioPathLength: *mut u32,
    pathName: *mut i8,
  ) -> i32;
  pub fn SecKeychainAttributeInfoForItemID(
    keychain: *mut OpaqueSecKeychainRef,
    itemID: u32,
    info: *mut *mut SecKeychainAttributeInfo,
  ) -> i32;
  pub fn SecKeychainFreeAttributeInfo(info: *mut SecKeychainAttributeInfo) -> i32;
  pub fn SecKeychainAddCallback(
    callbackFunction: extern "C" fn(
      SecKeychainEvent,
      *mut SecKeychainCallbackInfo,
      *mut c_void,
    ) -> i32,
    eventMask: SecKeychainEventMask,
    userContext: *mut c_void,
  ) -> i32;
  pub fn SecKeychainRemoveCallback(
    callbackFunction: extern "C" fn(
      SecKeychainEvent,
      *mut SecKeychainCallbackInfo,
      *mut c_void,
    ) -> i32,
  ) -> i32;
  pub fn SecKeychainAddInternetPassword(
    keychain: *mut OpaqueSecKeychainRef,
    serverNameLength: u32,
    serverName: *mut i8,
    securityDomainLength: u32,
    securityDomain: *mut i8,
    accountNameLength: u32,
    accountName: *mut i8,
    pathLength: u32,
    path: *mut i8,
    port: u16,
    protocol: SecProtocolType,
    authenticationType: SecAuthenticationType,
    passwordLength: u32,
    passwordData: *mut c_void,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainFindInternetPassword(
    keychainOrArray: *mut c_void,
    serverNameLength: u32,
    serverName: *mut i8,
    securityDomainLength: u32,
    securityDomain: *mut i8,
    accountNameLength: u32,
    accountName: *mut i8,
    pathLength: u32,
    path: *mut i8,
    port: u16,
    protocol: SecProtocolType,
    authenticationType: SecAuthenticationType,
    passwordLength: *mut u32,
    passwordData: *mut *mut c_void,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainAddGenericPassword(
    keychain: *mut OpaqueSecKeychainRef,
    serviceNameLength: u32,
    serviceName: *mut i8,
    accountNameLength: u32,
    accountName: *mut i8,
    passwordLength: u32,
    passwordData: *mut c_void,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainFindGenericPassword(
    keychainOrArray: *mut c_void,
    serviceNameLength: u32,
    serviceName: *mut i8,
    accountNameLength: u32,
    accountName: *mut i8,
    passwordLength: *mut u32,
    passwordData: *mut *mut c_void,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainSetUserInteractionAllowed(state: u8) -> i32;
  pub fn SecKeychainGetUserInteractionAllowed(state: *mut u8) -> i32;
  pub fn SecKeychainGetCSPHandle(keychain: *mut OpaqueSecKeychainRef, cspHandle: *mut isize)
    -> i32;
  pub fn SecKeychainGetDLDBHandle(
    keychain: *mut OpaqueSecKeychainRef,
    dldbHandle: *mut CSSM_DL_DB_HANDLE,
  ) -> i32;
  pub fn SecKeychainCopyAccess(
    keychain: *mut OpaqueSecKeychainRef,
    access: *mut *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecKeychainSetAccess(
    keychain: *mut OpaqueSecKeychainRef,
    access: *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecKeychainItemExport(
    keychainItemOrArray: *mut c_void,
    outputFormat: SecExternalFormat,
    flags: SecItemImportExportFlags,
    keyParams: *mut SecKeyImportExportParameters,
    exportedData: *mut *mut __CFData,
  ) -> i32;
  pub fn SecItemExport(
    secItemOrArray: *mut c_void,
    outputFormat: SecExternalFormat,
    flags: SecItemImportExportFlags,
    keyParams: *mut SecItemImportExportKeyParameters,
    exportedData: *mut *mut __CFData,
  ) -> i32;
  pub fn SecKeychainItemImport(
    importedData: *mut __CFData,
    fileNameOrExtension: *mut __CFString,
    inputFormat: *mut SecExternalFormat,
    itemType: *mut SecExternalItemType,
    flags: SecItemImportExportFlags,
    keyParams: *mut SecKeyImportExportParameters,
    importKeychain: *mut OpaqueSecKeychainRef,
    outItems: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecItemImport(
    importedData: *mut __CFData,
    fileNameOrExtension: *mut __CFString,
    inputFormat: *mut SecExternalFormat,
    itemType: *mut SecExternalItemType,
    flags: SecItemImportExportFlags,
    keyParams: *mut SecItemImportExportKeyParameters,
    importKeychain: *mut OpaqueSecKeychainRef,
    outItems: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecPKCS12Import(
    pkcs12_data: *mut __CFData,
    options: *mut __CFDictionary,
    items: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecTrustGetTypeID() -> usize;
  pub fn SecTrustCreateWithCertificates(
    certificates: *mut c_void,
    policies: *mut c_void,
    trust: *mut *mut __SecTrust,
  ) -> i32;
  pub fn SecTrustSetPolicies(trust: *mut __SecTrust, policies: *mut c_void) -> i32;
  pub fn SecTrustCopyPolicies(trust: *mut __SecTrust, policies: *mut *mut __CFArray) -> i32;
  pub fn SecTrustSetNetworkFetchAllowed(trust: *mut __SecTrust, allowFetch: u8) -> i32;
  pub fn SecTrustGetNetworkFetchAllowed(trust: *mut __SecTrust, allowFetch: *mut u8) -> i32;
  pub fn SecTrustSetAnchorCertificates(
    trust: *mut __SecTrust,
    anchorCertificates: *mut __CFArray,
  ) -> i32;
  pub fn SecTrustSetAnchorCertificatesOnly(
    trust: *mut __SecTrust,
    anchorCertificatesOnly: u8,
  ) -> i32;
  pub fn SecTrustCopyCustomAnchorCertificates(
    trust: *mut __SecTrust,
    anchors: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecTrustSetVerifyDate(trust: *mut __SecTrust, verifyDate: *mut __CFDate) -> i32;
  pub fn SecTrustGetVerifyTime(trust: *mut __SecTrust) -> f64;
  pub fn SecTrustEvaluate(trust: *mut __SecTrust, result: *mut SecTrustResultType) -> i32;
  pub fn SecTrustEvaluateAsync(trust: *mut __SecTrust, queue: *mut NSObject, result: ()) -> i32;
  pub fn SecTrustEvaluateWithError(trust: *mut __SecTrust, error: *mut *mut __CFError) -> bool;
  pub fn SecTrustGetTrustResult(trust: *mut __SecTrust, result: *mut SecTrustResultType) -> i32;
  pub fn SecTrustCopyPublicKey(trust: *mut __SecTrust) -> *mut OpaqueSecKeyRef;
  pub fn SecTrustGetCertificateCount(trust: *mut __SecTrust) -> isize;
  pub fn SecTrustGetCertificateAtIndex(
    trust: *mut __SecTrust,
    ix: isize,
  ) -> *mut OpaqueSecCertificateRef;
  pub fn SecTrustCopyExceptions(trust: *mut __SecTrust) -> *mut __CFData;
  pub fn SecTrustSetExceptions(trust: *mut __SecTrust, exceptions: *mut __CFData) -> bool;
  pub fn SecTrustCopyProperties(trust: *mut __SecTrust) -> *mut __CFArray;
  pub fn SecTrustCopyResult(trust: *mut __SecTrust) -> *mut __CFDictionary;
  pub fn SecTrustSetOCSPResponse(trust: *mut __SecTrust, responseData: *mut c_void) -> i32;
  pub fn SecTrustSetSignedCertificateTimestamps(
    trust: *mut __SecTrust,
    sctArray: *mut __CFArray,
  ) -> i32;
  pub fn SecTrustSetOptions(trustRef: *mut __SecTrust, options: SecTrustOptionFlags) -> i32;
  pub fn SecTrustSetParameters(
    trustRef: *mut __SecTrust,
    action: u32,
    actionData: *mut __CFData,
  ) -> i32;
  pub fn SecTrustSetKeychains(trust: *mut __SecTrust, keychainOrArray: *mut c_void) -> i32;
  pub fn SecTrustGetResult(
    trustRef: *mut __SecTrust,
    result: *mut SecTrustResultType,
    certChain: *mut *mut __CFArray,
    statusChain: *mut *mut CSSM_TP_APPLE_EVIDENCE_INFO,
  ) -> i32;
  pub fn SecTrustGetCssmResult(
    trust: *mut __SecTrust,
    result: *mut *mut cssm_tp_verify_context_result,
  ) -> i32;
  pub fn SecTrustGetCssmResultCode(trust: *mut __SecTrust, resultCode: *mut i32) -> i32;
  pub fn SecTrustGetTPHandle(trust: *mut __SecTrust, handle: *mut isize) -> i32;
  pub fn SecTrustCopyAnchorCertificates(anchors: *mut *mut __CFArray) -> i32;
  pub fn SSLContextGetTypeID() -> usize;
  pub fn SSLCreateContext(
    alloc: *mut __CFAllocator,
    protocolSide: SSLProtocolSide,
    connectionType: SSLConnectionType,
  ) -> *mut SSLContext;
  pub fn SSLNewContext(isServer: u8, contextPtr: *mut *mut SSLContext) -> i32;
  pub fn SSLDisposeContext(context: *mut SSLContext) -> i32;
  pub fn SSLGetSessionState(context: *mut SSLContext, state: *mut SSLSessionState) -> i32;
  pub fn SSLSetSessionOption(context: *mut SSLContext, option: SSLSessionOption, value: u8) -> i32;
  pub fn SSLGetSessionOption(
    context: *mut SSLContext,
    option: SSLSessionOption,
    value: *mut u8,
  ) -> i32;
  pub fn SSLSetIOFuncs(
    context: *mut SSLContext,
    readFunc: extern "C" fn(*mut c_void, *mut c_void, *mut usize) -> i32,
    writeFunc: extern "C" fn(*mut c_void, *mut c_void, *mut usize) -> i32,
  ) -> i32;
  pub fn SSLSetSessionConfig(context: *mut SSLContext, config: *mut __CFString) -> i32;
  pub fn SSLSetProtocolVersionMin(context: *mut SSLContext, minVersion: SSLProtocol) -> i32;
  pub fn SSLGetProtocolVersionMin(context: *mut SSLContext, minVersion: *mut SSLProtocol) -> i32;
  pub fn SSLSetProtocolVersionMax(context: *mut SSLContext, maxVersion: SSLProtocol) -> i32;
  pub fn SSLGetProtocolVersionMax(context: *mut SSLContext, maxVersion: *mut SSLProtocol) -> i32;
  pub fn SSLSetProtocolVersionEnabled(
    context: *mut SSLContext,
    protocol: SSLProtocol,
    enable: u8,
  ) -> i32;
  pub fn SSLGetProtocolVersionEnabled(
    context: *mut SSLContext,
    protocol: SSLProtocol,
    enable: *mut u8,
  ) -> i32;
  pub fn SSLSetProtocolVersion(context: *mut SSLContext, version: SSLProtocol) -> i32;
  pub fn SSLGetProtocolVersion(context: *mut SSLContext, protocol: *mut SSLProtocol) -> i32;
  pub fn SSLSetCertificate(context: *mut SSLContext, certRefs: *mut __CFArray) -> i32;
  pub fn SSLSetConnection(context: *mut SSLContext, connection: *mut c_void) -> i32;
  pub fn SSLGetConnection(context: *mut SSLContext, connection: *mut *mut c_void) -> i32;
  pub fn SSLSetPeerDomainName(
    context: *mut SSLContext,
    peerName: *mut i8,
    peerNameLen: usize,
  ) -> i32;
  pub fn SSLGetPeerDomainNameLength(context: *mut SSLContext, peerNameLen: *mut usize) -> i32;
  pub fn SSLGetPeerDomainName(
    context: *mut SSLContext,
    peerName: *mut i8,
    peerNameLen: *mut usize,
  ) -> i32;
  pub fn SSLCopyRequestedPeerName(
    context: *mut SSLContext,
    peerName: *mut i8,
    peerNameLen: *mut usize,
  ) -> i32;
  pub fn SSLCopyRequestedPeerNameLength(ctx: *mut SSLContext, peerNameLen: *mut usize) -> i32;
  pub fn SSLSetDatagramHelloCookie(
    dtlsContext: *mut SSLContext,
    cookie: *mut c_void,
    cookieLen: usize,
  ) -> i32;
  pub fn SSLSetMaxDatagramRecordSize(dtlsContext: *mut SSLContext, maxSize: usize) -> i32;
  pub fn SSLGetMaxDatagramRecordSize(dtlsContext: *mut SSLContext, maxSize: *mut usize) -> i32;
  pub fn SSLGetNegotiatedProtocolVersion(
    context: *mut SSLContext,
    protocol: *mut SSLProtocol,
  ) -> i32;
  pub fn SSLGetNumberSupportedCiphers(context: *mut SSLContext, numCiphers: *mut usize) -> i32;
  pub fn SSLGetSupportedCiphers(
    context: *mut SSLContext,
    ciphers: *mut u32,
    numCiphers: *mut usize,
  ) -> i32;
  pub fn SSLSetEnabledCiphers(
    context: *mut SSLContext,
    ciphers: *mut u32,
    numCiphers: usize,
  ) -> i32;
  pub fn SSLGetNumberEnabledCiphers(context: *mut SSLContext, numCiphers: *mut usize) -> i32;
  pub fn SSLGetEnabledCiphers(
    context: *mut SSLContext,
    ciphers: *mut u32,
    numCiphers: *mut usize,
  ) -> i32;
  pub fn SSLSetSessionTicketsEnabled(context: *mut SSLContext, enabled: u8) -> i32;
  pub fn SSLSetEnableCertVerify(context: *mut SSLContext, enableVerify: u8) -> i32;
  pub fn SSLGetEnableCertVerify(context: *mut SSLContext, enableVerify: *mut u8) -> i32;
  pub fn SSLSetAllowsExpiredCerts(context: *mut SSLContext, allowsExpired: u8) -> i32;
  pub fn SSLGetAllowsExpiredCerts(context: *mut SSLContext, allowsExpired: *mut u8) -> i32;
  pub fn SSLSetAllowsExpiredRoots(context: *mut SSLContext, allowsExpired: u8) -> i32;
  pub fn SSLGetAllowsExpiredRoots(context: *mut SSLContext, allowsExpired: *mut u8) -> i32;
  pub fn SSLSetAllowsAnyRoot(context: *mut SSLContext, anyRoot: u8) -> i32;
  pub fn SSLGetAllowsAnyRoot(context: *mut SSLContext, anyRoot: *mut u8) -> i32;
  pub fn SSLSetTrustedRoots(
    context: *mut SSLContext,
    trustedRoots: *mut __CFArray,
    replaceExisting: u8,
  ) -> i32;
  pub fn SSLCopyTrustedRoots(context: *mut SSLContext, trustedRoots: *mut *mut __CFArray) -> i32;
  pub fn SSLCopyPeerCertificates(context: *mut SSLContext, certs: *mut *mut __CFArray) -> i32;
  pub fn SSLCopyPeerTrust(context: *mut SSLContext, trust: *mut *mut __SecTrust) -> i32;
  pub fn SSLSetPeerID(context: *mut SSLContext, peerID: *mut c_void, peerIDLen: usize) -> i32;
  pub fn SSLGetPeerID(
    context: *mut SSLContext,
    peerID: *mut *mut c_void,
    peerIDLen: *mut usize,
  ) -> i32;
  pub fn SSLGetNegotiatedCipher(context: *mut SSLContext, cipherSuite: *mut u32) -> i32;
  pub fn SSLSetALPNProtocols(context: *mut SSLContext, protocols: *mut __CFArray) -> i32;
  pub fn SSLCopyALPNProtocols(context: *mut SSLContext, protocols: *mut *mut __CFArray) -> i32;
  pub fn SSLSetOCSPResponse(context: *mut SSLContext, response: *mut __CFData) -> i32;
  pub fn SSLSetEncryptionCertificate(context: *mut SSLContext, certRefs: *mut __CFArray) -> i32;
  pub fn SSLSetClientSideAuthenticate(context: *mut SSLContext, auth: SSLAuthenticate) -> i32;
  pub fn SSLAddDistinguishedName(
    context: *mut SSLContext,
    derDN: *mut c_void,
    derDNLen: usize,
  ) -> i32;
  pub fn SSLSetCertificateAuthorities(
    context: *mut SSLContext,
    certificateOrArray: *mut c_void,
    replaceExisting: u8,
  ) -> i32;
  pub fn SSLCopyCertificateAuthorities(
    context: *mut SSLContext,
    certificates: *mut *mut __CFArray,
  ) -> i32;
  pub fn SSLCopyDistinguishedNames(context: *mut SSLContext, names: *mut *mut __CFArray) -> i32;
  pub fn SSLGetClientCertificateState(
    context: *mut SSLContext,
    clientState: *mut SSLClientCertificateState,
  ) -> i32;
  pub fn SSLSetDiffieHellmanParams(
    context: *mut SSLContext,
    dhParams: *mut c_void,
    dhParamsLen: usize,
  ) -> i32;
  pub fn SSLGetDiffieHellmanParams(
    context: *mut SSLContext,
    dhParams: *mut *mut c_void,
    dhParamsLen: *mut usize,
  ) -> i32;
  pub fn SSLSetRsaBlinding(context: *mut SSLContext, blinding: u8) -> i32;
  pub fn SSLGetRsaBlinding(context: *mut SSLContext, blinding: *mut u8) -> i32;
  pub fn SSLHandshake(context: *mut SSLContext) -> i32;
  pub fn SSLReHandshake(context: *mut SSLContext) -> i32;
  pub fn SSLWrite(
    context: *mut SSLContext,
    data: *mut c_void,
    dataLength: usize,
    processed: *mut usize,
  ) -> i32;
  pub fn SSLRead(
    context: *mut SSLContext,
    data: *mut c_void,
    dataLength: usize,
    processed: *mut usize,
  ) -> i32;
  pub fn SSLGetBufferedReadSize(context: *mut SSLContext, bufSize: *mut usize) -> i32;
  pub fn SSLGetDatagramWriteSize(dtlsContext: *mut SSLContext, bufSize: *mut usize) -> i32;
  pub fn SSLClose(context: *mut SSLContext) -> i32;
  pub fn SSLSetError(context: *mut SSLContext, status: i32) -> i32;
  pub fn sec_trust_create(trust: *mut __SecTrust) -> *mut NSObject;
  pub fn sec_trust_copy_ref(trust: *mut NSObject) -> *mut __SecTrust;
  pub fn sec_identity_create(identity: *mut OpaqueSecIdentityRef) -> *mut NSObject;
  pub fn sec_identity_create_with_certificates(
    identity: *mut OpaqueSecIdentityRef,
    certificates: *mut __CFArray,
  ) -> *mut NSObject;
  pub fn sec_identity_copy_ref(identity: *mut NSObject) -> *mut OpaqueSecIdentityRef;
  pub fn sec_identity_copy_certificates_ref(identity: *mut NSObject) -> *mut __CFArray;
  pub fn sec_certificate_create(certificate: *mut OpaqueSecCertificateRef) -> *mut NSObject;
  pub fn sec_certificate_copy_ref(certificate: *mut NSObject) -> *mut OpaqueSecCertificateRef;
  pub fn sec_protocol_metadata_get_negotiated_protocol(metadata: *mut NSObject) -> *mut i8;
  pub fn sec_protocol_metadata_copy_peer_public_key(metadata: *mut NSObject) -> *mut NSObject;
  pub fn sec_protocol_metadata_get_negotiated_protocol_version(
    metadata: *mut NSObject,
  ) -> SSLProtocol;
  pub fn sec_protocol_metadata_get_negotiated_ciphersuite(metadata: *mut NSObject) -> u32;
  pub fn sec_protocol_metadata_get_early_data_accepted(metadata: *mut NSObject) -> bool;
  pub fn sec_protocol_metadata_access_peer_certificate_chain(
    metadata: *mut NSObject,
    handler: (),
  ) -> bool;
  pub fn sec_protocol_metadata_access_ocsp_response(metadata: *mut NSObject, handler: ()) -> bool;
  pub fn sec_protocol_metadata_access_supported_signature_algorithms(
    metadata: *mut NSObject,
    handler: (),
  ) -> bool;
  pub fn sec_protocol_metadata_access_distinguished_names(
    metadata: *mut NSObject,
    handler: (),
  ) -> bool;
  pub fn sec_protocol_metadata_peers_are_equal(
    metadataA: *mut NSObject,
    metadataB: *mut NSObject,
  ) -> bool;
  pub fn sec_protocol_metadata_challenge_parameters_are_equal(
    metadataA: *mut NSObject,
    metadataB: *mut NSObject,
  ) -> bool;
  pub fn sec_protocol_metadata_create_secret(
    metadata: *mut NSObject,
    label_len: usize,
    label: *mut i8,
    exporter_length: usize,
  ) -> *mut NSObject;
  pub fn sec_protocol_metadata_create_secret_with_context(
    metadata: *mut NSObject,
    label_len: usize,
    label: *mut i8,
    context_len: usize,
    context: *mut u8,
    exporter_length: usize,
  ) -> *mut NSObject;
  pub fn sec_protocol_options_set_local_identity(
    options: *mut NSObject,
    identity: *mut NSObject,
  ) -> ();
  pub fn sec_protocol_options_add_tls_ciphersuite(options: *mut NSObject, ciphersuite: u32) -> ();
  pub fn sec_protocol_options_add_tls_ciphersuite_group(
    options: *mut NSObject,
    group: SSLCiphersuiteGroup,
  ) -> ();
  pub fn sec_protocol_options_set_tls_min_version(
    options: *mut NSObject,
    version: SSLProtocol,
  ) -> ();
  pub fn sec_protocol_options_set_tls_max_version(
    options: *mut NSObject,
    version: SSLProtocol,
  ) -> ();
  pub fn sec_protocol_options_add_tls_application_protocol(
    options: *mut NSObject,
    application_protocol: *mut i8,
  ) -> ();
  pub fn sec_protocol_options_set_tls_server_name(
    options: *mut NSObject,
    server_name: *mut i8,
  ) -> ();
  pub fn sec_protocol_options_set_tls_diffie_hellman_parameters(
    options: *mut NSObject,
    params: *mut NSObject,
  ) -> ();
  pub fn sec_protocol_options_add_pre_shared_key(
    options: *mut NSObject,
    psk: *mut NSObject,
    psk_identity: *mut NSObject,
  ) -> ();
  pub fn sec_protocol_options_set_tls_tickets_enabled(
    options: *mut NSObject,
    tickets_enabled: bool,
  ) -> ();
  pub fn sec_protocol_options_set_tls_is_fallback_attempt(
    options: *mut NSObject,
    is_fallback_attempt: bool,
  ) -> ();
  pub fn sec_protocol_options_set_tls_resumption_enabled(
    options: *mut NSObject,
    resumption_enabled: bool,
  ) -> ();
  pub fn sec_protocol_options_set_tls_false_start_enabled(
    options: *mut NSObject,
    false_start_enabled: bool,
  ) -> ();
  pub fn sec_protocol_options_set_tls_ocsp_enabled(
    options: *mut NSObject,
    ocsp_enabled: bool,
  ) -> ();
  pub fn sec_protocol_options_set_tls_sct_enabled(options: *mut NSObject, sct_enabled: bool) -> ();
  pub fn sec_protocol_options_set_tls_renegotiation_enabled(
    options: *mut NSObject,
    renegotiation_enabled: bool,
  ) -> ();
  pub fn sec_protocol_options_set_peer_authentication_required(
    options: *mut NSObject,
    peer_authentication_required: bool,
  ) -> ();
  pub fn sec_protocol_options_set_key_update_block(
    options: *mut NSObject,
    key_update_block: (),
    key_update_queue: *mut NSObject,
  ) -> ();
  pub fn sec_protocol_options_set_challenge_block(
    options: *mut NSObject,
    challenge_block: (),
    challenge_queue: *mut NSObject,
  ) -> ();
  pub fn sec_protocol_options_set_verify_block(
    options: *mut NSObject,
    verify_block: (),
    verify_block_queue: *mut NSObject,
  ) -> ();
  pub fn AuthorizationCreate(
    rights: *mut AuthorizationItemSet,
    environment: *mut AuthorizationItemSet,
    flags: AuthorizationFlags,
    authorization: *mut *mut AuthorizationOpaqueRef,
  ) -> i32;
  pub fn AuthorizationFree(
    authorization: *mut AuthorizationOpaqueRef,
    flags: AuthorizationFlags,
  ) -> i32;
  pub fn AuthorizationCopyRights(
    authorization: *mut AuthorizationOpaqueRef,
    rights: *mut AuthorizationItemSet,
    environment: *mut AuthorizationItemSet,
    flags: AuthorizationFlags,
    authorizedRights: *mut *mut AuthorizationItemSet,
  ) -> i32;
  pub fn AuthorizationCopyRightsAsync(
    authorization: *mut AuthorizationOpaqueRef,
    rights: *mut AuthorizationItemSet,
    environment: *mut AuthorizationItemSet,
    flags: AuthorizationFlags,
    callbackBlock: (),
  ) -> ();
  pub fn AuthorizationCopyInfo(
    authorization: *mut AuthorizationOpaqueRef,
    tag: *mut i8,
    info: *mut *mut AuthorizationItemSet,
  ) -> i32;
  pub fn AuthorizationMakeExternalForm(
    authorization: *mut AuthorizationOpaqueRef,
    extForm: *mut AuthorizationExternalForm,
  ) -> i32;
  pub fn AuthorizationCreateFromExternalForm(
    extForm: *mut AuthorizationExternalForm,
    authorization: *mut *mut AuthorizationOpaqueRef,
  ) -> i32;
  pub fn AuthorizationFreeItemSet(set: *mut AuthorizationItemSet) -> i32;
  pub fn AuthorizationExecuteWithPrivileges(
    authorization: *mut AuthorizationOpaqueRef,
    pathToTool: *mut i8,
    options: AuthorizationFlags,
    arguments: *mut *const i8,
    communicationsPipe: *mut *mut FILE,
  ) -> i32;
  pub fn AuthorizationCopyPrivilegedReference(
    authorization: *mut *mut AuthorizationOpaqueRef,
    flags: AuthorizationFlags,
  ) -> i32;
  pub fn SessionGetInfo(
    session: u32,
    sessionId: *mut u32,
    attributes: *mut SessionAttributeBits,
  ) -> i32;
  pub fn SessionCreate(flags: SessionCreationFlags, attributes: SessionAttributeBits) -> i32;
  pub fn CSSM_Init(
    Version: *mut CSSM_VERSION,
    Scope: u32,
    CallerGuid: *mut CSSM_GUID,
    KeyHierarchy: u32,
    PvcPolicy: *mut u32,
    Reserved: *mut c_void,
  ) -> i32;
  pub fn CSSM_Terminate() -> i32;
  pub fn CSSM_ModuleLoad(
    ModuleGuid: *mut CSSM_GUID,
    KeyHierarchy: u32,
    AppNotifyCallback: Option<extern "C" fn(*mut CSSM_GUID, *mut c_void, u32, u32, u32) -> i32>,
    AppNotifyCallbackCtx: *mut c_void,
  ) -> i32;
  pub fn CSSM_ModuleUnload(
    ModuleGuid: *mut CSSM_GUID,
    AppNotifyCallback: Option<extern "C" fn(*mut CSSM_GUID, *mut c_void, u32, u32, u32) -> i32>,
    AppNotifyCallbackCtx: *mut c_void,
  ) -> i32;
  pub fn CSSM_Introduce(ModuleID: *mut CSSM_GUID, KeyHierarchy: u32) -> i32;
  pub fn CSSM_Unintroduce(ModuleID: *mut CSSM_GUID) -> i32;
  pub fn CSSM_ModuleAttach(
    ModuleGuid: *mut CSSM_GUID,
    Version: *mut CSSM_VERSION,
    MemoryFuncs: *mut CSSM_MEMORY_FUNCS,
    SubserviceID: u32,
    SubServiceType: u32,
    AttachFlags: u32,
    KeyHierarchy: u32,
    FunctionTable: *mut CSSM_FUNC_NAME_ADDR,
    NumFunctionTable: u32,
    Reserved: *mut c_void,
    NewModuleHandle: *mut isize,
  ) -> i32;
  pub fn CSSM_ModuleDetach(ModuleHandle: isize) -> i32;
  pub fn CSSM_SetPrivilege(Privilege: u64) -> i32;
  pub fn CSSM_GetPrivilege(Privilege: *mut u64) -> i32;
  pub fn CSSM_GetModuleGUIDFromHandle(ModuleHandle: isize, ModuleGUID: *mut cssm_guid) -> i32;
  pub fn CSSM_GetSubserviceUIDFromHandle(
    ModuleHandle: isize,
    SubserviceUID: *mut cssm_subservice_uid,
  ) -> i32;
  pub fn CSSM_ListAttachedModuleManagers(
    NumberOfModuleManagers: *mut u32,
    ModuleManagerGuids: *mut cssm_guid,
  ) -> i32;
  pub fn CSSM_GetAPIMemoryFunctions(
    AddInHandle: isize,
    AppMemoryFuncs: *mut CSSM_MEMORY_FUNCS,
  ) -> i32;
  pub fn CSSM_CSP_CreateSignatureContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateSymmetricContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    Mode: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    InitVector: *mut CSSM_DATA,
    Padding: u32,
    Reserved: *mut c_void,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateDigestContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateMacContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    Key: *mut CSSM_KEY,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateRandomGenContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    Seed: *mut CSSM_CRYPTO_DATA,
    Length: usize,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateAsymmetricContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    Padding: u32,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateDeriveKeyContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    DeriveKeyType: u32,
    DeriveKeyLengthInBits: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    BaseKey: *mut CSSM_KEY,
    IterationCount: u32,
    Salt: *mut CSSM_DATA,
    Seed: *mut CSSM_CRYPTO_DATA,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreateKeyGenContext(
    CSPHandle: isize,
    AlgorithmID: u32,
    KeySizeInBits: u32,
    Seed: *mut CSSM_CRYPTO_DATA,
    Salt: *mut CSSM_DATA,
    StartDate: *mut CSSM_DATE,
    EndDate: *mut CSSM_DATE,
    Params: *mut CSSM_DATA,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_CSP_CreatePassThroughContext(
    CSPHandle: isize,
    Key: *mut CSSM_KEY,
    NewContextHandle: *mut u64,
  ) -> i32;
  pub fn CSSM_GetContext(CCHandle: u64, Context: *mut *mut cssm_context) -> i32;
  pub fn CSSM_FreeContext(Context: *mut cssm_context) -> i32;
  pub fn CSSM_SetContext(CCHandle: u64, Context: *mut CSSM_CONTEXT) -> i32;
  pub fn CSSM_DeleteContext(CCHandle: u64) -> i32;
  pub fn CSSM_GetContextAttribute(
    Context: *mut CSSM_CONTEXT,
    AttributeType: u32,
    ContextAttribute: *mut *mut cssm_context_attribute,
  ) -> i32;
  pub fn CSSM_UpdateContextAttributes(
    CCHandle: u64,
    NumberOfAttributes: u32,
    ContextAttributes: *mut CSSM_CONTEXT_ATTRIBUTE,
  ) -> i32;
  pub fn CSSM_DeleteContextAttributes(
    CCHandle: u64,
    NumberOfAttributes: u32,
    ContextAttributes: *mut CSSM_CONTEXT_ATTRIBUTE,
  ) -> i32;
  pub fn CSSM_CSP_Login(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    LoginName: *mut CSSM_DATA,
    Reserved: *mut c_void,
  ) -> i32;
  pub fn CSSM_CSP_Logout(CSPHandle: isize) -> i32;
  pub fn CSSM_CSP_GetLoginAcl(
    CSPHandle: isize,
    SelectionTag: *mut [i8; 68],
    NumberOfAclInfos: *mut u32,
    AclInfos: *mut *mut cssm_acl_entry_info,
  ) -> i32;
  pub fn CSSM_CSP_ChangeLoginAcl(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    AclEdit: *mut CSSM_ACL_EDIT,
  ) -> i32;
  pub fn CSSM_GetKeyAcl(
    CSPHandle: isize,
    Key: *mut CSSM_KEY,
    SelectionTag: *mut [i8; 68],
    NumberOfAclInfos: *mut u32,
    AclInfos: *mut *mut cssm_acl_entry_info,
  ) -> i32;
  pub fn CSSM_ChangeKeyAcl(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    AclEdit: *mut CSSM_ACL_EDIT,
    Key: *mut CSSM_KEY,
  ) -> i32;
  pub fn CSSM_GetKeyOwner(
    CSPHandle: isize,
    Key: *mut CSSM_KEY,
    Owner: *mut cssm_acl_owner_prototype,
  ) -> i32;
  pub fn CSSM_ChangeKeyOwner(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    NewOwner: *mut CSSM_ACL_OWNER_PROTOTYPE,
  ) -> i32;
  pub fn CSSM_CSP_GetLoginOwner(CSPHandle: isize, Owner: *mut cssm_acl_owner_prototype) -> i32;
  pub fn CSSM_CSP_ChangeLoginOwner(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    NewOwner: *mut CSSM_ACL_OWNER_PROTOTYPE,
  ) -> i32;
  pub fn CSSM_SignData(
    CCHandle: u64,
    DataBufs: *mut CSSM_DATA,
    DataBufCount: u32,
    DigestAlgorithm: u32,
    Signature: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_SignDataInit(CCHandle: u64) -> i32;
  pub fn CSSM_SignDataUpdate(CCHandle: u64, DataBufs: *mut CSSM_DATA, DataBufCount: u32) -> i32;
  pub fn CSSM_SignDataFinal(CCHandle: u64, Signature: *mut cssm_data) -> i32;
  pub fn CSSM_VerifyData(
    CCHandle: u64,
    DataBufs: *mut CSSM_DATA,
    DataBufCount: u32,
    DigestAlgorithm: u32,
    Signature: *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_VerifyDataInit(CCHandle: u64) -> i32;
  pub fn CSSM_VerifyDataUpdate(CCHandle: u64, DataBufs: *mut CSSM_DATA, DataBufCount: u32) -> i32;
  pub fn CSSM_VerifyDataFinal(CCHandle: u64, Signature: *mut CSSM_DATA) -> i32;
  pub fn CSSM_DigestData(
    CCHandle: u64,
    DataBufs: *mut CSSM_DATA,
    DataBufCount: u32,
    Digest: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_DigestDataInit(CCHandle: u64) -> i32;
  pub fn CSSM_DigestDataUpdate(CCHandle: u64, DataBufs: *mut CSSM_DATA, DataBufCount: u32) -> i32;
  pub fn CSSM_DigestDataClone(CCHandle: u64, ClonednewCCHandle: *mut u64) -> i32;
  pub fn CSSM_DigestDataFinal(CCHandle: u64, Digest: *mut cssm_data) -> i32;
  pub fn CSSM_GenerateMac(
    CCHandle: u64,
    DataBufs: *mut CSSM_DATA,
    DataBufCount: u32,
    Mac: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_GenerateMacInit(CCHandle: u64) -> i32;
  pub fn CSSM_GenerateMacUpdate(CCHandle: u64, DataBufs: *mut CSSM_DATA, DataBufCount: u32) -> i32;
  pub fn CSSM_GenerateMacFinal(CCHandle: u64, Mac: *mut cssm_data) -> i32;
  pub fn CSSM_VerifyMac(
    CCHandle: u64,
    DataBufs: *mut CSSM_DATA,
    DataBufCount: u32,
    Mac: *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_VerifyMacInit(CCHandle: u64) -> i32;
  pub fn CSSM_VerifyMacUpdate(CCHandle: u64, DataBufs: *mut CSSM_DATA, DataBufCount: u32) -> i32;
  pub fn CSSM_VerifyMacFinal(CCHandle: u64, Mac: *mut CSSM_DATA) -> i32;
  pub fn CSSM_QuerySize(
    CCHandle: u64,
    Encrypt: i32,
    QuerySizeCount: u32,
    DataBlockSizes: *mut cssm_query_size_data,
  ) -> i32;
  pub fn CSSM_EncryptData(
    CCHandle: u64,
    ClearBufs: *mut CSSM_DATA,
    ClearBufCount: u32,
    CipherBufs: *mut cssm_data,
    CipherBufCount: u32,
    bytesEncrypted: *mut usize,
    RemData: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_EncryptDataP(
    CCHandle: u64,
    ClearBufs: *mut CSSM_DATA,
    ClearBufCount: u32,
    CipherBufs: *mut cssm_data,
    CipherBufCount: u32,
    bytesEncrypted: *mut usize,
    RemData: *mut cssm_data,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_EncryptDataInit(CCHandle: u64) -> i32;
  pub fn CSSM_EncryptDataInitP(CCHandle: u64, Privilege: u64) -> i32;
  pub fn CSSM_EncryptDataUpdate(
    CCHandle: u64,
    ClearBufs: *mut CSSM_DATA,
    ClearBufCount: u32,
    CipherBufs: *mut cssm_data,
    CipherBufCount: u32,
    bytesEncrypted: *mut usize,
  ) -> i32;
  pub fn CSSM_EncryptDataFinal(CCHandle: u64, RemData: *mut cssm_data) -> i32;
  pub fn CSSM_DecryptData(
    CCHandle: u64,
    CipherBufs: *mut CSSM_DATA,
    CipherBufCount: u32,
    ClearBufs: *mut cssm_data,
    ClearBufCount: u32,
    bytesDecrypted: *mut usize,
    RemData: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_DecryptDataP(
    CCHandle: u64,
    CipherBufs: *mut CSSM_DATA,
    CipherBufCount: u32,
    ClearBufs: *mut cssm_data,
    ClearBufCount: u32,
    bytesDecrypted: *mut usize,
    RemData: *mut cssm_data,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_DecryptDataInit(CCHandle: u64) -> i32;
  pub fn CSSM_DecryptDataInitP(CCHandle: u64, Privilege: u64) -> i32;
  pub fn CSSM_DecryptDataUpdate(
    CCHandle: u64,
    CipherBufs: *mut CSSM_DATA,
    CipherBufCount: u32,
    ClearBufs: *mut cssm_data,
    ClearBufCount: u32,
    bytesDecrypted: *mut usize,
  ) -> i32;
  pub fn CSSM_DecryptDataFinal(CCHandle: u64, RemData: *mut cssm_data) -> i32;
  pub fn CSSM_QueryKeySizeInBits(
    CSPHandle: isize,
    CCHandle: u64,
    Key: *mut CSSM_KEY,
    KeySize: *mut cssm_key_size,
  ) -> i32;
  pub fn CSSM_GenerateKey(
    CCHandle: u64,
    KeyUsage: u32,
    KeyAttr: u32,
    KeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    Key: *mut cssm_key,
  ) -> i32;
  pub fn CSSM_GenerateKeyP(
    CCHandle: u64,
    KeyUsage: u32,
    KeyAttr: u32,
    KeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    Key: *mut cssm_key,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_GenerateKeyPair(
    CCHandle: u64,
    PublicKeyUsage: u32,
    PublicKeyAttr: u32,
    PublicKeyLabel: *mut CSSM_DATA,
    PublicKey: *mut cssm_key,
    PrivateKeyUsage: u32,
    PrivateKeyAttr: u32,
    PrivateKeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    PrivateKey: *mut cssm_key,
  ) -> i32;
  pub fn CSSM_GenerateKeyPairP(
    CCHandle: u64,
    PublicKeyUsage: u32,
    PublicKeyAttr: u32,
    PublicKeyLabel: *mut CSSM_DATA,
    PublicKey: *mut cssm_key,
    PrivateKeyUsage: u32,
    PrivateKeyAttr: u32,
    PrivateKeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    PrivateKey: *mut cssm_key,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_GenerateRandom(CCHandle: u64, RandomNumber: *mut cssm_data) -> i32;
  pub fn CSSM_CSP_ObtainPrivateKeyFromPublicKey(
    CSPHandle: isize,
    PublicKey: *mut CSSM_KEY,
    PrivateKey: *mut cssm_key,
  ) -> i32;
  pub fn CSSM_WrapKey(
    CCHandle: u64,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    DescriptiveData: *mut CSSM_DATA,
    WrappedKey: *mut CSSM_KEY,
  ) -> i32;
  pub fn CSSM_UnwrapKey(
    CCHandle: u64,
    PublicKey: *mut CSSM_KEY,
    WrappedKey: *mut CSSM_KEY,
    KeyUsage: u32,
    KeyAttr: u32,
    KeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    UnwrappedKey: *mut cssm_key,
    DescriptiveData: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_WrapKeyP(
    CCHandle: u64,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    Key: *mut CSSM_KEY,
    DescriptiveData: *mut CSSM_DATA,
    WrappedKey: *mut CSSM_KEY,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_UnwrapKeyP(
    CCHandle: u64,
    PublicKey: *mut CSSM_KEY,
    WrappedKey: *mut CSSM_KEY,
    KeyUsage: u32,
    KeyAttr: u32,
    KeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    UnwrappedKey: *mut cssm_key,
    DescriptiveData: *mut cssm_data,
    Privilege: u64,
  ) -> i32;
  pub fn CSSM_DeriveKey(
    CCHandle: u64,
    Param: *mut cssm_data,
    KeyUsage: u32,
    KeyAttr: u32,
    KeyLabel: *mut CSSM_DATA,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    DerivedKey: *mut cssm_key,
  ) -> i32;
  pub fn CSSM_FreeKey(
    CSPHandle: isize,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    KeyPtr: *mut cssm_key,
    Delete: i32,
  ) -> i32;
  pub fn CSSM_GenerateAlgorithmParams(CCHandle: u64, ParamBits: u32, Param: *mut cssm_data) -> i32;
  pub fn CSSM_CSP_GetOperationalStatistics(
    CSPHandle: isize,
    Statistics: *mut CSSM_CSP_OPERATIONAL_STATISTICS,
  ) -> i32;
  pub fn CSSM_GetTimeValue(CSPHandle: isize, TimeAlgorithm: u32, TimeData: *mut CSSM_DATA) -> i32;
  pub fn CSSM_RetrieveUniqueId(CSPHandle: isize, UniqueID: *mut cssm_data) -> i32;
  pub fn CSSM_RetrieveCounter(CSPHandle: isize, Counter: *mut cssm_data) -> i32;
  pub fn CSSM_VerifyDevice(CSPHandle: isize, DeviceCert: *mut CSSM_DATA) -> i32;
  pub fn CSSM_CSP_PassThrough(
    CCHandle: u64,
    PassThroughId: u32,
    InData: *mut c_void,
    OutData: *mut *mut c_void,
  ) -> i32;
  pub fn CSSM_TP_SubmitCredRequest(
    TPHandle: isize,
    PreferredAuthority: *mut CSSM_TP_AUTHORITY_ID,
    RequestType: u32,
    RequestInput: *mut CSSM_TP_REQUEST_SET,
    CallerAuthContext: *mut CSSM_TP_CALLERAUTH_CONTEXT,
    EstimatedTime: *mut i32,
    ReferenceIdentifier: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_RetrieveCredResult(
    TPHandle: isize,
    ReferenceIdentifier: *mut CSSM_DATA,
    CallerAuthCredentials: *mut CSSM_TP_CALLERAUTH_CONTEXT,
    EstimatedTime: *mut i32,
    ConfirmationRequired: *mut i32,
    RetrieveOutput: *mut *mut cssm_tp_result_set,
  ) -> i32;
  pub fn CSSM_TP_ConfirmCredResult(
    TPHandle: isize,
    ReferenceIdentifier: *mut CSSM_DATA,
    CallerAuthCredentials: *mut CSSM_TP_CALLERAUTH_CONTEXT,
    Responses: *mut CSSM_TP_CONFIRM_RESPONSE,
    PreferredAuthority: *mut CSSM_TP_AUTHORITY_ID,
  ) -> i32;
  pub fn CSSM_TP_ReceiveConfirmation(
    TPHandle: isize,
    ReferenceIdentifier: *mut CSSM_DATA,
    Responses: *mut *mut cssm_tp_confirm_response,
    ElapsedTime: *mut i32,
  ) -> i32;
  pub fn CSSM_TP_CertReclaimKey(
    TPHandle: isize,
    CertGroup: *mut CSSM_CERTGROUP,
    CertIndex: u32,
    KeyCacheHandle: u64,
    CSPHandle: isize,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
  ) -> i32;
  pub fn CSSM_TP_CertReclaimAbort(TPHandle: isize, KeyCacheHandle: u64) -> i32;
  pub fn CSSM_TP_FormRequest(
    TPHandle: isize,
    PreferredAuthority: *mut CSSM_TP_AUTHORITY_ID,
    FormType: u32,
    BlankForm: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_FormSubmit(
    TPHandle: isize,
    FormType: u32,
    Form: *mut CSSM_DATA,
    ClearanceAuthority: *mut CSSM_TP_AUTHORITY_ID,
    RepresentedAuthority: *mut CSSM_TP_AUTHORITY_ID,
    Credentials: *mut cssm_access_credentials,
  ) -> i32;
  pub fn CSSM_TP_CertGroupVerify(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    CertGroupToBeVerified: *mut CSSM_CERTGROUP,
    VerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    VerifyContextResult: *mut cssm_tp_verify_context_result,
  ) -> i32;
  pub fn CSSM_TP_CertCreateTemplate(
    TPHandle: isize,
    CLHandle: isize,
    NumberOfFields: u32,
    CertFields: *mut CSSM_FIELD,
    CertTemplate: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_CertGetAllTemplateFields(
    TPHandle: isize,
    CLHandle: isize,
    CertTemplate: *mut CSSM_DATA,
    NumberOfFields: *mut u32,
    CertFields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_TP_CertSign(
    TPHandle: isize,
    CLHandle: isize,
    CCHandle: u64,
    CertTemplateToBeSigned: *mut CSSM_DATA,
    SignerCertGroup: *mut CSSM_CERTGROUP,
    SignerVerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    SignerVerifyResult: *mut cssm_tp_verify_context_result,
    SignedCert: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_CrlVerify(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    CrlToBeVerified: *mut CSSM_ENCODED_CRL,
    SignerCertGroup: *mut CSSM_CERTGROUP,
    VerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    RevokerVerifyResult: *mut cssm_tp_verify_context_result,
  ) -> i32;
  pub fn CSSM_TP_CrlCreateTemplate(
    TPHandle: isize,
    CLHandle: isize,
    NumberOfFields: u32,
    CrlFields: *mut CSSM_FIELD,
    NewCrlTemplate: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_CertRevoke(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    OldCrlTemplate: *mut CSSM_DATA,
    CertGroupToBeRevoked: *mut CSSM_CERTGROUP,
    RevokerCertGroup: *mut CSSM_CERTGROUP,
    RevokerVerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    RevokerVerifyResult: *mut cssm_tp_verify_context_result,
    Reason: u32,
    NewCrlTemplate: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_CertRemoveFromCrlTemplate(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    OldCrlTemplate: *mut CSSM_DATA,
    CertGroupToBeRemoved: *mut CSSM_CERTGROUP,
    RevokerCertGroup: *mut CSSM_CERTGROUP,
    RevokerVerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    RevokerVerifyResult: *mut cssm_tp_verify_context_result,
    NewCrlTemplate: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_CrlSign(
    TPHandle: isize,
    CLHandle: isize,
    CCHandle: u64,
    CrlToBeSigned: *mut CSSM_ENCODED_CRL,
    SignerCertGroup: *mut CSSM_CERTGROUP,
    SignerVerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    SignerVerifyResult: *mut cssm_tp_verify_context_result,
    SignedCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_TP_ApplyCrlToDb(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    CrlToBeApplied: *mut CSSM_ENCODED_CRL,
    SignerCertGroup: *mut CSSM_CERTGROUP,
    ApplyCrlVerifyContext: *mut CSSM_TP_VERIFY_CONTEXT,
    ApplyCrlVerifyResult: *mut cssm_tp_verify_context_result,
  ) -> i32;
  pub fn CSSM_TP_CertGroupConstruct(
    TPHandle: isize,
    CLHandle: isize,
    CSPHandle: isize,
    DBList: *mut CSSM_DL_DB_LIST,
    ConstructParams: *mut c_void,
    CertGroupFrag: *mut CSSM_CERTGROUP,
    CertGroup: *mut *mut cssm_certgroup,
  ) -> i32;
  pub fn CSSM_TP_CertGroupPrune(
    TPHandle: isize,
    CLHandle: isize,
    DBList: *mut CSSM_DL_DB_LIST,
    OrderedCertGroup: *mut CSSM_CERTGROUP,
    PrunedCertGroup: *mut *mut cssm_certgroup,
  ) -> i32;
  pub fn CSSM_TP_CertGroupToTupleGroup(
    TPHandle: isize,
    CLHandle: isize,
    CertGroup: *mut CSSM_CERTGROUP,
    TupleGroup: *mut *mut cssm_tuplegroup,
  ) -> i32;
  pub fn CSSM_TP_TupleGroupToCertGroup(
    TPHandle: isize,
    CLHandle: isize,
    TupleGroup: *mut CSSM_TUPLEGROUP,
    CertTemplates: *mut *mut cssm_certgroup,
  ) -> i32;
  pub fn CSSM_TP_PassThrough(
    TPHandle: isize,
    CLHandle: isize,
    CCHandle: u64,
    DBList: *mut CSSM_DL_DB_LIST,
    PassThroughId: u32,
    InputParams: *mut c_void,
    OutputParams: *mut *mut c_void,
  ) -> i32;
  pub fn CSSM_AC_AuthCompute(
    ACHandle: isize,
    BaseAuthorizations: *mut CSSM_TUPLEGROUP,
    Credentials: *mut CSSM_TUPLEGROUP,
    NumberOfRequestors: u32,
    Requestors: *mut CSSM_LIST,
    RequestedAuthorizationPeriod: *mut CSSM_LIST,
    RequestedAuthorization: *mut CSSM_LIST,
    AuthorizationResult: *mut cssm_tuplegroup,
  ) -> i32;
  pub fn CSSM_AC_PassThrough(
    ACHandle: isize,
    TPHandle: isize,
    CLHandle: isize,
    CCHandle: u64,
    DBList: *mut CSSM_DL_DB_LIST,
    PassThroughId: u32,
    InputParams: *mut c_void,
    OutputParams: *mut *mut c_void,
  ) -> i32;
  pub fn CSSM_CL_CertCreateTemplate(
    CLHandle: isize,
    NumberOfFields: u32,
    CertFields: *mut CSSM_FIELD,
    CertTemplate: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertGetAllTemplateFields(
    CLHandle: isize,
    CertTemplate: *mut CSSM_DATA,
    NumberOfFields: *mut u32,
    CertFields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_CL_CertSign(
    CLHandle: isize,
    CCHandle: u64,
    CertTemplate: *mut CSSM_DATA,
    SignScope: *mut CSSM_FIELD,
    ScopeSize: u32,
    SignedCert: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertVerify(
    CLHandle: isize,
    CCHandle: u64,
    CertToBeVerified: *mut CSSM_DATA,
    SignerCert: *mut CSSM_DATA,
    VerifyScope: *mut CSSM_FIELD,
    ScopeSize: u32,
  ) -> i32;
  pub fn CSSM_CL_CertVerifyWithKey(
    CLHandle: isize,
    CCHandle: u64,
    CertToBeVerified: *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_CL_CertGetFirstFieldValue(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    CertField: *mut CSSM_DATA,
    ResultsHandle: *mut isize,
    NumberOfMatchedFields: *mut u32,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertGetNextFieldValue(
    CLHandle: isize,
    ResultsHandle: isize,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertAbortQuery(CLHandle: isize, ResultsHandle: isize) -> i32;
  pub fn CSSM_CL_CertGetKeyInfo(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    Key: *mut *mut cssm_key,
  ) -> i32;
  pub fn CSSM_CL_CertGetAllFields(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    NumberOfFields: *mut u32,
    CertFields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_CL_FreeFields(
    CLHandle: isize,
    NumberOfFields: u32,
    Fields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_CL_FreeFieldValue(
    CLHandle: isize,
    CertOrCrlOid: *mut CSSM_DATA,
    Value: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertCache(CLHandle: isize, Cert: *mut CSSM_DATA, CertHandle: *mut isize) -> i32;
  pub fn CSSM_CL_CertGetFirstCachedFieldValue(
    CLHandle: isize,
    CertHandle: isize,
    CertField: *mut CSSM_DATA,
    ResultsHandle: *mut isize,
    NumberOfMatchedFields: *mut u32,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertGetNextCachedFieldValue(
    CLHandle: isize,
    ResultsHandle: isize,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertAbortCache(CLHandle: isize, CertHandle: isize) -> i32;
  pub fn CSSM_CL_CertGroupToSignedBundle(
    CLHandle: isize,
    CCHandle: u64,
    CertGroupToBundle: *mut CSSM_CERTGROUP,
    BundleInfo: *mut CSSM_CERT_BUNDLE_HEADER,
    SignedBundle: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CertGroupFromVerifiedBundle(
    CLHandle: isize,
    CCHandle: u64,
    CertBundle: *mut CSSM_CERT_BUNDLE,
    SignerCert: *mut CSSM_DATA,
    CertGroup: *mut *mut cssm_certgroup,
  ) -> i32;
  pub fn CSSM_CL_CertDescribeFormat(
    CLHandle: isize,
    NumberOfFields: *mut u32,
    OidList: *mut *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_CL_CrlCreateTemplate(
    CLHandle: isize,
    NumberOfFields: u32,
    CrlTemplate: *mut CSSM_FIELD,
    NewCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlSetFields(
    CLHandle: isize,
    NumberOfFields: u32,
    CrlTemplate: *mut CSSM_FIELD,
    OldCrl: *mut CSSM_DATA,
    ModifiedCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlAddCert(
    CLHandle: isize,
    CCHandle: u64,
    Cert: *mut CSSM_DATA,
    NumberOfFields: u32,
    CrlEntryFields: *mut CSSM_FIELD,
    OldCrl: *mut CSSM_DATA,
    NewCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlRemoveCert(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    OldCrl: *mut CSSM_DATA,
    NewCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlSign(
    CLHandle: isize,
    CCHandle: u64,
    UnsignedCrl: *mut CSSM_DATA,
    SignScope: *mut CSSM_FIELD,
    ScopeSize: u32,
    SignedCrl: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlVerify(
    CLHandle: isize,
    CCHandle: u64,
    CrlToBeVerified: *mut CSSM_DATA,
    SignerCert: *mut CSSM_DATA,
    VerifyScope: *mut CSSM_FIELD,
    ScopeSize: u32,
  ) -> i32;
  pub fn CSSM_CL_CrlVerifyWithKey(
    CLHandle: isize,
    CCHandle: u64,
    CrlToBeVerified: *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_CL_IsCertInCrl(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    Crl: *mut CSSM_DATA,
    CertFound: *mut i32,
  ) -> i32;
  pub fn CSSM_CL_CrlGetFirstFieldValue(
    CLHandle: isize,
    Crl: *mut CSSM_DATA,
    CrlField: *mut CSSM_DATA,
    ResultsHandle: *mut isize,
    NumberOfMatchedFields: *mut u32,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlGetNextFieldValue(
    CLHandle: isize,
    ResultsHandle: isize,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlAbortQuery(CLHandle: isize, ResultsHandle: isize) -> i32;
  pub fn CSSM_CL_CrlGetAllFields(
    CLHandle: isize,
    Crl: *mut CSSM_DATA,
    NumberOfCrlFields: *mut u32,
    CrlFields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_CL_CrlCache(CLHandle: isize, Crl: *mut CSSM_DATA, CrlHandle: *mut isize) -> i32;
  pub fn CSSM_CL_IsCertInCachedCrl(
    CLHandle: isize,
    Cert: *mut CSSM_DATA,
    CrlHandle: isize,
    CertFound: *mut i32,
    CrlRecordIndex: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlGetFirstCachedFieldValue(
    CLHandle: isize,
    CrlHandle: isize,
    CrlRecordIndex: *mut CSSM_DATA,
    CrlField: *mut CSSM_DATA,
    ResultsHandle: *mut isize,
    NumberOfMatchedFields: *mut u32,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlGetNextCachedFieldValue(
    CLHandle: isize,
    ResultsHandle: isize,
    Value: *mut *mut cssm_data,
  ) -> i32;
  pub fn CSSM_CL_CrlGetAllCachedRecordFields(
    CLHandle: isize,
    CrlHandle: isize,
    CrlRecordIndex: *mut CSSM_DATA,
    NumberOfFields: *mut u32,
    CrlFields: *mut *mut cssm_field,
  ) -> i32;
  pub fn CSSM_CL_CrlAbortCache(CLHandle: isize, CrlHandle: isize) -> i32;
  pub fn CSSM_CL_CrlDescribeFormat(
    CLHandle: isize,
    NumberOfFields: *mut u32,
    OidList: *mut *mut CSSM_DATA,
  ) -> i32;
  pub fn CSSM_CL_PassThrough(
    CLHandle: isize,
    CCHandle: u64,
    PassThroughId: u32,
    InputParams: *mut c_void,
    OutputParams: *mut *mut c_void,
  ) -> i32;
  pub fn CSSM_DL_DbOpen(
    DLHandle: isize,
    DbName: *mut i8,
    DbLocation: *mut CSSM_NET_ADDRESS,
    AccessRequest: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    OpenParameters: *mut c_void,
    DbHandle: *mut isize,
  ) -> i32;
  pub fn CSSM_DL_DbClose(DLDBHandle: CSSM_DL_DB_HANDLE) -> i32;
  pub fn CSSM_DL_DbCreate(
    DLHandle: isize,
    DbName: *mut i8,
    DbLocation: *mut CSSM_NET_ADDRESS,
    DBInfo: *mut CSSM_DBINFO,
    AccessRequest: u32,
    CredAndAclEntry: *mut CSSM_RESOURCE_CONTROL_CONTEXT,
    OpenParameters: *mut c_void,
    DbHandle: *mut isize,
  ) -> i32;
  pub fn CSSM_DL_DbDelete(
    DLHandle: isize,
    DbName: *mut i8,
    DbLocation: *mut CSSM_NET_ADDRESS,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
  ) -> i32;
  pub fn CSSM_DL_CreateRelation(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    RelationID: u32,
    RelationName: *mut i8,
    NumberOfAttributes: u32,
    pAttributeInfo: *mut CSSM_DB_SCHEMA_ATTRIBUTE_INFO,
    NumberOfIndexes: u32,
    pIndexInfo: *mut CSSM_DB_SCHEMA_INDEX_INFO,
  ) -> i32;
  pub fn CSSM_DL_DestroyRelation(DLDBHandle: CSSM_DL_DB_HANDLE, RelationID: u32) -> i32;
  pub fn CSSM_DL_Authenticate(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    AccessRequest: u32,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
  ) -> i32;
  pub fn CSSM_DL_GetDbAcl(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    SelectionTag: *mut [i8; 68],
    NumberOfAclInfos: *mut u32,
    AclInfos: *mut *mut cssm_acl_entry_info,
  ) -> i32;
  pub fn CSSM_DL_ChangeDbAcl(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    AclEdit: *mut CSSM_ACL_EDIT,
  ) -> i32;
  pub fn CSSM_DL_GetDbOwner(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    Owner: *mut cssm_acl_owner_prototype,
  ) -> i32;
  pub fn CSSM_DL_ChangeDbOwner(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    AccessCred: *mut CSSM_ACCESS_CREDENTIALS,
    NewOwner: *mut CSSM_ACL_OWNER_PROTOTYPE,
  ) -> i32;
  pub fn CSSM_DL_GetDbNames(DLHandle: isize, NameList: *mut *mut cssm_name_list) -> i32;
  pub fn CSSM_DL_GetDbNameFromHandle(DLDBHandle: CSSM_DL_DB_HANDLE, DbName: *mut *mut i8) -> i32;
  pub fn CSSM_DL_FreeNameList(DLHandle: isize, NameList: *mut cssm_name_list) -> i32;
  pub fn CSSM_DL_DataInsert(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    RecordType: u32,
    Attributes: *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
    Data: *mut CSSM_DATA,
    UniqueId: *mut *mut cssm_db_unique_record,
  ) -> i32;
  pub fn CSSM_DL_DataDelete(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    UniqueRecordIdentifier: *mut CSSM_DB_UNIQUE_RECORD,
  ) -> i32;
  pub fn CSSM_DL_DataModify(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    RecordType: u32,
    UniqueRecordIdentifier: *mut cssm_db_unique_record,
    AttributesToBeModified: *mut CSSM_DB_RECORD_ATTRIBUTE_DATA,
    DataToBeModified: *mut CSSM_DATA,
    ModifyMode: u32,
  ) -> i32;
  pub fn CSSM_DL_DataGetFirst(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    Query: *mut CSSM_QUERY,
    ResultsHandle: *mut isize,
    Attributes: *mut cssm_db_record_attribute_data,
    Data: *mut cssm_data,
    UniqueId: *mut *mut cssm_db_unique_record,
  ) -> i32;
  pub fn CSSM_DL_DataGetNext(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    ResultsHandle: isize,
    Attributes: *mut cssm_db_record_attribute_data,
    Data: *mut cssm_data,
    UniqueId: *mut *mut cssm_db_unique_record,
  ) -> i32;
  pub fn CSSM_DL_DataAbortQuery(DLDBHandle: CSSM_DL_DB_HANDLE, ResultsHandle: isize) -> i32;
  pub fn CSSM_DL_DataGetFromUniqueRecordId(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    UniqueRecord: *mut CSSM_DB_UNIQUE_RECORD,
    Attributes: *mut cssm_db_record_attribute_data,
    Data: *mut cssm_data,
  ) -> i32;
  pub fn CSSM_DL_FreeUniqueRecord(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    UniqueRecord: *mut cssm_db_unique_record,
  ) -> i32;
  pub fn CSSM_DL_PassThrough(
    DLDBHandle: CSSM_DL_DB_HANDLE,
    PassThroughId: u32,
    InputParams: *mut c_void,
    OutputParams: *mut *mut c_void,
  ) -> i32;
  pub fn MDS_Initialize(
    pCallerGuid: *mut CSSM_GUID,
    pMemoryFunctions: *mut CSSM_MEMORY_FUNCS,
    pDlFunctions: *mut mds_funcs,
    hMds: *mut isize,
  ) -> i32;
  pub fn MDS_Terminate(MdsHandle: isize) -> i32;
  pub fn MDS_Install(MdsHandle: isize) -> i32;
  pub fn MDS_Uninstall(MdsHandle: isize) -> i32;
  pub fn SecACLGetTypeID() -> usize;
  pub fn SecACLCreateFromSimpleContents(
    access: *mut OpaqueSecAccessRef,
    applicationList: *mut __CFArray,
    description: *mut __CFString,
    promptSelector: *mut CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
    newAcl: *mut *mut OpaqueSecTrustRef,
  ) -> i32;
  pub fn SecACLCreateWithSimpleContents(
    access: *mut OpaqueSecAccessRef,
    applicationList: *mut __CFArray,
    description: *mut __CFString,
    promptSelector: SecKeychainPromptSelector,
    newAcl: *mut *mut OpaqueSecTrustRef,
  ) -> i32;
  pub fn SecACLRemove(aclRef: *mut OpaqueSecTrustRef) -> i32;
  pub fn SecACLCopySimpleContents(
    acl: *mut OpaqueSecTrustRef,
    applicationList: *mut *mut __CFArray,
    description: *mut *mut __CFString,
    promptSelector: *mut CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
  ) -> i32;
  pub fn SecACLCopyContents(
    acl: *mut OpaqueSecTrustRef,
    applicationList: *mut *mut __CFArray,
    description: *mut *mut __CFString,
    promptSelector: *mut SecKeychainPromptSelector,
  ) -> i32;
  pub fn SecACLSetSimpleContents(
    acl: *mut OpaqueSecTrustRef,
    applicationList: *mut __CFArray,
    description: *mut __CFString,
    promptSelector: *mut CSSM_ACL_KEYCHAIN_PROMPT_SELECTOR,
  ) -> i32;
  pub fn SecACLSetContents(
    acl: *mut OpaqueSecTrustRef,
    applicationList: *mut __CFArray,
    description: *mut __CFString,
    promptSelector: SecKeychainPromptSelector,
  ) -> i32;
  pub fn SecACLGetAuthorizations(
    acl: *mut OpaqueSecTrustRef,
    tags: *mut i32,
    tagCount: *mut u32,
  ) -> i32;
  pub fn SecACLCopyAuthorizations(acl: *mut OpaqueSecTrustRef) -> *mut __CFArray;
  pub fn SecACLSetAuthorizations(acl: *mut OpaqueSecTrustRef, tags: *mut i32, tagCount: u32)
    -> i32;
  pub fn SecACLUpdateAuthorizations(
    acl: *mut OpaqueSecTrustRef,
    authorizations: *mut __CFArray,
  ) -> i32;
  pub fn SecIdentitySearchGetTypeID() -> usize;
  pub fn SecIdentitySearchCreate(
    keychainOrArray: *mut c_void,
    keyUsage: u32,
    searchRef: *mut *mut OpaqueSecIdentitySearchRef,
  ) -> i32;
  pub fn SecIdentitySearchCopyNext(
    searchRef: *mut OpaqueSecIdentitySearchRef,
    identity: *mut *mut OpaqueSecIdentityRef,
  ) -> i32;
  pub fn SecKeychainItemGetTypeID() -> usize;
  pub fn SecKeychainItemModifyAttributesAndData(
    itemRef: *mut OpaqueSecKeychainItemRef,
    attrList: *mut SecKeychainAttributeList,
    length: u32,
    data: *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemCreateFromContent(
    itemClass: SecItemClass,
    attrList: *mut SecKeychainAttributeList,
    length: u32,
    data: *mut c_void,
    keychainRef: *mut OpaqueSecKeychainRef,
    initialAccess: *mut OpaqueSecAccessRef,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainItemModifyContent(
    itemRef: *mut OpaqueSecKeychainItemRef,
    attrList: *mut SecKeychainAttributeList,
    length: u32,
    data: *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemCopyContent(
    itemRef: *mut OpaqueSecKeychainItemRef,
    itemClass: *mut SecItemClass,
    attrList: *mut SecKeychainAttributeList,
    length: *mut u32,
    outData: *mut *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemFreeContent(
    attrList: *mut SecKeychainAttributeList,
    data: *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemCopyAttributesAndData(
    itemRef: *mut OpaqueSecKeychainItemRef,
    info: *mut SecKeychainAttributeInfo,
    itemClass: *mut SecItemClass,
    attrList: *mut *mut SecKeychainAttributeList,
    length: *mut u32,
    outData: *mut *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemFreeAttributesAndData(
    attrList: *mut SecKeychainAttributeList,
    data: *mut c_void,
  ) -> i32;
  pub fn SecKeychainItemDelete(itemRef: *mut OpaqueSecKeychainItemRef) -> i32;
  pub fn SecKeychainItemCopyKeychain(
    itemRef: *mut OpaqueSecKeychainItemRef,
    keychainRef: *mut *mut OpaqueSecKeychainRef,
  ) -> i32;
  pub fn SecKeychainItemCreateCopy(
    itemRef: *mut OpaqueSecKeychainItemRef,
    destKeychainRef: *mut OpaqueSecKeychainRef,
    initialAccess: *mut OpaqueSecAccessRef,
    itemCopy: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainItemCreatePersistentReference(
    itemRef: *mut OpaqueSecKeychainItemRef,
    persistentItemRef: *mut *mut __CFData,
  ) -> i32;
  pub fn SecKeychainItemCopyFromPersistentReference(
    persistentItemRef: *mut __CFData,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecKeychainItemGetDLDBHandle(
    keyItemRef: *mut OpaqueSecKeychainItemRef,
    dldbHandle: *mut CSSM_DL_DB_HANDLE,
  ) -> i32;
  pub fn SecKeychainItemGetUniqueRecordID(
    itemRef: *mut OpaqueSecKeychainItemRef,
    uniqueRecordID: *mut *mut CSSM_DB_UNIQUE_RECORD,
  ) -> i32;
  pub fn SecKeychainItemCopyAccess(
    itemRef: *mut OpaqueSecKeychainItemRef,
    access: *mut *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecKeychainItemSetAccess(
    itemRef: *mut OpaqueSecKeychainItemRef,
    access: *mut OpaqueSecAccessRef,
  ) -> i32;
  pub fn SecKeychainSearchGetTypeID() -> usize;
  pub fn SecKeychainSearchCreateFromAttributes(
    keychainOrArray: *mut c_void,
    itemClass: SecItemClass,
    attrList: *mut SecKeychainAttributeList,
    searchRef: *mut *mut OpaqueSecKeychainSearchRef,
  ) -> i32;
  pub fn SecKeychainSearchCopyNext(
    searchRef: *mut OpaqueSecKeychainSearchRef,
    itemRef: *mut *mut OpaqueSecKeychainItemRef,
  ) -> i32;
  pub fn SecPolicySearchGetTypeID() -> usize;
  pub fn SecPolicySearchCreate(
    certType: u32,
    policyOID: *mut CSSM_DATA,
    value: *mut CSSM_DATA,
    searchRef: *mut *mut OpaquePolicySearchRef,
  ) -> i32;
  pub fn SecPolicySearchCopyNext(
    searchRef: *mut OpaquePolicySearchRef,
    policyRef: *mut *mut OpaqueSecPolicyRef,
  ) -> i32;
  pub fn SecTrustedApplicationGetTypeID() -> usize;
  pub fn SecTrustedApplicationCreateFromPath(
    path: *mut i8,
    app: *mut *mut OpaqueSecTrustedApplicationRef,
  ) -> i32;
  pub fn SecTrustedApplicationCopyData(
    appRef: *mut OpaqueSecTrustedApplicationRef,
    data: *mut *mut __CFData,
  ) -> i32;
  pub fn SecTrustedApplicationSetData(
    appRef: *mut OpaqueSecTrustedApplicationRef,
    data: *mut __CFData,
  ) -> i32;
  pub fn SecTrustSettingsCopyTrustSettings(
    certRef: *mut OpaqueSecCertificateRef,
    domain: SecTrustSettingsDomain,
    trustSettings: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecTrustSettingsSetTrustSettings(
    certRef: *mut OpaqueSecCertificateRef,
    domain: SecTrustSettingsDomain,
    trustSettingsDictOrArray: *mut c_void,
  ) -> i32;
  pub fn SecTrustSettingsRemoveTrustSettings(
    certRef: *mut OpaqueSecCertificateRef,
    domain: SecTrustSettingsDomain,
  ) -> i32;
  pub fn SecTrustSettingsCopyCertificates(
    domain: SecTrustSettingsDomain,
    certArray: *mut *mut __CFArray,
  ) -> i32;
  pub fn SecTrustSettingsCopyModificationDate(
    certRef: *mut OpaqueSecCertificateRef,
    domain: SecTrustSettingsDomain,
    modificationDate: *mut *mut __CFDate,
  ) -> i32;
  pub fn SecTrustSettingsCreateExternalRepresentation(
    domain: SecTrustSettingsDomain,
    trustSettings: *mut *mut __CFData,
  ) -> i32;
  pub fn SecTrustSettingsImportExternalRepresentation(
    domain: SecTrustSettingsDomain,
    trustSettings: *mut __CFData,
  ) -> i32;
  pub fn SecStaticCodeGetTypeID() -> usize;
  pub fn SecStaticCodeCreateWithPath(
    path: *mut __CFURL,
    flags: SecCSFlags,
    staticCode: *mut *mut __SecCode,
  ) -> i32;
  pub fn SecStaticCodeCreateWithPathAndAttributes(
    path: *mut __CFURL,
    flags: SecCSFlags,
    attributes: *mut __CFDictionary,
    staticCode: *mut *mut __SecCode,
  ) -> i32;
  pub fn SecStaticCodeCheckValidity(
    staticCode: *mut __SecCode,
    flags: SecCSFlags,
    requirement: *mut __SecRequirement,
  ) -> i32;
  pub fn SecStaticCodeCheckValidityWithErrors(
    staticCode: *mut __SecCode,
    flags: SecCSFlags,
    requirement: *mut __SecRequirement,
    errors: *mut *mut __CFError,
  ) -> i32;
  pub fn SecCodeGetTypeID() -> usize;
  pub fn SecCodeCopySelf(flags: SecCSFlags, self_: *mut *mut __SecCode) -> i32;
  pub fn SecCodeCopyStaticCode(
    code: *mut __SecCode,
    flags: SecCSFlags,
    staticCode: *mut *mut __SecCode,
  ) -> i32;
  pub fn SecCodeCopyHost(
    guest: *mut __SecCode,
    flags: SecCSFlags,
    host: *mut *mut __SecCode,
  ) -> i32;
  pub fn SecCodeCopyGuestWithAttributes(
    host: *mut __SecCode,
    attributes: *mut __CFDictionary,
    flags: SecCSFlags,
    guest: *mut *mut __SecCode,
  ) -> i32;
  pub fn SecCodeCheckValidity(
    code: *mut __SecCode,
    flags: SecCSFlags,
    requirement: *mut __SecRequirement,
  ) -> i32;
  pub fn SecCodeCheckValidityWithErrors(
    code: *mut __SecCode,
    flags: SecCSFlags,
    requirement: *mut __SecRequirement,
    errors: *mut *mut __CFError,
  ) -> i32;
  pub fn SecCodeCopyPath(
    staticCode: *mut __SecCode,
    flags: SecCSFlags,
    path: *mut *mut __CFURL,
  ) -> i32;
  pub fn SecCodeCopyDesignatedRequirement(
    code: *mut __SecCode,
    flags: SecCSFlags,
    requirement: *mut *mut __SecRequirement,
  ) -> i32;
  pub fn SecCodeCopySigningInformation(
    code: *mut __SecCode,
    flags: SecCSFlags,
    information: *mut *mut __CFDictionary,
  ) -> i32;
  pub fn SecCodeMapMemory(code: *mut __SecCode, flags: SecCSFlags) -> i32;
  pub fn SecHostCreateGuest(
    host: u32,
    status: u32,
    path: *mut __CFURL,
    attributes: *mut __CFDictionary,
    flags: SecCSFlags,
    newGuest: *mut u32,
  ) -> i32;
  pub fn SecHostRemoveGuest(host: u32, guest: u32, flags: SecCSFlags) -> i32;
  pub fn SecHostSelectGuest(guestRef: u32, flags: SecCSFlags) -> i32;
  pub fn SecHostSelectedGuest(flags: SecCSFlags, guestRef: *mut u32) -> i32;
  pub fn SecHostSetGuestStatus(
    guestRef: u32,
    status: u32,
    attributes: *mut __CFDictionary,
    flags: SecCSFlags,
  ) -> i32;
  pub fn SecHostSetHostingPort(hostingPort: u32, flags: SecCSFlags) -> i32;
  pub fn SecRequirementGetTypeID() -> usize;
  pub fn SecRequirementCreateWithData(
    data: *mut __CFData,
    flags: SecCSFlags,
    requirement: *mut *mut __SecRequirement,
  ) -> i32;
  pub fn SecRequirementCreateWithString(
    text: *mut __CFString,
    flags: SecCSFlags,
    requirement: *mut *mut __SecRequirement,
  ) -> i32;
  pub fn SecRequirementCreateWithStringAndErrors(
    text: *mut __CFString,
    flags: SecCSFlags,
    errors: *mut *mut __CFError,
    requirement: *mut *mut __SecRequirement,
  ) -> i32;
  pub fn SecRequirementCopyData(
    requirement: *mut __SecRequirement,
    flags: SecCSFlags,
    data: *mut *mut __CFData,
  ) -> i32;
  pub fn SecRequirementCopyString(
    requirement: *mut __SecRequirement,
    flags: SecCSFlags,
    text: *mut *mut __CFString,
  ) -> i32;
  pub fn SecTaskGetTypeID() -> usize;
  pub fn SecTaskCreateWithAuditToken(
    allocator: *mut __CFAllocator,
    token: audit_token_t,
  ) -> *mut __SecTask;
  pub fn SecTaskCreateFromSelf(allocator: *mut __CFAllocator) -> *mut __SecTask;
  pub fn SecTaskCopyValueForEntitlement(
    task: *mut __SecTask,
    entitlement: *mut __CFString,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecTaskCopyValuesForEntitlements(
    task: *mut __SecTask,
    entitlements: *mut __CFArray,
    error: *mut *mut __CFError,
  ) -> *mut __CFDictionary;
  pub fn SecTaskCopySigningIdentifier(
    task: *mut __SecTask,
    error: *mut *mut __CFError,
  ) -> *mut __CFString;
  pub fn AuthorizationRightGet(
    rightName: *mut i8,
    rightDefinition: *mut *mut __CFDictionary,
  ) -> i32;
  pub fn AuthorizationRightSet(
    authRef: *mut AuthorizationOpaqueRef,
    rightName: *mut i8,
    rightDefinition: *mut c_void,
    descriptionKey: *mut __CFString,
    bundle: *mut __CFBundle,
    localeTableName: *mut __CFString,
  ) -> i32;
  pub fn AuthorizationRightRemove(authRef: *mut AuthorizationOpaqueRef, rightName: *mut i8) -> i32;
  pub fn CMSDecoderGetTypeID() -> usize;
  pub fn CMSDecoderCreate(cmsDecoderOut: *mut *mut _CMSDecoder) -> i32;
  pub fn CMSDecoderUpdateMessage(
    cmsDecoder: *mut _CMSDecoder,
    msgBytes: *mut c_void,
    msgBytesLen: usize,
  ) -> i32;
  pub fn CMSDecoderFinalizeMessage(cmsDecoder: *mut _CMSDecoder) -> i32;
  pub fn CMSDecoderSetDetachedContent(
    cmsDecoder: *mut _CMSDecoder,
    detachedContent: *mut __CFData,
  ) -> i32;
  pub fn CMSDecoderCopyDetachedContent(
    cmsDecoder: *mut _CMSDecoder,
    detachedContentOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSDecoderSetSearchKeychain(
    cmsDecoder: *mut _CMSDecoder,
    keychainOrArray: *mut c_void,
  ) -> i32;
  pub fn CMSDecoderGetNumSigners(cmsDecoder: *mut _CMSDecoder, numSignersOut: *mut usize) -> i32;
  pub fn CMSDecoderCopySignerStatus(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    policyOrArray: *mut c_void,
    evaluateSecTrust: u8,
    signerStatusOut: *mut CMSSignerStatus,
    secTrustOut: *mut *mut __SecTrust,
    certVerifyResultCodeOut: *mut i32,
  ) -> i32;
  pub fn CMSDecoderCopySignerEmailAddress(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    signerEmailAddressOut: *mut *mut __CFString,
  ) -> i32;
  pub fn CMSDecoderCopySignerCert(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    signerCertOut: *mut *mut OpaqueSecCertificateRef,
  ) -> i32;
  pub fn CMSDecoderIsContentEncrypted(cmsDecoder: *mut _CMSDecoder, isEncryptedOut: *mut u8)
    -> i32;
  pub fn CMSDecoderCopyEncapsulatedContentType(
    cmsDecoder: *mut _CMSDecoder,
    eContentTypeOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSDecoderCopyAllCerts(cmsDecoder: *mut _CMSDecoder, certsOut: *mut *mut __CFArray)
    -> i32;
  pub fn CMSDecoderCopyContent(cmsDecoder: *mut _CMSDecoder, contentOut: *mut *mut __CFData)
    -> i32;
  pub fn CMSDecoderCopySignerSigningTime(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    signingTime: *mut f64,
  ) -> i32;
  pub fn CMSDecoderCopySignerTimestamp(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    timestamp: *mut f64,
  ) -> i32;
  pub fn CMSDecoderCopySignerTimestampWithPolicy(
    cmsDecoder: *mut _CMSDecoder,
    timeStampPolicy: *mut c_void,
    signerIndex: usize,
    timestamp: *mut f64,
  ) -> i32;
  pub fn CMSDecoderCopySignerTimestampCertificates(
    cmsDecoder: *mut _CMSDecoder,
    signerIndex: usize,
    certificateRefs: *mut *mut __CFArray,
  ) -> i32;
  pub fn CMSEncoderGetTypeID() -> usize;
  pub fn CMSEncoderCreate(cmsEncoderOut: *mut *mut _CMSEncoder) -> i32;
  pub fn CMSEncoderSetSignerAlgorithm(
    cmsEncoder: *mut _CMSEncoder,
    digestAlgorithm: *mut __CFString,
  ) -> i32;
  pub fn CMSEncoderAddSigners(cmsEncoder: *mut _CMSEncoder, signerOrArray: *mut c_void) -> i32;
  pub fn CMSEncoderCopySigners(
    cmsEncoder: *mut _CMSEncoder,
    signersOut: *mut *mut __CFArray,
  ) -> i32;
  pub fn CMSEncoderAddRecipients(
    cmsEncoder: *mut _CMSEncoder,
    recipientOrArray: *mut c_void,
  ) -> i32;
  pub fn CMSEncoderCopyRecipients(
    cmsEncoder: *mut _CMSEncoder,
    recipientsOut: *mut *mut __CFArray,
  ) -> i32;
  pub fn CMSEncoderSetHasDetachedContent(cmsEncoder: *mut _CMSEncoder, detachedContent: u8) -> i32;
  pub fn CMSEncoderGetHasDetachedContent(
    cmsEncoder: *mut _CMSEncoder,
    detachedContentOut: *mut u8,
  ) -> i32;
  pub fn CMSEncoderSetEncapsulatedContentType(
    cmsEncoder: *mut _CMSEncoder,
    eContentType: *mut CSSM_DATA,
  ) -> i32;
  pub fn CMSEncoderSetEncapsulatedContentTypeOID(
    cmsEncoder: *mut _CMSEncoder,
    eContentTypeOID: *mut c_void,
  ) -> i32;
  pub fn CMSEncoderCopyEncapsulatedContentType(
    cmsEncoder: *mut _CMSEncoder,
    eContentTypeOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSEncoderAddSupportingCerts(
    cmsEncoder: *mut _CMSEncoder,
    certOrArray: *mut c_void,
  ) -> i32;
  pub fn CMSEncoderCopySupportingCerts(
    cmsEncoder: *mut _CMSEncoder,
    certsOut: *mut *mut __CFArray,
  ) -> i32;
  pub fn CMSEncoderAddSignedAttributes(
    cmsEncoder: *mut _CMSEncoder,
    signedAttributes: CMSSignedAttributes,
  ) -> i32;
  pub fn CMSEncoderSetCertificateChainMode(
    cmsEncoder: *mut _CMSEncoder,
    chainMode: CMSCertificateChainMode,
  ) -> i32;
  pub fn CMSEncoderGetCertificateChainMode(
    cmsEncoder: *mut _CMSEncoder,
    chainModeOut: *mut CMSCertificateChainMode,
  ) -> i32;
  pub fn CMSEncoderUpdateContent(
    cmsEncoder: *mut _CMSEncoder,
    content: *mut c_void,
    contentLen: usize,
  ) -> i32;
  pub fn CMSEncoderCopyEncodedContent(
    cmsEncoder: *mut _CMSEncoder,
    encodedContentOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSEncode(
    signers: *mut c_void,
    recipients: *mut c_void,
    eContentType: *mut CSSM_DATA,
    detachedContent: u8,
    signedAttributes: CMSSignedAttributes,
    content: *mut c_void,
    contentLen: usize,
    encodedContentOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSEncodeContent(
    signers: *mut c_void,
    recipients: *mut c_void,
    eContentTypeOID: *mut c_void,
    detachedContent: u8,
    signedAttributes: CMSSignedAttributes,
    content: *mut c_void,
    contentLen: usize,
    encodedContentOut: *mut *mut __CFData,
  ) -> i32;
  pub fn CMSEncoderCopySignerTimestamp(
    cmsEncoder: *mut _CMSEncoder,
    signerIndex: usize,
    timestamp: *mut f64,
  ) -> i32;
  pub fn CMSEncoderCopySignerTimestampWithPolicy(
    cmsEncoder: *mut _CMSEncoder,
    timeStampPolicy: *mut c_void,
    signerIndex: usize,
    timestamp: *mut f64,
  ) -> i32;
  pub fn SecTransformGetTypeID() -> usize;
  pub fn SecGroupTransformGetTypeID() -> usize;
  pub fn SecTransformCreateFromExternalRepresentation(
    dictionary: *mut __CFDictionary,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecTransformCopyExternalRepresentation(transformRef: *mut c_void) -> *mut __CFDictionary;
  pub fn SecTransformCreateGroupTransform() -> *mut c_void;
  pub fn SecTransformConnectTransforms(
    sourceTransformRef: *mut c_void,
    sourceAttributeName: *mut __CFString,
    destinationTransformRef: *mut c_void,
    destinationAttributeName: *mut __CFString,
    group: *mut c_void,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecTransformSetAttribute(
    transformRef: *mut c_void,
    key: *mut __CFString,
    value: *mut c_void,
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn SecTransformGetAttribute(transformRef: *mut c_void, key: *mut __CFString) -> *mut c_void;
  pub fn SecTransformFindByName(transform: *mut c_void, name: *mut __CFString) -> *mut c_void;
  pub fn SecTransformExecute(
    transformRef: *mut c_void,
    errorRef: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecTransformExecuteAsync(
    transformRef: *mut c_void,
    deliveryQueue: *mut NSObject,
    deliveryBlock: (),
  ) -> ();
  pub fn SecTransformSetAttributeAction(
    ref_: *mut OpaqueSecTransformImplementation,
    action: *mut __CFString,
    attribute: *mut c_void,
    newAction: (),
  ) -> *mut __CFError;
  pub fn SecTransformSetDataAction(
    ref_: *mut OpaqueSecTransformImplementation,
    action: *mut __CFString,
    newAction: (),
  ) -> *mut __CFError;
  pub fn SecTransformSetTransformAction(
    ref_: *mut OpaqueSecTransformImplementation,
    action: *mut __CFString,
    newAction: (),
  ) -> *mut __CFError;
  pub fn SecTranformCustomGetAttribute(
    ref_: *mut OpaqueSecTransformImplementation,
    attribute: *mut c_void,
    type_: SecTransformMetaAttributeType,
  ) -> *mut c_void;
  pub fn SecTransformCustomGetAttribute(
    ref_: *mut OpaqueSecTransformImplementation,
    attribute: *mut c_void,
    type_: SecTransformMetaAttributeType,
  ) -> *mut c_void;
  pub fn SecTransformCustomSetAttribute(
    ref_: *mut OpaqueSecTransformImplementation,
    attribute: *mut c_void,
    type_: SecTransformMetaAttributeType,
    value: *mut c_void,
  ) -> *mut c_void;
  pub fn SecTransformPushbackAttribute(
    ref_: *mut OpaqueSecTransformImplementation,
    attribute: *mut c_void,
    value: *mut c_void,
  ) -> *mut c_void;
  pub fn SecTransformRegister(
    uniqueName: *mut __CFString,
    createTransformFunction: extern "C" fn(
      *mut __CFString,
      *mut c_void,
      *mut OpaqueSecTransformImplementation,
    ) -> (),
    error: *mut *mut __CFError,
  ) -> u8;
  pub fn SecTransformCreate(name: *mut __CFString, error: *mut *mut __CFError) -> *mut c_void;
  pub fn SecTransformNoData() -> *mut c_void;
  pub fn SecEncodeTransformCreate(
    encodeType: *mut c_void,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecDecodeTransformCreate(
    DecodeType: *mut c_void,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecDigestTransformCreate(
    digestType: *mut c_void,
    digestLength: isize,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecDigestTransformGetTypeID() -> usize;
  pub fn SecEncryptTransformCreate(
    keyRef: *mut OpaqueSecKeyRef,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecDecryptTransformCreate(
    keyRef: *mut OpaqueSecKeyRef,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecDecryptTransformGetTypeID() -> usize;
  pub fn SecEncryptTransformGetTypeID() -> usize;
  pub fn SecSignTransformCreate(
    key: *mut OpaqueSecKeyRef,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecVerifyTransformCreate(
    key: *mut OpaqueSecKeyRef,
    signature: *mut __CFData,
    error: *mut *mut __CFError,
  ) -> *mut c_void;
  pub fn SecTransformCreateReadTransformWithReadStream(
    inputStream: *mut __CFReadStream,
  ) -> *mut c_void;
}
