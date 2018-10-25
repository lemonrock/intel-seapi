# intel-seapi

[intel-seapi] is a FFI wrapper for the Intel Single Event API (SEAPI, also known as [IntelSEAPI]) and `ittnotify`, as used by Intel VTune. ***This particular wrapper uses a fork of [IntelSEAPI] because the upstream code base doesn't support cross-compilation or the MUSL C library.***

It provides a static link and generates Rust FFI bindings to the `libittnotify.a/.obj` library, and also compiles , but does not link or generate Rust bindings for, the`libIntelSEAPI.dylib/.so/.dll` dynamic library (this is because it is only ever built dynamically and because it is designed to be used from C++).

Downstream crates can use the generated build variables `cargo:include`, `cargo:libdir` and `cargo:root`.


## Limitations

* JIT profiling is not yet supported;
* The dynamic library `IntelSEAPI` is built but not linked and no bindings are generated for it.
* May not be actively supported, as I only needed this functionality once for a project I was working on.


## Cross-Compilation

This is brittle, and may fail, mostly because the underlying software uses a mixture of CMake, a C++ toolchain with Rust bindings and wrappers, and [IntelSEAPI] is not cross-compile friendly. We use a forked version to fix some cross-compilation mistakes in their `CMakeLists.txt` files.

Firstly, when compiling for a Mac OS X target, it is assumed the binary `lipo` is in the `PATH` as the native Mac OS X CMake build produces 'fat' archives which `rustc` can't handle. It tries to strip out the architecture of the target.

Secondly, when compiling for MUSL targets, Rust's `cc` crate assumes the C compiler is `musl-gcc` and the C++ compiler if `musl-g++`. Since the `cc` crate is managed by the `cmake` crate, this is something we can't easy change. This crate attempts to set the environment variable `CROSS_COMPILE` if not set already when cross-compiling using a MUSL target.


### Known cross-compilations that work.

* On Mac OS X, with filo-sottie's foked musl homebrew keg installed (`lemonrock/musl-cross/musl-cross`), `cargo build --target x86_64-unknown-linux-musl`.


## Licensing

The license for this project is MIT.

[intel-seapi]: https://github.com/lemonrock/intel-seapi "intel-seapi GitHub page"
[IntelSEAPI]: https:://gtihub.com/intel/IntelSEAPI "IntelSEAPI GitHub page"
