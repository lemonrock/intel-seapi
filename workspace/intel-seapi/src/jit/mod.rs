// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


use super::*;
use ::std::ptr::null_mut;


include!("is_jit_profiling_inactive.rs");
include!("is_jit_profiling_sampling.rs");
include!("notify_method_load_finished.rs");
include!("notify_method_load_update.rs");
include!("notify_method_shutdown.rs");
include!("notify_method_unload_start.rs");
include!("unique_method_identifier.rs");
