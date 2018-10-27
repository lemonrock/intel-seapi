// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Represents a 'String Handle'.
///
/// Can never be destroyed or free'd.
///
/// Available to any thread, irrespective of which thread created it.
///
/// Calling `new()` again with the same value of the `name` parameter will cause a reference to an already created string handle to be returned; this is useful for instantiating the same string handle on different threads without having to pass data between them.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringHandle(NonNull<__itt_string_handle >);

impl<'a> From<&'a str> for StringHandle
{
	#[inline(always)]
	fn from(name: &'a str) -> Self
	{
		Self::new(name)
	}
}

impl StringHandle
{
	/// Name can be almost anything (although it must not contain ASCII NUL), but a URI or Java-like style of `com.my_company.my_application` is recommended by Intel.
	///
	/// Call is thread-safe.
	///
	/// Calling more than once with the same `name` parameter will return a reference to the same string handle created for the first call.
	#[cfg(unix)]
	#[inline(always)]
	pub fn new(name: &str) -> Self
	{
		let name = CString::new(name).unwrap();
		let inner = unsafe { __itt_string_handle_create(name.as_ptr()) };
		assert!(!inner.is_null());
		StringHandle(unsafe { NonNull::new_unchecked(inner)})
	}

	/// Name can be almost anything (although it must not contain ASCII NUL), but a URI or Java-like style of `com.my_company.my_application` is recommended by Intel.
	///
	/// Call is thread-safe.
	///
	/// Calling more than once with the same `name` parameter will return a reference to the same string handle created for the first call.
	#[cfg(windows)]
	#[inline(always)]
	pub fn new(name: &str) -> Result<Self, ()>
	{
		let name = CString::new(name).unwrap();
		let inner = unsafe { __itt_string_handle_createA(name.as_ptr()) };
		if inner.is_null()
		{
			Err(())
		}
		else
		{
			Ok(StringHandle(unsafe { NonNull::new_unchecked(inner)}))
		}
	}

	#[inline(always)]
	fn mutable_pointer(self) -> *mut __itt_string_handle
	{
		self.0.as_ptr()
	}
}
