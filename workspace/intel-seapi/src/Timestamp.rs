// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Represents a 'Domain'.
///
/// Can never be destroyed or free'd.
///
/// Available to any thread, irrespective of which thread created it.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(__itt_timestamp);

impl Timestamp
{
	/// Equivalent to no timestamp.
	pub(crate) const None: Self = Timestamp(::std::u64::MAX as __itt_timestamp);

	/// Timestamp for the current moment.
	#[inline(always)]
	pub fn now() -> Self
	{
		Timestamp(unsafe { __itt_get_timestamp() })
	}
}
