// This file is part of Bizinikiwi.

// Copyright (C) Parity Technologies (UK) Ltd. and Dijital Kurdistan Tech Institute
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(deprecated, clippy::deprecated_semver)]

use super::{pezframe_system, Block};
use crate::derive_impl;

#[crate::pezpallet(dev_mode)]
mod pezpallet_basic {
	use super::pezframe_system;

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T>(_);

	#[pezpallet::config]
	pub trait Config: pezframe_system::Config {}
}

impl pezpallet_basic::Config for Runtime {}

#[crate::pezpallet(dev_mode)]
mod pezpallet_with_disabled_call {
	use super::pezframe_system;

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T>(_);

	#[pezpallet::config]
	pub trait Config: pezframe_system::Config {}
}

impl pezpallet_with_disabled_call::Config for Runtime {}

#[crate::pezpallet(dev_mode)]
mod pezpallet_with_disabled_unsigned {
	use super::pezframe_system;

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T>(_);

	#[pezpallet::config]
	pub trait Config: pezframe_system::Config {}
}

impl pezpallet_with_disabled_unsigned::Config for Runtime {}

#[crate::pezpallet]
mod pezpallet_with_instance {
	use super::pezframe_system;

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T, I = ()>(_);

	#[pezpallet::config]
	pub trait Config<I: 'static = ()>: pezframe_system::Config {}
}

#[allow(unused)]
type Instance1 = pezpallet_with_instance::Pezpallet<pezpallet_with_instance::Instance1>;

impl pezpallet_with_instance::Config<pezpallet_with_instance::Instance1> for Runtime {}

#[allow(unused)]
type Instance2 = pezpallet_with_instance::Pezpallet<pezpallet_with_instance::Instance2>;

impl pezpallet_with_instance::Config<pezpallet_with_instance::Instance2> for Runtime {}

#[derive_impl(pezframe_system::config_preludes::TestDefaultConfig)]
impl pezframe_system::Config for Runtime {
	type Block = Block;
}

#[docify::export(runtime_macro)]
#[crate::runtime]
mod runtime {
	// The main runtime
	#[runtime::runtime]
	// Runtime Types to be generated
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Runtime;

	// Use the concrete pezpallet type
	#[runtime::pezpallet_index(0)]
	pub type System = pezframe_system::Pezpallet<Runtime>;

	// Use path to the pezpallet
	#[runtime::pezpallet_index(1)]
	pub type Basic = pezpallet_basic;

	// Use the concrete pezpallet type with instance
	#[runtime::pezpallet_index(2)]
	pub type PalletWithInstance1 = pezpallet_with_instance::Pezpallet<Runtime, Instance1>;

	// Use path to the pezpallet with instance
	#[runtime::pezpallet_index(3)]
	pub type PalletWithInstance2 = pezpallet_with_instance<Instance2>;

	// Ensure that the runtime does not export the calls from the pezpallet
	#[runtime::pezpallet_index(4)]
	#[runtime::disable_call]
	#[deprecated = "example"]
	pub type PalletWithDisabledCall = pezpallet_with_disabled_call::Pezpallet<Runtime>;

	// Ensure that the runtime does not export the unsigned calls from the pezpallet
	#[runtime::pezpallet_index(5)]
	#[runtime::disable_unsigned]
	pub type PalletWithDisabledUnsigned = pezpallet_with_disabled_unsigned::Pezpallet<Runtime>;
}
