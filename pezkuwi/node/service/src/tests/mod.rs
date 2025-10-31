// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Pezkuwi.

// Pezkuwi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Pezkuwi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Pezkuwi.  If not, see <http://www.gnu.org/licenses/>.

//! Pezkuwi Service Test Suite
//!
//! This module organizes all integration tests for the pezkuwi-service crate.

/// Relay chain selection and finality tests
/// Tests internal logic for chain selection, dispute handling, and approval voting
mod tests;

/// End-to-end integration tests
/// Tests chain spec loading, genesis building, and network configurations
mod e2e_tests;