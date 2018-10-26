// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// Is JIT profiling active and sampling?
#[inline(always)]
pub fn is_jit_profiling_sampling() -> bool
{
	(unsafe { iJIT_IsProfilingActive() }) == iJIT_IsProfilingActiveFlags::iJIT_SAMPLING_ON
}
