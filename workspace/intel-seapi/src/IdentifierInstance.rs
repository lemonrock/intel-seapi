// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// An identifier instance.
#[derive(Debug)]
pub struct IdentifierInstance<'a>(&'a Domain, Identifier<'a>);

impl<'a> IdentifierInstance<'a>
{
	/// Creates a new instance.
	///
	/// This establishes the beginning of the lifetime of an instance ofthe given ID in the trace.
	/// Once this lifetime starts, the ID can be used to tag named entity instances in calls such as `__itt_task_begin`, and to specify relationships among identified named entity instances, using the relations APIs.
	///
	/// Intel states "Instance IDs are not domain specific" (sic)!
	#[inline(always)]
	pub fn new(domain: &'a Domain, identifier: Identifier<'a>) -> Self
	{
		assert!(!identifier.is_null(), "Identifier is null-equivalent");

		unsafe { __itt_id_create(domain.constant_pointer(), identifier.0.clone()) };
		IdentifierInstance(domain, identifier)
	}

	/// Begins a frame instance.
	#[inline(always)]
	pub fn begin_frame(self) -> Frame<'a>
	{
		Frame::begin(Right(self))
	}

	/// Begins a region.
	#[inline(always)]
	pub fn begin_region<'b : 'a, 's>(self, name: &'s StringHandle, parent: Option<IdentifierInstance<'b>>) -> Region<'a>
	{
		Region::begin(self, name, parent)
	}

	/// Destroys instance, enabling re-use of the identifier.
	///
	/// This ends the lifetime of the current instance of the given ID value in the trace.
	/// Any relationships that are established after this lifetime ends are invalid.
	/// This call must be performed before the given ID value can be reused for a different named entity instance.
	#[inline(always)]
	pub fn destroy(self) -> Identifier<'a>
	{
		unsafe { __itt_id_destroy(self.0.constant_pointer(), (self.1).0.clone()) }
		self.1
	}
}
