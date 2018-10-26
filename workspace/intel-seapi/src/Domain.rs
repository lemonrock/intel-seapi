// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Represents a 'Domain'.
///
/// Can never be destroyed or free'd.
///
/// Available to any thread, irrespective of which thread created it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Domain(NonNull<__itt_domain>);

impl Domain
{
	/// Name can be almost anything (although it must not contain ASCII NUL), but a URI or Java-like style of `com.my_company.my_application` is recommended by Intel.
	///
	/// Call is thread-safe.
	#[inline(always)]
	pub fn new(name: &str) -> Result<Self, ()>
	{
		let name = CString::new(name).unwrap();
		let inner = unsafe { __itt_domain_create(name.as_ptr()) };
		if inner.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Domain(unsafe { NonNull::new_unchecked(inner)}))
		}
	}

	/// Is collection of any statistics associated with this domain enabled?
	#[inline(always)]
	pub fn is_enabled(&self) -> bool
	{
		(unsafe { read_volatile(&self.0.as_ref().flags) }) == 1
	}

	/// Is collection of any statistics associated with this domain disabled?
	#[inline(always)]
	pub fn is_disabled(&self) -> bool
	{
		(unsafe { read_volatile(&self.0.as_ref().flags) }) == 0
	}

	/// Disable collection of any statistics associated with this domain.
	#[inline(always)]
	pub fn disable(&mut self)
	{
		unsafe { write_volatile(&mut self.0.as_mut().flags, 0) }
	}

	/// Enable collection of any statistics associated with this domain.
	///
	/// Domains, when created using `new()`, are already enabled.
	#[inline(always)]
	pub fn enable(&mut self)
	{
		unsafe { write_volatile(&mut self.0.as_mut().flags, 1) }
	}

	#[inline(always)]
	fn constant_pointer(&self) -> *const __itt_domain
	{
		self.0.as_ptr() as *const _
	}
}
