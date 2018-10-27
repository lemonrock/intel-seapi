// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


/// A region.
#[derive(Debug)]
pub struct Region<'a>(IdentifierInstance<'a>);

impl<'a> Region<'a>
{
	/// Begins a region.
	#[inline(always)]
	pub fn begin<'b : 'a, 's>(instance_identifier: IdentifierInstance<'a>, name: &'s StringHandle, parent: Option<IdentifierInstance<'b>>) -> Self
	{
		let parent = match parent
		{
			None => Identifier::Null.0.clone(),
			Some(parent) => (parent.1).0.clone(),
		};

		unsafe { __itt_region_begin(instance_identifier.0.constant_pointer(), (instance_identifier.1).0.clone(), parent, name.mutable_pointer())  };

		Region(instance_identifier)
	}

	/// Ends a region.
	#[inline(always)]
	pub fn end(self) -> IdentifierInstance<'a>
	{
		unsafe { __itt_region_end((self.0).0.constant_pointer(), ((self.0).1).0.clone()) };
		self.0
	}
}
