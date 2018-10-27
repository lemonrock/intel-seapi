// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]


//! #intel-seapi
//!
//!
//! ## Overview.
//!
//! The ITT (Instrumentation and Tracing Technology) API is used to annotate a user's program with additional information that can be used by correctness and performance tools.
//! The user inserts calls in their program.
//! Those calls generate information that is collected at runtime, and used by Intel® Threading Tools.
//!
//! It is easiest to work with the `Event` and `Task` concepts, and control collection using `StatisticCollectionControl`.
//!
//! Note that no wrappers are currently provided for Counters, Heap Allocation and Thread Synchronization.


extern crate either;
extern crate intel_seapi_sys;


pub use intel_seapi_sys::iJIT_CodeArchitecture;
pub use intel_seapi_sys::iJIT_Method_Load_V3;
pub use intel_seapi_sys::iJIT_Method_Load;


use ::either::*;
use ::intel_seapi_sys::*;
use ::std::ffi::CString;
use ::std::marker::PhantomData;
use ::std::ptr::null_mut;
use ::std::ptr::NonNull;
use ::std::ptr::read_volatile;
use ::std::ptr::write_volatile;


/// Wrappers to make it easier to using the JIT event listener agent.
pub mod jit;

#[cfg(test)] pub(crate) mod tests;


include!("Domain.rs");
include!("Event.rs");
include!("Frame.rs");
include!("Identifier.rs");
include!("IdentifierInstance.rs");
include!("Region.rs");
include!("StartedEvent.rs");
include!("StatisticCollectionControl.rs");
include!("StringHandle.rs");
include!("Task.rs");
include!("Timestamp.rs");

/// Wraps up creation of a static mut Domain field.
#[macro_export]
macro_rules! domain
{
	($name: ident) =>
	{
		unsafe
		{
			static mut $name: Option<$crate::intel_seapi::Domain> = None;

			if $name.is_none()
			{
				$name = Some(stringify!($name).into())
			}
			$name.as_ref().unwrap()
		}
	}
}

/// Wraps up creation of a static mut Event field.
#[macro_export]
macro_rules! event
{
	($name: ident) =>
	{
		unsafe
		{
			static mut $name: Option<$crate::intel_seapi::Event> = None;

			if $name.is_none()
			{
				$name = Some(stringify!($name).into())
			}
			$name.as_mut().unwrap()
		}
	}
}

/// Wraps up creation of a static mut StringHandle field.
#[macro_export]
macro_rules! string_handle
{
	($name: ident) =>
	{
		unsafe
		{
			static mut $name: Option<$crate::intel_seapi::StringHandle> = None;

			if $name.is_none()
			{
				$name = Some(stringify!($name).into())
			}
			$name.as_ref().unwrap()
		}
	}
}
