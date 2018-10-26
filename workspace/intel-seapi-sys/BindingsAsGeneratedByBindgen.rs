// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


pub const __itt_suppress_all_errors: u32 = 2147483647;
pub const __itt_suppress_threading_errors: u32 = 255;
pub const __itt_suppress_memory_errors: u32 = 65280;
pub const __itt_attr_barrier: u32 = 1;
pub const __itt_attr_mutex: u32 = 2;
pub const __itt_heap_leaks: u32 = 1;
pub const __itt_heap_growth: u32 = 2;
extern "C" {
/// @defgroup control Collection Control
/// @ingroup public
/// General behavior: application continues to run, but no profiling information is being collected
///
/// Pausing occurs not only for the current thread but for all process as well as spawned processes
/// - Intel(R) Parallel Inspector and Intel(R) Inspector XE:
///   - Does not analyze or report errors that involve memory access.
///   - Other errors are reported as usual. Pausing data collection in
///     Intel(R) Parallel Inspector and Intel(R) Inspector XE
///     only pauses tracing and analyzing memory access.
///     It does not pause tracing or analyzing threading APIs.
///   .
/// - Intel(R) Parallel Amplifier and Intel(R) VTune(TM) Amplifier XE:
///   - Does continue to record when new threads are started.
///   .
/// - Other effects:
///   - Possible reduction of runtime overhead.
///   .
/// @{
////** @brief Pause collection
	#[link_name = "\u{1}___itt_pause"]
	pub fn __itt_pause();
}
extern "C" {
	/// @brief Resume collection
	#[link_name = "\u{1}___itt_resume"]
	pub fn __itt_resume();
}
extern "C" {
	/// @brief Detach collection
	#[link_name = "\u{1}___itt_detach"]
	pub fn __itt_detach();
}
pub type __itt_pause_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_pause_ptr__3_0"]
	pub static mut __itt_pause_ptr__3_0: __itt_pause_ptr__3_0_t;
}
pub type __itt_resume_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_resume_ptr__3_0"]
	pub static mut __itt_resume_ptr__3_0: __itt_resume_ptr__3_0_t;
}
pub type __itt_detach_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_detach_ptr__3_0"]
	pub static mut __itt_detach_ptr__3_0: __itt_detach_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_thread_set_name"]
	pub fn __itt_thread_set_name(name: *const ::libc::c_char);
}
pub type __itt_thread_set_name_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_thread_set_name_ptr__3_0"]
	pub static mut __itt_thread_set_name_ptr__3_0: __itt_thread_set_name_ptr__3_0_t;
}
extern "C" {
	/// @brief Mark current thread as ignored from this point on, for the duration of its existence.
	#[link_name = "\u{1}___itt_thread_ignore"]
	pub fn __itt_thread_ignore();
}
pub type __itt_thread_ignore_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_thread_ignore_ptr__3_0"]
	pub static mut __itt_thread_ignore_ptr__3_0: __itt_thread_ignore_ptr__3_0_t;
}
extern "C" {
	/// @brief Start suppressing errors identified in mask on this thread
	#[link_name = "\u{1}___itt_suppress_push"]
	pub fn __itt_suppress_push(mask: ::libc::c_uint);
}
pub type __itt_suppress_push_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(mask: ::libc::c_uint)>;
extern "C" {
	#[link_name = "\u{1}___itt_suppress_push_ptr__3_0"]
	pub static mut __itt_suppress_push_ptr__3_0: __itt_suppress_push_ptr__3_0_t;
}
extern "C" {
	/// @brief Undo the effects of the matching call to __itt_suppress_push
	#[link_name = "\u{1}___itt_suppress_pop"]
	pub fn __itt_suppress_pop();
}
pub type __itt_suppress_pop_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_suppress_pop_ptr__3_0"]
	pub static mut __itt_suppress_pop_ptr__3_0: __itt_suppress_pop_ptr__3_0_t;
}
#[repr(u32)]
/// @enum __itt_model_disable
/// @brief Enumerator for the disable methods
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_suppress_mode
{
	__itt_unsuppress_range = 0,
	__itt_suppress_range = 1,
}
pub use self::__itt_suppress_mode as __itt_suppress_mode_t;
extern "C" {
	/// @brief Mark a range of memory for error suppression or unsuppression for error types included in mask
	#[link_name = "\u{1}___itt_suppress_mark_range"]
	pub fn __itt_suppress_mark_range(mode: __itt_suppress_mode_t, mask: ::libc::c_uint, address: *mut ::libc::c_void, size: usize);
}
pub type __itt_suppress_mark_range_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(mode: __itt_suppress_mode_t, mask: ::libc::c_uint, address: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_suppress_mark_range_ptr__3_0"]
	pub static mut __itt_suppress_mark_range_ptr__3_0: __itt_suppress_mark_range_ptr__3_0_t;
}
extern "C" {
	/// @brief Undo the effect of a matching call to __itt_suppress_mark_range.   If not matching
	///        call is found, nothing is changed.
	#[link_name = "\u{1}___itt_suppress_clear_range"]
	pub fn __itt_suppress_clear_range(mode: __itt_suppress_mode_t, mask: ::libc::c_uint, address: *mut ::libc::c_void, size: usize);
}
pub type __itt_suppress_clear_range_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(mode: __itt_suppress_mode_t, mask: ::libc::c_uint, address: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_suppress_clear_range_ptr__3_0"]
	pub static mut __itt_suppress_clear_range_ptr__3_0: __itt_suppress_clear_range_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_sync_create"]
	pub fn __itt_sync_create(addr: *mut ::libc::c_void, objtype: *const ::libc::c_char, objname: *const ::libc::c_char, attribute: ::libc::c_int);
}
pub type __itt_sync_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, objtype: *const ::libc::c_char, objname: *const ::libc::c_char, attribute: ::libc::c_int)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_create_ptr__3_0"]
	pub static mut __itt_sync_create_ptr__3_0: __itt_sync_create_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_sync_rename"]
	pub fn __itt_sync_rename(addr: *mut ::libc::c_void, name: *const ::libc::c_char);
}
pub type __itt_sync_rename_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_rename_ptr__3_0"]
	pub static mut __itt_sync_rename_ptr__3_0: __itt_sync_rename_ptr__3_0_t;
}
extern "C" {
	/// @brief Destroy a synchronization object.
	/// @param addr Handle for the synchronization object.
	#[link_name = "\u{1}___itt_sync_destroy"]
	pub fn __itt_sync_destroy(addr: *mut ::libc::c_void);
}
pub type __itt_sync_destroy_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_destroy_ptr__3_0"]
	pub static mut __itt_sync_destroy_ptr__3_0: __itt_sync_destroy_ptr__3_0_t;
}
extern "C" {
	/////**
	/// @name group of functions is used for performance measurement tools
	////** @{ */
	////**
	/// @brief Enter spin loop on user-defined sync object
	#[link_name = "\u{1}___itt_sync_prepare"]
	pub fn __itt_sync_prepare(addr: *mut ::libc::c_void);
}
pub type __itt_sync_prepare_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_prepare_ptr__3_0"]
	pub static mut __itt_sync_prepare_ptr__3_0: __itt_sync_prepare_ptr__3_0_t;
}
extern "C" {
	/// @brief Quit spin loop without acquiring spin object
	#[link_name = "\u{1}___itt_sync_cancel"]
	pub fn __itt_sync_cancel(addr: *mut ::libc::c_void);
}
pub type __itt_sync_cancel_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_cancel_ptr__3_0"]
	pub static mut __itt_sync_cancel_ptr__3_0: __itt_sync_cancel_ptr__3_0_t;
}
extern "C" {
	/// @brief Successful spin loop completion (sync object acquired)
	#[link_name = "\u{1}___itt_sync_acquired"]
	pub fn __itt_sync_acquired(addr: *mut ::libc::c_void);
}
pub type __itt_sync_acquired_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_acquired_ptr__3_0"]
	pub static mut __itt_sync_acquired_ptr__3_0: __itt_sync_acquired_ptr__3_0_t;
}
extern "C" {
	/// @brief Start sync object releasing code. Is called before the lock release call.
	#[link_name = "\u{1}___itt_sync_releasing"]
	pub fn __itt_sync_releasing(addr: *mut ::libc::c_void);
}
pub type __itt_sync_releasing_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_sync_releasing_ptr__3_0"]
	pub static mut __itt_sync_releasing_ptr__3_0: __itt_sync_releasing_ptr__3_0_t;
}
extern "C" {
	/////**
	/// @name group of functions is used for correctness checking tools
	////** @{ */
	////**
	/// @ingroup legacy
	/// @deprecated Legacy API
	/// @brief Fast synchronization which does no require spinning.
	/// - This special function is to be used by TBB and OpenMP libraries only when they know
	///   there is no spin but they need to suppress TC warnings about shared variable modifications.
	/// - It only has corresponding pointers in static library and does not have corresponding function
	///   in dynamic library.
	/// @see void __itt_sync_prepare(void* addr);
	#[link_name = "\u{1}___itt_fsync_prepare"]
	pub fn __itt_fsync_prepare(addr: *mut ::libc::c_void);
}
pub type __itt_fsync_prepare_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_fsync_prepare_ptr__3_0"]
	pub static mut __itt_fsync_prepare_ptr__3_0: __itt_fsync_prepare_ptr__3_0_t;
}
extern "C" {
	/// @ingroup legacy
	/// @deprecated Legacy API
	/// @brief Fast synchronization which does no require spinning.
	/// - This special function is to be used by TBB and OpenMP libraries only when they know
	///   there is no spin but they need to suppress TC warnings about shared variable modifications.
	/// - It only has corresponding pointers in static library and does not have corresponding function
	///   in dynamic library.
	/// @see void __itt_sync_cancel(void *addr);
	#[link_name = "\u{1}___itt_fsync_cancel"]
	pub fn __itt_fsync_cancel(addr: *mut ::libc::c_void);
}
pub type __itt_fsync_cancel_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_fsync_cancel_ptr__3_0"]
	pub static mut __itt_fsync_cancel_ptr__3_0: __itt_fsync_cancel_ptr__3_0_t;
}
extern "C" {
	/// @ingroup legacy
	/// @deprecated Legacy API
	/// @brief Fast synchronization which does no require spinning.
	/// - This special function is to be used by TBB and OpenMP libraries only when they know
	///   there is no spin but they need to suppress TC warnings about shared variable modifications.
	/// - It only has corresponding pointers in static library and does not have corresponding function
	///   in dynamic library.
	/// @see void __itt_sync_acquired(void *addr);
	#[link_name = "\u{1}___itt_fsync_acquired"]
	pub fn __itt_fsync_acquired(addr: *mut ::libc::c_void);
}
pub type __itt_fsync_acquired_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_fsync_acquired_ptr__3_0"]
	pub static mut __itt_fsync_acquired_ptr__3_0: __itt_fsync_acquired_ptr__3_0_t;
}
extern "C" {
	/// @ingroup legacy
	/// @deprecated Legacy API
	/// @brief Fast synchronization which does no require spinning.
	/// - This special function is to be used by TBB and OpenMP libraries only when they know
	///   there is no spin but they need to suppress TC warnings about shared variable modifications.
	/// - It only has corresponding pointers in static library and does not have corresponding function
	///   in dynamic library.
	/// @see void __itt_sync_releasing(void* addr);
	#[link_name = "\u{1}___itt_fsync_releasing"]
	pub fn __itt_fsync_releasing(addr: *mut ::libc::c_void);
}
pub type __itt_fsync_releasing_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_fsync_releasing_ptr__3_0"]
	pub static mut __itt_fsync_releasing_ptr__3_0: __itt_fsync_releasing_ptr__3_0_t;
}
pub type __itt_model_site = *mut ::libc::c_void;
pub type __itt_model_site_instance = *mut ::libc::c_void;
pub type __itt_model_task = *mut ::libc::c_void;
pub type __itt_model_task_instance = *mut ::libc::c_void;
#[repr(u32)]
/// @enum __itt_model_disable
/// @brief Enumerator for the disable methods
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_model_disable
{
	__itt_model_disable_observation = 0,
	__itt_model_disable_collection = 1,
}
extern "C" {
	/// @brief ANNOTATE_SITE_BEGIN/ANNOTATE_SITE_END support.
	///
	/// site_begin/end model a potential concurrency site.
	/// site instances may be recursively nested with themselves.
	/// site_end exits the most recently started but unended site for the current
	/// thread.  The handle passed to end may be used to validate structure.
	/// Instances of a site encountered on different threads concurrently
	/// are considered completely distinct. If the site name for two different
	/// lexical sites match, it is unspecified whether they are treated as the
	/// same or different for data presentation.
	#[link_name = "\u{1}___itt_model_site_begin"]
	pub fn __itt_model_site_begin(site: *mut __itt_model_site, instance: *mut __itt_model_site_instance, name: *const ::libc::c_char);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_site_beginA"]
	pub fn __itt_model_site_beginA(name: *const ::libc::c_char);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_site_beginAL"]
	pub fn __itt_model_site_beginAL(name: *const ::libc::c_char, siteNameLen: usize);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_site_end"]
	pub fn __itt_model_site_end(site: *mut __itt_model_site, instance: *mut __itt_model_site_instance);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_site_end_2"]
	pub fn __itt_model_site_end_2();
}
pub type __itt_model_site_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(site: *mut __itt_model_site, instance: *mut __itt_model_site_instance, name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_site_begin_ptr__3_0"]
	pub static mut __itt_model_site_begin_ptr__3_0: __itt_model_site_begin_ptr__3_0_t;
}
pub type __itt_model_site_beginA_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_site_beginA_ptr__3_0"]
	pub static mut __itt_model_site_beginA_ptr__3_0: __itt_model_site_beginA_ptr__3_0_t;
}
pub type __itt_model_site_beginAL_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, siteNameLen: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_site_beginAL_ptr__3_0"]
	pub static mut __itt_model_site_beginAL_ptr__3_0: __itt_model_site_beginAL_ptr__3_0_t;
}
pub type __itt_model_site_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(site: *mut __itt_model_site, instance: *mut __itt_model_site_instance)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_site_end_ptr__3_0"]
	pub static mut __itt_model_site_end_ptr__3_0: __itt_model_site_end_ptr__3_0_t;
}
pub type __itt_model_site_end_2_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_model_site_end_2_ptr__3_0"]
	pub static mut __itt_model_site_end_2_ptr__3_0: __itt_model_site_end_2_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_TASK_BEGIN/ANNOTATE_TASK_END support
	///
	/// task_begin/end model a potential task, which is contained within the most
	/// closely enclosing dynamic site.  task_end exits the most recently started
	/// but unended task.  The handle passed to end may be used to validate
	/// structure.  It is unspecified if bad dynamic nesting is detected.  If it
	/// is, it should be encoded in the resulting data collection.  The collector
	/// should not fail due to construct nesting issues, nor attempt to directly
	/// indicate the problem.
	#[link_name = "\u{1}___itt_model_task_begin"]
	pub fn __itt_model_task_begin(task: *mut __itt_model_task, instance: *mut __itt_model_task_instance, name: *const ::libc::c_char);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_task_beginA"]
	pub fn __itt_model_task_beginA(name: *const ::libc::c_char);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_task_beginAL"]
	pub fn __itt_model_task_beginAL(name: *const ::libc::c_char, taskNameLen: usize);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_iteration_taskA"]
	pub fn __itt_model_iteration_taskA(name: *const ::libc::c_char);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_iteration_taskAL"]
	pub fn __itt_model_iteration_taskAL(name: *const ::libc::c_char, taskNameLen: usize);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_task_end"]
	pub fn __itt_model_task_end(task: *mut __itt_model_task, instance: *mut __itt_model_task_instance);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_task_end_2"]
	pub fn __itt_model_task_end_2();
}
pub type __itt_model_task_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(task: *mut __itt_model_task, instance: *mut __itt_model_task_instance, name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_task_begin_ptr__3_0"]
	pub static mut __itt_model_task_begin_ptr__3_0: __itt_model_task_begin_ptr__3_0_t;
}
pub type __itt_model_task_beginA_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_task_beginA_ptr__3_0"]
	pub static mut __itt_model_task_beginA_ptr__3_0: __itt_model_task_beginA_ptr__3_0_t;
}
pub type __itt_model_task_beginAL_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, taskNameLen: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_task_beginAL_ptr__3_0"]
	pub static mut __itt_model_task_beginAL_ptr__3_0: __itt_model_task_beginAL_ptr__3_0_t;
}
pub type __itt_model_iteration_taskA_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_iteration_taskA_ptr__3_0"]
	pub static mut __itt_model_iteration_taskA_ptr__3_0: __itt_model_iteration_taskA_ptr__3_0_t;
}
pub type __itt_model_iteration_taskAL_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, taskNameLen: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_iteration_taskAL_ptr__3_0"]
	pub static mut __itt_model_iteration_taskAL_ptr__3_0: __itt_model_iteration_taskAL_ptr__3_0_t;
}
pub type __itt_model_task_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(task: *mut __itt_model_task, instance: *mut __itt_model_task_instance)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_task_end_ptr__3_0"]
	pub static mut __itt_model_task_end_ptr__3_0: __itt_model_task_end_ptr__3_0_t;
}
pub type __itt_model_task_end_2_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_model_task_end_2_ptr__3_0"]
	pub static mut __itt_model_task_end_2_ptr__3_0: __itt_model_task_end_2_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_LOCK_ACQUIRE/ANNOTATE_LOCK_RELEASE support
	///
	/// lock_acquire/release model a potential lock for both lockset and
	/// performance modeling.  Each unique address is modeled as a separate
	/// lock, with invalid addresses being valid lock IDs.  Specifically:
	/// no storage is accessed by the API at the specified address - it is only
	/// used for lock identification.  Lock acquires may be self-nested and are
	/// unlocked by a corresponding number of releases.
	/// (These closely correspond to __itt_sync_acquired/__itt_sync_releasing,
	/// but may not have identical semantics.)
	#[link_name = "\u{1}___itt_model_lock_acquire"]
	pub fn __itt_model_lock_acquire(lock: *mut ::libc::c_void);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_acquire_2"]
	pub fn __itt_model_lock_acquire_2(lock: *mut ::libc::c_void);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_release"]
	pub fn __itt_model_lock_release(lock: *mut ::libc::c_void);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_release_2"]
	pub fn __itt_model_lock_release_2(lock: *mut ::libc::c_void);
}
pub type __itt_model_lock_acquire_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(lock: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_acquire_ptr__3_0"]
	pub static mut __itt_model_lock_acquire_ptr__3_0: __itt_model_lock_acquire_ptr__3_0_t;
}
pub type __itt_model_lock_acquire_2_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(lock: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_acquire_2_ptr__3_0"]
	pub static mut __itt_model_lock_acquire_2_ptr__3_0: __itt_model_lock_acquire_2_ptr__3_0_t;
}
pub type __itt_model_lock_release_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(lock: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_release_ptr__3_0"]
	pub static mut __itt_model_lock_release_ptr__3_0: __itt_model_lock_release_ptr__3_0_t;
}
pub type __itt_model_lock_release_2_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(lock: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_lock_release_2_ptr__3_0"]
	pub static mut __itt_model_lock_release_2_ptr__3_0: __itt_model_lock_release_2_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_RECORD_ALLOCATION/ANNOTATE_RECORD_DEALLOCATION support
	///
	/// record_allocation/deallocation describe user-defined memory allocator
	/// behavior, which may be required for correctness modeling to understand
	/// when storage is not expected to be actually reused across threads.
	#[link_name = "\u{1}___itt_model_record_allocation"]
	pub fn __itt_model_record_allocation(addr: *mut ::libc::c_void, size: usize);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_record_deallocation"]
	pub fn __itt_model_record_deallocation(addr: *mut ::libc::c_void);
}
pub type __itt_model_record_allocation_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_record_allocation_ptr__3_0"]
	pub static mut __itt_model_record_allocation_ptr__3_0: __itt_model_record_allocation_ptr__3_0_t;
}
pub type __itt_model_record_deallocation_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_record_deallocation_ptr__3_0"]
	pub static mut __itt_model_record_deallocation_ptr__3_0: __itt_model_record_deallocation_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_INDUCTION_USES support
	///
	/// Note particular storage is inductive through the end of the current site
	#[link_name = "\u{1}___itt_model_induction_uses"]
	pub fn __itt_model_induction_uses(addr: *mut ::libc::c_void, size: usize);
}
pub type __itt_model_induction_uses_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_induction_uses_ptr__3_0"]
	pub static mut __itt_model_induction_uses_ptr__3_0: __itt_model_induction_uses_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_REDUCTION_USES support
	///
	/// Note particular storage is used for reduction through the end
	/// of the current site
	#[link_name = "\u{1}___itt_model_reduction_uses"]
	pub fn __itt_model_reduction_uses(addr: *mut ::libc::c_void, size: usize);
}
pub type __itt_model_reduction_uses_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_reduction_uses_ptr__3_0"]
	pub static mut __itt_model_reduction_uses_ptr__3_0: __itt_model_reduction_uses_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_OBSERVE_USES support
	///
	/// Have correctness modeling record observations about uses of storage
	/// through the end of the current site
	#[link_name = "\u{1}___itt_model_observe_uses"]
	pub fn __itt_model_observe_uses(addr: *mut ::libc::c_void, size: usize);
}
pub type __itt_model_observe_uses_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void, size: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_observe_uses_ptr__3_0"]
	pub static mut __itt_model_observe_uses_ptr__3_0: __itt_model_observe_uses_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_CLEAR_USES support
	///
	/// Clear the special handling of a piece of storage related to induction,
	/// reduction or observe_uses
	#[link_name = "\u{1}___itt_model_clear_uses"]
	pub fn __itt_model_clear_uses(addr: *mut ::libc::c_void);
}
pub type __itt_model_clear_uses_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_clear_uses_ptr__3_0"]
	pub static mut __itt_model_clear_uses_ptr__3_0: __itt_model_clear_uses_ptr__3_0_t;
}
extern "C" {
	/// @brief ANNOTATE_DISABLE_*_PUSH/ANNOTATE_DISABLE_*_POP support
	///
	/// disable_push/disable_pop push and pop disabling based on a parameter.
	/// Disabling observations stops processing of memory references during
	/// correctness modeling, and all annotations that occur in the disabled
	/// region.  This allows description of code that is expected to be handled
	/// specially during conversion to parallelism or that is not recognized
	/// by tools (e.g. some kinds of synchronization operations.)
	/// This mechanism causes all annotations in the disabled region, other
	/// than disable_push and disable_pop, to be ignored.  (For example, this
	/// might validly be used to disable an entire parallel site and the contained
	/// tasks and locking in it for data collection purposes.)
	/// The disable for collection is a more expensive operation, but reduces
	/// collector overhead significantly.  This applies to BOTH correctness data
	/// collection and performance data collection.  For example, a site
	/// containing a task might only enable data collection for the first 10
	/// iterations.  Both performance and correctness data should reflect this,
	/// and the program should run as close to full speed as possible when
	/// collection is disabled.
	#[link_name = "\u{1}___itt_model_disable_push"]
	pub fn __itt_model_disable_push(x: __itt_model_disable);
}
extern "C" {
	#[link_name = "\u{1}___itt_model_disable_pop"]
	pub fn __itt_model_disable_pop();
}
extern "C" {
	#[link_name = "\u{1}___itt_model_aggregate_task"]
	pub fn __itt_model_aggregate_task(x: usize);
}
pub type __itt_model_disable_push_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(x: __itt_model_disable)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_disable_push_ptr__3_0"]
	pub static mut __itt_model_disable_push_ptr__3_0: __itt_model_disable_push_ptr__3_0_t;
}
pub type __itt_model_disable_pop_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_model_disable_pop_ptr__3_0"]
	pub static mut __itt_model_disable_pop_ptr__3_0: __itt_model_disable_pop_ptr__3_0_t;
}
pub type __itt_model_aggregate_task_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(x: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_model_aggregate_task_ptr__3_0"]
	pub static mut __itt_model_aggregate_task_ptr__3_0: __itt_model_aggregate_task_ptr__3_0_t;
}
/// @defgroup heap Heap
/// @ingroup public
/// Heap group
/// @{
pub type __itt_heap_function = *mut ::libc::c_void;
extern "C" {
	#[link_name = "\u{1}___itt_heap_function_create"]
	pub fn __itt_heap_function_create(name: *const ::libc::c_char, domain: *const ::libc::c_char) -> __itt_heap_function;
}
pub type __itt_heap_function_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, domain: *const ::libc::c_char) -> __itt_heap_function>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_function_create_ptr__3_0"]
	pub static mut __itt_heap_function_create_ptr__3_0: __itt_heap_function_create_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an allocation begin occurrence.
	#[link_name = "\u{1}___itt_heap_allocate_begin"]
	pub fn __itt_heap_allocate_begin(h: __itt_heap_function, size: usize, initialized: ::libc::c_int);
}
pub type __itt_heap_allocate_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, size: usize, initialized: ::libc::c_int)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_allocate_begin_ptr__3_0"]
	pub static mut __itt_heap_allocate_begin_ptr__3_0: __itt_heap_allocate_begin_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an allocation end occurrence.
	#[link_name = "\u{1}___itt_heap_allocate_end"]
	pub fn __itt_heap_allocate_end(h: __itt_heap_function, addr: *mut *mut ::libc::c_void, size: usize, initialized: ::libc::c_int);
}
pub type __itt_heap_allocate_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, addr: *mut *mut ::libc::c_void, size: usize, initialized: ::libc::c_int)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_allocate_end_ptr__3_0"]
	pub static mut __itt_heap_allocate_end_ptr__3_0: __itt_heap_allocate_end_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an free begin occurrence.
	#[link_name = "\u{1}___itt_heap_free_begin"]
	pub fn __itt_heap_free_begin(h: __itt_heap_function, addr: *mut ::libc::c_void);
}
pub type __itt_heap_free_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_free_begin_ptr__3_0"]
	pub static mut __itt_heap_free_begin_ptr__3_0: __itt_heap_free_begin_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an free end occurrence.
	#[link_name = "\u{1}___itt_heap_free_end"]
	pub fn __itt_heap_free_end(h: __itt_heap_function, addr: *mut ::libc::c_void);
}
pub type __itt_heap_free_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, addr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_free_end_ptr__3_0"]
	pub static mut __itt_heap_free_end_ptr__3_0: __itt_heap_free_end_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an reallocation begin occurrence.
	#[link_name = "\u{1}___itt_heap_reallocate_begin"]
	pub fn __itt_heap_reallocate_begin(h: __itt_heap_function, addr: *mut ::libc::c_void, new_size: usize, initialized: ::libc::c_int);
}
pub type __itt_heap_reallocate_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, addr: *mut ::libc::c_void, new_size: usize, initialized: ::libc::c_int)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_reallocate_begin_ptr__3_0"]
	pub static mut __itt_heap_reallocate_begin_ptr__3_0: __itt_heap_reallocate_begin_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an reallocation end occurrence.
	#[link_name = "\u{1}___itt_heap_reallocate_end"]
	pub fn __itt_heap_reallocate_end(h: __itt_heap_function, addr: *mut ::libc::c_void, new_addr: *mut *mut ::libc::c_void, new_size: usize, initialized: ::libc::c_int);
}
pub type __itt_heap_reallocate_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(h: __itt_heap_function, addr: *mut ::libc::c_void, new_addr: *mut *mut ::libc::c_void, new_size: usize, initialized: ::libc::c_int)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_reallocate_end_ptr__3_0"]
	pub static mut __itt_heap_reallocate_end_ptr__3_0: __itt_heap_reallocate_end_ptr__3_0_t;
}
extern "C" {
	/// @brief internal access begin
	#[link_name = "\u{1}___itt_heap_internal_access_begin"]
	pub fn __itt_heap_internal_access_begin();
}
pub type __itt_heap_internal_access_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_internal_access_begin_ptr__3_0"]
	pub static mut __itt_heap_internal_access_begin_ptr__3_0: __itt_heap_internal_access_begin_ptr__3_0_t;
}
extern "C" {
	/// @brief internal access end
	#[link_name = "\u{1}___itt_heap_internal_access_end"]
	pub fn __itt_heap_internal_access_end();
}
pub type __itt_heap_internal_access_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_internal_access_end_ptr__3_0"]
	pub static mut __itt_heap_internal_access_end_ptr__3_0: __itt_heap_internal_access_end_ptr__3_0_t;
}
extern "C" {
	/// @brief record memory growth begin
	#[link_name = "\u{1}___itt_heap_record_memory_growth_begin"]
	pub fn __itt_heap_record_memory_growth_begin();
}
pub type __itt_heap_record_memory_growth_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_record_memory_growth_begin_ptr__3_0"]
	pub static mut __itt_heap_record_memory_growth_begin_ptr__3_0: __itt_heap_record_memory_growth_begin_ptr__3_0_t;
}
extern "C" {
	/// @brief record memory growth end
	#[link_name = "\u{1}___itt_heap_record_memory_growth_end"]
	pub fn __itt_heap_record_memory_growth_end();
}
pub type __itt_heap_record_memory_growth_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_record_memory_growth_end_ptr__3_0"]
	pub static mut __itt_heap_record_memory_growth_end_ptr__3_0: __itt_heap_record_memory_growth_end_ptr__3_0_t;
}
extern "C" {
	/// @brief heap reset detection
	#[link_name = "\u{1}___itt_heap_reset_detection"]
	pub fn __itt_heap_reset_detection(reset_mask: ::libc::c_uint);
}
pub type __itt_heap_reset_detection_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(reset_mask: ::libc::c_uint)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_reset_detection_ptr__3_0"]
	pub static mut __itt_heap_reset_detection_ptr__3_0: __itt_heap_reset_detection_ptr__3_0_t;
}
extern "C" {
	/// @brief report
	#[link_name = "\u{1}___itt_heap_record"]
	pub fn __itt_heap_record(record_mask: ::libc::c_uint);
}
pub type __itt_heap_record_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(record_mask: ::libc::c_uint)>;
extern "C" {
	#[link_name = "\u{1}___itt_heap_record_ptr__3_0"]
	pub static mut __itt_heap_record_ptr__3_0: __itt_heap_record_ptr__3_0_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_domain
{
	/// < Zero if disabled, non-zero if enabled. The meaning of different non-zero values is reserved to the runtime
	pub flags: ::libc::c_int,
	/// < Copy of original name in ASCII.
	pub nameA: *const ::libc::c_char,
	pub nameW: *mut ::libc::c_void,
	/// < Reserved to the runtime
	pub extra1: ::libc::c_int,
	/// < Reserved to the runtime
	pub extra2: *mut ::libc::c_void,
	pub next: *mut ___itt_domain,
}
impl Default for ___itt_domain
{
	fn default() -> Self
	{
		unsafe { ::core::mem::zeroed() }
	}
}
pub type __itt_domain = ___itt_domain;
extern "C" {
	#[link_name = "\u{1}___itt_domain_create"]
	pub fn __itt_domain_create(name: *const ::libc::c_char) -> *mut __itt_domain;
}
pub type __itt_domain_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char) -> *mut __itt_domain>;
extern "C" {
	#[link_name = "\u{1}___itt_domain_create_ptr__3_0"]
	pub static mut __itt_domain_create_ptr__3_0: __itt_domain_create_ptr__3_0_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_id
{
	pub d1: ::libc::c_ulonglong,
	pub d2: ::libc::c_ulonglong,
	pub d3: ::libc::c_ulonglong,
}
pub type __itt_id = ___itt_id;
extern "C" {
	#[link_name = "\u{1}___itt_null"]
	pub static __itt_null: __itt_id;
}
extern "C" {
	/// @ingroup ids
	/// @brief Create an instance of identifier.
	/// This establishes the beginning of the lifetime of an instance of
	/// the given ID in the trace. Once this lifetime starts, the ID
	/// can be used to tag named entity instances in calls such as
	/// __itt_task_begin, and to specify relationships among
	/// identified named entity instances, using the \ref relations APIs.
	/// Instance IDs are not domain specific!
	/// @param[in] domain The domain controlling the execution of this call.
	/// @param[in] id The ID to create.
	#[link_name = "\u{1}___itt_id_create"]
	pub fn __itt_id_create(domain: *const __itt_domain, id: __itt_id);
}
pub type __itt_id_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_id_create_ptr__3_0"]
	pub static mut __itt_id_create_ptr__3_0: __itt_id_create_ptr__3_0_t;
}
extern "C" {
	/// @ingroup ids
	/// @brief Destroy an instance of identifier.
	/// This ends the lifetime of the current instance of the given ID value in the trace.
	/// Any relationships that are established after this lifetime ends are invalid.
	/// This call must be performed before the given ID value can be reused for a different
	/// named entity instance.
	/// @param[in] domain The domain controlling the execution of this call.
	/// @param[in] id The ID to destroy.
	#[link_name = "\u{1}___itt_id_destroy"]
	pub fn __itt_id_destroy(domain: *const __itt_domain, id: __itt_id);
}
pub type __itt_id_destroy_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_id_destroy_ptr__3_0"]
	pub static mut __itt_id_destroy_ptr__3_0: __itt_id_destroy_ptr__3_0_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_string_handle
{
	/// < Copy of original string in ASCII.
	pub strA: *const ::libc::c_char,
	pub strW: *mut ::libc::c_void,
	/// < Reserved. Must be zero
	pub extra1: ::libc::c_int,
	/// < Reserved. Must be zero
	pub extra2: *mut ::libc::c_void,
	pub next: *mut ___itt_string_handle,
}
impl Default for ___itt_string_handle
{
	fn default() -> Self
	{
		unsafe { ::core::mem::zeroed() }
	}
}
pub type __itt_string_handle = ___itt_string_handle;
extern "C" {
	#[link_name = "\u{1}___itt_string_handle_create"]
	pub fn __itt_string_handle_create(name: *const ::libc::c_char) -> *mut __itt_string_handle;
}
pub type __itt_string_handle_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char) -> *mut __itt_string_handle>;
extern "C" {
	#[link_name = "\u{1}___itt_string_handle_create_ptr__3_0"]
	pub static mut __itt_string_handle_create_ptr__3_0: __itt_string_handle_create_ptr__3_0_t;
}
/// @cond exclude_from_documentation
pub type __itt_timestamp = ::libc::c_ulonglong;
extern "C" {
	/// @ingroup timestamps
	/// @brief Return timestamp corresponding to the current moment.
	/// This returns the timestamp in the format that is the most relevant for the current
	/// host or platform (RDTSC, QPC, and others). You can use the "<" operator to
	/// compare __itt_timestamp values.
	#[link_name = "\u{1}___itt_get_timestamp"]
	pub fn __itt_get_timestamp() -> __itt_timestamp;
}
pub type __itt_get_timestamp_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn() -> __itt_timestamp>;
extern "C" {
	#[link_name = "\u{1}___itt_get_timestamp_ptr__3_0"]
	pub static mut __itt_get_timestamp_ptr__3_0: __itt_get_timestamp_ptr__3_0_t;
}
extern "C" {
	/// @defgroup regions Regions
	/// @ingroup public
	/// Regions group
	/// @{
	////**
	/// @ingroup regions
	/// @brief Begin of region instance.
	/// Successive calls to __itt_region_begin with the same ID are ignored
	/// until a call to __itt_region_end with the same ID
	/// @param[in] domain The domain for this region instance
	/// @param[in] id The instance ID for this region instance. Must not be __itt_null
	/// @param[in] parentid The instance ID for the parent of this region instance, or __itt_null
	/// @param[in] name The name of this region
	#[link_name = "\u{1}___itt_region_begin"]
	pub fn __itt_region_begin(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup regions
	/// @brief End of region instance.
	/// The first call to __itt_region_end with a given ID ends the
	/// region. Successive calls with the same ID are ignored, as are
	/// calls that do not have a matching __itt_region_begin call.
	/// @param[in] domain The domain for this region instance
	/// @param[in] id The instance ID for this region instance
	#[link_name = "\u{1}___itt_region_end"]
	pub fn __itt_region_end(domain: *const __itt_domain, id: __itt_id);
}
pub type __itt_region_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_region_begin_ptr__3_0"]
	pub static mut __itt_region_begin_ptr__3_0: __itt_region_begin_ptr__3_0_t;
}
pub type __itt_region_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_region_end_ptr__3_0"]
	pub static mut __itt_region_end_ptr__3_0: __itt_region_end_ptr__3_0_t;
}
extern "C" {
	/// @ingroup frames
	/// @brief Begin a frame instance.
	/// Successive calls to __itt_frame_begin with the
	/// same ID are ignored until a call to __itt_frame_end with the same ID.
	/// @param[in] domain The domain for this frame instance
	/// @param[in] id The instance ID for this frame instance or NULL
	#[link_name = "\u{1}___itt_frame_begin_v3"]
	pub fn __itt_frame_begin_v3(domain: *const __itt_domain, id: *mut __itt_id);
}
extern "C" {
	/// @ingroup frames
	/// @brief End a frame instance.
	/// The first call to __itt_frame_end with a given ID
	/// ends the frame. Successive calls with the same ID are ignored, as are
	/// calls that do not have a matching __itt_frame_begin call.
	/// @param[in] domain The domain for this frame instance
	/// @param[in] id The instance ID for this frame instance or NULL for current
	#[link_name = "\u{1}___itt_frame_end_v3"]
	pub fn __itt_frame_end_v3(domain: *const __itt_domain, id: *mut __itt_id);
}
extern "C" {
	/// @ingroup frames
	/// @brief Submits a frame instance.
	/// Successive calls to __itt_frame_begin or __itt_frame_submit with the
	/// same ID are ignored until a call to __itt_frame_end or __itt_frame_submit
	/// with the same ID.
	/// Passing special __itt_timestamp_none value as "end" argument means
	/// take the current timestamp as the end timestamp.
	/// @param[in] domain The domain for this frame instance
	/// @param[in] id The instance ID for this frame instance or NULL
	/// @param[in] begin Timestamp of the beginning of the frame
	/// @param[in] end Timestamp of the end of the frame
	#[link_name = "\u{1}___itt_frame_submit_v3"]
	pub fn __itt_frame_submit_v3(domain: *const __itt_domain, id: *mut __itt_id, begin: __itt_timestamp, end: __itt_timestamp);
}
pub type __itt_frame_begin_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: *mut __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_frame_begin_v3_ptr__3_0"]
	pub static mut __itt_frame_begin_v3_ptr__3_0: __itt_frame_begin_v3_ptr__3_0_t;
}
pub type __itt_frame_end_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: *mut __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_frame_end_v3_ptr__3_0"]
	pub static mut __itt_frame_end_v3_ptr__3_0: __itt_frame_end_v3_ptr__3_0_t;
}
pub type __itt_frame_submit_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: *mut __itt_id, begin: __itt_timestamp, end: __itt_timestamp)>;
extern "C" {
	#[link_name = "\u{1}___itt_frame_submit_v3_ptr__3_0"]
	pub static mut __itt_frame_submit_v3_ptr__3_0: __itt_frame_submit_v3_ptr__3_0_t;
}
extern "C" {
	/// @defgroup taskgroup Task Group
	/// @ingroup public
	/// Task Group
	/// @{
	////**
	/// @ingroup task_groups
	/// @brief Denotes a task_group instance.
	/// Successive calls to __itt_task_group with the same ID are ignored.
	/// @param[in] domain The domain for this task_group instance
	/// @param[in] id The instance ID for this task_group instance. Must not be __itt_null.
	/// @param[in] parentid The instance ID for the parent of this task_group instance, or __itt_null.
	/// @param[in] name The name of this task_group
	#[link_name = "\u{1}___itt_task_group"]
	pub fn __itt_task_group(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);
}
pub type __itt_task_group_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_group_ptr__3_0"]
	pub static mut __itt_task_group_ptr__3_0: __itt_task_group_ptr__3_0_t;
}
extern "C" {
	/// @ingroup tasks
	/// @brief Begin a task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] taskid The instance ID for this task instance, or __itt_null
	/// @param[in] parentid The parent instance to which this task instance belongs, or __itt_null
	/// @param[in] name The name of this task
	#[link_name = "\u{1}___itt_task_begin"]
	pub fn __itt_task_begin(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup tasks
	/// @brief Begin a task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] taskid The identifier for this task instance (may be 0)
	/// @param[in] parentid The parent of this task (may be 0)
	/// @param[in] fn The pointer to the function you are tracing
	#[link_name = "\u{1}___itt_task_begin_fn"]
	pub fn __itt_task_begin_fn(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, fn_: *mut ::libc::c_void);
}
extern "C" {
	/// @ingroup tasks
	/// @brief End the current task instance.
	/// @param[in] domain The domain for this task
	#[link_name = "\u{1}___itt_task_end"]
	pub fn __itt_task_end(domain: *const __itt_domain);
}
extern "C" {
	/// @ingroup tasks
	/// @brief Begin an overlapped task instance.
	/// @param[in] domain The domain for this task.
	/// @param[in] taskid The identifier for this task instance, *cannot* be __itt_null.
	/// @param[in] parentid The parent of this task, or __itt_null.
	/// @param[in] name The name of this task.
	#[link_name = "\u{1}___itt_task_begin_overlapped"]
	pub fn __itt_task_begin_overlapped(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup tasks
	/// @brief End an overlapped task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] taskid Explicit ID of finished task
	#[link_name = "\u{1}___itt_task_end_overlapped"]
	pub fn __itt_task_end_overlapped(domain: *const __itt_domain, taskid: __itt_id);
}
pub type __itt_task_begin_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_begin_ptr__3_0"]
	pub static mut __itt_task_begin_ptr__3_0: __itt_task_begin_ptr__3_0_t;
}
pub type __itt_task_begin_fn_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, parentid: __itt_id, fn_: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_begin_fn_ptr__3_0"]
	pub static mut __itt_task_begin_fn_ptr__3_0: __itt_task_begin_fn_ptr__3_0_t;
}
pub type __itt_task_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_end_ptr__3_0"]
	pub static mut __itt_task_end_ptr__3_0: __itt_task_end_ptr__3_0_t;
}
pub type __itt_task_begin_overlapped_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, taskid: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_begin_overlapped_ptr__3_0"]
	pub static mut __itt_task_begin_overlapped_ptr__3_0: __itt_task_begin_overlapped_ptr__3_0_t;
}
pub type __itt_task_end_overlapped_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, taskid: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_end_overlapped_ptr__3_0"]
	pub static mut __itt_task_end_overlapped_ptr__3_0: __itt_task_end_overlapped_ptr__3_0_t;
}
#[repr(u32)]
/// @brief Describes the scope of an event object in the trace.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_scope
{
	__itt_scope_unknown = 0,
	__itt_scope_global = 1,
	__itt_scope_track_group = 2,
	__itt_scope_track = 3,
	__itt_scope_task = 4,
	__itt_scope_marker = 5,
}
extern "C" {
	/// @ingroup markers
	/// @brief Create a marker instance
	/// @param[in] domain The domain for this marker
	/// @param[in] id The instance ID for this marker or __itt_null
	/// @param[in] name The name for this marker
	/// @param[in] scope The scope for this marker
	#[link_name = "\u{1}___itt_marker"]
	pub fn __itt_marker(domain: *const __itt_domain, id: __itt_id, name: *mut __itt_string_handle, scope: __itt_scope);
}
pub type __itt_marker_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, name: *mut __itt_string_handle, scope: __itt_scope)>;
extern "C" {
	#[link_name = "\u{1}___itt_marker_ptr__3_0"]
	pub static mut __itt_marker_ptr__3_0: __itt_marker_ptr__3_0_t;
}
#[repr(u32)]
/// @ingroup parameters
/// @brief describes the type of metadata
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_metadata_type
{
	__itt_metadata_unknown = 0,
	/// < Unsigned 64-bit integer
	__itt_metadata_u64 = 1,
	/// < Signed 64-bit integer
	__itt_metadata_s64 = 2,
	/// < Unsigned 32-bit integer
	__itt_metadata_u32 = 3,
	/// < Signed 32-bit integer
	__itt_metadata_s32 = 4,
	/// < Unsigned 16-bit integer
	__itt_metadata_u16 = 5,
	/// < Signed 16-bit integer
	__itt_metadata_s16 = 6,
	/// < Signed 32-bit floating-point
	__itt_metadata_float = 7,
	/// < SIgned 64-bit floating-point
	__itt_metadata_double = 8,
}
extern "C" {
	/// @ingroup parameters
	/// @brief Add metadata to an instance of a named entity.
	/// @param[in] domain The domain controlling the call
	/// @param[in] id The identifier of the instance to which the metadata is to be added, or __itt_null to add to the current task
	/// @param[in] key The name of the metadata
	/// @param[in] type The type of the metadata
	/// @param[in] count The number of elements of the given type. If count == 0, no metadata will be added.
	/// @param[in] data The metadata itself
	#[link_name = "\u{1}___itt_metadata_add"]
	pub fn __itt_metadata_add(domain: *const __itt_domain, id: __itt_id, key: *mut __itt_string_handle, type_: __itt_metadata_type, count: usize, data: *mut ::libc::c_void);
}
pub type __itt_metadata_add_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, key: *mut __itt_string_handle, type_: __itt_metadata_type, count: usize, data: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_metadata_add_ptr__3_0"]
	pub static mut __itt_metadata_add_ptr__3_0: __itt_metadata_add_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_metadata_str_add"]
	pub fn __itt_metadata_str_add(domain: *const __itt_domain, id: __itt_id, key: *mut __itt_string_handle, data: *const ::libc::c_char, length: usize);
}
pub type __itt_metadata_str_add_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, id: __itt_id, key: *mut __itt_string_handle, data: *const ::libc::c_char, length: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_metadata_str_add_ptr__3_0"]
	pub static mut __itt_metadata_str_add_ptr__3_0: __itt_metadata_str_add_ptr__3_0_t;
}
extern "C" {
	/// @ingroup parameters
	/// @brief Add metadata to an instance of a named entity.
	/// @param[in] domain The domain controlling the call
	/// @param[in] scope The scope of the instance to which the metadata is to be added
	///
	/// @param[in] id The identifier of the instance to which the metadata is to be added, or __itt_null to add to the current task
	///
	/// @param[in] key The name of the metadata
	/// @param[in] type The type of the metadata
	/// @param[in] count The number of elements of the given type. If count == 0, no metadata will be added.
	/// @param[in] data The metadata itself
	#[link_name = "\u{1}___itt_metadata_add_with_scope"]
	pub fn __itt_metadata_add_with_scope(domain: *const __itt_domain, scope: __itt_scope, key: *mut __itt_string_handle, type_: __itt_metadata_type, count: usize, data: *mut ::libc::c_void);
}
pub type __itt_metadata_add_with_scope_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, scope: __itt_scope, key: *mut __itt_string_handle, type_: __itt_metadata_type, count: usize, data: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_metadata_add_with_scope_ptr__3_0"]
	pub static mut __itt_metadata_add_with_scope_ptr__3_0: __itt_metadata_add_with_scope_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_metadata_str_add_with_scope"]
	pub fn __itt_metadata_str_add_with_scope(domain: *const __itt_domain, scope: __itt_scope, key: *mut __itt_string_handle, data: *const ::libc::c_char, length: usize);
}
pub type __itt_metadata_str_add_with_scope_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, scope: __itt_scope, key: *mut __itt_string_handle, data: *const ::libc::c_char, length: usize)>;
extern "C" {
	#[link_name = "\u{1}___itt_metadata_str_add_with_scope_ptr__3_0"]
	pub static mut __itt_metadata_str_add_with_scope_ptr__3_0: __itt_metadata_str_add_with_scope_ptr__3_0_t;
}
#[repr(u32)]
/// @ingroup relations
/// @brief The kind of relation between two instances is specified by the enumerated type __itt_relation.
/// Relations between instances can be added with an API call. The relation
/// API uses instance IDs. Relations can be added before or after the actual
/// instances are created and persist independently of the instances. This
/// is the motivation for having different lifetimes for instance IDs and
/// the actual instances.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_relation
{
	__itt_relation_is_unknown = 0,
	/// < "A is dependent on B" means that A cannot start until B completes
	__itt_relation_is_dependent_on = 1,
	/// < "A is sibling of B" means that A and B were created as a group
	__itt_relation_is_sibling_of = 2,
	/// < "A is parent of B" means that A created B
	__itt_relation_is_parent_of = 3,
	/// < "A is continuation of B" means that A assumes the dependencies of B
	__itt_relation_is_continuation_of = 4,
	/// < "A is child of B" means that A was created by B (inverse of is_parent_of)
	__itt_relation_is_child_of = 5,
	/// < "A is continued by B" means that B assumes the dependencies of A (inverse of is_continuation_of)
	__itt_relation_is_continued_by = 6,
	/// < "A is predecessor to B" means that B cannot start until A completes (inverse of is_dependent_on)
	__itt_relation_is_predecessor_to = 7,
}
extern "C" {
	/// @ingroup relations
	/// @brief Add a relation to the current task instance.
	/// The current task instance is the head of the relation.
	/// @param[in] domain The domain controlling this call
	/// @param[in] relation The kind of relation
	/// @param[in] tail The ID for the tail of the relation
	#[link_name = "\u{1}___itt_relation_add_to_current"]
	pub fn __itt_relation_add_to_current(domain: *const __itt_domain, relation: __itt_relation, tail: __itt_id);
}
extern "C" {
	/// @ingroup relations
	/// @brief Add a relation between two instance identifiers.
	/// @param[in] domain The domain controlling this call
	/// @param[in] head The ID for the head of the relation
	/// @param[in] relation The kind of relation
	/// @param[in] tail The ID for the tail of the relation
	#[link_name = "\u{1}___itt_relation_add"]
	pub fn __itt_relation_add(domain: *const __itt_domain, head: __itt_id, relation: __itt_relation, tail: __itt_id);
}
pub type __itt_relation_add_to_current_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, relation: __itt_relation, tail: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_relation_add_to_current_ptr__3_0"]
	pub static mut __itt_relation_add_to_current_ptr__3_0: __itt_relation_add_to_current_ptr__3_0_t;
}
pub type __itt_relation_add_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, head: __itt_id, relation: __itt_relation, tail: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_relation_add_ptr__3_0"]
	pub static mut __itt_relation_add_ptr__3_0: __itt_relation_add_ptr__3_0_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_clock_info
{
	/// < Clock domain frequency
	pub clock_freq: ::libc::c_ulonglong,
	/// < Clock domain base timestamp
	pub clock_base: ::libc::c_ulonglong,
}
pub type __itt_clock_info = ___itt_clock_info;
/// @cond exclude_from_documentation
pub type __itt_get_clock_info_fn = ::core::option::Option<unsafe extern "C" fn(clock_info: *mut __itt_clock_info, data: *mut ::libc::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_clock_domain
{
	/// < Most recent clock domain info
	pub info: __itt_clock_info,
	/// < Callback function pointer
	pub fn_: __itt_get_clock_info_fn,
	/// < Input argument for the callback function
	pub fn_data: *mut ::libc::c_void,
	/// < Reserved. Must be zero
	pub extra1: ::libc::c_int,
	/// < Reserved. Must be zero
	pub extra2: *mut ::libc::c_void,
	pub next: *mut ___itt_clock_domain,
}
impl Default for ___itt_clock_domain
{
	fn default() -> Self
	{
		unsafe { ::core::mem::zeroed() }
	}
}
pub type __itt_clock_domain = ___itt_clock_domain;
extern "C" {
	/// @ingroup clockdomains
	/// @brief Create a clock domain.
	/// Certain applications require the capability to trace their application using
	/// a clock domain different than the CPU, for instance the instrumentation of events
	/// that occur on a GPU.
	/// Because the set of domains is expected to be static over the application's execution time,
	/// there is no mechanism to destroy a domain.
	/// Any domain can be accessed by any thread in the process, regardless of which thread created
	/// the domain. This call is thread-safe.
	/// @param[in] fn A pointer to a callback function which retrieves alternative CPU timestamps
	/// @param[in] fn_data Argument for a callback function; may be NULL
	#[link_name = "\u{1}___itt_clock_domain_create"]
	pub fn __itt_clock_domain_create(fn_: __itt_get_clock_info_fn, fn_data: *mut ::libc::c_void) -> *mut __itt_clock_domain;
}
pub type __itt_clock_domain_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(fn_: __itt_get_clock_info_fn, fn_data: *mut ::libc::c_void) -> *mut __itt_clock_domain>;
extern "C" {
	#[link_name = "\u{1}___itt_clock_domain_create_ptr__3_0"]
	pub static mut __itt_clock_domain_create_ptr__3_0: __itt_clock_domain_create_ptr__3_0_t;
}
extern "C" {
	/// @ingroup clockdomains
	/// @brief Recalculate clock domains frequences and clock base timestamps.
	#[link_name = "\u{1}___itt_clock_domain_reset"]
	pub fn __itt_clock_domain_reset();
}
pub type __itt_clock_domain_reset_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_clock_domain_reset_ptr__3_0"]
	pub static mut __itt_clock_domain_reset_ptr__3_0: __itt_clock_domain_reset_ptr__3_0_t;
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Create an instance of identifier. This establishes the beginning of the lifetime of
	/// an instance of the given ID in the trace. Once this lifetime starts, the ID can be used to
	/// tag named entity instances in calls such as __itt_task_begin, and to specify relationships among
	/// identified named entity instances, using the \ref relations APIs.
	/// @param[in] domain The domain controlling the execution of this call.
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] id The ID to create.
	#[link_name = "\u{1}___itt_id_create_ex"]
	pub fn __itt_id_create_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id);
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Destroy an instance of identifier. This ends the lifetime of the current instance of the
	/// given ID value in the trace. Any relationships that are established after this lifetime ends are
	/// invalid. This call must be performed before the given ID value can be reused for a different
	/// named entity instance.
	/// @param[in] domain The domain controlling the execution of this call.
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] id The ID to destroy.
	#[link_name = "\u{1}___itt_id_destroy_ex"]
	pub fn __itt_id_destroy_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id);
}
pub type __itt_id_create_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_id_create_ex_ptr__3_0"]
	pub static mut __itt_id_create_ex_ptr__3_0: __itt_id_create_ex_ptr__3_0_t;
}
pub type __itt_id_destroy_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_id_destroy_ex_ptr__3_0"]
	pub static mut __itt_id_destroy_ex_ptr__3_0: __itt_id_destroy_ex_ptr__3_0_t;
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Begin a task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] taskid The instance ID for this task instance, or __itt_null
	/// @param[in] parentid The parent instance to which this task instance belongs, or __itt_null
	/// @param[in] name The name of this task
	#[link_name = "\u{1}___itt_task_begin_ex"]
	pub fn __itt_task_begin_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, taskid: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Begin a task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] taskid The identifier for this task instance, or __itt_null
	/// @param[in] parentid The parent of this task, or __itt_null
	/// @param[in] fn The pointer to the function you are tracing
	#[link_name = "\u{1}___itt_task_begin_fn_ex"]
	pub fn __itt_task_begin_fn_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, taskid: __itt_id, parentid: __itt_id, fn_: *mut ::libc::c_void);
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief End the current task instance.
	/// @param[in] domain The domain for this task
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	#[link_name = "\u{1}___itt_task_end_ex"]
	pub fn __itt_task_end_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong);
}
pub type __itt_task_begin_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id, parentid: __itt_id, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_begin_ex_ptr__3_0"]
	pub static mut __itt_task_begin_ex_ptr__3_0: __itt_task_begin_ex_ptr__3_0_t;
}
pub type __itt_task_begin_fn_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id, parentid: __itt_id, fn_: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_begin_fn_ex_ptr__3_0"]
	pub static mut __itt_task_begin_fn_ex_ptr__3_0: __itt_task_begin_fn_ex_ptr__3_0_t;
}
pub type __itt_task_end_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong)>;
extern "C" {
	#[link_name = "\u{1}___itt_task_end_ex_ptr__3_0"]
	pub static mut __itt_task_end_ex_ptr__3_0: __itt_task_end_ex_ptr__3_0_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___itt_counter
{
	_unused: [u8; 0],
}
/// @brief opaque structure for counter identification
////** @cond exclude_from_documentation
pub type __itt_counter = *mut ___itt_counter;
extern "C" {
	#[link_name = "\u{1}___itt_counter_create"]
	pub fn __itt_counter_create(name: *const ::libc::c_char, domain: *const ::libc::c_char) -> __itt_counter;
}
pub type __itt_counter_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, domain: *const ::libc::c_char) -> __itt_counter>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_create_ptr__3_0"]
	pub static mut __itt_counter_create_ptr__3_0: __itt_counter_create_ptr__3_0_t;
}
extern "C" {
	/// @brief Increment the unsigned 64 bits integer counter value
	///
	/// Calling this function to non-unsigned 64 bits integer counters has no effect
	#[link_name = "\u{1}___itt_counter_inc"]
	pub fn __itt_counter_inc(id: __itt_counter);
}
pub type __itt_counter_inc_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_inc_ptr__3_0"]
	pub static mut __itt_counter_inc_ptr__3_0: __itt_counter_inc_ptr__3_0_t;
}
extern "C" {
	/// @endcond */
	////**
	/// @brief Increment the unsigned 64 bits integer counter value with x
	///
	/// Calling this function to non-unsigned 64 bits integer counters has no effect
	#[link_name = "\u{1}___itt_counter_inc_delta"]
	pub fn __itt_counter_inc_delta(id: __itt_counter, value: ::libc::c_ulonglong);
}
pub type __itt_counter_inc_delta_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter, value: ::libc::c_ulonglong)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_inc_delta_ptr__3_0"]
	pub static mut __itt_counter_inc_delta_ptr__3_0: __itt_counter_inc_delta_ptr__3_0_t;
}
extern "C" {
	/// @brief Decrement the unsigned 64 bits integer counter value
	///
	/// Calling this function to non-unsigned 64 bits integer counters has no effect
	#[link_name = "\u{1}___itt_counter_dec"]
	pub fn __itt_counter_dec(id: __itt_counter);
}
pub type __itt_counter_dec_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_dec_ptr__3_0"]
	pub static mut __itt_counter_dec_ptr__3_0: __itt_counter_dec_ptr__3_0_t;
}
extern "C" {
	/// @endcond */
	////**
	/// @brief Decrement the unsigned 64 bits integer counter value with x
	///
	/// Calling this function to non-unsigned 64 bits integer counters has no effect
	#[link_name = "\u{1}___itt_counter_dec_delta"]
	pub fn __itt_counter_dec_delta(id: __itt_counter, value: ::libc::c_ulonglong);
}
pub type __itt_counter_dec_delta_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter, value: ::libc::c_ulonglong)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_dec_delta_ptr__3_0"]
	pub static mut __itt_counter_dec_delta_ptr__3_0: __itt_counter_dec_delta_ptr__3_0_t;
}
extern "C" {
	/// @ingroup counters
	/// @brief Increment a counter by one.
	/// The first call with a given name creates a counter by that name and sets its
	/// value to zero. Successive calls increment the counter value.
	/// @param[in] domain The domain controlling the call. Counter names are not domain specific.
	///            The domain argument is used only to enable or disable the API calls.
	/// @param[in] name The name of the counter
	#[link_name = "\u{1}___itt_counter_inc_v3"]
	pub fn __itt_counter_inc_v3(domain: *const __itt_domain, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup counters
	/// @brief Increment a counter by the value specified in delta.
	/// @param[in] domain The domain controlling the call. Counter names are not domain specific.
	///            The domain argument is used only to enable or disable the API calls.
	/// @param[in] name The name of the counter
	/// @param[in] delta The amount by which to increment the counter
	#[link_name = "\u{1}___itt_counter_inc_delta_v3"]
	pub fn __itt_counter_inc_delta_v3(domain: *const __itt_domain, name: *mut __itt_string_handle, delta: ::libc::c_ulonglong);
}
pub type __itt_counter_inc_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_inc_v3_ptr__3_0"]
	pub static mut __itt_counter_inc_v3_ptr__3_0: __itt_counter_inc_v3_ptr__3_0_t;
}
pub type __itt_counter_inc_delta_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, name: *mut __itt_string_handle, delta: ::libc::c_ulonglong)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_inc_delta_v3_ptr__3_0"]
	pub static mut __itt_counter_inc_delta_v3_ptr__3_0: __itt_counter_inc_delta_v3_ptr__3_0_t;
}
extern "C" {
	/// @ingroup counters
	/// @brief Decrement a counter by one.
	/// The first call with a given name creates a counter by that name and sets its
	/// value to zero. Successive calls decrement the counter value.
	/// @param[in] domain The domain controlling the call. Counter names are not domain specific.
	///            The domain argument is used only to enable or disable the API calls.
	/// @param[in] name The name of the counter
	#[link_name = "\u{1}___itt_counter_dec_v3"]
	pub fn __itt_counter_dec_v3(domain: *const __itt_domain, name: *mut __itt_string_handle);
}
extern "C" {
	/// @ingroup counters
	/// @brief Decrement a counter by the value specified in delta.
	/// @param[in] domain The domain controlling the call. Counter names are not domain specific.
	///            The domain argument is used only to enable or disable the API calls.
	/// @param[in] name The name of the counter
	/// @param[in] delta The amount by which to decrement the counter
	#[link_name = "\u{1}___itt_counter_dec_delta_v3"]
	pub fn __itt_counter_dec_delta_v3(domain: *const __itt_domain, name: *mut __itt_string_handle, delta: ::libc::c_ulonglong);
}
pub type __itt_counter_dec_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, name: *mut __itt_string_handle)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_dec_v3_ptr__3_0"]
	pub static mut __itt_counter_dec_v3_ptr__3_0: __itt_counter_dec_v3_ptr__3_0_t;
}
pub type __itt_counter_dec_delta_v3_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, name: *mut __itt_string_handle, delta: ::libc::c_ulonglong)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_dec_delta_v3_ptr__3_0"]
	pub static mut __itt_counter_dec_delta_v3_ptr__3_0: __itt_counter_dec_delta_v3_ptr__3_0_t;
}
extern "C" {
	/// @brief Set the counter value
	#[link_name = "\u{1}___itt_counter_set_value"]
	pub fn __itt_counter_set_value(id: __itt_counter, value_ptr: *mut ::libc::c_void);
}
pub type __itt_counter_set_value_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter, value_ptr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_set_value_ptr__3_0"]
	pub static mut __itt_counter_set_value_ptr__3_0: __itt_counter_set_value_ptr__3_0_t;
}
extern "C" {
	/// @brief Set the counter value
	#[link_name = "\u{1}___itt_counter_set_value_ex"]
	pub fn __itt_counter_set_value_ex(id: __itt_counter, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, value_ptr: *mut ::libc::c_void);
}
pub type __itt_counter_set_value_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, value_ptr: *mut ::libc::c_void)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_set_value_ex_ptr__3_0"]
	pub static mut __itt_counter_set_value_ex_ptr__3_0: __itt_counter_set_value_ex_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_counter_create_typed"]
	pub fn __itt_counter_create_typed(name: *const ::libc::c_char, domain: *const ::libc::c_char, type_: __itt_metadata_type) -> __itt_counter;
}
pub type __itt_counter_create_typed_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, domain: *const ::libc::c_char, type_: __itt_metadata_type) -> __itt_counter>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_create_typed_ptr__3_0"]
	pub static mut __itt_counter_create_typed_ptr__3_0: __itt_counter_create_typed_ptr__3_0_t;
}
extern "C" {
	/// @brief Destroy the counter identified by the pointer previously returned by __itt_counter_create() or
	/// __itt_counter_create_typed()
	#[link_name = "\u{1}___itt_counter_destroy"]
	pub fn __itt_counter_destroy(id: __itt_counter);
}
pub type __itt_counter_destroy_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(id: __itt_counter)>;
extern "C" {
	#[link_name = "\u{1}___itt_counter_destroy_ptr__3_0"]
	pub static mut __itt_counter_destroy_ptr__3_0: __itt_counter_destroy_ptr__3_0_t;
}
extern "C" {
	/// @ingroup markers
	/// @brief Create a marker instance.
	/// @param[in] domain The domain for this marker
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] id The instance ID for this marker, or __itt_null
	/// @param[in] name The name for this marker
	/// @param[in] scope The scope for this marker
	#[link_name = "\u{1}___itt_marker_ex"]
	pub fn __itt_marker_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id, name: *mut __itt_string_handle, scope: __itt_scope);
}
pub type __itt_marker_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, id: __itt_id, name: *mut __itt_string_handle, scope: __itt_scope)>;
extern "C" {
	#[link_name = "\u{1}___itt_marker_ex_ptr__3_0"]
	pub static mut __itt_marker_ex_ptr__3_0: __itt_marker_ex_ptr__3_0_t;
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Add a relation to the current task instance.
	/// The current task instance is the head of the relation.
	/// @param[in] domain The domain controlling this call
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] relation The kind of relation
	/// @param[in] tail The ID for the tail of the relation
	#[link_name = "\u{1}___itt_relation_add_to_current_ex"]
	pub fn __itt_relation_add_to_current_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, relation: __itt_relation, tail: __itt_id);
}
extern "C" {
	/// @ingroup clockdomain
	/// @brief Add a relation between two instance identifiers.
	/// @param[in] domain The domain controlling this call
	/// @param[in] clock_domain The clock domain controlling the execution of this call.
	/// @param[in] timestamp The user defined timestamp.
	/// @param[in] head The ID for the head of the relation
	/// @param[in] relation The kind of relation
	/// @param[in] tail The ID for the tail of the relation
	#[link_name = "\u{1}___itt_relation_add_ex"]
	pub fn __itt_relation_add_ex(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, head: __itt_id, relation: __itt_relation, tail: __itt_id);
}
pub type __itt_relation_add_to_current_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, relation: __itt_relation, tail: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_relation_add_to_current_ex_ptr__3_0"]
	pub static mut __itt_relation_add_to_current_ex_ptr__3_0: __itt_relation_add_to_current_ex_ptr__3_0_t;
}
pub type __itt_relation_add_ex_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(domain: *const __itt_domain, clock_domain: *mut __itt_clock_domain, timestamp: ::libc::c_ulonglong, head: __itt_id, relation: __itt_relation, tail: __itt_id)>;
extern "C" {
	#[link_name = "\u{1}___itt_relation_add_ex_ptr__3_0"]
	pub static mut __itt_relation_add_ex_ptr__3_0: __itt_relation_add_ex_ptr__3_0_t;
}
#[repr(u32)]
/// @cond exclude_from_documentation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ___itt_track_group_type
{
	__itt_track_group_type_normal = 0,
}
pub use self::___itt_track_group_type as __itt_track_group_type;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_track_group
{
	/// < Name of the track group
	pub name: *mut __itt_string_handle,
	/// < List of child tracks
	pub track: *mut ___itt_track,
	/// < Type of the track group
	pub tgtype: __itt_track_group_type,
	/// < Reserved. Must be zero
	pub extra1: ::libc::c_int,
	/// < Reserved. Must be zero
	pub extra2: *mut ::libc::c_void,
	pub next: *mut ___itt_track_group,
}
impl Default for ___itt_track_group
{
	fn default() -> Self
	{
		unsafe { ::core::mem::zeroed() }
	}
}
pub type __itt_track_group = ___itt_track_group;
#[repr(u32)]
/// @brief Placeholder for custom track types. Currently, "normal" custom track
/// is the only available track type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ___itt_track_type
{
	__itt_track_type_normal = 0,
}
pub use self::___itt_track_type as __itt_track_type;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ___itt_track
{
	/// < Name of the track group
	pub name: *mut __itt_string_handle,
	/// < Parent group to a track
	pub group: *mut __itt_track_group,
	/// < Type of the track
	pub ttype: __itt_track_type,
	/// < Reserved. Must be zero
	pub extra1: ::libc::c_int,
	/// < Reserved. Must be zero
	pub extra2: *mut ::libc::c_void,
	pub next: *mut ___itt_track,
}
impl Default for ___itt_track
{
	fn default() -> Self
	{
		unsafe { ::core::mem::zeroed() }
	}
}
pub type __itt_track = ___itt_track;
extern "C" {
	/// @brief Create logical track group.
	#[link_name = "\u{1}___itt_track_group_create"]
	pub fn __itt_track_group_create(name: *mut __itt_string_handle, track_group_type: __itt_track_group_type) -> *mut __itt_track_group;
}
pub type __itt_track_group_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *mut __itt_string_handle, track_group_type: __itt_track_group_type) -> *mut __itt_track_group>;
extern "C" {
	#[link_name = "\u{1}___itt_track_group_create_ptr__3_0"]
	pub static mut __itt_track_group_create_ptr__3_0: __itt_track_group_create_ptr__3_0_t;
}
extern "C" {
	/// @brief Create logical track.
	#[link_name = "\u{1}___itt_track_create"]
	pub fn __itt_track_create(track_group: *mut __itt_track_group, name: *mut __itt_string_handle, track_type: __itt_track_type) -> *mut __itt_track;
}
pub type __itt_track_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(track_group: *mut __itt_track_group, name: *mut __itt_string_handle, track_type: __itt_track_type) -> *mut __itt_track>;
extern "C" {
	#[link_name = "\u{1}___itt_track_create_ptr__3_0"]
	pub static mut __itt_track_create_ptr__3_0: __itt_track_create_ptr__3_0_t;
}
extern "C" {
	/// @brief Set the logical track.
	#[link_name = "\u{1}___itt_set_track"]
	pub fn __itt_set_track(track: *mut __itt_track);
}
pub type __itt_set_track_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(track: *mut __itt_track)>;
extern "C" {
	#[link_name = "\u{1}___itt_set_track_ptr__3_0"]
	pub static mut __itt_set_track_ptr__3_0: __itt_set_track_ptr__3_0_t;
}
/// @cond exclude_from_gpa_documentation */
////**
/// @defgroup events Events
/// @ingroup public
/// Events group
/// @{
////** @brief user event type
pub type __itt_event = ::libc::c_int;
extern "C" {
	#[link_name = "\u{1}___itt_event_create"]
	pub fn __itt_event_create(name: *const ::libc::c_char, namelen: ::libc::c_int) -> __itt_event;
}
pub type __itt_event_create_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(name: *const ::libc::c_char, namelen: ::libc::c_int) -> __itt_event>;
extern "C" {
	#[link_name = "\u{1}___itt_event_create_ptr__3_0"]
	pub static mut __itt_event_create_ptr__3_0: __itt_event_create_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an event occurrence.
	/// @return __itt_err upon failure (invalid event id/user event feature not enabled)
	#[link_name = "\u{1}___itt_event_start"]
	pub fn __itt_event_start(event: __itt_event) -> ::libc::c_int;
}
pub type __itt_event_start_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(event: __itt_event) -> ::libc::c_int>;
extern "C" {
	#[link_name = "\u{1}___itt_event_start_ptr__3_0"]
	pub static mut __itt_event_start_ptr__3_0: __itt_event_start_ptr__3_0_t;
}
extern "C" {
	/// @brief Record an event end occurrence.
	/// @note It is optional if events do not have durations.
	/// @return __itt_err upon failure (invalid event id/user event feature not enabled)
	#[link_name = "\u{1}___itt_event_end"]
	pub fn __itt_event_end(event: __itt_event) -> ::libc::c_int;
}
pub type __itt_event_end_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(event: __itt_event) -> ::libc::c_int>;
extern "C" {
	#[link_name = "\u{1}___itt_event_end_ptr__3_0"]
	pub static mut __itt_event_end_ptr__3_0: __itt_event_end_ptr__3_0_t;
}
impl __itt_av_data_type
{
	pub const __itt_e_char: __itt_av_data_type = __itt_av_data_type::__itt_e_first;
}
impl __itt_av_data_type
{
	pub const __itt_e_last: __itt_av_data_type = __itt_av_data_type::__itt_e_double;
}
#[repr(u32)]
/// @enum __itt_av_data_type
/// @brief Defines types of arrays data (for C/C++ intrinsic types)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum __itt_av_data_type
{
	__itt_e_first = 0,
	__itt_e_uchar = 1,
	__itt_e_int16 = 2,
	__itt_e_uint16 = 3,
	__itt_e_int32 = 4,
	__itt_e_uint32 = 5,
	__itt_e_int64 = 6,
	__itt_e_uint64 = 7,
	__itt_e_float = 8,
	__itt_e_double = 9,
}
extern "C" {
	#[link_name = "\u{1}___itt_av_save"]
	pub fn __itt_av_save(data: *mut ::libc::c_void, rank: ::libc::c_int, dimensions: *const ::libc::c_int, type_: ::libc::c_int, filePath: *const ::libc::c_char, columnOrder: ::libc::c_int) -> ::libc::c_int;
}
pub type __itt_av_save_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(data: *mut ::libc::c_void, rank: ::libc::c_int, dimensions: *const ::libc::c_int, type_: ::libc::c_int, filePath: *const ::libc::c_char, columnOrder: ::libc::c_int) -> ::libc::c_int>;
extern "C" {
	#[link_name = "\u{1}___itt_av_save_ptr__3_0"]
	pub static mut __itt_av_save_ptr__3_0: __itt_av_save_ptr__3_0_t;
}
extern "C" {
	/// @endcond
	#[link_name = "\u{1}___itt_enable_attach"]
	pub fn __itt_enable_attach();
}
pub type __itt_enable_attach_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn()>;
extern "C" {
	#[link_name = "\u{1}___itt_enable_attach_ptr__3_0"]
	pub static mut __itt_enable_attach_ptr__3_0: __itt_enable_attach_ptr__3_0_t;
}
extern "C" {
	#[link_name = "\u{1}___itt_module_load"]
	pub fn __itt_module_load(start_addr: *mut ::libc::c_void, end_addr: *mut ::libc::c_void, path: *const ::libc::c_char);
}
pub type __itt_module_load_ptr__3_0_t = ::core::option::Option<unsafe extern "C" fn(start_addr: *mut ::libc::c_void, end_addr: *mut ::libc::c_void, path: *const ::libc::c_char)>;
extern "C" {
	#[link_name = "\u{1}___itt_module_load_ptr__3_0"]
	pub static mut __itt_module_load_ptr__3_0: __itt_module_load_ptr__3_0_t;
}
