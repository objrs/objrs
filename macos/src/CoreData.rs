#[allow(unused_imports)]
use crate::objc::*;
use crate::Foundation::NSArray;
use crate::Foundation::NSBundle;
use crate::Foundation::NSCodingProto;
use crate::Foundation::NSCopyingProto;
use crate::Foundation::NSData;
use crate::Foundation::NSDate;
use crate::Foundation::NSDictionary;
use crate::Foundation::NSError;
use crate::Foundation::NSExpression;
use crate::Foundation::NSFastEnumerationProto;
use crate::Foundation::NSIndexPath;
use crate::Foundation::NSKeyValueSetMutationKind;
use crate::Foundation::NSLockingProto;
use crate::Foundation::NSMutableDictionary;
use crate::Foundation::NSNotification;
use crate::Foundation::NSPredicate;
use crate::Foundation::NSProgress;
use crate::Foundation::NSSecureCodingProto;
use crate::Foundation::NSSet;
use crate::Foundation::NSSortDescriptor;
use crate::Foundation::NSString;
use crate::Foundation::NSUndoManager;
use crate::Foundation::NSValue;
use crate::Foundation::NSURL;
use crate::NSObject::NSObject;
use crate::NSObject::NSObjectProto;
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
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPropertyDescription;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSAttributeType {
  NSUndefinedAttributeType = 0,
  NSInteger16AttributeType = 100,
  NSInteger32AttributeType = 200,
  NSInteger64AttributeType = 300,
  NSDecimalAttributeType = 400,
  NSDoubleAttributeType = 500,
  NSFloatAttributeType = 600,
  NSStringAttributeType = 700,
  NSBooleanAttributeType = 800,
  NSDateAttributeType = 900,
  NSBinaryDataAttributeType = 1000,
  NSUUIDAttributeType = 1100,
  NSURIAttributeType = 1200,
  NSTransformableAttributeType = 1800,
  NSObjectIDAttributeType = 2000,
}
# [ objrs ( class , super = NSPropertyDescription ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSAttributeDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSEntityDescription;
# [ objrs ( class , super = NSPropertyDescription ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchedPropertyDescription;
# [ objrs ( class , super = NSPropertyDescription ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSExpressionDescription;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSDeleteRule {
  NSNoActionDeleteRule = 0,
  NSNullifyDeleteRule = 1,
  NSCascadeDeleteRule = 2,
  NSDenyDeleteRule = 3,
}
# [ objrs ( class , super = NSPropertyDescription ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSRelationshipDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchIndexDescription;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFetchIndexElementType {
  NSFetchIndexElementTypeBinary = 0,
  NSFetchIndexElementTypeRTree = 1,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchIndexElementDescription;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPersistentStoreRequestType {
  NSFetchRequestType = 1,
  NSSaveRequestType = 2,
  NSBatchUpdateRequestType = 6,
  NSBatchDeleteRequestType = 7,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStoreRequest;
bitflags! { # [ repr ( C ) ] pub struct NSSnapshotEventType : usize { const NSSnapshotEventUndoInsertion = 2 ; const NSSnapshotEventUndoDeletion = 4 ; const NSSnapshotEventUndoUpdate = 8 ; const NSSnapshotEventRollback = 16 ; const NSSnapshotEventRefresh = 32 ; const NSSnapshotEventMergePolicy = 64 ; } }
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSManagedObject;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSManagedObjectID;
bitflags! { # [ repr ( C ) ] pub struct NSFetchRequestResultType : usize { const NSManagedObjectResultType = 0 ; const NSManagedObjectIDResultType = 1 ; const NSDictionaryResultType = 2 ; const NSCountResultType = 4 ; } }
#[objrs(protocol)]
#[link(name = "CoreData", kind = "framework")]
pub trait NSFetchRequestResultProto {}
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchRequest;
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSAsynchronousFetchRequest;
# [ objrs ( class , super = NSExpression ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchRequestExpression;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSManagedObjectModel;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSManagedObjectContextConcurrencyType {
  NSConfinementConcurrencyType = 0,
  NSPrivateQueueConcurrencyType = 1,
  NSMainQueueConcurrencyType = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSManagedObjectContext;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStoreCoordinator;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSPersistentStoreUbiquitousTransitionType {
  NSPersistentStoreUbiquitousTransitionTypeAccountAdded = 1,
  NSPersistentStoreUbiquitousTransitionTypeAccountRemoved = 2,
  NSPersistentStoreUbiquitousTransitionTypeContentRemoved = 3,
  NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStore;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSAtomicStoreCacheNode;
# [ objrs ( class , super = NSPersistentStore ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSAtomicStore;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSEntityMigrationPolicy;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSMappingModel;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSEntityMappingType {
  NSUndefinedEntityMappingType = 0,
  NSCustomEntityMappingType = 1,
  NSAddEntityMappingType = 2,
  NSRemoveEntityMappingType = 3,
  NSCopyEntityMappingType = 4,
  NSTransformEntityMappingType = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSEntityMapping;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPropertyMapping;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSMigrationManager;
# [ objrs ( class , super = NSPersistentStore ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSIncrementalStore;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSIncrementalStoreNode;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBatchUpdateRequestResultType {
  NSStatusOnlyResultType = 0,
  NSUpdatedObjectIDsResultType = 1,
  NSUpdatedObjectsCountResultType = 2,
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSBatchDeleteRequestResultType {
  NSBatchDeleteResultTypeStatusOnly = 0,
  NSBatchDeleteResultTypeObjectIDs = 1,
  NSBatchDeleteResultTypeCount = 2,
}
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPersistentHistoryResultType {
  NSPersistentHistoryResultTypeStatusOnly = 0,
  NSPersistentHistoryResultTypeObjectIDs = 1,
  NSPersistentHistoryResultTypeCount = 2,
  NSPersistentHistoryResultTypeTransactionsOnly = 3,
  NSPersistentHistoryResultTypeChangesOnly = 4,
  NSPersistentHistoryResultTypeTransactionsAndChanges = 5,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStoreResult;
# [ objrs ( class , super = NSPersistentStoreResult ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStoreAsynchronousResult;
# [ objrs ( class , super = NSPersistentStoreAsynchronousResult ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSAsynchronousFetchResult;
# [ objrs ( class , super = NSPersistentStoreResult ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSBatchUpdateResult;
# [ objrs ( class , super = NSPersistentStoreResult ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSBatchDeleteResult;
# [ objrs ( class , super = NSPersistentStoreResult ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentHistoryResult;
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSSaveChangesRequest;
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSBatchUpdateRequest;
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSBatchDeleteRequest;
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSMergePolicyType {
  NSErrorMergePolicyType = 0,
  NSMergeByPropertyStoreTrumpMergePolicyType = 1,
  NSMergeByPropertyObjectTrumpMergePolicyType = 2,
  NSOverwriteMergePolicyType = 3,
  NSRollbackMergePolicyType = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSMergeConflict;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSConstraintConflict;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSMergePolicy;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSFetchedResultsController;
#[objrs(protocol)]
#[link(name = "CoreData", kind = "framework")]
pub trait NSFetchedResultsSectionInfoProto {}
#[objrs(protocol)]
#[link(name = "CoreData", kind = "framework")]
pub trait NSFetchedResultsControllerDelegateProto {
  #[objrs(selector = "controller:didChangeObject:atIndexPath:forChangeType:newIndexPath:")]
  #[cfg(feature = "RK_CoreData")]
  fn controller_didChangeObject_atIndexPath_forChangeType_newIndexPath_(
    &self,
    controller: &NSFetchedResultsController,
    anObject: &Object,
    indexPath: Option<&NSIndexPath>,
    type_: NSFetchedResultsChangeType,
    newIndexPath: Option<&NSIndexPath>,
  ) -> ();
  #[objrs(selector = "controller:didChangeSection:atIndex:forChangeType:")]
  #[cfg(feature = "RK_CoreData")]
  fn controller_didChangeSection_atIndex_forChangeType_(
    &self,
    controller: &NSFetchedResultsController,
    sectionInfo: &Object,
    sectionIndex: usize,
    type_: NSFetchedResultsChangeType,
  ) -> ();
  #[objrs(selector = "controllerWillChangeContent:")]
  #[cfg(feature = "RK_CoreData")]
  fn controllerWillChangeContent_(&self, controller: &NSFetchedResultsController) -> ();
  #[objrs(selector = "controllerDidChangeContent:")]
  #[cfg(feature = "RK_CoreData")]
  fn controllerDidChangeContent_(&self, controller: &NSFetchedResultsController) -> ();
  #[objrs(selector = "controller:sectionIndexTitleForSectionName:")]
  #[cfg(feature = "RK_CoreData")]
  fn controller_sectionIndexTitleForSectionName_(
    &self,
    controller: &NSFetchedResultsController,
    sectionName: &NSString,
  ) -> Option<Arc<NSString>>;
}
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum NSFetchedResultsChangeType {
  NSFetchedResultsChangeInsert = 1,
  NSFetchedResultsChangeDelete = 2,
  NSFetchedResultsChangeMove = 3,
  NSFetchedResultsChangeUpdate = 4,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSQueryGenerationToken;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentStoreDescription;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentContainer;
#[repr(isize)]
#[derive(Copy, Clone)]
pub enum NSPersistentHistoryChangeType {
  NSPersistentHistoryChangeTypeInsert = 0,
  NSPersistentHistoryChangeTypeUpdate = 1,
  NSPersistentHistoryChangeTypeDelete = 2,
}
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentHistoryChange;
# [ objrs ( class , super = NSPersistentStoreRequest ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentHistoryChangeRequest;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentHistoryToken;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSPersistentHistoryTransaction;
# [ objrs ( class , super = NSObject ) ]
#[link(name = "CoreData", kind = "framework")]
pub struct NSCoreDataCoreSpotlightDelegate;
#[cfg(feature = "RK_CoreData")]
#[link(name = "CoreData", kind = "framework")]
extern "C" {}
