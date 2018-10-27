// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// A frame.
///
/// Frames are similar to regions, but are intended to be easier to use.
///
/// In particular:-
///
/// * Frames always represent periods of elapsed time;
/// * By default, frames have no nesting relationships.
#[derive(Debug)]
pub struct Frame<'a>(Either<&'a Domain, IdentifierInstance<'a>>);

impl<'a> Frame<'a>
{
	/// Begins a frame instance.
	#[inline(always)]
	pub fn begin(on: Either<&'a Domain, IdentifierInstance<'a>>) -> Self
	{
		match on
		{
			Left(domain) => unsafe { __itt_frame_begin_v3(domain.constant_pointer(), null_mut()) },
			Right(ref identifier_instance) => unsafe { __itt_frame_begin_v3(identifier_instance.0.constant_pointer(), &identifier_instance.1 as *const _ as *mut _) },
		}

		Frame(on)
	}

	/// Submits a frame instance.
	///
	/// `end` must not be before `before`.
	#[inline(always)]
	pub fn submit(&self, begin: Timestamp, end: Timestamp)
	{
		debug_assert!(begin >= end, "end can not be before begin");

		match self.0
		{
			Left(domain) => unsafe { __itt_frame_submit_v3(domain.constant_pointer(), null_mut(), begin.0, end.0) },
			Right(ref identifier_instance) => unsafe { __itt_frame_submit_v3(identifier_instance.0.constant_pointer(), &identifier_instance.1 as *const _ as *mut _, begin.0, end.0) },
		}
	}

	/// Submits a frame instance, with an end timestamp of `now`.
	#[inline(always)]
	pub fn submit_ending_now(&self, begin: Timestamp)
	{
		self.submit(begin, Timestamp::None)
	}

	/// Ends a frame instance.
	#[inline(always)]
	pub fn end(self) -> Either<&'a Domain, IdentifierInstance<'a>>
	{
		let on = self.0;

		match on
		{
			Left(domain) => unsafe { __itt_frame_end_v3(domain.constant_pointer(), null_mut()) },
			Right(ref identifier_instance) => unsafe { __itt_frame_end_v3(identifier_instance.0.constant_pointer(), &identifier_instance.1 as *const _ as *mut _) },
		}

		on
	}
}
