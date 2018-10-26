// This file is part of intel-seapi. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT. No part of intel-seapi, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of intel-seapi. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/intel-seapi/master/COPYRIGHT.


#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![no_std]


//! #intel-seapi-sys
//!
//!
//! ## Overview.
//!
//! The ITT (Instrumentation and Tracing Technology) API is used to annotate a user's program with additional information that can be used by correctness and performance tools.
//! The user inserts calls in their program.
//! Those calls generate information that is collected at runtime, and used by Intel® Threading Tools.
//!
//!
//! ## API Concepts
//!
//! The following general concepts are used throughout the API.
//!
//!
//! ### Unicode Support
//!
//! Many API functions take character string arguments.
//! On Windows, there are two versions of each such function.
//! The function name is suffixed by W if Unicode support is enabled, and by A otherwise.
//! Any API function that takes a character string argument adheres to this convention.
//!
//!
//! ### Domains
//!
//! Domains provide a way to separate notification for different modules or libraries in a program.
//! Domains are specified by dotted character strings, eg `TBB.Internal.Control`.
//!
//! A mechanism (to be specified) is provided to enable and disable domains.
//! By default, all domains are enabled.
//!
//! Create a domain using some domain name: the URI naming style is recommended.
//! Because the set of domains is expected to be static over the application's execution time, there is no mechanism to destroy a domain.
//! Any domain can be accessed by any thread in the process, regardless of which thread created the domain. Creating a domain is thread-safe.
//!
//!
//! ### Named Entities and Instances
//!
//! Named entities (frames, regions, tasks, and markers) communicate information about the program to the analysis tools.
//! A named entity often refers to a section of program code, or to some set of logical concepts that the programmer wants to group together.
//! 
//! Named entities relate to the programmer's static view of the program.
//! When the program actually executes, many instances of a given named entity may be created.
//! 
//! The API annotations denote instances of named entities.
//! The actual named entities are displayed using the analysis tools.
//! In other words, the named entities come into existence when instances are created.
//! 
//! Instances of named entities may have instance identifiers (IDs).
//! Some API calls use instance identifiers to create relationships between
//! different instances of named entities.
//! Other API calls associate data with instances of named entities.
//! 
//! Some named entities must always have instance IDs.
//! In particular, regions and frames always have IDs.
//! Task and markers need IDs only if the ID is needed in another API call (such as adding a relation or metadata).
//! 
//! The lifetime of instance IDs is distinct from the lifetime of instances.
//! This allows various relationships to be specified separatefrom the actual execution of instances.
//! This flexibility comes at the expense of extra API calls.
//! 
//! The same ID may not be reused for different instances, unless a previous `__itt_id_destroy()` call for that ID has been issued.
//!
//!
//! ## Collection Control
//!
//! General behavior: application continues to run, but no profiling information is being collected
//!
//! Pausing occurs not only for the current thread but for all process as well as spawned processes.
//! It also possibly reduces runtime overhead.
//! Pausing also varies by collector application:-
//!
//!
//! ### Intel® Parallel Inspector and Intel® Inspector XE
//!
//! Does not analyze or report errors that involve memory access.
//!
//! Other errors are reported as usual.
//! Pausing data collection in Intel® Parallel Inspector and Intel® Inspector XE only pauses tracing and analyzing memory access.
//! It does not pause tracing or analyzing threading APIs.
//! 
//!
//! ### Intel® Parallel Amplifier and Intel® VTune™ Amplifier XE
//!
//! Does continue to record when new threads are started.
//!
//!
//! ## Modeling by Intel® Parallel Advisor
//! 
//! This is the subset of itt used for modeling by Intel® Parallel Advisor.
//! This API is called ONLY using `annotate.h`, by `Annotation" macros the user places in their sources during the parallelism modeling steps.
//! 
//! `site_begin` / `site_end` and `task_begin` / `task_end` take the address of handle variables, which are writeable by the API.
//! Handles must be 0 initialized prior to the first call to begin, or may cause a run-time failure.
//! The handles are initialized in a multi-thread safe way by the API if the handle is 0.
//! The commonly expected idiom is one static handle to identify a site or task.
//! If a site or task of the same name has already been started during this collection, the same handle MAY be returned, but is not required to be - it is unspecified if data merging is done based on name.
//! These routines also take an instance variable.
//! Like the lexical instance, these must be 0 initialized.
//! Unlike the lexical instance, this is used to track a single dynamic instance.
//! 
//! The API used by the Intel® Parallel Advisor allows one to describe potential concurrency and related activities. //! User-added source annotations expand to calls to these procedures to enable modeling of a hypothetical concurrent execution serially.
//!
//!
//! ###  `site_begin` / `site_end` model a potential concurrency site
//! 
//! site instances may be recursively nested with themselves.
//! `site_end` exits the most recently started but unended site for the current thread.
//! The handle passed to end may be used to validate structure
//! Instances of a site encountered on different threads concurrently are considered completely distinct.
//! If the site name for two different lexical sites match, it is unspecified whether they are treated as the same or different for data presentation.
//! 
//! 
//! ### `task_begin` / `task_end` model a potential task
//! 
//! The potential task is contained within the most closely enclosing dynamic site.
//! `task_end` exits the most recently started but unended task.
//! The handle passed to end may be used to validate structure.
//! It is unspecified if bad dynamic nesting is detected.
//! If it is, it should be encoded in the resulting data collection.
//! The collector should not fail due to construct nesting issues, nor attempt to directly indicate the problem.


extern crate core;
extern crate libc;



include!(concat!(env!("OUT_DIR"), "/ittnotify.bindgen.rs"));

include!(concat!(env!("OUT_DIR"), "/jitprofiling.bindgen.rs"));
