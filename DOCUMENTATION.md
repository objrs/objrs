# Features and documentation

objrs supports the following Objective-C features:

- External classes (e.g. from an Apple framework) and custom classes (e.g. your own custom definition of a class), complete with inheritance.
- Instance variables (even if they have nontrivial constructors or destructors).
- Class and instance methods.
- External protocols (e.g. from an Apple framework) (see below; they're only partially supported).

## `#[objrs(class)]`

Apply the `#[objrs(class)]` attribute to a `struct` item to declare it as an Objective-C class. Instance variables are supported with the limitation that they must implement Rust's `Default` trait.

Parameters:
- `name = "LITERAL_STR"`. Optional. The literal string is the class's name (i.e. what [`NSStringFromClass`](https://developer.apple.com/documentation/foundation/1395143-nsstringfromclass?language=objc) would return). If this is omitted, the class name will be the same as the struct's' identifier (e.g. `struct NSObject` → `"NSObject"`; `struct NSArray<T>` → `"NSArray"`). It is suggested you omit this and just give the struct the same name as the class it represents. If you use this, [see the full documentation](DOCUMENTATION.md) for additional parameters you'll have to pass to subclasses.
- `root_class`. One of `root_clas` or `super = ...` must be specified. In practice this is only useful for `NSObject`.
- `super = TYPE`. The class's super type.
- `extern`. Optional. If this is omitted, objrs will automatically treat the class as an external class if a `#[link(...)]` attribute is present. An external class is one provided by an external framework (e.g. `NSArray`, `UIApplication`, etc.).

Both of these definitions for `NSObject` are equivalent:
```rust
#[objrs(class, root_class, extern)]
struct NSObject;

#[objrs(class, root_class)]
#[link(name = "Foundation", kind = "framework")]
struct NSObject;
```

Both `NSString` and `NSArray` inherit from `NSObject` and are external classes:
```rust
#[objrs(class, super = NSObject)]
#[link(name = "Foundation", kind = "framework")]
struct NSString;

// Generics are supported, but they can only be Objective-C class types.
#[objrs(class, super = NSObject)]
#[link(name = "Foundation", kind = "framework")]
struct NSArray<T: objrs::marker::Class + ?Sized>;
```

Custom classes can be created by omitting `extern` and the `#[link(...)]` attribute. Instance variables are supported, even if they have non-trivial constructors and destructors:
```rust
#[objrs(class, super = NSObject)]
struct MyCustomObject {
  ivar1: usize,
  ivar2: String,
}
```

> **Syntax**
>
> *Attribute* :<br>
> &nbsp;&nbsp; `#` `[` `objrs` `(` `class` *Name* *RootOrSuper* *Extern* `)` `]`
>
> *Name* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `name` `=` LITERAL_STR<br>
>
> *RootOrSuper* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `root_class`<br>
> &nbsp;&nbsp; | `,` *Super* *RootClass*<br>
>
> *Super* :<br>
> &nbsp;&nbsp; | `super` `=` TYPE<br>
> &nbsp;&nbsp; | `super` `(` `name` `=` LITERAL_STR `,` `type` `=` TYPE `)`<br>
>
> *RootClass* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `root_class` `=` LITERAL_STR<br>
>
> *Extern* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `extern`

## `#[objrs(protocol)]`

Objective-C protocols are implemented in objrs as traits. Use `#[objrs(protocol)]` on a Rust trait to declare it as an Objective-C protocol.

Parameters:
- `name = "LITERAL_STR"`. Optional. The literal string is the protocol's name (i.e. what [`NSStringFromProtocol`](https://developer.apple.com/documentation/foundation/1395298-nsstringfromprotocol?language=objc) would return). If this is omitted, the protocol name will be the same as the trait's identifier (e.g. `trait NSCopying` → `"NSCopying"`). It is suggested you omit this and just give the trait the same name as the protocol it represents. If you use this, [see the full documentation](DOCUMENTATION.md) for additional parameters you'll have to pass to implementations.
- `id_ident = IDENT`. Required. This is an unfortunate hack that will hopefully be temporary. [RFC 255](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md) is the reason this is required, but there's no reason the RFC couldn't be relaxed a little. RFC 255 requires us to use a real struct object type (rather than a trait object type) when representing an Objective-C `id<ProtocolName>` type (e.g. `id<NSCopying>`). The identifier provided here will be injected into the code as a struct type that implements the protocol. It is suggested that you use the same name as the trait, but with an "`Id`" suffix (e.g. `trait NSCopying` → `id_ident = NSCopyingId`).
- `extern`. Optional. If this is omitted, objrs will automatically treat the protocol as an external protocol if a `#[link(...)]` attribute is present. Only external protocols are currently supported (custom protocols are not yet supported).

```rust
#[objrs(protocol, id_ident = NSObjectId)]
#[link(name = "Foundation", kind = "framework")]
trait NSObject {
  #[objrs(selector = "hash"]
  fn hash(&self) -> usize;
}
```

Due to RFC 255's limitations, you have to use `NSObjectId` if you want the equivalent of Objective-C's `id<NSObject>`:
```rust
fn do_nothing(id: &NSObjectId) -> &NSObjectId {
  return id;
}
```

> **Syntax**
>
> *Attribute* :<br>
> &nbsp;&nbsp; `#` `[` `objrs` `(` `protocol` *Name* *IdIdent* *Properties* *Extern* `)` `]`
>
> *Name* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `name` `=` LITERAL_STR<br>
>
> *IdIdent* :<br>
> &nbsp;&nbsp; &nbsp; `,` `id_ident` `=` IDENT<br>
>
> *Properties* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | *Property* *Properties*<br>
>
> *Property* :<br>
> &nbsp;&nbsp; &nbsp; `property` `(` IDENT `:` TYPE *Class* *ReadWrite* *Assign* *UnsafeUnretained* *Copy* *WeakStrong* *Retain* *Nullability* *Atomicity* *NullResettable* *Getter* *Setter* `)`<br>
>
> *Class* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `class`<br>
>
> *ReadWrite* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `readonly`<br>
> &nbsp;&nbsp; | `,` `readwrite`<br>
>
> *Assign* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `assign`<br>
>
> *UnsafeUnretained* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `unsafe_unretained`<br>
>
> *Copy* : (note: must be EMPTY if `assign` or `unsafe_unretained` are present)<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `copy`<br>
>
> *WeakStrong* : (note: must be EMPTY if `assign`, `unsafe_unretained`, or `copy` are present)<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `weak`<br>
> &nbsp;&nbsp; | `,` `strong`<br>
>
> *Retain* : (note: must be EMPTY if `assign`, `unsafe_unretained`, `copy`, or `weak` are present)<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `retain`<br>
>
> *Nullability* : (note: must not be `nonnull` if `weak` is present)<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `nullable`<br>
> &nbsp;&nbsp; | `,` `nonnull`<br>
> &nbsp;&nbsp; | `,` `null_unspecified`<br>
>
> *Atomicity* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `atomic`<br>
> &nbsp;&nbsp; | `,` `nonatomic`<br>
>
> *NullResettable* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `null_resettable`<br>
>
> *Getter* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `getter` `=` LITERAL_STR<br>
>
> *Setter* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `setter` `=` LITERAL_STR<br>
>
> *Extern* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `extern`

## `#[objrs(impl)]`

Use `#[objrs(impl)]` on an `impl` item when implementing a class (e.g. `impl Class`) or when implementing a protocol for a class (e.g. `impl Protocol for Class`).

Parameters:
- `extern`. Optional. If this is omitted, objrs will automatically treat the class as an external class if a `#[link(...)]` attribute is present. An external class is one provided by an external framework (e.g. `NSArray`, `UIApplication`, etc.).

Implementing a method `foo` for the previous (external) `NSObject` class:
```rust
#[objrs(impl)]
#[link(name = "Foundation", kind = "framework")]
impl NSObject {
  #[objrs(selector = "hash")]
  pub fn hash(&self) -> usize {}
}
```

Implementing a method `foo` for the previous `MyCustomObject` class:
```rust
#[objrs(impl)]
impl MyCustomObject {
  #[objrs(selector = "foo")]
  pub fn foo(&mut self) {
    self.ivar1 = 42;
    self.ivar2 = "Hello, world!".to_string();
  }
}
```

Implementing the `NSObject` protocol for the previous `MyCustomObject` class:
```rust
#[objrs(impl)]
impl NSObject for MyCustomObject {
  #[objrs(selector = "hash")]
  fn hash(&self) -> usize {
    return self.ivar1;
  }
}
```

> **Syntax**
>
> *Attribute* :<br>
> &nbsp;&nbsp; `#` `[` `objrs` `(` `impl` *ClassName* *TraitName* *Extern* `)` `]`
>
> *ClassName* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `class_name` `=` LITERAL_STR<br>
>
> *TraitName* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `protocol_name` `=` LITERAL_STR<br>
> &nbsp;&nbsp; | `,` `category_name` `=` LITERAL_STR<br>
>
> *Extern* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `extern`

## `#[objrs(selector)]`

Use `#[objrs(selector)]` on a method to declare it as an Objective-C method.

Parameters:
- `selector = "LITERAL_STR"`. Required. This is the actual name of the selector (e.g. `"doFoo:withBar:"`).
- `super` or `no_impl`. Optional. Use `super` to declare the method as a super method invocation. Use `no_impl` to simply declare that the class responds to that selector but does so by using the super's implementation. The difference between `super` and `no_impl` is subtle, so see the examples below.
- `class` or `instance`. Optional. objrs will inspect the method's signature and if it takes `self`, the method will be an instance method. If it lacks `self`, it will be a class method. Sometimes you can't take a `self` parameter when you need to, though, which means objrs's auto-deductions aren't useful. In these situations, you can explicitly declare the method as being a `class` or `instance` method.
- `optional`. Optional. Equivalent to Objective-C's `@optional`. Only use this in protocol traits. Attempting to call an `optional` method that the class doesn't implement will result in a panic (or exception, if calling from Objective-C).

Calling a super's method:
```rust
#[objrs(impl)]
impl MyCustomObject {
  #[objrs(selector = "hash")]
  pub fn hash(&self) -> usize {
    println!("-[MyCustomObject hash] is being called. Forwarding to [super hash]");
    return self.super_hash();
  }

  #[objrs(selector = "hash", super)]
  fn super_hash(&self) -> usize {}
}
```

Here, the super's `hash` method will be invoked. Using `no_impl` can be useful for class methods, where our `Deref` hack can't emulate inheritance.
```rust
#[objrs(impl)]
impl MyCustomObject {
  #[objrs(selector = "hash", no_impl)]
  pub fn hash(&self) -> usize {}
}
```

Both `init1` and `init2` do the same thing. objrs will inspect the method, and if it takes a `self` parameter, it will make the method an instance method. If the method lacks a `self` parameter, it will be a class method. You can use [arbitrary self types](https://github.com/rust-lang/rust/issues/44874). If you don't want to use arbitrary self types, but find yourself in a position where you can't use `self`, you can use `class` or `instance` to explicitly tell objrs whether the method is a class or instance method.
```rust
#[objrs(impl)]
impl MyCustomObject {
  // Without arbitrary_self_types, we need to tell objrs the init method is an instance method.
  #[objrs(selector = "init", no_impl, instance)]
  pub fn init1(this: objrs::Alloc<MyCustomObject>) -> usize {}

  // With arbitrary_self_types, objrs will detect this as an instance method since it takes self.
  #[objrs(selector = "init", no_impl)]
  pub fn init2(self: objrs::Alloc<MyCustomObject>) -> usize {}
}
```

> **Syntax**
>
> *Attribute* :<br>
> &nbsp;&nbsp; `#` `[` `objrs` `(` `selector` `=` LITERAL_STR *Impl* *Type* *Optional* `)` `]`
>
> *Impl* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `super`<br>
> &nbsp;&nbsp; | `,` `no_impl`<br>
>
> *Type* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `class`<br>
> &nbsp;&nbsp; | `,` `instance`<br>
>
> *Optional* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `optional`

## `#[objrs(ivar)]`

It's unlikely you'll need to use `#[objrs(ivar)]`. It can usually be omitted when declaring instance variables in a class. But if you need it, it's there, and can be applied to individual instance variables.

Parameters:
- `name = "LITERAL_STR"`. Optional. This will be the name of the instance variable that the Objective-C runtime sees.
- `default = EXPR`. Optional. This is the initial value of the instance variable when the class is first allocated. If this is omitted and the type implements `objrs::marker::Zeroed`, the initial value of the instance variable will be zero. Otherwise, the initial value of the instance variable will be `Default::default()`.

One major limitation of instance variables (regardless of whether `#[objrs(ivar)]` is used or not) is that they can only be accessed via `self.`. Attempting to access an instance variable from a non-self identifier will not work (e.g. `self.ivar` will work, `some_other_instance_with_the_same_type_as_self.ivar` will not work). This is because objrs has to rewrite your method such that instance variables are properly dereferenced. objrs knows that `self` is a class (and it has all the necessary information); objrs does not know if some arbitrary identifer is a class (with its necessary metadata).

> **Syntax**
>
> *Attribute* :<br>
> &nbsp;&nbsp; `#` `[` `objrs` `(` `ivar` *Name* *Default* `)` `]`
>
> *Name* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `name` `=` LITERAL_STR<br>
>
> *Default* :<br>
> &nbsp;&nbsp; &nbsp; EMPTY<br>
> &nbsp;&nbsp; | `,` `default` `=` EXPR

## Literals

Literal `NSString`s and `SEL`s and be created with the `objrs_frameworks_foundation::nsstring!` and `objrs::selector!` macros (respectively). Example: 

```rust
// nsstring! is provided by the objrs_frameworks_foundation crate. It creates &'static objrs_frameworks_foundation::NSString objects.
static HELLO_WORLD_EN: &'static NSString = nsstring!("Hello, world!");
static HELLO_WORLD_JP: &'static NSString = nsstring!("こんにちは世界！");

// selector! is provided by the objrs crate. It creates objrs::runtime::SEL objects.
let sel = selector!("doFoo:withBar:");
```

Macros for other types (including CoreFoundation types) are planned but not yet implemented.

## Exceptions

`objrs::runtime::throw` and `objrs::runtime::catch_exception` allow you to throw and catch an Objective-C exception. It's equivalent to `@try { ... } @catch (id exception) { ... }` in Objective-C. `catch_exception` is designed to mirror [`std::panic::catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html): it returns a `Result` that contains the `Ok` return value of the closure (if no exception occurred) or `Err` with the value of the exception.

```rust
let result = catch_exception(|| {
  throw(some_exception_object);
});
```

Throwing an exception across FFI boundaries is technically undefined behavior, but it works okay.

## Autorelease pools

You can use `objrs::autoreleasepool` to create an autorelease pool, just like Objective-C's `@autoreleasepool { ... }`.

```rust
autoreleasepool(|| {
  // Do your work here in the closure.
});
```

These can be nested. Note that `autoreleasepool` takes an `Fn` (not `FnOnce`), so you can't mutate any captured state. This is done to prevent dangling references to autoreleased values. This restriction may be loosened at some point.

## Marker traits

### `objrs::marker::Class`

objrs will automatically implement the `objrs::marker::Class` trait for any type that is an Objective-C class. Do not manually implement this trait for a type.

### `objrs::marker::RootClass`

objrs will automatically implement the `objrs::marker::RootClass` trait for any type that is an Objective-C root class. Do not manually implement this trait for a type. In practice, `NSObject` is the only root class you will encounter.

### `objrs::marker::NonRootClass`

objrs will automatically implement the `objrs::marker::NonRootClass` trait for any type that is an Objective-C class that inherits from some super class. Do not manually implement this trait for a type. In practice, everything but `NSObject` is a non-root class.

### `objrs::marker::Zeroed`

This trait signals to objrs that the type can be safely initialized with zeroed-out memory. That is to say, if `T` implements `Zeroed`, then you are guaranteeing that `core::mem::zeroed::<T>()` is a valid, safe value.

You should manually implement this trait for all types that you reasonably can. Object creation is more efficient if its instance variables implement `Zeroed`.

objrs implements this trait for some basic types (e.g. `f32`, `usize`, `bool`, `Option<&T>`, etc.).

This trait is unsafe given the potential for undefined behavior if implemented incorrectly.

### `objrs::marker::Forgettable`

This trait signals to objrs that the type can be safely forgotten (e.g. with `core::mem::forget()`) without adverse consequences (e.g. leaking memory).

You should manually implement this trait for all types that you reasonably can. Object destruction is more efficient if its instance variables implement `Forgettable`.

objrs implements this trait for some basic types (e.g. `f32`, `usize`, `bool`, etc.).

This trait is safe given that Rust considers leaking memory and resources safe behavior.

# Limitations

There are some things that objrs does not support (and has no intentions of supporting) in an effort to reduce the scope and complexity of objrs. These include:

- Non-Apple operating systems.
- i386 (32-bit x86). Note that 64-bit x86_64 is supported.
- GNU toolchains (GCC, GNU libobjc). Only the native Apple toolchains (i.e. libobjc, LLVM, ld64) are supported.
- Swift (it's ABI hasn't stablized yet).

Additionally, Rust and Objective-C are two very different languages. objrs tries to blur the boundaries between the two. For some things, this is easy. For others, it's like forcing a square peg through a round hole: something has to break. Feedback is (very much) wanted as objrs experimentally navigates how to best expose Objective-C features, APIs, and frameworks to Rust code. Some of these difficulties (which limit objrs in one way or another) include:

- Rust's lack of support for (traditional) object oriented programming. Objective-C code tends to make heavy use of inheritance, which Rust completely lacks. Possible workarounds include using [traits](https://doc.rust-lang.org/book/first-edition/traits.html#inheritance) or the [`Deref` hack](https://github.com/rust-unofficial/patterns/blob/master/anti_patterns/deref.md). These aren't complete workarounds, though, as they don't reproduce the full complexity of inheritance.
- When Rust inlines a function, it inlines the code but not the function's local static variables. There are magic symbol names that Objective-C and objrs use that allow LLVM and ld64 to compile and link your program into an efficient executable. These symbol names are local: they can't be shared across translation units. The implication of this is that any Rust code that directly calls Objetive-C code (via objrs's macros, which will create some local static variables within the function) must be contained in a `#[inline(never)]` function (though this may be done several layers up at the module's public API level). If you get errors that look like `LLVM ERROR: unsupported relocation of undefined symbol 'L_...`, you need a  `#[inline(never)]` somewhere. You might think using `const` could work around this issue (since they're always inlined), but you can't specify linker attributes (which objrs needs) on a `const` in Rust.
- Rust's incremental compilation works by splitting an individual `*.rs` file into multiple `*.o` translation units (at e.g. the function level). This results in the same kind of issue as the aformentioned limitation of local static symbols crossing translation unit boundaries. Some care has been taking in objrs to explicitly `#[inline(never)]` certain portions of generated code (e.g. generics) to help mitigate this.
- Rust doesn't have proper weak symbol support, and even if it did, Objective-C's protocols are... difficult. In Objective-C, protocols are header-only (the framework's dylib doesn't provide a definition for them) and are instantiated per-object-file (like C++ templates) with weak symbols (to deduplicate).

# License

See the [COPYRIGHT](COPYRIGHT) file. objrs is triple-licensed under the Apache License 2.0, MIT License, and Mozilla Public License 2.0 terms.
