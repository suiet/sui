// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Operational configurations of a consensus authority.
///
/// All fields should tolerate inconsistencies among authorities, without affecting safety of the
/// protocol. Otherwise, they need to be part of Sui protocol config or epoch state on-chain.
///
/// NOTE: default values should make sense, so most operators should not need to specify any field.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameters {
    /// Time to wait for parent round leader before sealing a block.
    #[serde(default = "Parameters::default_leader_timeout")]
    pub leader_timeout: Duration,

    /// The database path. The path should be provided in order for the node to be able to boot
    pub db_path: Option<PathBuf>,
}

impl Parameters {
    pub fn default_leader_timeout() -> Duration {
        Duration::from_millis(250)
    }

    pub fn db_path_str_unsafe(&self) -> String {
        self.db_path
            .clone()
            .expect("DB path is not set")
            .as_path()
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            leader_timeout: Parameters::default_leader_timeout(),
            db_path: None,
        }
    }
}
