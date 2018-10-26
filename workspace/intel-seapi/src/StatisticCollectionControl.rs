// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// API for controlling overall statistics collection.
///
/// NOTE: Do not call `pause()` and `resume()` close together; for small workloads, use frames.
pub struct StatisticCollectionControl;

impl StatisticCollectionControl
{
	/// Pauses collection of statistics.
	#[inline(always)]
	pub fn pause()
	{
		unsafe { __itt_pause() }
	}

	/// Resumes collection of statistics after pause (or after application started running).
	#[inline(always)]
	pub fn resume()
	{
		unsafe { __itt_resume() }
	}

	/// Permanently stops collection of statistics.
	#[inline(always)]
	pub fn detach()
	{
		unsafe { __itt_detach() }
	}

	/// No statistics will be collected for the current thread.
	#[inline(always)]
	pub fn ignore_current_thread()
	{
		unsafe { __itt_thread_ignore() }
	}

	/// Applies a name to the current thread.
	///
	/// Must not contain ASCII NUL.
	#[cfg(unix)]
	#[inline(always)]
	pub fn name_current_thread(name: &str)
	{
		let name = CString::new(name).unwrap();
		unsafe { __itt_thread_set_name(name.as_ptr()) }
	}

	/// Applies a name to the current thread.
	///
	/// Must not contain ASCII NUL.
	#[cfg(windows)]
	#[inline(always)]
	pub fn name_current_thread(name: &str)
	{
		let name = CString::new(name).unwrap();
		unsafe { __itt_thread_set_nameA(name.as_ptr()) }
	}
}
