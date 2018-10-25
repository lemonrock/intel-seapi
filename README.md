# intel-seapi

[intel-seapi] is a FFI wrapper for the Intel Single Event API (SEAPI, also known as [IntelSEAPI]) and `ittnotify`, as used by Intel VTune.

It provides a static link and generates Rust FFI bindings to the `libittnotify.a/.obj` library, and also compiles , but does not link or generate Rust bindings for, the`libIntelSEAPI.dylib/.so/.dll` dynamic library (this is because it is only ever built dynamically and because it is designed to be used from C++).

Currently this crate is incapable of working with cross-compilation. This is a limitation of the underling CMake build system ussed by `IntelSEAPI`.

Downstream crates can use the generated build variables `cargo:include`, `cargo:libdir` and `cargo:root`.


## Licensing

The license for this project is MIT.

[intel-seapi]: https://github.com/lemonrock/intel-seapi "intel-seapi GitHub page"
[IntelSEAPI]: https:://gtihub.com/intel/IntelSEAPI "IntelSEAPI GitHub page"
