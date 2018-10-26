// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// A started timed event.
#[derive(Debug)]
pub struct StartedEvent<'a>(__itt_event, bool, PhantomData<&'a mut ()>);

impl<'a> Drop for StartedEvent<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.1
		{
			return;
		}
		unsafe { __itt_event_end(self.0) };
	}
}

impl<'a> StartedEvent<'a>
{
	/// A timed event has a time it starts and a time it ends.
	#[inline(always)]
	pub fn stop(mut self)
	{
		if self.1
		{
			return
		}
		unsafe { __itt_event_end(self.0) };
		self.1 = true
	}
}
