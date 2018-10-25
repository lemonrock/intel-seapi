// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


extern crate bindgen;
extern crate cmake;


use ::bindgen::*;
use ::cmake::Config;
use ::std::env::var;
use ::std::fs::create_dir;
use ::std::fs::remove_dir;
use ::std::path::PathBuf;


fn main()
{
	const PROJECT_TREE: &'static str = "lib/IntelSEAPI";
	
	let out_dir = var("OUT_DIR").unwrap();
	
	// Install path.
	let mut install_folder_path = PathBuf::from(&out_dir);
	install_folder_path.push("install");
	
	#[allow(unused_must_use)]
	{
		remove_dir(&install_folder_path);
		create_dir(&install_folder_path);
	}
	
	// The CMakeLists.txt for IntelSEAPI don't create these folders but depend on them and include them in .gitignore.
	for missing_folder in ["bin", "obj"].iter()
	{
		let mut missing_folder_path = PathBuf::from(PROJECT_TREE);
		missing_folder_path.push(missing_folder);
		#[allow(unused_must_use)]
		{
			remove_dir(&missing_folder_path);
			create_dir(&missing_folder_path);
		}
	}
	
	Config::new(PROJECT_TREE)
		.define("CMAKE_INSTALL_PREFIX", &install_folder_path)
		.build();
	
	
	// This bin folder is created by CMakeLists.txt in the wrong location, ignoring CMAKE_INSTALL_PREFIX!
	let mut mislocated_bin_folder_path = PathBuf::from(&out_dir);
	mislocated_bin_folder_path.pop();
	mislocated_bin_folder_path.push("bin");
	
	
	println!("cargo:root={}", install_folder_path.to_str().unwrap());
	
	let mut include_folder_path = PathBuf::from(install_folder_path);
	include_folder_path.push("include");
	println!("cargo:include={}", include_folder_path.to_str().unwrap());
	
	println!("cargo:libdir={}", mislocated_bin_folder_path.to_str().unwrap());
	
	println!("cargo:rustc-link-search=native={}", mislocated_bin_folder_path.to_str().unwrap());
	
	// Rustc (1.31.0-nightly 2018-10-16) on Mac OS X fails to statically link the libittnotify.a library with an inaccurate error message of "error: failed to add native library ... File too small to be an archive".
	// This is because it does not correctly recognise fat (multi-architecture) object archives.
	// This fix is not cross-compile friendly.
	if cfg!(target_os = "macos")
	{
		use ::std::process::Command;
		
		let mut original_archive_file_path = mislocated_bin_folder_path.clone();
		original_archive_file_path.push("libittnotify.a");
		
		let mut thin_archive_file_path = mislocated_bin_folder_path.clone();
		thin_archive_file_path.push("libittnotify.a");
		
		Command::new("lipo").current_dir(mislocated_bin_folder_path.to_str().unwrap()).arg("libittnotify.a").arg("-thin").arg("x86_64").arg("-output").arg("libittnotify-thin.a").status().expect("failed to execute lipo");
		
		println!("cargo:rustc-link-lib=static=ittnotify-thin");
	}
	else
	{
		println!("cargo:rustc-link-lib=static=ittnotify");
	}
	
	let mut header_filer_path = include_folder_path.clone();
	header_filer_path.push("ittnotify.h");
	
	let mut rust_fmt_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	rust_fmt_file_path.push("rustfmt.toml");
	
	let mut bindings_file_path = PathBuf::from(&out_dir);
	bindings_file_path.push("ittnotify.bindgen.rs");
	
	Builder::default()
		.header(header_filer_path.to_str().unwrap())
		.ctypes_prefix("::libc")
		.default_enum_style(EnumVariation::Rust)
		.derive_copy(true)
		.derive_debug(true)
		.derive_default(true)
		.derive_eq(true)
		.derive_hash(true)
		.derive_ord(true)
		.derive_partialeq(true)
		.derive_partialord(true)
		.generate_comments(true)
		.impl_debug(true)
		.impl_partialeq(true)
		.layout_tests(false)
		.prepend_enum_name(false)
		.rustfmt_bindings(true)
		.rustfmt_configuration_file(Some(rust_fmt_file_path))
		.rust_target(RustTarget::Stable_1_26)
		.use_core()
		.whitelist_function("__itt.*")
		.whitelist_type("__itt.*")
		.whitelist_var("__itt.*")
		
		.generate()
		.expect("Unable to generate bindings for ittnotify.h")
		.write_to_file(bindings_file_path)
		.expect("Could not write bindgen bindings for ittnotify.h to ittnotify.bindgen.rs")
}
