// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Notify the JIT event listener agent that a JIT'd method has been created but not yet executed.
#[inline(always)]
pub fn notify_method_load_finished(event_data: &mut iJIT_Method_Load_V3)
{
	unsafe { iJIT_NotifyEvent(iJIT_JVM_EVENT::iJVM_EVENT_TYPE_METHOD_LOAD_FINISHED_V3, event_data as *mut iJIT_Method_Load_V3 as *mut _) };
}