// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![no_std]


//! #intel-seapi-sys
//!
//!
//! ## Overview.
//!
//! The ITT (Instrumentation and Tracing Technology) API is used to annotate a user's program with additional information that can be used by correctness and performance tools.
//! The user inserts calls in their program.
//! Those calls generate information that is collected at runtime, and used by Intel® Threading Tools.
//!
//!
//! ## API Concepts
//!
//! The following general concepts are used throughout the API.
//!
//!
//! ### Unicode Support
//!
//! Many API functions take character string arguments.
//! On Windows, there are two versions of each such function.
//! The function name is suffixed by W if Unicode support is enabled, and by A otherwise.
//! Any API function that takes a character string argument adheres to this convention.
//!
//!
//! ### Domains
//!
//! Domains provide a way to separate notification for different modules or libraries in a program.
//! Domains are specified by dotted character strings, eg `TBB.Internal.Control`.
//!
//! A mechanism (to be specified) is provided to enable and disable domains.
//! By default, all domains are enabled.
//!
//! Create a domain using some domain name: the URI naming style is recommended.
//! Because the set of domains is expected to be static over the application's execution time, there is no mechanism to destroy a domain.
//! Any domain can be accessed by any thread in the process, regardless of which thread created the domain. Creating a domain is thread-safe.
//!
//!
//! ### Named Entities and Instances
//!
//! Named entities (frames, regions, tasks, and markers) communicate information about the program to the analysis tools.
//! A named entity often refers to a section of program code, or to some set of logical concepts that the programmer wants to group together.
//! 
//! Named entities relate to the programmer's static view of the program.
//! When the program actually executes, many instances of a given named entity may be created.
//! 
//! The API annotations denote instances of named entities.
//! The actual named entities are displayed using the analysis tools.
//! In other words, the named entities come into existence when instances are created.
//! 
//! Instances of named entities may have instance identifiers (IDs).
//! Some API calls use instance identifiers to create relationships between
//! different instances of named entities.
//! Other API calls associate data with instances of named entities.
//! 
//! Some named entities must always have instance IDs.
//! In particular, regions and frames always have IDs.
//! Task and markers need IDs only if the ID is needed in another API call (such as adding a relation or metadata).
//! 
//! The lifetime of instance IDs is distinct from the lifetime of instances.
//! This allows various relationships to be specified separatefrom the actual execution of instances.
//! This flexibility comes at the expense of extra API calls.
//! 
//! The same ID may not be reused for different instances, unless a previous `__itt_id_destroy()` call for that ID has been issued.
//!
//!
//! ## Collection Control
//!
//! General behavior: application continues to run, but no profiling information is being collected
//!
//! Pausing occurs not only for the current thread but for all process as well as spawned processes.
//! It also possibly reduces runtime overhead.
//! Pausing also varies by collector application:-
//!
//!
//! ### Intel® Parallel Inspector and Intel® Inspector XE
//!
//! Does not analyze or report errors that involve memory access.
//!
//! Other errors are reported as usual.
//! Pausing data collection in Intel® Parallel Inspector and Intel® Inspector XE only pauses tracing and analyzing memory access.
//! It does not pause tracing or analyzing threading APIs.
//! 
//!
//! ### Intel® Parallel Amplifier and Intel® VTune™ Amplifier XE
//!
//! Does continue to record when new threads are started.
//!
//!
//! ## Modeling by Intel® Parallel Advisor
//! 
//! This is the subset of itt used for modeling by Intel® Parallel Advisor.
//! This API is called ONLY using `annotate.h`, by `Annotation" macros the user places in their sources during the parallelism modeling steps.
//! 
//! `site_begin` / `site_end` and `task_begin` / `task_end` take the address of handle variables, which are writeable by the API.
//! Handles must be 0 initialized prior to the first call to begin, or may cause a run-time failure.
//! The handles are initialized in a multi-thread safe way by the API if the handle is 0.
//! The commonly expected idiom is one static handle to identify a site or task.
//! If a site or task of the same name has already been started during this collection, the same handle MAY be returned, but is not required to be - it is unspecified if data merging is done based on name.
//! These routines also take an instance variable.
//! Like the lexical instance, these must be 0 initialized.
//! Unlike the lexical instance, this is used to track a single dynamic instance.
//! 
//! The API used by the Intel® Parallel Advisor allows one to describe potential concurrency and related activities. //! User-added source annotations expand to calls to these procedures to enable modeling of a hypothetical concurrent execution serially.
//!
//!
//! ###  `site_begin` / `site_end` model a potential concurrency site
//! 
//! site instances may be recursively nested with themselves.
//! `site_end` exits the most recently started but unended site for the current thread.
//! The handle passed to end may be used to validate structure
//! Instances of a site encountered on different threads concurrently are considered completely distinct.
//! If the site name for two different lexical sites match, it is unspecified whether they are treated as the same or different for data presentation.
//! 
//! 
//! ### `task_begin` / `task_end` model a potential task
//! 
//! The potential task is contained within the most closely enclosing dynamic site.
//! `task_end` exits the most recently started but unended task.
//! The handle passed to end may be used to validate structure.
//! It is unspecified if bad dynamic nesting is detected.
//! If it is, it should be encoded in the resulting data collection.
//! The collector should not fail due to construct nesting issues, nor attempt to directly indicate the problem.


