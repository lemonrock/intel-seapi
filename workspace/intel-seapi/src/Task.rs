// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// A region.
#[derive(Debug)]
pub struct Task(Domain);

impl Task
{
	/// Begins a task.
	#[inline(always)]
	pub fn begin<'b>(domain: Domain, name: StringHandle, parent: Option<IdentifierInstance<'b>>) -> Self
	{
		let parent = match parent
		{
			None => Identifier::Null,
			Some(parent) => parent.0.clone(),
		};

		unsafe { __itt_task_begin(instance_identifier.0.constant_pointer(), Identifier::Null, parent, name.0.mutable_pointer())  };

		Task(domain)
	}

	/// Ends a task.
	#[inline(always)]
	pub fn end(self) -> Domain
	{
		unsafe { __itt_task_end (self.0.constant_pointer()) };
		self.0
	}
}

