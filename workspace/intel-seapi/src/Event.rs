// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// An event instance.
#[derive(Debug)]
pub struct Event(__itt_event);

impl Event
{
	/// Creates a new instance.
	#[cfg(unix)]
	#[inline(always)]
	pub fn new(name: &str) -> Self
	{
		Event(unsafe { __itt_event_create(name.as_bytes().as_ptr() as *const i8, name.len() as i32) })
	}

	/// Creates a new instance.
	#[cfg(windows)]
	#[inline(always)]
	pub fn new(name: &str) -> Self
	{
		Event(unsafe { __itt_event_createA(name.as_bytes().as_ptr() as *const i8, name.len() as i32) })
	}

	/// A ping event just occurs again and again, but has no associated time it occupies.
	#[inline(always)]
	pub fn ping(&self)
	{
		unsafe { __itt_event_start(self.0) };
	}

	/// A timed event has a time it starts and a time it ends.
	///
	/// If the result of this call is dropped, the
	#[inline(always)]
	pub fn time<'a>(&'a mut self) -> StartedEvent<'a>
	{
		unsafe { __itt_event_start(self.0) };
		StartedEvent(self.0, false, PhantomData)
	}
}