extern crate libc;


use ::core::mem::zeroed;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_ulonglong;
use ::libc::c_void;
#[cfg(windows)] use ::libc::wchar_t;


#[cfg(test)] pub(crate) mod tests;


include!(concat!(env!("OUT_DIR"), "/jitprofiling.bindgen.rs"));


pub const __itt_suppress_all_errors: u32 = 2147483647;

pub const __itt_suppress_threading_errors: u32 = 255;

pub const __itt_suppress_memory_errors: u32 = 65280;

pub const __itt_attr_barrier: u32 = 1;

pub const __itt_attr_mutex: u32 = 2;

pub const __itt_heap_leaks: u32 = 1;

pub const __itt_heap_growth: u32 = 2;

#[link(name = "ittnotify", kind = "static")]
extern "C"
{
	#[link_name = "__itt_pause_init_3_0"]
	pub fn __itt_pause();

	#[link_name = "__itt_resume_init_3_0"]
	pub fn __itt_resume();

	#[link_name = "__itt_detach_init_3_0"]
	pub fn __itt_detach();

	#[cfg(unix)]
	#[link_name = "__itt_thread_set_name_init_3_0"]
	pub fn __itt_thread_set_name(name: *const c_char);

	#[cfg(windows)]
	#[link_name = "__itt_thread_set_nameA_init_3_0"]
	pub fn __itt_thread_set_nameA(name: *const c_char);

	#[cfg(windows)]
	#[link_name = "__itt_thread_set_nameW_init_3_0"]
	pub fn __itt_thread_set_nameW(name: *const wchar_t);

	/// Mark current thread as ignored from this point on, for the duration of its existence.
	#[link_name = "__itt_thread_ignore_init_3_0"]
	pub fn __itt_thread_ignore();

	/// Begin of region instance.
	///
	/// Successive calls to __itt_region_begin with the same ID are ignored until a call to __itt_region_end with the same ID.
	///
	/// * `domain`: The domain for this region instance.
	/// * `id`: The instance ID for this region instance. Must not be `__itt_null`.
	/// * `parentid`: The instance ID for the parent of this region instance, or `__itt_null`.
	/// * `name`: The name of this region.
	#[link_name = "__itt_region_begin_init_3_0"]
	pub fn __itt_region_begin(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);

	/// End of region instance
	///
	/// The first call to __itt_region_end with a given ID ends the region.
	///
	/// Successive calls with the same ID are ignored, as are calls that do not have a matching `__itt_region_begin` call.
	/// * `domain`: The domain for this region instance.
	/// * `id`: The instance ID for this region instance.
	#[link_name = "__itt_region_end_init_3_0"]
	pub fn __itt_region_end(domain: *const __itt_domain, id: __itt_id);

	/// Create a domain.
	#[cfg(unix)]
	#[link_name = "__itt_domain_create_init_3_0"]
	pub fn __itt_domain_create(name: *const c_char) -> *mut __itt_domain;

	/// Create a domain.
	#[cfg(windows)]
	#[link_name = "__itt_domain_createA_init_3_0"]
	pub fn __itt_domain_createA(name: *const c_char) -> *mut __itt_domain;

	/// Create a domain.
	#[cfg(windows)]
	#[link_name = "__itt_domain_createW_init_3_0"]
	pub fn __itt_domain_createW(name: *const wchar_t) -> *mut __itt_domain;

	/// Create a string handle.
	#[cfg(unix)]
	#[link_name = "__itt_string_handle_create_init_3_0"]
	pub fn __itt_string_handle_create(name: *const c_char) -> *mut __itt_string_handle;

	/// Create a string handle.
	#[cfg(windows)]
	#[link_name = "__itt_string_handle_createA_init_3_0"]
	pub fn __itt_string_handle_createA(name: *const c_char) -> *mut __itt_string_handle;

	/// Create a string handle.
	#[cfg(windows)]
	#[link_name = "__itt_string_handle_createW_init_3_0"]
	pub fn __itt_string_handle_createW(name: *const wchar_t) -> *mut __itt_string_handle;

	/// Return timestamp corresponding to the current moment.
	///
	/// This returns the timestamp in the format that is the most relevant for the current host or platform (RDTSC, QPC, and others).
	///
	/// You can use the "<" operator to compare `__itt_timestamp` values.
	#[link_name = "__itt_get_timestamp_init_3_0"]
	pub fn __itt_get_timestamp() -> __itt_timestamp;

	/// The `name` is not terminated by an ASCII NUL (`\0`) and `namelen` should be the exact length, ie it is equivalent to a Rust slice.
	///
	/// Characters should probably be ASCII.
	#[cfg(unix)]
	#[link_name = "__itt_event_create_init_3_0"]
	pub fn __itt_event_create(name: *const c_char, namelen: c_int) -> __itt_event;

	/// The `name` is not terminated by an ASCII NUL (`\0`) and `namelen` should be the exact length, ie it is equivalent to a Rust slice.
	///
	/// Characters should be ASCII.
	#[cfg(windows)]
	#[link_name = "__itt_event_createA_init_3_0"]
	pub fn __itt_event_createA(name: *const c_char, namelen: c_int) -> __itt_event;

	/// The `name` is not terminated by an ASCII NUL (`\0`) and `namelen` should be the exact length, ie it is equivalent to a Rust slice.
	#[cfg(windows)]
	#[link_name = "__itt_event_createW_init_3_0"]
	pub fn __itt_event_createW(name: *const wchar_t, namelen: c_int) -> __itt_event;

	/// Record an event occurrence.
	///
	/// Returns `__itt_err` upon failure (invalid event id/user event feature not enabled).
	#[link_name = "__itt_event_start_init_3_0"]
	pub fn __itt_event_start(event: __itt_event) -> c_int;

	/// Record an event end occurrence.
	///
	/// Calling this is optional if events do not have durations.
	///
	/// Returns `__itt_err` upon failure (invalid event id/user event feature not enabled).
	#[link_name = "__itt_event_end_init_3_0"]
	pub fn __itt_event_end(event: __itt_event) -> c_int;

	/// Create an instance of identifier.
	///
	/// This establishes the beginning of the lifetime of an instance of the given ID in the trace.
	/// Once this lifetime starts, the ID can be used to tag named entity instances in calls such as `__itt_task_begin()`, and to specify relationships among identified named entity instances, using the relations APIs.
	///
	/// Instance IDs are not domain specific!
	///
	/// * `domain`: The domain controlling the execution of this call.
	/// * `id`: The ID to create.
	#[link_name = "__itt_id_create_init_3_0"]
	pub fn __itt_id_create(domain: *const __itt_domain, id: __itt_id);

	/// Destroy an instance of identifier.
	///
	/// This ends the lifetime of the current instance of the given ID value in the trace.
	/// Any relationships that are established after this lifetime ends are invalid.
	/// This call must be performed before the given ID value can be reused for a different named entity instance.
	///
	/// * `domain`: The domain controlling the execution of this call.
	/// * `id`: The ID to destroy.
	#[link_name = "__itt_id_destroy_init_3_0"]
	pub fn __itt_id_destroy(domain: *const __itt_domain, id: __itt_id);

	/// Begin a frame instance.
	///
	/// Successive calls to `__itt_frame_begin_v3()` with the same ID are ignored until a call to `__itt_frame_end_v3()` with the same ID.
	///
	/// * `domain` The domain for this frame instance.
	/// * `id`: The instance ID for this frame instance or NULL.
	#[link_name = "__itt_frame_begin_v3_init_3_0"]
	pub fn __itt_frame_begin_v3(domain: *const __itt_domain, id: *mut __itt_id);

	/// End a frame instance.
	///
	/// The first call to `__itt_frame_end_v3()` with a given ID ends the frame.
	/// Successive calls with the same ID are ignored, as are calls that do not have a matching `__itt_frame_begin_v3()` call.
	///
	/// * `domain`: The domain for this frame instance.
	/// * `id`: The instance ID for this frame instance or NULL for current.
	#[link_name = "__itt_frame_end_v3_init_3_0"]
	pub fn __itt_frame_end_v3(domain: *const __itt_domain, id: *mut __itt_id);

	/// Submits a frame instance.
	///
	/// Successive calls to` __itt_frame_begin_v3()` or `__itt_frame_submit_v3()` with the/ same ID are ignored until a call to `__itt_frame_end_v3()` or `__itt_frame_submit_v3()` with the same ID.
	///
	/// Passing the special `__itt_timestamp_none` value as the `end` argument means take the current timestamp as the end timestamp.
	///
	/// * `domain`: The domain for this frame instance.
	/// * `id`: The instance ID for this frame instance or NULL.
	/// * `begin`: Timestamp of the beginning of the frame.
	/// * `end`: Timestamp of the end of the frame.
	#[link_name = "__itt_frame_submit_v3_init_3_0"]
	pub fn __itt_frame_submit_v3(domain: *const __itt_domain, id: *mut __itt_id, begin: __itt_timestamp, end: __itt_timestamp);

	/// Begin a task instance.
	///
	/// * `domain`: The domain for this task
	/// * `taskid`: The instance ID for this task instance, or `__itt_null`.
	/// * `parentid`: The parent instance to which this task instance belongs, or `__itt_null`.
	/// * `name`: The name of this task
	#[link_name = "__itt_task_begin_init_3_0"]
	pub fn __itt_task_begin(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);

	/// Begin a task instance.
	///
	/// * `domain`: The domain for this task.
	/// * `taskid`: The identifier for this task instance, or `__itt_null`.
	/// * `parentid`: The parent of this task, or `__itt_null`.
	/// * `fn`: The pointer to the function you are tracing.
	#[link_name = "__itt_task_begin_fn_init_3_0"]
	pub fn __itt_task_begin_fn(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, fn_: *mut ::libc::c_void);

	/// End the current task instance.
	///
	/// * `domain`: The domain for this task.
	#[link_name = "__itt_task_end_init_3_0"]
	pub fn __itt_task_end(domain: *const __itt_domain);
}

