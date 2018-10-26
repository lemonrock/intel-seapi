// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// A started timed event.
#[derive(Debug)]
pub struct StartedEvent(__itt_event, bool);

impl Drop for StartedEvent
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

impl StartedEvent
{
	/// A timed event has a time it starts and a time it ends.
	#[inline(always)]
	pub fn stop(mut self) -> Event
	{
		if self.1
		{
			return Event(self.0)
		}
		unsafe { __itt_event_end(self.0) };
		self.1 = true;
		Event(self.0)
	}
}
