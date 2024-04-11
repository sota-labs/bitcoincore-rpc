// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Client for Bitcoin Core API
//!
//! This is a client library for the Bitcoin Core JSON-RPC API.
//!

#![crate_name = "bitcoincore_rpc_sotatek"]
#![crate_type = "rlib"]

extern crate log;
extern crate serde;

pub extern crate jsonrpc;

pub extern crate bitcoincore_rpc_json;
pub use crate::json::bitcoin;
pub use bitcoincore_rpc_json as json;

mod client;

pub use crate::client::*;