/// Internal event type.
pub type __itt_event = c_int;

/// Internal timestamp type.
pub type __itt_timestamp = c_ulonglong;

/// Internal domain handle type.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __itt_domain
{
	/// Zero if disabled, non-zero if enabled
	///
	/// The meaning of different non-zero values is reserved to the runtime.
	pub flags: c_int,

	/// Copy of original name in ASCII.
	nameA: *const c_char,

	/// Copy of original name in Windows wchar_t.
	nameW: *mut c_void,

	/// Reserved to the runtime
	extra1: c_int,

	/// Reserved to the runtime.
	extra2: *mut c_void,

	next: *mut __itt_domain,
}

impl Default for __itt_domain
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

/// Internal string handle type.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __itt_string_handle
{
	/// Copy of original string in ASCII.
	strA: *const c_char,

	/// Copy of original name in Windows wchar_t.
	strW: *mut c_void,
	
	/// Reserved. Must be zero
	extra1: c_int,
	
	/// Reserved. Must be zero
	extra2: *mut c_void,
	
	next: *mut __itt_string_handle,
}

impl Default for __itt_string_handle
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

/// Internal identifier type.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct __itt_id
{
	pub d1: c_ulonglong,
	pub d2: c_ulonglong,
	pub d3: c_ulonglong,
}
