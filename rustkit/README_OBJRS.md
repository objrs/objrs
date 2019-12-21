The "objrs" feature emits code in Objective Rust format.

The notes below are personal thoughts and goals of this project as it evolves.

# Crates
The Platform SDK of each target platform, ios or macos, is compiled into a single crate.
The crate has one module per framework. More below.


# Current State
Both the iOS and Mac crate build against the Xcode 10.x SDK.
More recent SDKs are not yet supported, and fail. Also, clang 8.0.1, with the patch mentioned in README.md, are required. Note: the tests don't run for iOS yet.

The crates compile for the rustkit format, that is, with the "objrs" feature turned off.

With the "objrs" feature selected, only portions of the macos crate are currently emitted, and
require a bit of hand-patching to compile. The hand-patched sources have been checked into the "macos/src" directory of the parent objrs project, into which they are generated.

# Building RustKit
1. Install Xcode 10. Since you're probably using Xcode 11, you will need a side-by-side installation of Xcode 10. Extract Xcode 10 in a temporary directory, and rename Xcode.app to Xcode10.app so that it doesn't conflict with Xcode 11, which is also named Xcode.app.  Then move Xcode10.app to /Applications. Verify that the SDK paths match the paths in lines 2225 and 2227 of `rustkit_bindgen/src/lib.rs`. This is where rustkit will look for the SDKs.
   
2. You will need to build clang 8.0.1, with the patch mentioned in README.md, is required. 
The repo is at https://github.com/llvm/llvm-project.git.
The tested version is the head of the release/8.x branch, which is commit 89de0d.
Set LIBCLANG_PATH as directed in README.md.

3. Run `cargo run` in the rustkit directory to build and run rustkit. This will generate the bindings in `objrs/macos/src` (minus the hand patches). Git-revert the output for now so that you still have the checked-in bindings.

4. Build the `objrs/macos` crate with the nightly build. The bindings should compile without error (but with a ton of warnings).

5. There's a macos-demo project that doesn't compile yet, which uses these bindings.

# Module Layout
Each of the two platform crates has one module per framework. The frameworks are Foundation, AppKit, UIKit, 
MetalKit, CoreGraphics etc.

Thus, we have
macos (crate)
+ Foundation
+ CoreGraphics
+ AppKit
+ ... etc

and 

ios (crate) [This does not exist yet]
+ Foundation
+ CoreGraphics
+ UIKit
+ ... etc


[TODO: add feature selection per framework to speed up compile times for users of objrs.]

In addition there are currently several top-level headers that need to be organized:

MacTypes.h:
sys/acl.h: 
hfs_unistr.h: Mac only. contains a single type, `HFSUniStr255`. Is this needed?
NSObject.h: part of the Objective-C runtime, but for simplicity we expose it as part of Foundation.
mach/message.h: 
simd/types.h: 


# Classes

## Normal classes

	#[objrs(class, super = NSObject)]
	#[link(name = "AppKit", kind = "framework")]
	pub struct NSMenuIttem;

## Generics

Objrs allows generics to be constructed by hand. We may need external type annotations
to facilitate this.

	#[objrs(class, super = nsobject::NSObject)]
	#[link(name = "Foundation", kind = "framework")]
	pub struct NSArray<T: objrs::marker::Class + ?Sized>;

# Methods (Instance and Class Properties, Instance and Class Methods)

Consider the Objective-C property `NSMenuItem::subMenu`

In Rustkit, The syntax is:

    pub fn submenu(&self) -> Option<Arc<NSMenu>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut NSMenu =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_submenu);
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn setSubmenu_(&self, submenu: Option<&NSMenu>) -> () {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut NSMenu) -> () =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_setSubmenu_,
                submenu
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }

The objrs syntax is:
	#[objrs(selector = "submenu")]
	pub fn submenu(&self) -> &NSMenu {} // NOTE: This was handed-coded. Not sure if it is correct.

	#[objrs(selector = "setSubmenu:")]  
	pub fn set_submenu(&mut self, menu: objrs::Strong<NSMenu>) {}

# Method Argument And Return Types
RustKit uses &mut and &Option<Arc<Object>> to define arguments. 

    fn countByEnumeratingWithState_objects_count_(
        &self,
        state: &NSFastEnumerationState,
        buffer: &mut &Option<Arc<Object>>,
        len: usize,
    ) -> usize;

# Protocols

Rustkit:

	pub trait NSDiscardableContentProto: ObjCClass {
		...
	}

Objrs:

Objective-C protocols are implemented in objrs as traits. Use #[objrs(protocol)] on a Rust trait to declare it as an Objective-C protocol.

	#[objrs(protocol, id_ident = NSDiscardableContentId)]
	#[link(name = "Foundation", kind = "framework")]
	trait NSDiscardableContent {
		#[objrs(selector = "hash"]
		fn hash(&self) -> usize;
	}

