// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


use super::*;


#[test]
pub fn confirm_linking_has_worked()
{
	StatisticCollectionControl::pause();

	let domain = Domain::new("HelloWorld").unwrap();

	let string_handle = StringHandle::new("StringHandle").unwrap();

	let task = Task::begin(domain, string_handle, None);

	task.end();

	let _timestamp = Timestamp::now();

	let mut event = Event::new("Event");

	event.ping();

	let started = event.time();

	started.stop();
}
