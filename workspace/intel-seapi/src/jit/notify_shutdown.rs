// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Notify the JIT event listener agent that it should shutdown.
///
/// Returns true if shutdown succeeded.
#[inline(always)]
pub fn notify_shutdown(&self) -> bool
{
	let result = unsafe { iJIT_NotifyEvent(iJIT_JVM_EVENT::iJVM_EVENT_TYPE_SHUTDOWN, null_mut()) };
	match result
	{
		1 => true,
		0 => false,
		_ => panic!("Invalid result `{}` from iJIT_NotifyEvent(iJIT_JVM_EVENT::iJVM_EVENT_TYPE_SHUTDOWN, null_mut())", result)
	}
}