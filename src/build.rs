// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


extern crate bindgen;
extern crate cmake;


use ::bindgen::*;
use ::cmake::Config;
use ::std::env::set_var;
use ::std::env::var;
use ::std::fs::create_dir;
use ::std::fs::remove_dir;
use ::std::path::Path;
use ::std::path::PathBuf;


const PROJECT_TREE: &'static str = "lib/IntelSEAPI";


fn main()
{	
	let out_dir = getenv_unwrap("OUT_DIR");
	
	let install_folder_path = install_folder_path(&out_dir);
	
	create_missing_folders_that_cmakelists_requires();
	
	make_sure_cross_compilation_works_for_musl();
	
	cmake(&install_folder_path);
	
	let mislocated_bin_folder_path = mislocated_bin_folder_path(&out_dir);
	
	let include_folder_path = include_folder_path(&install_folder_path);
	
	print_cargo_path_value("root", &install_folder_path);
	
	print_cargo_path_value("include", &include_folder_path);
	
	print_cargo_path_value("libdir", &mislocated_bin_folder_path);
	
	print_cargo_path_value("rustc-link-search", &mislocated_bin_folder_path);
		
	print_cargo_rustc_link_lib(&mislocated_bin_folder_path);
	
	bindgen(&include_folder_path, &out_dir)
}

fn install_folder_path(out_dir: &str) -> PathBuf
{
	let mut install_folder_path = PathBuf::from(out_dir);
	install_folder_path.push("install");

	#[allow(unused_must_use)]
	{
		remove_dir(&install_folder_path);
		create_dir(&install_folder_path);
	}
	
	install_folder_path
}

fn create_missing_folders_that_cmakelists_requires()
{
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
}

fn make_sure_cross_compilation_works_for_musl()
{
	let host = getenv_unwrap("HOST");
	let target = getenv_unwrap("TARGET");
	
	if host != target
	{
		let is_musl_which_cmake_and_cc_crates_make_a_mess_of_cross_compile_naming = match target.as_str()
		{
			"i586-unknown-linux-musl" | "i686-unknown-linux-musl" | "x86_64-unknown-linux-musl" => true,
			_ => false,
		};
		
		if is_musl_which_cmake_and_cc_crates_make_a_mess_of_cross_compile_naming && var("CROSS_COMPILE").is_err()
		{
			let mut cross_compile = target.replace("-unknown-linux", "-linux");
			cross_compile.push('-');
			set_var("CROSS_COMPILE", cross_compile)
		}
	}
}

fn cmake(install_folder_path: &Path)
{
	let mut config = Config::new(PROJECT_TREE);
	
	config.define("CMAKE_INSTALL_PREFIX", &install_folder_path);
	config.very_verbose(true);

	let host = getenv_unwrap("HOST");
	let target = getenv_unwrap("TARGET");
	let is_cross_compiling = host != target;
	if is_cross_compiling
	{
		config.define("CMAKE_CROSSCOMPILING:BOOL", "ON");

		let mut target_pieces = target.split('-');
		target_pieces.next().unwrap();
		
		let cmake_system_name = match (target_pieces.next().unwrap(), target_pieces.next())
		{
			("apple", Some("darwin")) | ("apple", Some("ios")) => "Darwin",
			("linux", Some("android")) | ("linux", Some("androideabi")) => "Android",
			("none", Some(_)) | ("unknown", Some("none")) | ("unknown", Some("unknown")) => "Generic",
			("rumprun", Some("netbsd")) | ("unknown", Some("netbsd")) => "NetBSD",
			("unknown", Some("freebsd")) => "FreeBSD",
			("unknown", Some("linux")) => "Linux",
			("pc", Some("windows")) => "Windows",
			("sun", Some("solaris")) => "SunOS",
			
			("unknown", Some("cloudabi")) => "Generic",
			("unknown", Some("redox")) => "Generic",
			("unknown", Some("emscripten")) => "Generic",
			("unknown", Some("hermit")) => "Generic",
			("fuchsia", None) => "Generic",
			
			_ => "Generic",
		};
		
		config.define("CMAKE_SYSTEM_NAME", cmake_system_name);
		

		// There is a bug in the IntelSEAPI which means cross-compiling doesn't work.
		
		
		
		// if(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
	}
	
	// CMAKE_OSX_ARCHITECTURES needs to be changed to avoid lipo.
	
	// Something is setting 'APPLE'.
	
	config.build();
}

fn mislocated_bin_folder_path(out_dir: &str) -> PathBuf
{
	// This bin folder is created by CMakeLists.txt in the wrong location, ignoring CMAKE_INSTALL_PREFIX!
	let mut mislocated_bin_folder_path = PathBuf::from(out_dir);
	mislocated_bin_folder_path.pop();
	mislocated_bin_folder_path.push("bin");
	mislocated_bin_folder_path
}

fn include_folder_path(install_folder_path: &Path) -> PathBuf
{
	let mut include_folder_path = PathBuf::from(install_folder_path);
	include_folder_path.push("include");
	include_folder_path
}
	
fn print_cargo_path_value(name: &str, path_value: &Path)
{
	println!("cargo:{}={}", name, path_value.to_str().unwrap());
}

fn print_cargo_rustc_link_lib(mislocated_bin_folder_path: &Path)
{
	let target = getenv_unwrap("TARGET");
	let is_mac_os_x = target.ends_with("-darwin") || target.ends_with("-ios");
	
	// Rustc (1.31.0-nightly 2018-10-16) on Mac OS X fails to statically link the libittnotify.a library with an inaccurate error message of "error: failed to add native library ... File too small to be an archive".
	// This is because it does not correctly recognise fat (multi-architecture) object archives.
	if is_mac_os_x
	{
		use ::std::process::Command;
	
		let mut original_archive_file_path = PathBuf::from(mislocated_bin_folder_path);
		original_archive_file_path.push("libittnotify.a");
	
		let mut thin_archive_file_path = PathBuf::from(mislocated_bin_folder_path);
		thin_archive_file_path.push("libittnotify.a");
		
		let target_architecture = match target.split('-').next().unwrap()
		{
			"aarch64" => "arm64",
			ok @ _ => ok,
		};
		
		Command::new("lipo").current_dir(mislocated_bin_folder_path.to_str().unwrap()).arg("libittnotify.a").arg("-thin").arg(target_architecture).arg("-output").arg("libittnotify-thin.a").status().expect("failed to execute `lipo`");
	
		println!("cargo:rustc-link-lib=static=ittnotify-thin");
	}
	else
	{
		println!("cargo:rustc-link-lib=static=ittnotify");
	}
}

fn bindgen(include_folder_path: &Path, out_dir: &str)
{
	let mut header_filer_path = PathBuf::from(include_folder_path);
	header_filer_path.push("ittnotify.h");

	let mut rust_fmt_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	rust_fmt_file_path.push("rustfmt.toml");

	let mut bindings_file_path = PathBuf::from(out_dir);
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

fn getenv_unwrap(environment_variable_name: &str) -> String
{
    match var(environment_variable_name)
	{
        Ok(value) => value,
		
        Err(..) => panic!(format!("environment variable `{}` not defined", environment_variable_name)),
    }
}