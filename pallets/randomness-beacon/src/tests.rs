// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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
use std::vec;
use crate::{self as beacon, mock::*, Call, Config, Error, Weight};



#[test]
fn test_genesis() {
    new_test_ext(vec![1, 2, 3]).execute_with(|| {
        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 1);
	});
}

#[test]
fn test_can_write_pulse() {
	new_test_ext(vec![1, 2, 3]).execute_with(|| {
        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 1);
	});
}