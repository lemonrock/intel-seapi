// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![no_std]


//! #intel-seapi
//!
//!
//! ## Overview.
//!
//! The ITT (Instrumentation and Tracing Technology) API is used to annotate a user's program with additional information that can be used by correctness and performance tools.
//! The user inserts calls in their program.
//! Those calls generate information that is collected at runtime, and used by Intel® Threading Tools.


extern crate core;
extern crate libc;
extern crate intel_seapi_sys;


/// Wrappers to make it easier to using the JIT event listener agent.
pub mod jit;
