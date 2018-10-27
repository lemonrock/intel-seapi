// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// An identifier.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(__itt_id, PhantomData<&'a ()>);

impl<'a> Identifier<'a>
{
	/// A 'Null' identifer.
	pub const Null: Self = Identifier
	(
		__itt_id
		{
			d1: 0,
			d2: 0,
			d3: 0,
		},
		PhantomData,
	);

	/// Is this a null identifier?
	#[inline(always)]
	pub fn is_null(&self) -> bool
	{
		self == &Self::Null
	}

	/// Creates a new identifier.
	#[inline(always)]
	pub fn new<T: 'a>(address: &T, extra: u64) -> Self
	{
		Identifier
		(
			__itt_id
			{
				d1: address as *const T as usize as u64,
				d2: extra,
				d3: 0,
			},
			PhantomData
		)
	}

	/// Creates a new identifier.
	#[inline(always)]
	pub fn wrap<T: 'a>(address: &T) -> Self
	{
		Self::new(address, 0)
	}

	/// Creates a new identifier.
	#[inline(always)]
	pub fn new_without_lifetime<T>(address: NonNull<T>, extra: u64) -> Self
	{
		Identifier
		(
			__itt_id
			{
				d1: address.as_ptr() as usize as u64,
				d2: extra,
				d3: 0,
			},
			PhantomData
		)
	}

	/// Creates a new identifier.
	#[inline(always)]
	pub fn wrap_without_lifetime<T>(address: NonNull<T>) -> Self
	{
		Self::new_without_lifetime(address, 0)
	}
}
